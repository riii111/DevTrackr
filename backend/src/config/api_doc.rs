use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::dto::responses::work_logs::{WorkLogsCreatedResponse, WorkLogsResponse};
use crate::endpoints::{projects, work_logs};
use crate::errors::app_error::{AppError, ErrorResponse};
use crate::models::projects::{ProjectCreate, ProjectStatus, ProjectUpdate};
use crate::models::work_logs::{WorkLogsCreate, WorkLogsUpdate};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        projects::get_project_by_id,
        projects::create_project,
        projects::update_project_by_id,
        work_logs::get_work_logs_by_id,
        work_logs::create_work_logs,
        work_logs::update_work_logs_by_id,
    ),
    components(
        schemas(
            ProjectResponse,
            ProjectCreate,
            ProjectCreatedResponse,
            ProjectUpdate,
            ProjectStatus,
            ErrorResponse,
            AppError,
            WorkLogsResponse,
            WorkLogsCreate,
            WorkLogsCreatedResponse,
            WorkLogsUpdate,
        )
    ),
    tags(
        (name = "projects", description = "プロジェクト関連のエンドポイント"),
        (name = "work_logs", description = "勤怠関連のエンドポイント"),
    )
)]
pub struct ApiDoc;
