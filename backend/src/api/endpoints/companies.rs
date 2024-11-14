use crate::dto::responses::companies::{
    CompaniesWithProjects, CompaniesWithProjectsResponse, CompanyCreatedResponse, CompanyResponse,
};
use crate::errors::app_error::AppError;
use crate::models::companies::{CompanyCreate, CompanyUpdate};
use crate::repositories::companies::MongoCompanyRepository;
use crate::usecases::companies::CompanyUseCase;
use actix_web::{get, post, put, web, HttpResponse};
use bson::oid::ObjectId;
use log::info;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/companies/",
    responses(
        (status = 200, description = "企業の取得に成功", body = Vec<CompanyResponse>),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
#[get("/")]
pub async fn get_all_companies(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_all_companies!!");
    let companies = usecase.get_all_companies().await?;
    let response: Vec<CompanyResponse> = companies
        .into_iter()
        .map(CompanyResponse::try_from)
        .collect::<Result<_, _>>()
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/companies/with-projects/",
    responses(
        (status = 200, description = "企業の取得に成功", body = CompaniesWithProjectsResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    ),
)]
#[get("/with-projects/")]
pub async fn get_all_companies_with_projects(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_all_companies_with_projects!!");
    let companies = usecase.get_all_companies_with_projects().await?;
    let total = companies.len() as u64;
    let response: Vec<CompaniesWithProjects> = companies
        .into_iter()
        .map(CompaniesWithProjects::try_from)
        .collect::<Result<_, _>>()
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    let companies_with_projects_response = CompaniesWithProjectsResponse {
        companies: response,
        total,
    };

    Ok(HttpResponse::Ok().json(companies_with_projects_response))
}

#[utoipa::path(
    get,
    path = "/api/companies/{id}/",
    responses(
        (status = 200, description = "企業の取得に成功", body = CompanyResponse),
        (status = 400, description = "無効なIDです", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 404, description = "企業が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "企業ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/{id}/")]
pub async fn get_company_by_id(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
    path: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_company_by_id!!");

    let obj_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    let company = match usecase.get_company_by_id(&obj_id).await {
        Ok(Some(company)) => company,
        Ok(None) => return Err(AppError::NotFound("企業が見つかりません".to_string())),
        Err(e) => return Err(e), // AppErrorをそのまま返す
    };

    let response = CompanyResponse::try_from(company)
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/api/companies/",
    request_body = CompanyCreate,
    responses(
        (status = 201, description = "企業の作成に成功", body = CompanyCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/")]
pub async fn create_company(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
    company_dto: web::Json<CompanyCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_company!!");

    // バリデーションを実行
    company_dto
        .validate_all()
        .map_err(AppError::ValidationError)?;

    let company_id = usecase.create_company(company_dto.into_inner()).await?;

    Ok(HttpResponse::Created().json(CompanyCreatedResponse::from(company_id)))
}

#[utoipa::path(
    put,
    path = "/api/companies/{id}/",
    request_body = CompanyUpdate,
    responses(
        (status = 204, description = "企業の更新に成功"),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 404, description = "企業が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "企業ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/{id}/")]
pub async fn update_company_by_id(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
    path: web::Path<String>,
    update_dto: web::Json<CompanyUpdate>,
) -> Result<HttpResponse, AppError> {
    info!("called PUT update_company_by_id!!");

    let obj_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    // バリデーションを実行
    update_dto
        .validate_all()
        .map_err(AppError::ValidationError)?;

    usecase
        .update_company_by_id(&obj_id, &update_dto.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
