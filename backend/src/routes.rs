use actix_web::{web, get, HttpResponse, Responder};
use tera::Tera;

use crate::endpoints::{posts, projects, working_times};

macro_rules! define_routes {
    ($($method:ident $path:expr => $handler:expr),*) => {
        |cfg: &mut web::ServiceConfig| {
            $(
                cfg.route($path, web::$method().to($handler));
            )*
        }
    };
}

pub fn app(cfg: &mut web::ServiceConfig) {
    let tera = web::Data::new(Tera::new("templates/**/*.html").unwrap());

    // 各ルーティング先.
    let project_routes = define_routes!(
        get "/{id}" => projects::get_project
    );

    let working_routes = define_routes!(
        get "/{id}" => working_times::get_working_time,
        post "" => working_times::create_working_time,
        put "/{id}" => working_times::update_working_time
    );

    // ルーティング全体
    cfg.app_data(tera.clone())
        .service(crate::routes::index)
        .service(health_check)
        .service(
            web::scope("/projects")
                .configure(project_routes)
        )
        .service(
            web::scope("/working_times")
                .configure(working_routes)
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
