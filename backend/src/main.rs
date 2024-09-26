use crate::config::di;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use adapters::async_queue_worker;
use config::db;
use env_logger::Env;
use std::env;
use std::io::Result;
use std::sync::Arc;

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

    // ユースケース初期化
    let project_usecase = di::init_project_usecase(db.clone());
    let working_time_usecase = di::init_working_time_usecase(db.clone());

    // // キュー初期化とタスク生成
    // let (queue_adapter, receiver) = di::init_async_queue();
    // let working_time_usecase = di::init_working_time_usecase(
    // db.clone(),
    // Arc::new(queue_adapter), project_usecase.clone()
    // );

    // `working_time_usecase` をクローンしてクロージャに渡す
    // let working_time_usecase_clone = working_time_usecase.clone();

    // tokio::spawn(async move {
    //     async_queue_worker::run_async_queue_worker(receiver, working_time_usecase_clone).await
    // });

    // let working_time_usecase_for_server = working_time_usecase.clone();
    HttpServer::new(move || {
        App::new()
            // .app_data(web::Data::new(working_time_usecase_for_server.clone()))
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
