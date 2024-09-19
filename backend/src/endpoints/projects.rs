use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::errors::app_error::AppError;
use crate::models::projects::ProjectCreate;
use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use actix_web::{get, post, web, HttpResponse};
use log::info;
use std::sync::Arc;
use utoipa::OpenApi;

#[get("/{id}")]
pub async fn get_project_by_id(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_project_by_id!!");
    let project = usecase
        .get_project_by_id(&id)
        .await?
        .ok_or(AppError::NotFound)?;
    Ok(HttpResponse::Ok().json(ProjectResponse::try_from(project)))
}

#[post("")]
pub async fn create_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    project: web::Json<ProjectCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_project!!");
    let project_id = usecase.create_project(project.into_inner()).await?;
    Ok(HttpResponse::Created().json(ProjectCreatedResponse::from(project_id)))
}
