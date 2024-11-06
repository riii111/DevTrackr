use actix_web::web;
use devtrackr_api::api::endpoints::auth::{login, logout, refresh, register};

pub fn api_config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api").service(
            web::scope("/auth")
                .service(login)
                .service(register)
                .service(logout)
                .service(refresh),
        ), // 他のAPIエンドポイントもここに追加していく
    );
}
