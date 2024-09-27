use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use serde_with::{serde_as, DefaultOnNull};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, ToSchema)]
pub enum ProjectStatus {
    #[default]
    Planning,   // 企画中
    InProgress, // 進行中
    Completed,  // 完了
    OnHold,     // 一時中断
    Cancelled,  // キャンセル
}

#[derive(Deserialize, Debug, Validate, ToSchema)]
pub struct ProjectCreate {
    #[validate(length(min = 1, max = 100, message = "タイトルは1〜100文字である必要があります"))]
    pub title: String,
    #[validate(length(max = 1000, message = "説明は1000文字以内である必要があります"))]
    pub description: Option<String>,
    #[validate(length(max = 10, message = "スキルラベルは最大10個まで登録できます"))]
    pub skill_labels: Option<Vec<String>>,
    // pub company_id: ObjectId,  // TODO: 後で追加する
    #[validate(range(min = 0, message = "時給は0以上である必要があります"))]
    pub hourly_pay: Option<i32>,
    pub status: ProjectStatus,
}

#[derive(Serialize, Deserialize, Debug, Validate, ToSchema)]
pub struct ProjectUpdate {
    #[validate(length(min = 1, max = 100, message = "タイトルは1〜100文字である必要があります"))]
    pub title: String,
    #[validate(length(max = 1000, message = "説明は1000文字以内である必要があります"))]
    pub description: Option<String>,
    #[validate(length(max = 10, message = "スキルラベルは最大10個まで登録できます"))]
    pub skill_labels: Option<Vec<String>>,
    // #[schema(value_type = String, example = "70a6c1e9f0f7b9001234abcd")]
    // pub company_id: ObjectId,  // TODO: 後で追加する
    #[validate(range(min = 0, message = "時給は0以上である必要があります"))]
    pub hourly_pay: Option<i32>,
    pub status: ProjectStatus,
    #[validate(range(min = 0, message = "総作業時間は0以上である必要があります"))]
    pub total_working_time: i64,
}

#[serde_as]
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ProjectInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,
    pub title: String,
    pub description: Option<String>,
    pub skill_labels: Option<Vec<String>>,
    // #[schema(value_type = String, example = "70a6c1e9f0f7b9001234abcd")]
    // pub company_id: ObjectId,  // TODO: 後で追加する
    pub hourly_pay: Option<i32>,
    #[serde_as(as = "DefaultOnNull")]
    pub status: ProjectStatus,
    pub total_working_time: i64,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}

impl From<ProjectInDB> for ProjectUpdate {
    fn from(project: ProjectInDB) -> Self {
        ProjectUpdate {
            title: project.title,
            description: project.description,
            skill_labels: project.skill_labels,
            // company_id: project.company_id,  // TODO: 後で追加する
            hourly_pay: project.hourly_pay,
            status: project.status,
            total_working_time: project.total_working_time,
        }
    }
}
