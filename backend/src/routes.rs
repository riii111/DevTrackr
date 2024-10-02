use actix_web::{web, Scope};

use crate::endpoints::{companies, projects, work_logs};

pub fn projects_scope() -> Scope {
    web::scope("/projects")
        .service(projects::get_projects)
        .service(projects::get_project_by_id)
        .service(projects::create_project)
        .service(projects::update_project_by_id)
}

pub fn work_logs_scope() -> Scope {
    web::scope("/work_logs")
        .service(work_logs::get_all_work_logs)
        .service(work_logs::get_work_logs_by_id)
        .service(work_logs::create_work_logs)
        .service(work_logs::update_work_logs_by_id)
}

pub fn companies_scope() -> Scope {
    web::scope("/companies")
        .service(companies::get_all_companies)
        .service(companies::get_company_by_id)
        .service(companies::create_company)
        .service(companies::update_company_by_id)
}
