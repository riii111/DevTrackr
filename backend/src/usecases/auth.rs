use crate::errors::app_error::AppError;
use crate::models::auth::AuthTokenInDB;
use crate::models::users::{UserCreate, UserInDB, UserUpdate};
use crate::repositories::auth::AuthRepository;
use crate::services::s3_service::S3Service;
use crate::utils::jwt;
use crate::utils::jwt::Claims;
use crate::utils::password::{hash_password, verify_password};
use bson::DateTime as BsonDateTime;
use chrono::{Duration, Utc};
use std::env;
use std::sync::Arc;

pub struct AuthUseCase<R: AuthRepository> {
    repository: Arc<R>,
    jwt_secret: Vec<u8>,
    s3_service: Arc<S3Service>,
}

impl<R: AuthRepository> AuthUseCase<R> {
    pub fn new(repository: Arc<R>, jwt_secret: &[u8], s3_service: Arc<S3Service>) -> Self {
        Self {
            repository,
            jwt_secret: jwt_secret.to_vec(),
            s3_service,
        }
    }

    /// ユーザーログイン処理
    ///
    /// - メールアドレスでユーザーを検索
    /// - パスワードを検証
    /// - 認証トークンを生成して保存
    pub async fn login(&self, email: &str, password: &str) -> Result<AuthTokenInDB, AppError> {
        let user = self
            .repository
            .find_user_by_email(email)
            .await?
            .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

        if verify_password(password, &user.password_hash) {
            let auth_token = self.create_auth_token(&user.id.unwrap().to_string())?;
            self.repository.save_auth_token(&auth_token).await?;
            Ok(auth_token)
        } else {
            Err(AppError::Forbidden("無効な認証情報です".to_string()))
        }
    }

    /// ユーザー登録処理
    ///
    /// - パスワードをハッシュ化
    /// - ユーザーを作成
    /// - 認証トークンを生成して保存
    pub async fn register(&self, user_create: &UserCreate) -> Result<AuthTokenInDB, AppError> {
        let password_hash = hash_password(&user_create.password)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let user_id = self
            .repository
            .create_user(&user_create.email, &password_hash, &user_create.username)
            .await?;

        let auth_token = self.create_auth_token(&user_id.to_string())?;
        self.repository.save_auth_token(&auth_token).await?;
        Ok(auth_token)
    }

    /// ログイン中のユーザー更新処理
    pub async fn update_me(
        &self,
        access_token: &str,
        user_update: &mut UserUpdate,
    ) -> Result<UserInDB, AppError> {
        // 画像のアップロード処理
        if let Some(avatar_url) = &user_update.avatar_url {
            let new_avatar_url = self.s3_service.upload_avatar(avatar_url).await?;
            user_update.avatar_url = Some(new_avatar_url);
        }

        // パスワードのハッシュ化
        let password_hash = hash_password(&user_update.password)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        user_update.password = password_hash;

        // ユーザー情報を更新
        let updated = self
            .repository
            .update_user_by_access_token(access_token, user_update)
            .await?;

        if updated {
            // 更新されたユーザー情報を取得
            self.get_current_user(access_token).await
        } else {
            Err(AppError::NotFound("ユーザーが見つかりません".to_string()))
        }
    }

    /// ログイン中のユーザー情報を取得
    pub async fn get_current_user(&self, access_token: &str) -> Result<UserInDB, AppError> {
        // アクセストークンからユーザー情報を直接取得
        let user = self
            .repository
            .find_user_by_access_token(access_token)
            .await?
            .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

        Ok(user)
    }

    /// ユーザーログアウト処理
    ///
    /// - アクセストークンとリフレッシュトークンを削除
    pub async fn logout(&self, auth_header: &str) -> Result<(), AppError> {
        let token = self.extract_token(auth_header);

        // アクセストークンをキーに削除
        let result = self.repository.delete_auth_tokens(&token).await?;

        if result {
            Ok(())
        } else {
            Err(AppError::NotFound("トークンが見つかりません".to_string()))
        }
    }

    /// アクセストークンの有効期限を検証
    pub async fn verify_access_token(&self, access_token: &str) -> Result<Claims, AppError> {
        let claims = jwt::verify_token(access_token, &self.jwt_secret)
            .map_err(|_| AppError::Unauthorized("無効なアクセストークンです".to_string()))?;

        // DBからトークンを取得
        let auth_token = self
            .repository
            .find_auth_token(access_token)
            .await
            .map_err(|_| {
                AppError::InternalServerError("アクセストークンの検証に失敗しました".to_string())
            })?
            .ok_or_else(|| {
                AppError::Unauthorized("アクセストークンが見つかりません".to_string())
            })?;

        // 現在時刻と有効期限を比較
        if Utc::now() > auth_token.expires_at.into() {
            return Err(AppError::Unauthorized(
                "アクセストークンの有効期限が切れています".to_string(),
            ));
        }

        Ok(claims)
    }

