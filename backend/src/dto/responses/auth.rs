use crate::models::auth::AuthToken;
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    #[schema(value_type = String, example = "3600")]
    pub expires_in: i64,
}

impl From<AuthToken> for AuthResponse {
    fn from(token: AuthToken) -> Self {
        Self {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            token_type: "Bearer".to_string(),
            expires_in: (token.expires_at - chrono::Utc::now()).num_seconds(),
        }
    }
}
