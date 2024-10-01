use crate::models::auth::AuthToken;
use crate::utils::serializer::{serialize_bson_datetime, serialize_object_id};
use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, Debug, ToSchema)]
pub struct AuthResponse {
    #[serde(serialize_with = "serialize_object_id")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub user_id: ObjectId,
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: String,
    #[serde(serialize_with = "serialize_bson_datetime")]
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub expires_at: BsonDateTime,
}

impl From<AuthToken> for AuthResponse {
    fn from(token: AuthToken) -> Self {
        Self {
            user_id: token.user_id,
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            token_type: "Bearer".to_string(),
            expires_at: token.expires_at.into(),
        }
    }
}
