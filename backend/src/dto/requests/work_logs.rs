use crate::utils::deserializer::{deserialize_bson_date_time, deserialize_option_bson_date_time};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError, ValidationErrors};

// カスタムバリデーション用のトレイト
trait TimeValidator {
    fn validate_times(&self) -> Result<(), ValidationError>;
}

// バリデーションロジックを共通化するマクロ
macro_rules! impl_work_logs_validation {
    ($type:ty) => {
        impl TimeValidator for $type {
            // カスタムバリデーション: 開始時間と終了時間のバリデーションを行う
            fn validate_times(&self) -> Result<(), ValidationError> {
                let now = BsonDateTime::now();

                if self.start_time > now {
                    return Err(ValidationError::new(
                        "開始時間は現在時刻より前である必要があります",
                    ));
                }

                if let Some(end_time) = self.end_time {
                    if end_time <= self.start_time {
                        return Err(ValidationError::new(
                            "終了時間は開始時間より後である必要があります",
                        ));
                    }
                    if end_time > now {
                        return Err(ValidationError::new(
                            "終了時間は現在時刻より前である必要があります",
                        ));
                    }
                }

                Ok(())
            }
        }

        impl $type {
            pub fn validate_all(&self) -> Result<(), ValidationErrors> {
                let mut errors = ValidationErrors::new();

                // 既存のバリデーションを実行
                if let Err(e) = <Self as Validate>::validate(self) {
                    for (field, field_errors) in e.field_errors() {
                        for error in field_errors {
                            errors.add(field, error.clone());
                        }
                    }
                }

                // カスタムバリデーションを実行
                if let Err(e) = self.validate_times() {
                    errors.add("time", e);
                }

                if errors.is_empty() {
                    Ok(())
                } else {
                    Err(errors)
                }
            }
        }
    };
}

// マクロを使用してバリデーションロジックを実装
impl_work_logs_validation!(WorkLogsCreateRequest);
impl_work_logs_validation!(WorkLogsUpdateRequest);

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct WorkLogsCreateRequest {
    #[schema(value_type = String, example = "60a7e3e0f1c1b2a3b4c5d6e7")]
    pub project_id: ObjectId,
    #[serde(deserialize_with = "deserialize_bson_date_time")]
    #[schema(value_type = String, example = "2023-04-13T10:34:56Z")]
    pub start_time: BsonDateTime,
    #[serde(default, deserialize_with = "deserialize_option_bson_date_time")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub end_time: Option<BsonDateTime>,
    #[validate(length(min = 0, max = 1000, message = "メモは0〜1000文字である必要があります"))]
    #[schema(example = "今日はプロジェクトのキックオフミーティングを行いました。")]
    pub memo: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema, Validate)]
pub struct WorkLogsUpdateRequest {
    #[schema(value_type = String, example = "60a7e3e0f1c1b2a3b4c5d6e7")]
    pub project_id: ObjectId,
    #[serde(deserialize_with = "deserialize_bson_date_time")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub start_time: BsonDateTime,
    #[serde(default, deserialize_with = "deserialize_option_bson_date_time")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub end_time: Option<BsonDateTime>,
    #[validate(length(min = 0, max = 1000, message = "メモは0〜1000文字である必要があります"))]
    #[schema(example = "今日はプロジェクトのキックオフミーティングを行いました。")]
    pub memo: Option<String>,
}
