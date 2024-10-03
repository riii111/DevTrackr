use actix_cors::Cors;
use actix_web::http::{header, Method};
use std::env;

pub fn cors_middleware() -> Cors {
    let mut cors = Cors::default();

    // 許可オリジンの設定
    let origins =
        env::var("CORS_ALLOWED_ORIGINS").unwrap_or_else(|_| "http://localhost:3000".to_string());
    for origin in origins.split(',').map(str::trim) {
        cors = cors.allowed_origin(origin);
    }

    // 許可メソッドの設定
    let methods = env::var("CORS_ALLOWED_METHODS")
        .unwrap_or_else(|_| "GET,POST,PUT,DELETE,OPTIONS".to_string());
    let parsed_methods: Vec<Method> = methods
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();
    cors = cors.allowed_methods(parsed_methods);

    // 許可ヘッダーの設定
    cors = cors.allowed_headers(&[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    // クレデンシャルのサポート
    cors = cors.supports_credentials();

    // Max-Ageの設定
    cors = cors.max_age(3600);

    cors
}
