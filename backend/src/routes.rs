use actix_web::{get, web, HttpResponse, Responder, Scope};

use crate::endpoints::{posts, projects, working_times};

pub fn app(cfg: &mut web::ServiceConfig) {
    // ルーティング全体
    cfg.service(crate::routes::index)
        .service(health_check)
        .service(projects_scope())
        .service(working_times_scope())
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

fn projects_scope() -> Scope {
    web::scope("/projects")
        .service(projects::get_project_by_id)
        .service(projects::create_project)
}

fn working_times_scope() -> Scope {
    web::scope("/working_times")
        .service(working_times::get_working_time_by_id)
        .service(working_times::create_working_time)
        .service(working_times::update_working_time)
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
