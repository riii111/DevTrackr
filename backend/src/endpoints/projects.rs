use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::errors::ProjectError;
use crate::models::projects::ProjectCreate;
use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use actix_web::{get, post, web, HttpResponse, Responder};
use log::info;
use std::sync::Arc;

#[get("/{id}")]
pub async fn get_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    id: web::Path<String>,
) -> impl Responder {
    match usecase.get_project_by_id(&id).await {
        Ok(Some(project)) => HttpResponse::Ok().json(ProjectResponse::try_from(project)),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(ProjectError::InvalidId) => HttpResponse::BadRequest().finish(),
        Err(ProjectError::DatabaseError(_)) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
pub async fn create_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    project: web::Json<ProjectCreate>,
) -> impl Responder {
    info!("called POST create_working_time!!");
    match usecase.create_project(project.into_inner()).await {
        Ok(project_id) => HttpResponse::Created().json(ProjectCreatedResponse::from(project_id)),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
