use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;
use serde_json::json;
use thiserror::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

// 共通のアプリケーションエラー
#[derive(Debug, Error)]
pub enum AppError {
    #[error("バリデーションエラー: {0}")]
    ValidationError(ValidationErrors),

    #[error("不正なリクエストです: {0}")]
    BadRequest(String),

    #[error("認証エラー: {0}")]
    Unauthorized(String),

    #[error("アクセス権限がありません: {0}")]
    Forbidden(String),

    #[error("リソースが見つかりません: {0}")]
    NotFound(String),

    #[error("データベース接続後のエラー: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("内部サーバーエラー: {0}")]
    InternalServerError(String),

    #[error("ユニーク制約違反: {0}")]
    DuplicateError(String),
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
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::DuplicateError(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // エラーメッセージを取得
    fn error_message(&self) -> String {
        self.to_string()
    }
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        let (error_type, details) = match self {
            AppError::ValidationError(errors) => {
                let error_messages: Vec<String> = errors
                    .field_errors()
                    .into_iter()
                    .map(|(field, error_vec)| {
                        let messages: Vec<String> = error_vec
                            .iter()
                            .map(|error| {
                                error
                                    .message
                                    .as_ref()
                                    .map(|cow| cow.to_string())
                                    .unwrap_or_else(|| error.code.to_string())
                            })
                            .collect();
                        format!("{}: {}", field, messages.join(", "))
                    })
                    .collect();
                ("バリデーションエラー", error_messages)
            }
            AppError::DuplicateError(_) => ("ユニーク制約違反", vec![self.error_message()]),
            AppError::BadRequest(_) => ("不正なリクエスト", vec![self.error_message()]),
            AppError::Unauthorized(_) => ("認証エラー", vec![self.error_message()]),
            AppError::Forbidden(_) => ("アクセス権限がありません", vec![self.error_message()]),
            AppError::NotFound(_) => ("リソースが見つかりません", vec![self.error_message()]),
            AppError::DatabaseError(error) => ("データベースエラー", vec![error.to_string()]),
            AppError::InternalServerError(_) => ("内部サーバーエラー", vec![self.error_message()]),
        };

        HttpResponse::build(status).json(json!({
            "error": error_type,
            "details": details
        }))
    }
}
