use crate::clients::aws_s3::S3Client;
use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::auth::AuthTokenInDB;
use crate::models::users::{UserCreate, UserInDB, UserUpdate, UserUpdateInternal};
use crate::repositories::auth::AuthRepository;
use crate::utils::jwt;
use crate::utils::jwt::Claims;
use crate::utils::password::{hash_password, verify_password};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use bson::DateTime as BsonDateTime;
use chrono::Utc;
use std::sync::Arc;

pub struct AuthUseCase<R: AuthRepository> {
    repository: Arc<R>,
    jwt_secret: Vec<u8>,
    s3_client: Arc<S3Client>,
}

impl<R: AuthRepository> AuthUseCase<R> {
    pub fn new(repository: Arc<R>, jwt_secret: &[u8], s3_client: Arc<S3Client>) -> Self {
        Self {
            repository,
            jwt_secret: jwt_secret.to_vec(),
            s3_client,
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
            .await
            .map_err(|e| {
                if let RepositoryError::DuplicateError(e) = e {
                    // 他者の個人情報を推測できないようにするため、実際のエラー内容はログ出力のみとし返す文言を変更する
                    log::info!("ユーザー登録でユニーク制約違反が発生: {}", e);
                    AppError::BadRequest(
                        "バリデーションに失敗したか、処理中にエラーが発生しました".to_string(),
                    )
                } else {
                    AppError::from(e)
                }
            })?;

        let auth_token = self.create_auth_token(&user_id.to_string())?;
        self.repository.save_auth_token(&auth_token).await?;
        Ok(auth_token)
    }

    /// ログイン中のユーザー更新処理
    pub async fn update_me(
        &self,
        access_token: &str,
        user_update: &UserUpdate,
    ) -> Result<bool, AppError> {
        // MongoDBではPUTとPATCHともに部分更新できるので、全フィールド渡さずともNoneで上書きされる事はない
        let mut user_update_internal = UserUpdateInternal {
            email: user_update.email.clone(),
            password: None,
            username: user_update.username.clone(),
            role: user_update.role.clone(),
            avatar_url: None,
        };

        if let Some(avatar_data) = &user_update.avatar {
            // データURIスキーマを処理
            let base64_data = avatar_data
                .split_once(",")
                .map(|(_, data)| data)
                .ok_or_else(|| AppError::BadRequest("Invalid base64 data format".to_string()))?;

            let image_data = STANDARD
                .decode(base64_data)
                .map_err(|e| AppError::BadRequest(format!("無効なbase64データ: {}", e)))?;

            let new_avatar_key = self.s3_client.upload_avatar(&image_data).await?;
            let new_avatar_url = self.s3_client.get_public_url(&new_avatar_key);
            user_update_internal.avatar_url = Some(new_avatar_url);
        }

        // パスワードのハッシュ化
        if let Some(password) = &user_update.password {
            let password_hash = hash_password(password)
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;
            user_update_internal.password = Some(password_hash);
        }

        // ユーザー情報を更新
        Ok(self
            .repository
            .update_user_by_access_token(access_token, &user_update_internal)
            .await?)
    }

    /// ログイン中のユーザー情報を取得
    pub async fn get_current_user(&self, access_token: &str) -> Result<UserInDB, AppError> {
        // アクセストークンからユーザー情報を直接取得
        let mut user = self
            .repository
            .find_user_by_access_token(access_token)
            .await?
            .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

        if let Some(avatar_url) = &user.avatar_url {
            let public_url = self.s3_client.get_public_url(avatar_url);
            user.avatar_url = Some(public_url);
        }

        Ok(user)
    }

    /// ユーザーログアウト処理
    ///
    /// - アクセストークンとリフレッシュトークンを削除
    pub async fn logout(&self, auth_header: &str) -> Result<(), AppError> {
        let token = jwt::extract_token(auth_header);

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
            .map_err(|_| AppError::BadRequest("無効なリクエストです".to_string()))?; // あえて曖昧なエラーメッセージを返す

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
                AppError::InternalServerError("サーバーエラーが発生しました".to_string())
                // エラー内容はログ出力のみとし返す文言を変更する
            })?
            .ok_or_else(|| {
                AppError::BadRequest("無効なリクエストです".to_string()) // あえて曖昧なエラーメッセージを返す
            })?;

        // リフレッシュトークンの有効期限を比較
        if Utc::now() > auth_token.refresh_expires_at.into() {
            log::error!(
                "リフレッシュトークンの有効期限が切れています: {}",
                refresh_token
            );
            return Err(AppError::BadRequest("無効なリクエストです".to_string()));
            // あえて曖昧なエラーメッセージを返す
        }

        Ok(claims)
    }

    /// トークンのリフレッシュ処理
    ///
    /// - リフレッシュトークンを検証
    /// - 新しい認証トークンを生成して保存
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthTokenInDB, AppError> {
        let claims = self.verify_refresh_token(refresh_token).await?;

        // リフレッシュ専用の関数を使用
        let (new_access_token, new_expires_at) =
            jwt::create_refreshed_access_token(&claims.sub, &self.jwt_secret)
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

        // 既存のAuthTokenInDBを取得
        let mut auth_token = self
            .repository
            .find_by_refresh_token(refresh_token)
            .await?
            .ok_or_else(|| {
                log::error!("リフレッシュトークンが見つかりません: {}", refresh_token);
                AppError::BadRequest("無効なリクエストです".to_string()) // あえて曖昧なエラーメッセージを返す
            })?;

        // 更新内容を設定
        auth_token.access_token = new_access_token;
        auth_token.expires_at = new_expires_at;
        auth_token.updated_at = Some(BsonDateTime::now());

        // DBを更新
        self.repository.update_auth_token(&auth_token).await?;

        Ok(auth_token)
    }

    /// 認証トークンを生成
    fn create_auth_token(&self, user_id: &str) -> Result<AuthTokenInDB, AppError> {
        let (access_token, refresh_token, expires_at, refresh_expires_at) =
            jwt::create_token_pair(user_id, &self.jwt_secret)
                .map_err(|e| AppError::InternalServerError(e.to_string()))?;

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
}
