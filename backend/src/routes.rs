use actix_web::web;
use actix_web::{get, HttpResponse, Responder};
use tera::Tera;

use crate::endpoints::posts;
use crate::endpoints::projects;
use crate::repositories::projects::MongoProjectRepository;

pub fn app(cfg: &mut web::ServiceConfig) {
    let tera = web::Data::new(Tera::new("templates/**/*.html").unwrap());

    cfg.app_data(tera.clone())
        .service(crate::routes::index)
        .service(health_check)
        .service(
            web::scope("/projects")
                // .route("", web::get().to(projects::get_all_projects))
                .route("/{id}", web::get().to(projects::get_project)),
        )
        .service(
            web::scope("/api").service(
                web::scope("/posts")
                    .route("", web::get().to(posts::index))
                    .route("/{id}", web::get().to(posts::show))
                    .route("", web::post().to(posts::create)),
                // .route("", web::put().to(posts::update)),
            ),
        )
        .default_service(web::to(crate::endpoints::posts::not_found));
}

#[get("/")]
pub async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, Actix Web!")
}

#[get("/health")]
async fn health_check() -> impl Responder {
    log::info!("ヘルスチェックエンドポイントにアクセスがありました");
    HttpResponse::Ok().body("Healthy")
}
