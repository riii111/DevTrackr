use bson::{oid::ObjectId, DateTime as BsonDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, ToSchema, Clone)]
pub enum EngineerRole {
    #[default]
    None,
    FrontEnd,
    BackEnd,
    FullStack,
    DevOps,
    Security,
    ProductManager,
    ProjectManager,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Clone)]
pub struct UserCreate {
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

// 画像など部分更新をサポートするためにOptionを使用する
#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct UserUpdate {
    #[validate(email(message = "有効なメールアドレスを入力してください"))]
    #[schema(example = "user_updated@example.com")]
    pub email: String,

    #[validate(length(min = 8, message = "パスワードは8文字以上である必要があります"))]
    #[schema(example = "newpassword123")]
    pub password: Option<String>,

    #[validate(length(min = 1, message = "名前は1文字以上である必要があります"))]
    #[schema(example = "John Doe Updated")]
    pub username: String,

    #[schema(example = "Frontend")]
    pub role: Option<EngineerRole>,

    #[schema(
        example = "data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAYAAAAfFcSJAAAACklEQVR4nGMAAQAABQABDQottAAAAABJRU5ErkJggg=="
    )]
    pub avatar: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserUpdateInternal {
    pub email: String,

    pub password: Option<String>,

    pub username: String,

    pub role: Option<EngineerRole>,

    // avatar_path指定時に生成されるURL
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct UserInDB {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String, example = "507f1f77bcf86cd799439011")]
    pub id: Option<ObjectId>,

    pub email: String,

    pub password_hash: String,

    pub username: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<EngineerRole>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_url: Option<String>,

    #[schema(value_type = String, example = "2023-04-13T12:34:56Z")]
    pub created_at: BsonDateTime,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[schema(value_type = Option<String>, example = "2023-04-13T12:34:56Z")]
    pub updated_at: Option<BsonDateTime>,
}
