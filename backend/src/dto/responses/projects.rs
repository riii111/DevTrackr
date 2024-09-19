use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::Serialize;

use crate::models::projects::{ProjectInDB, ProjectStatus};

#[derive(Serialize, Debug)]
pub struct ProjectResponse {
    #[serde(serialize_with = "serialize_object_id")]
    pub id: ObjectId,
    pub title: String,
    pub description: String,
    pub company_name: String,
    pub status: ProjectStatus,
    pub skill_labels: Vec<String>,
    pub working_time_id: Vec<ObjectId>,
    pub total_working_time: Option<i64>,
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
impl TryFrom<ProjectInDB> for ProjectResponse {
    type Error = &'static str;

    fn try_from(db_project: ProjectInDB) -> Result<Self, Self::Error> {
        Ok(Self {
            id: db_project.id.ok_or("IDが存在しません")?,
            title: db_project.title,
            description: db_project.description.unwrap_or("".to_string()),
            company_name: db_project.company_name,
            status: db_project.status,
            skill_labels: db_project.skill_labels.unwrap_or(vec![]),
            working_time_id: db_project.working_time_id.unwrap_or(vec![]),
            total_working_time: db_project.total_working_time,
            created_at: db_project.created_at,
            updated_at: db_project.updated_at,
        })
    }
}

#[derive(Serialize, Debug)]
pub struct ProjectCreatedResponse {
    #[serde(serialize_with = "serialize_object_id")]
    pub id: ObjectId,
}

impl From<ObjectId> for ProjectCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id }
    }
}
