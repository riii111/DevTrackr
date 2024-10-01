use crate::errors::app_error::AppError;
use crate::errors::repositories_error::RepositoryError;
use crate::models::auth::AuthToken;
use crate::repositories::auth::AuthRepository;
use crate::utils::jwt;
use crate::utils::password::{hash_password, verify_password};
use chrono::{Duration, Utc};
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

    pub async fn logout(&self, auth_header: &str) -> Result<(), AppError> {
        let token = self.extract_token(auth_header)?;
        let deleted = self
            .repository
            .delete_auth_token(&token)
            .await
            .map_err(|e| match e {
                RepositoryError::ConnectionError => AppError::DatabaseConnectionError,
                RepositoryError::DatabaseError(err) => AppError::DatabaseError(err),
            })?;

        if deleted {
            Ok(())
        } else {
            Err(AppError::NotFound("トークンが見つかりません".to_string()))
        }
    }

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

    fn create_auth_token(&self, user_id: &str) -> Result<AuthToken, AppError> {
        let access_token = jwt::create_token(user_id, &self.jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let refresh_token = jwt::create_token(user_id, &self.jwt_secret)
            .map_err(|e| AppError::InternalServerError(e.to_string()))?;
        let expires_at = Utc::now() + Duration::hours(24);

        Ok(AuthToken {
            user_id: user_id.parse().unwrap(),
            access_token,
            refresh_token,
            expires_at,
        })
    }

    fn extract_token(&self, auth_header: &str) -> Result<String, AppError> {
        let parts: Vec<&str> = auth_header.split_whitespace().collect();
        if parts.len() != 2 || parts[0] != "Bearer" {
            return Err(AppError::Unauthorized("無効な認証ヘッダーです".to_string()));
        }
        Ok(parts[1].to_string())
    }
}
