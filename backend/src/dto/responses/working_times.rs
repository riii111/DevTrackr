use crate::models::working_times::WorkingTimeInDB;
use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WorkingTimeResponse {
    #[serde(serialize_with = "serialize_object_id")]
    pub id: ObjectId,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
}

// ObjectIdを16進数文字列としてシリアライズするためのヘルパー関数
fn serialize_object_id<S>(object_id: &ObjectId, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&object_id.to_hex())
}

//  パニック防止
impl TryFrom<WorkingTimeInDB> for WorkingTimeResponse {
    type Error = &'static str;

    fn try_from(db_working_time: WorkingTimeInDB) -> Result<Self, Self::Error> {
        Ok(Self {
            id: db_working_time.id.ok_or("IDが存在しません")?,
            start_time: db_working_time.start_time,
            end_time: db_working_time.end_time,
            created_at: db_working_time.created_at,
            updated_at: db_working_time.updated_at,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct WorkingTimeCreatedResponse {
    #[serde(serialize_with = "serialize_object_id")]
    pub id: ObjectId,
}

impl From<ObjectId> for WorkingTimeCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id }
    }
}
