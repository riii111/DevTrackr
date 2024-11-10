use actix_web::error::JsonPayloadError;
use actix_web::web::JsonConfig;
use actix_web::ResponseError;
use actix_web::{http::StatusCode, HttpResponse};
use log;
use serde::de;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use thiserror::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

// 共通のアプリケーションエラー
#[derive(Debug, Error)]
pub enum AppError {
    #[error("バリデーションエラー: {0}")]
    ValidationError(ValidationErrors),

    #[error("デシリアライズエラー: {0}")]
    DeserializeError(ErrorResponse),

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
#[derive(Debug, Serialize, ToSchema, Clone)]
pub struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    field_errors: Vec<FieldError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

// フィールドに関連するエラー用
#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct FieldError {
    field: String,
    message: String,
}

impl std::fmt::Display for ErrorResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // JSONとして整形して出力
        if let Ok(json) = serde_json::to_string(self) {
            write!(f, "{}", json)
        } else {
            // JSONシリアライズに失敗した場合のフォールバック
            write!(f, "Error: {}", self.error)
        }
    }
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
            AppError::DeserializeError(_) => StatusCode::BAD_REQUEST,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::DuplicateError(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            AppError::Forbidden(_) => StatusCode::FORBIDDEN,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    // バリデーションエラーのフォーマット
    // UX観点で見ると、ユーザーは通常、一度に1つの問題を修正する傾向があるため複数エラーを集計しない(実装も面倒そう)
    // Googleフォームなども同様
    fn format_validation_errors(errors: &ValidationErrors) -> Vec<FieldError> {
        // ValidationErrorsのfield_errorsメソッドの戻り値型に合わせて修正
        if let Some((field, error_vec)) = errors.field_errors().iter().next() {
            if let Some(error) = error_vec.first() {
                let message = error
                    .message
                    .as_ref()
                    .map(|cow| cow.to_string())
                    .unwrap_or_else(|| {
                        match error.code.as_ref() {
                            "required" => "必須項目です",
                            "email" => "有効なメールアドレスを入力してください",
                            "length" => "入力された文字数が無効です",
                            "range" => "入力された値が範囲外です",
                            _ => "不正な値です",
                        }
                        .to_string()
                    });

                vec![FieldError {
                    field: field.to_string(),
                    message,
                }]
            } else {
                vec![FieldError {
                    field: "unknown".to_string(),
                    message: "不正な値です".to_string(),
                }]
            }
        } else {
            vec![FieldError {
                field: "unknown".to_string(),
                message: "不正な値です".to_string(),
            }]
        }
    }

    // デシリアライズエラーのフォーマット
    fn format_deserialize_errors(error_str: &str) -> Vec<FieldError> {
        if error_str.contains("missing field") {
            // "missing field `field_name`" からフィールド名を抽出
            let field = error_str
                .split('`')
                .nth(1)
                .and_then(|s| s.split('`').next())
                .unwrap_or("unknown")
                .to_string();

            vec![FieldError {
                field,
                message: "必須項目です".to_string(),
            }]
        } else if error_str.contains("invalid type") {
            let field = error_str
                .split("at.")
                .nth(1)
                .and_then(|s| s.split(' ').next())
                .unwrap_or("unknown")
                .to_string();

            vec![FieldError {
                field,
                message: "入力された値の型が正しくありません".to_string(),
            }]
        } else {
            vec![FieldError {
                field: "unknown".to_string(),
                message: "入力形式が正しくありません".to_string(),
            }]
        }
    }

    fn log_error(&self) {
        match self {
            AppError::InternalServerError(msg) => log::error!("内部エラー: {}", msg),
            AppError::DatabaseError(err) => log::error!("DBエラー: {}", err),
            AppError::DeserializeError(_) => log::debug!("デシリアライズエラー: {}", self),
            _ => log::debug!("アプリケーションエラー: {}", self),
        }
    }

    fn to_response(&self) -> ErrorResponse {
        match self {
            // フィールドレベルのエラー
            AppError::ValidationError(errors) => ErrorResponse {
                error: "バリデーションエラー".to_string(),
                field_errors: Self::format_validation_errors(errors),
                message: None,
                code: None,
            },
            AppError::DeserializeError(error_response) => {
                // ErrorResponseをそのまま使用
                error_response.clone()
            }
            AppError::BadRequest(msg) => ErrorResponse {
                error: "不正なリクエスト".to_string(),
                field_errors: vec![],
                message: Some(msg.clone()),
                code: Some("BAD_REQUEST".to_string()),
            },
            AppError::DuplicateError(msg) => ErrorResponse {
                error: "重複エラー".to_string(),
                field_errors: vec![],
                message: Some(msg.clone()),
                code: Some("DUPLICATE_ENTRY".to_string()),
            },

            // 認証・認可エラー
            AppError::Unauthorized(msg) => ErrorResponse {
                error: "認証エラー".to_string(),
                field_errors: vec![],
                message: Some(msg.clone()),
                code: Some("UNAUTHORIZED".to_string()),
            },
            AppError::Forbidden(msg) => ErrorResponse {
                error: "アクセス権限エラー".to_string(),
                field_errors: vec![],
                message: Some(msg.clone()),
                code: Some("FORBIDDEN".to_string()),
            },

            // リソースエラー
            AppError::NotFound(msg) => ErrorResponse {
                error: "リソースが見つかりません".to_string(),
                field_errors: vec![],
                message: Some(msg.clone()),
                code: Some("NOT_FOUND".to_string()),
            },

            // システムエラー
            AppError::DatabaseError(_) => ErrorResponse {
                error: "データベースエラー".to_string(),
                field_errors: vec![],
                message: Some("データベース処理中にエラーが発生しました".to_string()),
                code: Some("DATABASE_ERROR".to_string()),
            },
            AppError::InternalServerError(_) => ErrorResponse {
                error: "内部サーバーエラー".to_string(),
                field_errors: vec![],
                message: Some("予期せぬエラーが発生しました".to_string()),
                code: Some("INTERNAL_SERVER_ERROR".to_string()),
            },
        }
    }
}

