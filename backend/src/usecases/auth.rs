use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::auth::AuthToken;
use crate::repositories::auth::AuthRepository;
use crate::utils::jwt;
use crate::utils::password::{hash_password, verify_password};
use chrono::{Duration, Utc};
use std::env;
use std::sync::Arc;

pub struct AuthUseCase<R: AuthRepository> {
    repository: Arc<R>,
    jwt_secret: Vec<u8>,
}

impl<R: AuthRepository> AuthUseCase<R> {
    pub fn new(repository: Arc<R>, jwt_secret: &[u8]) -> Self {
        Self {
            repository,
            jwt_secret: jwt_secret.to_vec(),
        }
    }

    /// ユーザーログイン処理
    ///
    /// - メールアドレスでユーザーを検索
    /// - パスワードを検証
    /// - 認証トークンを生成して保存
    pub async fn login(&self, email: &str, password: &str) -> Result<AuthToken, AppError> {
        let user = self
            .repository
            .find_user_by_email(email)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?
            .ok_or_else(|| AppError::NotFound("ユーザーが見つかりません".to_string()))?;

        if verify_password(password, &user.password_hash) {
            let auth_token = self.create_auth_token(&user.id.to_string())?;
            self.repository
                .save_auth_token(&auth_token)
                .await
                .map_err(|e| match e {
                    RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                    RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
                })?;
            Ok(auth_token)
        } else {
            Err(AppError::Unauthorized("無効な認証情報です".to_string()))
        }
    }

    /// ユーザー登録処理
    ///
    /// - パスワードをハッシュ化
    /// - ユーザーを作成
    /// - 認証トークンを生成して保存
    pub async fn register(
        &self,
        email: &str,
        password: &str,
        name: &str,
    ) -> Result<AuthToken, AppError> {
        let password_hash =
            hash_password(password).map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let user = self
            .repository
            .create_user(email, &password_hash, name)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;

        let auth_token = self.create_auth_token(&user.id.to_string())?;
        self.repository
            .save_auth_token(&auth_token)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;
        Ok(auth_token)
    }

    /// ユーザーログアウト処理
    ///
    /// - アクセストークンとリフレッシュトークンを削除
    pub async fn logout(&self, auth_header: &str) -> Result<(), AppError> {
        let token = self.extract_token(auth_header);
        // アクセストークンの削除
        let deleted_access =
            self.repository
                .delete_auth_token(&token)
                .await
                .map_err(|e| match e {
                    RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                    RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
                })?;

        // リフレッシュトークンの削除
        let deleted_refresh = self
            .repository
            .delete_refresh_token(&token)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;

        if deleted_access || deleted_refresh {
            Ok(())
        } else {
            Err(AppError::NotFound("トークンが見つかりません".to_string()))
        }
    }

    /// トークンのリフレッシュ処理
    ///
    /// - リフレッシュトークンを検証
    /// - 新しい認証トークンを生成して保存
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<AuthToken, AppError> {
        let claims = jwt::verify_token(refresh_token, &self.jwt_secret)
            .map_err(|_| AppError::Unauthorized("無効なリフレッシュトークンです".to_string()))?;

        let auth_token = self.create_auth_token(&claims.sub)?;
        self.repository
            .save_auth_token(&auth_token)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;
        Ok(auth_token)
    }

    /// 認証トークンを生成
    ///
    /// - アクセストークンとリフレッシュトークンを生成
    /// - 環境変数から有効期限を取得
    fn create_auth_token(&self, user_id: &str) -> Result<AuthToken, AppError> {
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

        let expires_at = Utc::now() + Duration::hours(access_token_exp);
        let refresh_expires_at = Utc::now() + Duration::days(refresh_token_exp);
        Ok(AuthToken {
            user_id: user_id.parse().unwrap(),
            access_token,
            refresh_token,
            expires_at,
            refresh_expires_at,
        })
    }

    /// 認証ヘッダーからトークンを抽出
    fn extract_token(&self, auth_header: &str) -> String {
        auth_header.trim_start_matches("Bearer ").to_string()
    }
}
