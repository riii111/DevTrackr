use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::db;
use env_logger::Env;
use std::env;
use std::io::Result;

mod config;
mod dto;
mod endpoints;
mod middleware;
mod models;
mod repositories;
mod request_params;
mod routes;
mod usecases;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    let message_framework = middleware::session::build_flash_messages_framework();

    let db = db::init_db().await.expect("Database Initialization Failed");
    HttpServer::new(move || {
        App::new()
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
