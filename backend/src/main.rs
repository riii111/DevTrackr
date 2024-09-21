use crate::config::di;
use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::dto::responses::working_times::{WorkingTimeCreatedResponse, WorkingTimeResponse};
use crate::endpoints::{projects, working_times};
use crate::errors::app_error::{AppError, ErrorResponse};
use crate::models::projects::{ProjectCreate, ProjectStatus};
use crate::models::working_times::WorkingTimeCreate;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::db;
use env_logger::Env;
use std::env;
use std::io::Result;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
mod dto;
mod endpoints;
mod errors;
mod middleware;
mod models;
mod repositories;
mod request_params;
mod routes;
mod usecases;
mod utils;

#[derive(OpenApi)]
#[openapi(
    paths(
        projects::get_project_by_id,
        projects::create_project,
        working_times::get_working_time_by_id,
        working_times::create_working_time,
        // working_times::update_working_time,
    ),
    components(
        schemas(
            ProjectResponse,
            ProjectCreate,
            ProjectCreatedResponse,
            ProjectStatus,
            ErrorResponse,
            AppError,
            WorkingTimeResponse,
            WorkingTimeCreate,
            WorkingTimeCreatedResponse,
        )
    ),
    tags(
        (name = "projects", description = "プロジェクト関連のエンドポイント"),
        (name = "working_times", description = "作業時間関連のエンドポイント"),
    )
)]
struct ApiDoc;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    let message_framework = middleware::session::build_flash_messages_framework();

    let db = db::init_db().await.expect("Database Initialization Failed");

    // 各ユースケースの初期化
    let working_time_usecase = di::init_working_time_usecase(&db);
    let project_usecase = di::init_project_usecase(&db);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(working_time_usecase.clone()))
            .app_data(web::Data::new(project_usecase.clone()))
            .configure(routes::app)
            .wrap(Logger::default())
            .wrap(message_framework.clone())
            .wrap(middleware::session::build_cookie_session_middleware(
                key.clone(),
            ))
            .service(
                SwaggerUi::new("/api-docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
            )
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("BACKEND_PORT").unwrap_or("8088".to_string())
    ))?
    .run()
    .await
}
