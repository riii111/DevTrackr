use bson::oid::ObjectId;
use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkingTime {
    pub id: ObjectId,
    pub start_time: Option<BsonDateTime>,
    pub end_time: Option<BsonDateTime>,
    pub created_at: BsonDateTime,
    pub updated_at: Option<BsonDateTime>,
}

// TODO: 単体バリデーション追加する（未来の時刻が入力されていないか、など）
