use actix_cors::Cors;
use actix_web::http::{header, Method};
use log;
use std::env;

pub fn cors_middleware() -> Cors {
    let mut cors = Cors::default();

    // 許可オリジンの設定
    let origins = env::var("CORS_ALLOWED_ORIGINS")
        .map_err(|e| {
            log::warn!("CORS_ALLOWED_ORIGINSの取得に失敗しました: {}", e);
            e
        })
        .unwrap_or_else(|_| "http://localhost:3000".to_string());
    for origin in origins.split(',').map(str::trim) {
        cors = cors.allowed_origin(origin);
    }

    // 許可メソッドの設定
    let methods = env::var("CORS_ALLOWED_METHODS")
        .map_err(|e| {
            log::warn!("CORS_ALLOWED_METHODSの取得に失敗しました: {}", e);
            e
        })
        .unwrap_or_else(|_| "GET,POST,PUT,OPTIONS".to_string());
    let parsed_methods: Vec<Method> = methods
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    cors = cors.allowed_methods(parsed_methods);

    // 許可ヘッダーの設定
    cors = cors.allowed_headers(&[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    // 公開ヘッダーの設定
    cors = cors.expose_headers(&[header::SET_COOKIE]);

    // クレデンシャルのサポート
    cors = cors.supports_credentials();

    // Max-Ageの設定
    cors = cors.max_age(3600);

    cors
}
