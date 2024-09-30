use actix_cors::Cors;
use actix_web::http::{header, Method};
use std::env;

pub fn cors_middleware() -> Cors {
    let mut cors = Cors::default();

    // 許可オリジンの設定
    if let Ok(origins) = env::var("CORS_ALLOWED_ORIGINS") {
        let origins: Vec<&str> = origins.split(',').map(str::trim).collect();
        for origin in origins {
            cors = cors.allowed_origin(origin);
        }
    }

    // 許可メソッドの設定
    if let Ok(methods) = env::var("CORS_ALLOWED_METHODS") {
        let methods: Vec<Method> = methods
            .split(',')
            .map(str::trim)
            .filter_map(|s| s.parse().ok())
            .collect();
        cors = cors.allowed_methods(methods);
    }

    // 許可ヘッダーの設定
    cors = cors.allowed_headers(&[header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE]);

    // Max-Ageの設定
    cors = cors.max_age(3600);

    cors
}
