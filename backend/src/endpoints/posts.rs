/* API用のposts */

use crate::dto::response::ResponseContent;
use crate::dto::response_builder::ApiResponseBuilder;
use crate::middleware::response_formatter::build_response;
use crate::models::posts::Post;
use crate::request_params::PostQueries;
use crate::usecases::posts as post_service;
use actix_web::{web, HttpRequest, HttpResponse};
use chrono::{DateTime, Local};
use log::{error, info};

// pub async fn index(_req: HttpRequest, query: web::Query<PostQueries>) -> impl Responder {
//     info!("Called index API");
//     let param = query.into_inner();
//     let response = post_service::get_all_posts();
//     build_response(&param.format, &response)
// }

pub async fn index(_req: HttpRequest, query: web::Query<PostQueries>) -> HttpResponse {
    info!("Called index API");
    let param = query.into_inner();
    match post_service::get_all_posts().await {
        Ok(response) => build_response(&param.format, &response),
        Err(e) => {
            error!("Error in index: {:?}", e);
            HttpResponse::InternalServerError().json(
                ApiResponseBuilder::new()
                    .status("Error".to_string())
                    .result(ResponseContent::Reason("Internal server error".to_string()))
                    .build(),
            )
        }
    }
}

// pub async fn show(info: web::Path<i32>, query: web::Query<PostQueries>) -> impl Responder {
//     info!("Called show API");
//     let info = info.into_inner();
//     let param = query.into_inner();
//     let response = post_service::get_post(info);
//     build_response(&param.format, &response)
// }
pub async fn show(info: web::Path<i32>, query: web::Query<PostQueries>) -> HttpResponse {
    info!("Called show API");
    let id = info.into_inner();
    let param = query.into_inner();
    match post_service::get_post(id).await {
        Ok(response) => build_response(&param.format, &response),
        Err(e) => {
            error!("Error in show: {:?}", e);
            HttpResponse::InternalServerError().json(
                ApiResponseBuilder::new()
                    .status("Error".to_string())
                    .result(ResponseContent::Reason("Internal server error".to_string()))
                    .build(),
            )
        }
    }
}

// pub async fn not_found() -> impl Responder {
//     let response = post_service::not_found();
//     HttpResponse::NotFound().json(response)
// }
pub async fn not_found() -> HttpResponse {
    let response = post_service::not_found().await;
    HttpResponse::NotFound().json(response)
}

// pub async fn create(params: web::Json<Post>) -> impl Responder {
//     info!("Called create API");
//     let now: DateTime<Local> = Local::now();
//     let message = Post {
//         id: 0,
//         posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
//         sender: params.sender.clone(),
//         content: params.content.clone(),
//     };
//     let response = post_service::create_post(message);
//     let format: Option<String> = Some("json".to_string());
//     build_response(&format, &response)
// }

pub async fn create(params: web::Json<Post>) -> HttpResponse {
    info!("Called create API");
    let now: DateTime<Local> = Local::now();
    let post = Post {
        id: 0, // This will be set in the repository
        posted: now.format("%Y-%m-%d %H:%M:%S").to_string(),
        sender: params.sender.clone(),
        content: params.content.clone(),
    };

    match post_service::create_post(post).await {
        Ok(response) => {
            let format = Some("json".to_string());
            build_response(&format, &response)
        }
        Err(e) => {
            error!("Error in create: {:?}", e);
            HttpResponse::InternalServerError().json(
                ApiResponseBuilder::new()
                    .status("Error".to_string())
                    .result(ResponseContent::Reason("Internal server error".to_string()))
                    .build(),
            )
        }
    }
}
// TODO: 編集機能.

// TODO: 削除機能.
