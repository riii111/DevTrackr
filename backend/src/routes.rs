use actix_web::{get, web, HttpResponse, Responder, Scope};

use crate::endpoints::{auth, companies, projects, work_logs};

// 認証が不要な公開APIのスコープを定義
pub fn public_auth_scope() -> Scope {
    web::scope("/auth")
        .service(auth::login)
        .service(auth::register)
        .service(auth::refresh)
}

// 認証が必要な保護されたAPIのスコープを定義
pub fn protected_scope() -> Scope {
    web::scope("")
        .service(web::scope("/auth").service(auth::logout))
        .service(projects_scope())
        .service(work_logs_scope())
        .service(companies_scope())
}

fn projects_scope() -> Scope {
    web::scope("/projects")
        .service(projects::get_projects)
        .service(projects::get_project_by_id)
        .service(projects::create_project)
        .service(projects::update_project_by_id)
}

fn work_logs_scope() -> Scope {
    web::scope("/work_logs")
        .service(work_logs::get_all_work_logs)
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
