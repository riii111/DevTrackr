use crate::api::endpoints::{auth, companies, projects, users, work_logs};
use crate::dto::responses::auth::{AuthResponse, AuthTokenCreatedResponse};
use crate::dto::responses::companies::{
    CompaniesWithProjects, CompaniesWithProjectsResponse, CompanyCreatedResponse, CompanyResponse,
};
use crate::dto::responses::projects::{ProjectCreatedResponse, ProjectResponse};
use crate::dto::responses::users::UserResponse;
use crate::dto::responses::work_logs::{WorkLogCreatedResponse, WorkLogResponse};
use crate::errors::app_error::FieldError;
use crate::errors::app_error::{AppError, ErrorResponse};
use crate::models::auth::{AuthTokenInDB, AuthTokenLogin};
use crate::models::companies::{
    AnnualSales, Bonus, CompanyCommon, CompanyCreate, CompanyStatus, CompanyUpdate, ContractType,
};
use crate::models::projects::{ProjectCreate, ProjectStatus, ProjectUpdate};
use crate::models::users::{EngineerRole, UserCreate, UserUpdate};
use crate::models::work_logs::{WorkLogCreate, WorkLogUpdate};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    paths(
        projects::get_project_by_id,
        projects::create_project,
        projects::update_project_by_id,
        projects::get_projects,
        work_logs::get_all_work_logs,
        work_logs::get_work_logs_by_id,
        work_logs::create_work_logs,
        work_logs::update_work_logs_by_id,
        companies::get_company_by_id,
        companies::create_company,
        companies::update_company_by_id,
        companies::get_all_companies,
        companies::get_all_companies_with_projects,
        auth::login,
        auth::logout,
        auth::refresh,
        auth::register,
        users::get_current_user,
        users::update_me,
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
            WorkLogResponse,
            WorkLogCreate,
            WorkLogCreatedResponse,
            WorkLogUpdate,
            CompanyResponse,
            CompaniesWithProjects,
            CompaniesWithProjectsResponse,
            CompanyCreate,
            CompanyCreatedResponse,
            CompanyUpdate,
            AnnualSales,
            Bonus,
            CompanyStatus,
            ContractType,
            CompanyCommon,
            AuthTokenLogin,
            AuthTokenInDB,
            AuthResponse,
            AuthTokenCreatedResponse,
            UserResponse,
            UserCreate,
            UserUpdate,
            EngineerRole,
            FieldError,
        )
    ),
    tags(
        (name = "projects", description = "プロジェクト関連のエンドポイント"),
        (name = "work_logs", description = "勤怠関連のエンドポイント"),
        (name = "companies", description = "企業関連のエンドポイント"),
        (name = "auth", description = "認証関連のエンドポイント"),
        (name = "users", description = "ユーザー関連のエンドポイント"),
    ),
    modifiers(&SecurityAddon)
)]
pub struct ApiDoc;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "bearer_auth",
            utoipa::openapi::security::SecurityScheme::Http(
                utoipa::openapi::security::HttpBuilder::new()
                    .scheme(utoipa::openapi::security::HttpAuthScheme::Bearer)
                    .bearer_format("JWT")
                    .build(),
            ),
        );
    }
}
