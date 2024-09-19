use crate::errors::ProjectError;
use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use actix_web::{get, web, HttpResponse, Responder};
use actix_web::{get, post, web, HttpResponse, Responder};
use std::sync::Arc;

#[get("/{id}")]
pub async fn get_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    id: web::Path<String>,
) -> impl Responder {
    match usecase.get_project_by_id(&id).await {
        Ok(Some(project)) => HttpResponse::Ok().json(project),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(ProjectError::InvalidId) => HttpResponse::BadRequest().finish(),
        Err(ProjectError::DatabaseError(_)) => HttpResponse::InternalServerError().finish(),
    }
}
