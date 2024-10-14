use crate::config::api_doc::ApiDoc;
use crate::config::di;
use actix_session::{storage::RedisSessionStore, SessionMiddleware};
use actix_web::cookie::{time::Duration as CookieDuration, Key};
use actix_web::{middleware::Logger, web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::middleware::HttpAuthentication;
use config::db_index;
use dotenv::dotenv;
use env_logger::Env;
use log;
use std::env;
use std::io::Result;
use std::sync::Arc;
use std::time::Duration;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;
mod config;
mod constants;
mod dto;
mod errors;
mod middleware;
mod models;
mod repositories;
mod usecases;
mod utils;

#[actix_web::main]
async fn main() -> Result<()> {
    dotenv().ok();

    // ロガーの設定
    std::env::set_var("RUST_LOG", "debug,actix_web=debug");
    env_logger::init_from_env(Env::default().default_filter_or("debug"));

    log::info!("Starting server...");

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

    // Redisセッションストアの作成
    let redis_store = RedisSessionStore::new(redis_url)
        .await
        .expect("Redisセッションストアの作成に失敗しました");
    let session_ttl = Duration::from_secs(
        env::var("SESSION_TTL")
            .map_err(|e| log::warn!("SESSION_TTL環境変数の取得に失敗: {}", e))
            .unwrap_or("3600".to_string())
            .parse()
            .map_err(|e| log::warn!("SESSION_TTLの解析に失敗: {}", e))
            .unwrap_or(3600),
    );
    let session_key = Key::from(
        env::var("SESSION_KEY")
            .expect("SESSION_KEYが設定されていません")
            .as_bytes(),
    );
    // セッションキーの長さをチェック
    if session_key.master().len() < 64 {
        panic!(
            "SESSION_KEYは少なくとも64バイト(512ビット)の長さが必要です。現在の長さ: {} バイト",
            session_key.master().len()
        );
    }

    // S3 (MinIO) クライアントの初期化
    let s3_client = match config::s3::init_s3_client().await {
        Ok(client) => {
            log::info!("Successfully initialized S3 (MinIO) client");
            client
        }
        Err(e) => {
            log::error!("S3 (MinIO) クライアントの初期化に失敗しました: {}", e);
            panic!("S3 (MinIO) クライアントの初期化に失敗しました");
        }
    };

    // データベースの初期化
    let db = db_index::init_db()
        .await
        .expect("Database Initialization Failed");

    // インデックスの作成
    if let Err(e) = db_index::create_indexes(&db).await {
        log::error!("インデックスの作成に失敗しました: {}", e);
    }

    // 各ユースケースの初期化
    let company_usecase = di::init_company_usecase(&db);
    let company_usecase_clone = company_usecase.clone();
    let project_usecase = di::init_project_usecase(&db, company_usecase_clone);

    let project_usecase_clone = project_usecase.clone();
    let work_logs_usecase = di::init_work_logs_usecase(&db, project_usecase_clone);
    let auth_usecase = di::init_auth_usecase(&db);
    let auth_usecase_clone = auth_usecase.clone();

    // JWT認証のミドルウェアを設定
    let jwt_auth_check = HttpAuthentication::bearer(move |req, credentials| {
        let auth_usecase_clone = auth_usecase.clone();
        Box::pin(async move {
            middleware::jwt::validator(req, credentials, web::Data::new(auth_usecase_clone)).await
        })
    });

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::csrf::csrf_middleware())
            .wrap(Logger::default())
            .wrap(middleware::security_headers::SecurityHeaders)
            .wrap(middleware::cors::cors_middleware())
            .wrap(middleware::rate_limit::RateLimiterMiddleware::new(
                redis_client.clone(),
                rate_limit_config.clone(),
            ))
            .wrap(
                SessionMiddleware::builder(redis_store.clone(), session_key.clone())
                    .session_lifecycle(
                        actix_session::config::PersistentSession::default()
                            .session_ttl(CookieDuration::seconds(session_ttl.as_secs() as i64)),
                    )
                    .build(),
            )
            .app_data(web::Data::new(s3_client.clone()))
            .service(SwaggerUi::new("/docs/{_:.*}").url("/docs/openapi.json", ApiDoc::openapi()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/auth")
                            .service(api::endpoints::auth::login)
                            .service(api::endpoints::auth::register)
                            .service(api::endpoints::auth::refresh)
                            .service(
                                // logoutのみ認証ミドルウェアを適用
                                web::scope("")
                                    .wrap(jwt_auth_check.clone())
                                    .service(api::endpoints::auth::logout),
                            ),
                    )
                    .service(
                        // 認証ミドルウェアを適用
                        web::scope("")
                            .wrap(jwt_auth_check.clone())
                            .service(api::routes::users_scope())
                            .service(api::routes::projects_scope())
                            .service(api::routes::work_logs_scope())
                            .service(api::routes::companies_scope()),
                    ),
            )
            .service(web::scope("/").service(web::resource("").to(index)))
            .service(web::scope("/health").service(web::resource("").to(health_check)))
            .default_service(web::route().to(not_found))
            .app_data(web::Data::new(work_logs_usecase.clone()))
            .app_data(web::Data::new(project_usecase.clone()))
            .app_data(web::Data::new(company_usecase.clone()))
            .app_data(web::Data::new(auth_usecase_clone.clone()))
    })
    .bind(format!(
        "0.0.0.0:{}",
        dotenv::var("BACKEND_PORT").unwrap_or("8088".to_string())
    ))?
    .run()
    .await
}

async fn not_found(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json("リソースが見つかりません")
}

pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

async fn health_check(_req: HttpRequest) -> impl Responder {
    log::info!("ヘルスチェックエンドポイントにアクセスがありました");
    HttpResponse::Ok().body("Healthy")
}
