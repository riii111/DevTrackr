use crate::common::response::{ApiResponse, ResponseContent};
use crate::common::response_builder::ApiResponseBuilder;
use crate::models::posts::Post;
use crate::repositories::posts_v2 as post_v2_repository;
use anyhow::Result;

pub async fn get_all_posts() -> Result<ApiResponse> {
    let posts = post_v2_repository::get_all().await?;
    Ok(ApiResponseBuilder::new()
        .status("OK".to_string())
        .result(ResponseContent::Items(posts))
        .build())
}

// pub fn get_post(id: i32) -> ApiResponse {
//     let post = post_repository::get(id);
//     ApiResponseBuilder::new()
//         .status("OK".to_string())
//         .result(ResponseContent::Item(post))
//         .build()
// }

pub async fn get_post(id: i32) -> Result<ApiResponse> {
    let post = post_v2_repository::get(id).await?;
    match post {
        Some(post) => Ok(ApiResponseBuilder::new()
            .status("OK".to_string())
            .result(ResponseContent::Item(post))
            .build()),
        None => Ok(ApiResponseBuilder::new()
            .status("Not Found".to_string())
            .result(ResponseContent::Reason("Post not found".to_string()))
            .build()),
    }
}

pub async fn create_post(post: Post) -> Result<ApiResponse> {
    let created_post = post_v2_repository::create(post).await?;
    Ok(ApiResponseBuilder::new()
        .status("OK".to_string())
        .result(ResponseContent::Item(created_post))
        .build())
}

pub async fn not_found() -> ApiResponse {
    ApiResponseBuilder::new()
        .status("NotFound".to_string())
        .result(ResponseContent::None)
        .build()
}
