use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
// use utoipa::openapi::schema::{Object, Schema};
use utoipa::ToSchema;
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Deserialize, Debug, ToSchema)]
pub struct ProjectCreate {
    pub title: String,
    pub description: Option<String>,
    pub skill_labels: Option<Vec<String>>,
    pub company_name: String,
    #[serde(default = "default_project_status")]
    pub status: ProjectStatus,
}

#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ProjectInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>, // DB側にID生成させるので任意
    pub title: String,
    pub description: Option<String>,
    pub skill_labels: Option<Vec<String>>,
    pub company_name: String,
    #[serde(default = "default_project_status")]
    pub status: ProjectStatus,
    #[schema(value_type = Vec<String>, example = json!(["507f1f77bcf86cd799439011", "507f1f77bcf86cd799439012"]))]
    pub working_time_id: Option<Vec<ObjectId>>, // TODO: 集計方法について要考慮
    pub total_working_time: Option<i64>,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}

fn default_project_status() -> ProjectStatus {
    ProjectStatus::Planning
}