    /// リフレッシュトークンの有効期限を検証
    pub async fn verify_refresh_token(&self, refresh_token: &str) -> Result<Claims, AppError> {
        let claims = jwt::verify_token(refresh_token, &self.jwt_secret)
            .map_err(|_| AppError::BadRequest("無効なリフレッシュトークンです".to_string()))?;

        log::info!("refresh_token: {}", refresh_token);
        // DBからリフレッシュトークンを取得
        let auth_token = self
            .repository
            .find_by_refresh_token(refresh_token)
            .await
            .map_err(|e| {
                log::error!(
                    "リフレッシュトークンの検証中にエラーが発生しました: {:?}",
                    e
                );
                AppError::InternalServerError(
                    "リフレッシュトークンの検証に失敗しました".to_string(),
                )
            })?
            .ok_or_else(|| {
                AppError::BadRequest("リフレッシュトークンが見つかりません".to_string())
            })?;

        // リフレッシュトークンの有効期限を比較
        if Utc::now() > auth_token.refresh_expires_at.into() {
            return Err(AppError::BadRequest(
                "リフレッシュトークンの有効期限が切れています".to_string(),
            ));
        }

        Ok(claims)
    }

    /// トークンのリフレッシュ処理
    ///
    /// - リフレッシュトークンを検証
    /// - 新しい認証トークンを生成して保存
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthTokenInDB, AppError> {
        let claims = self.verify_refresh_token(refresh_token).await?;

        // 既存のAuthTokenInDBを取得
        let mut auth_token = self
            .repository
            .find_by_refresh_token(refresh_token)
            .await?
            .ok_or_else(|| {
                AppError::BadRequest("リフレッシュトークンが見つかりません".to_string())
            })?;

        // 新しいアクセストークンを生成
        let new_access_token = jwt::create_access_token(&claims.sub, &self.jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        let access_token_exp = env::var("ACCESS_TOKEN_EXPIRY_HOURS")
            .expect("ACCESS_TOKEN_EXPIRY_HOURSが設定されていません")
            .parse::<i64>()
            .expect("ACCESS_TOKEN_EXPIRY_HOURSは有効な整数である必要があります");

        // アクセストークンと有効期限を更新
        auth_token.access_token = new_access_token;
        auth_token.expires_at =
            BsonDateTime::from_chrono(Utc::now() + Duration::hours(access_token_exp));
        auth_token.updated_at = Some(BsonDateTime::now());

        // 更新されたトークンを保存
        self.repository.update_auth_token(&auth_token).await?;

        Ok(auth_token)
    }

    /// 認証トークンを生成
    ///
    /// - アクセストークンとリフレッシュトークンを生成
    /// - 環境変数から有効期限を取得
    fn create_auth_token(&self, user_id: &str) -> Result<AuthTokenInDB, AppError> {
        let access_token = jwt::create_access_token(user_id, &self.jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let refresh_token = jwt::create_refresh_token(user_id, &self.jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // 環境変数から有効期限を取得
        let access_token_exp = env::var("ACCESS_TOKEN_EXPIRY_HOURS")
            .expect("ACCESS_TOKEN_EXPIRY_HOURSが設定されていません")
            .parse::<i64>()
            .expect("ACCESS_TOKEN_EXPIRY_HOURSは有効な整数である必要があります");
        let refresh_token_exp = env::var("REFRESH_TOKEN_EXPIRY_DAYS")
            .expect("REFRESH_TOKEN_EXPIRY_DAYSが設定されていません")
            .parse::<i64>()
            .expect("REFRESH_TOKEN_EXPIRY_DAYSは有効な整数である必要があります");

        let expires_at = BsonDateTime::from_chrono(Utc::now() + Duration::hours(access_token_exp));
        let refresh_expires_at =
            BsonDateTime::from_chrono(Utc::now() + Duration::days(refresh_token_exp));

        Ok(AuthTokenInDB {
            id: None,
            user_id: user_id.parse().unwrap(),
            access_token,
            refresh_token,
            expires_at,
            refresh_expires_at,
            created_at: BsonDateTime::now(),
            updated_at: None,
        })
    }

    /// 認証ヘッダーからトークンを抽出
    fn extract_token(&self, auth_header: &str) -> String {
        auth_header.trim_start_matches("Bearer ").to_string()
    }
}
