use crate::dto::responses::projects::ProjectCreatedResponse;
use crate::errors::app_error::AppError;
use crate::models::projects::ProjectCreate;
use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use actix_web::{get, post, web, HttpResponse};
use log::info;
use std::sync::Arc;

#[get("/{id}")]
pub async fn get_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_project!!");
    let project = usecase.get_project_by_id(&id).await?;
    Ok(HttpResponse::Ok().json(project))
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
