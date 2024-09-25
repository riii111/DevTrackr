use crate::{
    dto::responses::working_times::{WorkingTimeCreatedResponse, WorkingTimeResponse},
    errors::app_error::AppError,
    models::working_times::{WorkingTimeCreate, WorkingTimeUpdate},
    repositories::working_times::MongoWorkingTimeRepository,
    usecases::working_times::WorkingTimeUseCase,
};
use actix_web::{get, post, put, web, HttpResponse};
use bson::oid::ObjectId;
use log::info;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/working_times/{id}",
    responses(
        (status = 200, description = "勤怠の取得に成功", body = WorkingTimeResponse),
        (status = 400, description = "無効なIDです", body = ErrorResponse),
        (status = 404, description = "勤怠が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "勤怠ID")
    )
)]
#[get("/{id}")]
pub async fn get_working_time_by_id(
    usecase: web::Data<Arc<WorkingTimeUseCase<MongoWorkingTimeRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_working_time_by_id!!");

    let working_time = match usecase.get_working_time_by_id(&id).await {
        Ok(Some(working_time)) => working_time,
        Ok(None) => return Err(AppError::NotFound("勤怠が見つかりません".to_string())),
        Err(e) => return Err(e),
    };

    let response = WorkingTimeResponse::try_from(working_time)
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/working_times",
    request_body = WorkingTimeCreate,
    responses(
        (status = 201, description = "勤怠の作成に成功", body = WorkingTimeCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("")]
pub async fn create_working_time(
    usecase: web::Data<Arc<WorkingTimeUseCase<MongoWorkingTimeRepository>>>,
    working_time: web::Json<WorkingTimeCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_working_time!!");

    let working_time_id = usecase
        .create_working_time(&working_time.into_inner())
        .await?;
    info!("end POST create_working_time!!");
    Ok(HttpResponse::Created().json(WorkingTimeCreatedResponse::from(working_time_id)))
}

#[utoipa::path(
    put,
    path = "/working_times/{id}",
    request_body = WorkingTimeUpdate,
    responses(
        (status = 204, description = "勤怠の更新に成功"),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 404, description = "勤怠が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "勤怠ID")
    )
)]
#[put("/{id}")]
pub async fn update_working_time_by_id(
    usecase: web::Data<Arc<WorkingTimeUseCase<MongoWorkingTimeRepository>>>,
    path: web::Path<String>,
    working_time: web::Json<WorkingTimeUpdate>,
) -> Result<HttpResponse, AppError> {
    info!("called update_working_time_by_id!!");

    let obj_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    usecase
        .update_working_time(&obj_id, &working_time.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
