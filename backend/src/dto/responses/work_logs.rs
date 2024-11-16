use crate::models::work_logs::WorkLogInDB;
use crate::utils::serializer::{
    serialize_bson_datetime, serialize_object_id, serialize_option_bson_datetime,
};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct WorkLogResponse {
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

    #[schema(value_type = Option<i32>, example = 30)]
    pub break_time: Option<i32>,

    #[schema(value_type = Option<i32>, example = 120)]
    pub actual_work_minutes: Option<i32>,

    #[schema(example = "今日はプロジェクトのキックオフミーティングを行いました。")]
    pub memo: Option<String>,
}

//  パニック防止
impl TryFrom<WorkLogInDB> for WorkLogResponse {
    type Error = &'static str;

    fn try_from(db_work_log: WorkLogInDB) -> Result<Self, Self::Error> {
        Ok(Self {
            id: db_work_log.id.ok_or("IDが存在しません")?,
            project_id: db_work_log.project_id,
            start_time: db_work_log.start_time,
            end_time: db_work_log.end_time,
            created_at: db_work_log.created_at,
            updated_at: db_work_log.updated_at,
            break_time: db_work_log.break_time,
            actual_work_minutes: db_work_log.actual_work_minutes,
            memo: db_work_log.memo,
        })
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct WorkLogCreatedResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: ObjectId,
}

impl From<ObjectId> for WorkLogCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id }
    }
}
