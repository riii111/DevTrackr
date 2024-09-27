use crate::dto::responses::companies::{CompanyCreatedResponse, CompanyResponse};
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
    path = "/companies/{id}",
    responses(
        (status = 200, description = "企業の取得に成功", body = CompanyResponse),
        (status = 400, description = "無効なIDです", body = ErrorResponse),
        (status = 404, description = "企業が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "企業ID")
    )
)]
#[get("/{id}")]
pub async fn get_company_by_id(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_company_by_id!!");

    let company = match usecase.get_company_by_id(&id).await {
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
    path = "/companies",
    request_body = CompanyCreate,
    responses(
        (status = 201, description = "企業の作成に成功", body = CompanyCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("")]
pub async fn create_company(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
    company: web::Json<CompanyCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_company!!");

    let company_id = usecase.create_company(company.into_inner()).await?;

    Ok(HttpResponse::Created().json(CompanyCreatedResponse::from(company_id)))
}

#[utoipa::path(
    put,
    path = "/companies/{id}",
    request_body = CompanyUpdate,
    responses(
        (status = 204, description = "企業の更新に成功"),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 404, description = "企業が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "企業ID")
    )
)]
#[put("/{id}")]
pub async fn update_company_by_id(
    usecase: web::Data<Arc<CompanyUseCase<MongoCompanyRepository>>>,
    path: web::Path<String>,
    company: web::Json<CompanyUpdate>,
) -> Result<HttpResponse, AppError> {
    info!("called PUT update_company_by_id!!");

    let obj_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    usecase
        .update_company_by_id(&obj_id, &company.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
