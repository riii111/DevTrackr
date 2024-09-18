use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// memo: バックエンド側では時刻をUTCで統一し、フロント側で変換する事を想定.

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTimeCreate {
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTimeUpdate {
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTimeInDB {
    // app側では"id"として参照できるように
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // DB側にID生成を任せる
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// TODO: 単体バリデーション追加する（未来の時刻が入力されていないか、など）
