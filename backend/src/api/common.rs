use actix_web::{HttpRequest, HttpResponse, Responder};
use serde_json::json;

pub async fn not_found(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json(json!({
        "error": "リソースが見つかりません",
        "message": "リソースが見つかりません",
        "code": "NOT_FOUND"
    }))
}

pub async fn index(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

pub async fn health_check(_req: HttpRequest) -> impl Responder {
    log::info!("ヘルスチェックエンドポイントにアクセスがありました");
    HttpResponse::Ok().body("Healthy")
}
