use crate::models::auth::AuthTokenInDB;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub token_type: String,
    #[schema(value_type = i64, example = "3600")]
    pub expires_in: i64,
}

impl From<AuthTokenInDB> for AuthResponse {
    fn from(token: AuthTokenInDB) -> Self {
        Self {
            access_token: token.access_token,
            token_type: "Bearer".to_string(),
            expires_in: (token.expires_at - chrono::Utc::now()).num_seconds(),
        }
    }
}

#[derive(Serialize, Debug, ToSchema)]
pub struct AuthTokenCreatedResponse {
    pub message: String,
}