// 一般的なSerdeErrorのハンドリング
impl From<SerdeError> for AppError {
    fn from(err: SerdeError) -> Self {
        AppError::DeserializeError(ErrorResponse {
            error: "入力エラー".to_string(),
            field_errors: AppError::format_deserialize_errors(&err.to_string()),
            message: None,
            code: None,
        })
    }
}

// カスタムデシリアライズエラーのハンドリング
impl From<de::value::Error> for AppError {
    fn from(_: de::value::Error) -> Self {
        AppError::DeserializeError(ErrorResponse {
            error: "デシリアライズエラー".to_string(),
            field_errors: vec![FieldError {
                field: "unknown".to_string(),
                message: "入力形式が正しくありません".to_string(),
            }],
            message: None,
            code: None,
        })
    }
}

// JsonConfigのエラーハンドラー
pub fn json_error_handler() -> JsonConfig {
    JsonConfig::default()
        .limit(262_144) // リクエストボディのサイズ（256 * 1024 = 262,144）. リソース枯渇を防ぐ
        .error_handler(|err, _| {
            let error: AppError = match &err {
                JsonPayloadError::Deserialize(json_err) => {
                    let error_msg = json_err.to_string();

                    if error_msg.contains("missing field") {
                        // 必須フィールドの欠落エラー
                        let field = error_msg.split('`').nth(1).unwrap_or("unknown").to_string();

                        AppError::DeserializeError(ErrorResponse {
                            error: "入力エラー".to_string(),
                            field_errors: vec![FieldError {
                                field,
                                message: "必須項目です".to_string(),
                            }],
                            message: None,
                            code: None,
                        })
                    } else {
                        // その他のデシリアライズエラー
                        AppError::DeserializeError(ErrorResponse {
                            error: "入力エラー".to_string(),
                            field_errors: AppError::format_deserialize_errors(&error_msg),
                            message: None,
                            code: None,
                        })
                    }
                }
                JsonPayloadError::ContentType => AppError::DeserializeError(ErrorResponse {
                    error: "入力エラー".to_string(),
                    field_errors: vec![FieldError {
                        field: "content-type".to_string(),
                        message: "Content-Typeはapplication/jsonである必要があります".to_string(),
                    }],
                    message: None,
                    code: None,
                }),
                JsonPayloadError::Overflow { .. } => AppError::DeserializeError(ErrorResponse {
                    error: "入力エラー".to_string(),
                    field_errors: vec![FieldError {
                        field: "body".to_string(),
                        message: "リクエストデータが大きすぎます（上限: 256KB）".to_string(),
                    }],
                    message: None,
                    code: None,
                }),
                _ => AppError::DeserializeError(ErrorResponse {
                    error: "入力エラー".to_string(),
                    field_errors: vec![FieldError {
                        field: "body".to_string(),
                        message: "リクエストの形式が正しくありません".to_string(),
                    }],
                    message: None,
                    code: None,
                }),
            };

            // エラーレスポンスを生成
            let response = error.error_response();
            actix_web::error::InternalError::from_response("", response).into()
        })
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        self.log_error();

        let response = match self {
            AppError::DeserializeError(error_response) => error_response,
            _ => &self.to_response(),
        };

        HttpResponse::build(self.status_code())
            .content_type("application/json")
            .json(response)
    }
}
