use crate::utils::deserializer::{deserialize_bson_date_time, deserialize_option_bson_date_time};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};

// memo: バックエンド側では時刻をUTCで統一し、フロント側で変換する事を想定.

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTimeCreate {
    #[serde(deserialize_with = "deserialize_bson_date_time")]
    pub start_time: BsonDateTime,
    #[serde(default, deserialize_with = "deserialize_option_bson_date_time")]
    pub end_time: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTimeUpdate {
    #[serde(deserialize_with = "deserialize_bson_date_time")]
    pub start_time: BsonDateTime,
    #[serde(default, deserialize_with = "deserialize_option_bson_date_time")]
    pub end_time: Option<BsonDateTime>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTimeInDB {
    // app側では"id"として参照できるように
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // DB側にID生成させるので任意
    pub start_time: BsonDateTime,
    pub end_time: Option<BsonDateTime>,
    pub created_at: BsonDateTime,
    pub updated_at: Option<BsonDateTime>,
}

// TODO: 単体バリデーション追加する（未来の時刻が入力されていないか、など）
