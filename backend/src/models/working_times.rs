use crate::utils::deserializer::{deserialize_bson_date_time, deserialize_option_bson_date_time};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

// memo: バックエンド側では時刻をUTCで統一し、フロント側で変換する事を想定.

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct WorkingTimeCreate {
    #[serde(deserialize_with = "deserialize_bson_date_time")]
    #[schema(value_type = String, example = "2023-04-13T10:34:56Z")]
    pub start_time: BsonDateTime,
    #[serde(default, deserialize_with = "deserialize_option_bson_date_time")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub end_time: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct WorkingTimeUpdate {
    #[serde(deserialize_with = "deserialize_bson_date_time")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub start_time: BsonDateTime,
    #[serde(default, deserialize_with = "deserialize_option_bson_date_time")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub end_time: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct WorkingTimeInDB {
    // app側では"id"として参照できるように
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>, // DB側にID生成させるので任意
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub start_time: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub end_time: Option<BsonDateTime>,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}

// TODO: 単体バリデーション追加する（未来の時刻が入力されていないか、など）
