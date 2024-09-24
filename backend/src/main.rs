use crate::config::di;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use adapters::async_queue;
use config::db;
use env_logger::Env;
use std::env;
use std::io::Result;

mod adapters;
mod config;
mod dto;
mod endpoints;
mod errors;
mod middleware;
mod models;
mod repositories;
mod routes;
mod usecases;
mod utils;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    let message_framework = middleware::session::build_flash_messages_framework();

    // DBの初期化
    let db = db::init_db().await.expect("Database Initialization Failed");

    // 非同期キューの初期化
    let (queue_adapter, receiver) = di::init_async_queue();
    tokio::spawn(async move { async_queue::run_async_queue_worker(receiver).await });

    // 各ユースケースの初期化
    let working_time_usecase = di::init_working_time_usecase(db.clone(), queue_adapter);
    let project_usecase = di::init_project_usecase(db.clone());

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
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("BACKEND_PORT").unwrap_or("8088".to_string())
    ))?
    .run()
    .await
}
