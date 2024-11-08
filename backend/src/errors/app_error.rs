use actix_web::error::JsonPayloadError;
use actix_web::web::JsonConfig;
use actix_web::{http::StatusCode, HttpResponse};
use log;
use serde::de;
use serde::{Deserialize, Serialize};
use serde_json::Error as SerdeError;
use serde_path_to_error::Error as SerdePathError;
use thiserror::Error;
use utoipa::ToSchema;
use validator::ValidationErrors;

// 共通のアプリケーションエラー
#[derive(Debug, Error)]
pub enum AppError {
    #[error("バリデーションエラー: {0}")]
    ValidationError(ValidationErrors),

    #[error("デシリアライズエラー: {0}")]
    DeserializeError(String),

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
    #[serde(skip_serializing_if = "Vec::is_empty")]
    field_errors: Vec<FieldError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
}

// フィールドに関連するエラー用
#[derive(Serialize, Deserialize, ToSchema)]
pub struct FieldError {
    field: String,
    message: String,
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
            AppError::DeserializeError(msg) => log::debug!("デシリアライズエラー: {}", msg),
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
            AppError::DeserializeError(msg) => {
                // JSON文字列からFieldErrorのVecに戻す
                let field_errors: Vec<FieldError> = serde_json::from_str(msg).unwrap_or_default();
                ErrorResponse {
                    error: "入力エラー".to_string(),
                    field_errors,
                    message: None,
                    code: None,
                }
            }
            AppError::BadRequest(_) => ErrorResponse {
                error: "不正なリクエスト".to_string(),
                field_errors: vec![],
                message: Some(
                    "バリデーションに失敗したか、処理中にエラーが発生しました".to_string(),
                ),
                code: Some("BAD_REQUEST".to_string()),
            },
            AppError::DuplicateError(msg) => ErrorResponse {
                error: "重複エラー".to_string(),
                field_errors: vec![FieldError {
                    field: "email".to_string(),
                    message: "このメールアドレスは既に使用されています".to_string(),
                }],
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
        AppError::DeserializeError(format!("デシリアライズエラー: {}", err))
    }
}

// カスタムデシリアライズエラーのハンドリング
impl From<de::value::Error> for AppError {
    fn from(err: de::value::Error) -> Self {
        AppError::DeserializeError(format!("デシリアライズエラー: {}", err))
    }
}

impl From<SerdePathError<SerdeError>> for AppError {
    fn from(err: SerdePathError<SerdeError>) -> Self {
        let path = err.path().to_string().trim_start_matches('.').to_string();
        let error_message = match err.inner().classify() {
            serde_json::error::Category::Data => {
                if err.inner().to_string().contains("missing field") {
                    format!("{}: 必須項目です", path)
                } else {
                    format!("{}: 入力形式が正しくありません", path)
                }
            }
            serde_json::error::Category::Syntax => "リクエストの形式が正しくありません".to_string(),
            serde_json::error::Category::Eof => "リクエストデータが不完全です".to_string(),
            _ => "入力データの形式が正しくありません".to_string(),
        };
        AppError::DeserializeError(error_message)
    }
}

// JsonConfigのエラーハンドラー
pub fn json_error_handler() -> JsonConfig {
    JsonConfig::default()
        .limit(262_144)
        .error_handler(|err, _| {
            let error_details = match &err {
                JsonPayloadError::Deserialize(err) => {
                    log::debug!("デシリアライズエラー: {}", err);
                    AppError::format_deserialize_errors(&err.to_string())
                }
                JsonPayloadError::ContentType => vec![FieldError {
                    field: "content-type".to_string(),
                    message: "Content-Typeはapplication/jsonである必要があります".to_string(),
                }],
                JsonPayloadError::Payload(_) => vec![FieldError {
                    field: "body".to_string(),
                    message: "リクエストの処理中にエラーが発生しました".to_string(),
                }],
                JsonPayloadError::Overflow { .. } => vec![FieldError {
                    field: "body".to_string(),
                    message: "リクエストデータが大きすぎます（上限: 256KB）".to_string(),
                }],
                _ => vec![FieldError {
                    field: "unknown".to_string(),
                    message: "リクエストの形式が正しくありません".to_string(),
                }],
            };

            AppError::DeserializeError(serde_json::to_string(&error_details).unwrap_or_default())
                .into()
        })
}

impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> HttpResponse {
        self.log_error();
        HttpResponse::build(self.status_code()).json(self.to_response())
    }
}
