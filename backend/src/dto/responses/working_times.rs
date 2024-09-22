use crate::models::working_times::WorkingTimeInDB;
use crate::utils::serializer::{
    serialize_bson_datetime, serialize_object_id, serialize_option_bson_datetime,
};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct WorkingTimeResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: ObjectId,
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "60a7e3e0f1c1b2a3b4c5d6e7")]
    pub project_id: ObjectId,
    #[serde(serialize_with = "serialize_bson_datetime")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub start_time: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub end_time: Option<BsonDateTime>,
    #[serde(serialize_with = "serialize_bson_datetime")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}

//  パニック防止
impl TryFrom<WorkingTimeInDB> for WorkingTimeResponse {
    type Error = &'static str;

    fn try_from(db_working_time: WorkingTimeInDB) -> Result<Self, Self::Error> {
        Ok(Self {
            id: db_working_time.id.ok_or("IDが存在しません")?,
            project_id: db_working_time.project_id,
            start_time: db_working_time.start_time,
            end_time: db_working_time.end_time,
            created_at: db_working_time.created_at,
            updated_at: db_working_time.updated_at,
        })
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct WorkingTimeCreatedResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: ObjectId,
}

impl From<ObjectId> for WorkingTimeCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id }
    }
}
