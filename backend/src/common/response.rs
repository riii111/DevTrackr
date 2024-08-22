use crate::models::posts::Post;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ResponseContent {
    Items(Vec<Post>),
    Item(Post),
    Reason(String),
    None,
}

#[derive(Serialize, Debug)]
pub struct ApiResponse {
    pub status: String,
    pub result: Option<ResponseContent>,
}
