use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

// 共通のアプリケーションエラー
#[derive(Debug, Error)]
pub enum AppError {
    #[error("バリデーションエラー: {0}")]
    ValidationError(String),

    #[error("無効なIDです")]
    InvalidId,

    #[error("リソースが見つかりません")]
    NotFound,

    #[error("不正なリクエストです")]
    BadRequest,

    // #[error("内部サーバーエラーです")]
    // InternalServerError,
    #[error("データベースエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),
    // 必要に応じて他のエラーを追加
}

// エラーレスポンスの構造体
#[derive(Serialize, ToSchema)]
pub struct ErrorResponse {
    error: String,
}

// Swagger用に実装
impl<'a> ToSchema<'a> for AppError {
    fn schema() -> (
        &'a str,
        utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>,
    ) {
        let schema = utoipa::openapi::schema::ObjectBuilder::new()
            .title(Some("AppError"))
            .description(Some("アプリケーションエラー"))
            .property(
                "error",
                utoipa::openapi::schema::ObjectBuilder::new()
                    .property(
                        "message",
                        utoipa::openapi::schema::ObjectBuilder::new()
                            .schema_type(utoipa::openapi::schema::SchemaType::String)
                            .build(),
                    )
                    .build(),
            )
            .build();
        ("AppError", schema.into())
    }
}

impl AppError {
    // エラーごとのHTTPステータスコードをマッピング
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::BadRequest => StatusCode::BAD_REQUEST,
            AppError::InvalidId => StatusCode::BAD_REQUEST,
            AppError::NotFound => StatusCode::NOT_FOUND,
            // AppError::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
