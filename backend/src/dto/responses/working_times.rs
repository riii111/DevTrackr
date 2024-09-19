use crate::models::working_times::WorkingTimeInDB;
use crate::utils::serializer::{
    serialize_bson_datetime, serialize_object_id, serialize_option_bson_datetime,
};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WorkingTimeResponse {
    #[serde(serialize_with = "serialize_object_id")]
    pub id: ObjectId,
    #[serde(serialize_with = "serialize_bson_datetime")]
    pub start_time: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    pub end_time: Option<BsonDateTime>,
    #[serde(serialize_with = "serialize_bson_datetime")]
    pub created_at: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    pub updated_at: Option<BsonDateTime>,
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
