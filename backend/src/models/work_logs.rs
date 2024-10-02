use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct WorkLogsInDB {
    // app側では"id"として参照できるように
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>, // DB側にID生成させるので任意
    #[schema(value_type = String, example = "60a7e3e0f1c1b2a3b4c5d6e7")]
    pub project_id: ObjectId,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub start_time: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub end_time: Option<BsonDateTime>,
    #[schema(example = "今日はプロジェクトのキックオフミーティングを行いました。")]
    pub memo: Option<String>,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}
