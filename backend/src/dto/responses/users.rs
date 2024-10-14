use crate::models::users::{EngineerRole, UserInDB};
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct UserResponse {
    pub id: String,
    pub email: String,
    pub username: String,
    pub role: Option<EngineerRole>,
    pub avatar_url: Option<String>,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: DateTime<Utc>,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<DateTime<Utc>>,
}

impl From<UserInDB> for UserResponse {
    fn from(user: UserInDB) -> Self {
        Self {
            id: user.id.unwrap().to_string(),
            email: user.email,
            username: user.username,
            role: user.role,
            avatar_url: user.avatar_url,
            created_at: user.created_at.into(),
            updated_at: user.updated_at.map(|dt| dt.into()),
        }
    }
}
