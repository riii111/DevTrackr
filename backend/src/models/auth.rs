use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthTokenCreate {
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上である必要があります"))]
    #[schema(example = "password123")]
    pub password: String,

    #[validate(length(min = 1, message = "名前は1文字以上である必要があります"))]
    #[schema(example = "John Doe")]
    pub username: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthTokenLogin {
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    #[schema(example = "user@example.com")]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上である必要があります"))]
    #[schema(example = "password123")]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct AuthTokenInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub user_id: ObjectId,
    #[schema(value_type = String, example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub access_token: String,
    #[schema(value_type = String, example = "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...")]
    pub refresh_token: String,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub expires_at: BsonDateTime,
    #[schema(value_type = String, example = "2023-04-20T12:34:56Z")]
    pub refresh_expires_at: BsonDateTime,
    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}
