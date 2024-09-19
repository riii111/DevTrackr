use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Deserialize, Debug)]
pub struct ProjectCreate {
    pub title: String,
    pub description: Option<String>,
    pub skill_labels: Option<Vec<String>>,
    pub company_name: String,
    #[serde(default = "default_project_status")]
    pub status: ProjectStatus,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>, // DB側にID生成させるので任意
    pub title: String,
    pub description: Option<String>,
    pub skill_labels: Option<Vec<String>>,
    pub company_name: String,
    #[serde(default = "default_project_status")]
    pub status: ProjectStatus,
    pub working_time_id: Option<Vec<ObjectId>>, // TODO: 集計方法について要考慮
    pub total_working_time: Option<i64>,
    pub created_at: BsonDateTime,
    pub updated_at: Option<BsonDateTime>,
}

fn default_project_status() -> ProjectStatus {
    ProjectStatus::Planning
}
