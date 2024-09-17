use actix_web::{web, get, HttpResponse, Responder};
use tera::Tera;

use crate::endpoints::{posts, projects, working_times};

pub fn app(cfg: &mut web::ServiceConfig) {
    let tera = web::Data::new(Tera::new("templates/**/*.html").unwrap());

    cfg.app_data(tera.clone())
        .service(crate::routes::index)
        .service(health_check)
        // projects
        .service(
            web::scope("/projects")
                // .route("", web::get().to(projects::get_all_projects))
                .route("/{id}", web::get().to(projects::get_project)),
        )
        // working_times
        .service(
            web::scope("/working_times")
                .route("/{id}", web::get().to(working_times::get_working_time))
                .route("", web::post().to(working_times::create_working_time))
                .route("", web::put().to(working_times::update_working_time))
        )
        .service(
            web::scope("/api").service(
                web::scope("/posts")
                    .route("", web::get().to(posts::index))
                    .route("/{id}", web::get().to(posts::show))
                    .route("", web::post().to(posts::create)),
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
