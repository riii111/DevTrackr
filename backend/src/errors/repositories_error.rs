use crate::errors::app_error::AppError;
use thiserror::Error;

// 低レベルの階層用のエラー

#[derive(Error, Debug)]
pub enum RepositoryError {
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("ユニーク制約違反: {0}")]
    DuplicateError(String),
    #[error("アクセストークンの有効期限が切れています")]
    ExpiredAccessToken,
}

impl From<RepositoryError> for AppError {
    fn from(err: RepositoryError) -> Self {
        match err {
            RepositoryError::DatabaseError(e) => AppError::DatabaseError(e),
            RepositoryError::DuplicateError(e) => AppError::DuplicateError(e),
            RepositoryError::ExpiredAccessToken => {
                AppError::Forbidden("アクセストークンの有効期限が切れています".to_string())
            }
        }
    }
}
