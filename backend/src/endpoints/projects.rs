use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::errors::app_error::AppError;
use crate::models::projects::ProjectCreate;
use crate::repositories::projects::MongoProjectRepository;
use crate::usecases::projects::ProjectUseCase;
use actix_web::{get, post, web, HttpResponse};
use log::info;
use std::sync::Arc;
// use utoipa::OpenApi;

#[utoipa::path(
    get,
    path = "/projects/{id}",
    responses(
        (status = 200, description = "プロジェクトの取得に成功", body = ProjectResponse),
        (status = 404, description = "プロジェクトが見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "プロジェクトID")
    )
)]
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

#[utoipa::path(
    post,
    path = "/projects",
    request_body = ProjectCreate,
    responses(
        (status = 201, description = "プロジェクトの作成に成功", body = ProjectCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("")]
pub async fn create_project(
    usecase: web::Data<Arc<ProjectUseCase<MongoProjectRepository>>>,
    project: web::Json<ProjectCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_project!!");
    let project_id = usecase.create_project(project.into_inner()).await?;
    Ok(HttpResponse::Created().json(ProjectCreatedResponse::from(project_id)))
}
