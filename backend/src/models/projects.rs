use bson::oid::ObjectId;
use bson::DateTime as BsonDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub enum ProjectStatus {
    Planning,
    InProgress,
    Completed,
    OnHold,
    Cancelled,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    pub id: ObjectId,
    pub title: String,
    pub description: Option<String>,
    pub skill_labels: Vec<String>,
    pub company_name: String,
    pub status: ProjectStatus,
    pub working_time_id: Option<Vec<ObjectId>>, // TODO: 集計方法について要考慮
    pub total_working_time: Option<i64>,
    pub created_at: BsonDateTime,
    pub updated_at: Option<BsonDateTime>,
}
