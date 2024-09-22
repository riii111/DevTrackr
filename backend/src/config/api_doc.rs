use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::dto::responses::working_times::{WorkingTimeCreatedResponse, WorkingTimeResponse};
use crate::endpoints::{projects, working_times};
use crate::errors::app_error::{AppError, ErrorResponse};
use crate::models::projects::{ProjectCreate, ProjectStatus};
use crate::models::working_times::{WorkingTimeCreate, WorkingTimeUpdate};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        projects::get_project_by_id,
        projects::create_project,
        working_times::get_working_time_by_id,
        working_times::create_working_time,
        working_times::update_working_time,
    ),
    components(
        schemas(
            ProjectResponse,
            ProjectCreate,
            ProjectCreatedResponse,
            ProjectStatus,
            ErrorResponse,
            AppError,
            WorkingTimeResponse,
            WorkingTimeCreate,
            WorkingTimeCreatedResponse,
        )
    ),
    tags(
        (name = "projects", description = "プロジェクト関連のエンドポイント"),
        (name = "working_times", description = "作業時間関連のエンドポイント"),
    )
)]
pub struct ApiDoc;
