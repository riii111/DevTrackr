use crate::models::projects::{ProjectInDB, ProjectStatus};
use crate::utils::serializer::{
    serialize_bson_datetime, serialize_object_id, serialize_option_bson_datetime,
};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serialize;

#[derive(Serialize, Debug)]
#[derive(Serialize, Debug, ToSchema)]
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
    #[serde(serialize_with = "serialize_bson_datetime")]
    pub created_at: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    pub updated_at: Option<BsonDateTime>,
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
