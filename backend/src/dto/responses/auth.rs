use crate::models::auth::AuthTokenInDB;
use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct AuthResponse {
    pub token_type: String,
    #[schema(value_type = i64, example = "3600")]
    pub expires_in: i64,
}

impl From<AuthTokenInDB> for AuthResponse {
    fn from(token: AuthTokenInDB) -> Self {
        let expires_at: DateTime<Utc> = token.expires_at.into();
        let now: DateTime<Utc> = Utc::now();
        Self {
            token_type: "Bearer".to_string(),
            expires_in: (expires_at - now).num_seconds().max(0),
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct AuthTokenCreatedResponse {
    pub message: String,
}
