use crate::models::projects::{ProjectInDB, ProjectStatus};
use crate::utils::serializer::{
    serialize_bson_datetime, serialize_object_id, serialize_option_bson_datetime,
};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct ProjectResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: ObjectId,
    pub title: String,
    pub description: String,
    pub status: ProjectStatus,
    pub skill_labels: Vec<String>,
    #[schema(value_type = String, example = "70a6c1e9f0f7b9001234abcd")]
    pub company_id: ObjectId,
    #[schema(value_type = Vec<String>, example = json!(["507f1f77bcf86cd799439011", "507f1f77bcf86cd799439012"]))]
    pub total_working_time: Option<i64>,
    pub hourly_pay: Option<i32>,
    #[serde(serialize_with = "serialize_bson_datetime")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[serde(serialize_with = "serialize_option_bson_datetime")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
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
            status: db_project.status,
            skill_labels: db_project.skill_labels.unwrap_or(vec![]),
            company_id: db_project.company_id,
            total_working_time: Some(db_project.total_working_time),
            hourly_pay: db_project.hourly_pay,
            created_at: db_project.created_at,
            updated_at: db_project.updated_at,
        })
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct ProjectCreatedResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: ObjectId,
}

impl From<ObjectId> for ProjectCreatedResponse {
    fn from(id: ObjectId) -> Self {
        Self { id }
    }
}
