use crate::config::di;
use actix_web::cookie::Key;
use actix_web::{middleware::Logger, web, App, HttpServer};
use config::db;
use dotenv::dotenv;
use env_logger::Env;
use std::env;
use std::io::Result;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

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
    dotenv().ok();

    // ロガーの設定
    std::env::set_var("RUST_LOG", "info,actix_redis=debug,actix_web=debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    let message_framework = middleware::session::build_flash_messages_framework();

    // キャッシュレイヤ、レートリミットの値を初期化
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    log::info!("Attempting to connect to Redis at: {}", redis_url); // デバッグ用
    let mut retry_count = 0;
    let max_retries = 5;
    let mut redis_actor = None;

    // レートリミットの値を初期化
    let rate_limit_config = config::rate_limit::RateLimitConfig::from_env();

    // Redisの接続を試みる（わざと接続を遅らせる）
    while retry_count < max_retries {
        match config::redis::create_redis_actor(&redis_url) {
            Ok(actor) => {
                redis_actor = Some(actor);
                break;
            }
            Err(e) => {
                log::warn!(
                    "Failed to connect to Redis: {}. Retrying in 5 seconds...",
                    e
                );
                sleep(Duration::from_secs(5)).await;
                retry_count += 1;
            }
        }
    }

    // RedisClientの作成とArcでラップ
    let redis_client = Arc::new(utils::redis_client::RedisClient::new(
        redis_actor,
        redis_url.clone(),
    ));

    // データベースの初期化
    let db = db::init_db().await.expect("Database Initialization Failed");

    // インデックスの作成
    if let Err(e) = db::create_indexes(&db).await {
        log::error!("インデックスの作成に失敗しました: {}", e);
    }

    // 各ユースケースの初期化
    let project_usecase = di::init_project_usecase(&db);
    let company_usecase = di::init_company_usecase(&db);

    let project_usecase_clone = project_usecase.clone();
    let work_logs_usecase = di::init_work_logs_usecase(&db, project_usecase_clone);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(work_logs_usecase.clone()))
            .app_data(web::Data::new(project_usecase.clone()))
            .app_data(web::Data::new(company_usecase.clone()))
            .configure(routes::app)
            .wrap(Logger::default())
            .wrap(message_framework.clone())
            .wrap(middleware::session::build_cookie_session_middleware(
                key.clone(),
            ))
            .wrap(middleware::rate_limit::RateLimiterMiddleware::new(
                redis_client.clone(),
                rate_limit_config.clone(),
            ))
    })
    .bind(format!(
        "0.0.0.0:{}",
        dotenv::var("BACKEND_PORT").unwrap_or("8088".to_string())
    ))?
    .run()
    .await
}
