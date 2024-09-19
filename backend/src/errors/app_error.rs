// backend/src/errors/app_error.rs
use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;

// 共通のアプリケーションエラー
#[derive(Debug, Error)]
pub enum AppError {
    #[error("{0}")]
    ProjectError(#[from] crate::errors::project_error::ProjectError),

    #[error("{0}")]
    WorkingTimeError(#[from] crate::errors::working_time_error::WorkingTimeError),

    #[error("バリデーションエラー: {0}")]
    ValidationError(String),

    #[error("リソースが見つかりません")]
    NotFound,

    #[error("不正なリクエストです")]
    BadRequest,

    #[error("内部サーバーエラーです")]
    InternalServerError,
    // 必要に応じて他のエラーを追加
}

// エラーレスポンスの構造体
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

impl AppError {
    // エラーごとのHTTPステータスコードをマッピング
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ProjectError(e) => e.status_code(),
            AppError::WorkingTimeError(e) => e.status_code(),
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            // 他のエラーも適宜追加
        }
    }

    // エラーメッセージを取得
    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(ErrorResponse {
            error: self.error_message(),
        })
    }
}
