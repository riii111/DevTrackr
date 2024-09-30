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
    std::env::set_var("RUST_LOG", "info,actix_web=debug");
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // セッションキーの生成
    let key = Key::generate();
    let message_framework = middleware::session::build_flash_messages_framework();

    // RedisClientの作成
    let redis_url = env::var("REDIS_URL").expect("REDIS_URLが設定されていません");

    // レートリミットの値を初期化
    let rate_limit_config = config::rate_limit::RateLimitConfig::from_env();

    // RedisClientの生成
    let redis_client = match config::redis::create_redis_client(&redis_url) {
        Ok(client) => {
            log::info!("Successfully created Redis client");
            let timeout = Duration::from_secs(
                env::var("REDIS_TIMEOUT")
                    .unwrap_or("5".to_string())
                    .parse()
                    .unwrap_or(5),
            );
            Arc::new(utils::redis_client::RedisClient::new(client, timeout))
        }
        Err(e) => {
            log::error!("Redisクライアントの作成に失敗しました: {}", e);
            panic!("Redisクライアントの作成に失敗しました");
        }
    };

    // Redis接続テスト
    match redis_client.test_connection().await {
        Ok(response) => log::info!(
            "Successfully connected to Redis. PING response: {}",
            response
        ),
        Err(e) => log::error!("PINGコマンドの実行に失敗しました: {}", e),
    }

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
            .wrap(middleware::security_headers::SecurityHeaders)
            .wrap(middleware::cors::cors_middleware())
            // .wrap(middleware::csrf::csrf_middleware())
            .wrap(middleware::rate_limit::RateLimiterMiddleware::new(
                redis_client.clone(),
                rate_limit_config.clone(),
            ))
            .wrap(middleware::session::build_cookie_session_middleware(
                key.clone(),
            ))
            .wrap(message_framework.clone())
    })
    .bind(format!(
        "0.0.0.0:{}",
        dotenv::var("BACKEND_PORT").unwrap_or("8088".to_string())
    ))?
    .run()
    .await
}
