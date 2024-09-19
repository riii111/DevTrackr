// backend/src/errors/project_error.rs
use actix_web::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ProjectError {
    #[error("プロジェクトIDが無効です")]
    InvalidId,
    #[error("プロジェクトが見つかりません")]
    NotFound,
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    #[error("無効な入力データ: {0}")]
    InvalidInput(String),
    // 他にエラーがあれば追加
}

impl ProjectError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            ProjectError::InvalidId => StatusCode::BAD_REQUEST,
            ProjectError::NotFound => StatusCode::NOT_FOUND,
            ProjectError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            ProjectError::InvalidInput(_) => StatusCode::BAD_REQUEST,
        }
    }
}
