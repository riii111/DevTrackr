use crate::config::api_doc::ApiDoc;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder, Scope};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::endpoints::{companies, projects, work_logs};

pub fn app(cfg: &mut web::ServiceConfig) {
    // ルーティング全体
    cfg.service(crate::routes::index)
        .service(health_check)
        .service(projects_scope())
        .service(work_logs_scope())
        .service(companies_scope())
        .service(
            SwaggerUi::new("/api-docs/{_:.*}").url("/api-docs/openapi.json", ApiDoc::openapi()),
        )
        .default_service(web::route().to(not_found));
}

fn projects_scope() -> Scope {
    web::scope("/projects")
        .service(projects::get_all_projects)
        .service(projects::get_project_by_id)
        .service(projects::create_project)
        .service(projects::update_project_by_id)
}

fn work_logs_scope() -> Scope {
    web::scope("/work_logs")
        .service(work_logs::get_work_logs_by_id)
        .service(work_logs::create_work_logs)
        .service(work_logs::update_work_logs_by_id)
}

fn companies_scope() -> Scope {
    web::scope("/companies")
        .service(companies::get_all_companies)
        .service(companies::get_company_by_id)
        .service(companies::create_company)
        .service(companies::update_company_by_id)
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

async fn not_found(_req: HttpRequest) -> impl Responder {
    HttpResponse::NotFound().json("リソースが見つかりません")
}
