use crate::{
    dto::responses::work_logs::{WorkLogCreatedResponse, WorkLogResponse},
    errors::app_error::AppError,
    models::work_logs::{WorkLogCreate, WorkLogUpdate},
    repositories::work_logs::MongoWorkLogRepository,
    usecases::work_logs::WorkLogUseCase,
};
use actix_web::{get, post, put, web, HttpResponse};
use bson::oid::ObjectId;
use log::info;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/work-logs/",
    responses(
        (status = 200, description = "勤怠の取得に成功", body = Vec<WorkLogResponse>),
        (status = 401, description = "認証失敗", body = ErrorResponse),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/")]
pub async fn get_all_work_logs(
    usecase: web::Data<Arc<WorkLogUseCase<MongoWorkLogRepository>>>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_all_work_logs!!");

    let work_logs = match usecase.get_all_work_logs().await {
        Ok(work_logs) => work_logs,
        Err(e) => return Err(e),
    };

    let response = work_logs
        .into_iter()
        .map(WorkLogResponse::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/work-logs/{id}/",
    responses(
        (status = 200, description = "勤怠の取得に成功", body = WorkLogResponse),
        (status = 400, description = "無効なIDです", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 404, description = "勤怠が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "勤怠ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/{id}/")]
pub async fn get_work_logs_by_id(
    usecase: web::Data<Arc<WorkLogUseCase<MongoWorkLogRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_work_logs_by_id!!");

    let work_logs = match usecase.get_work_logs_by_id(&id).await {
        Ok(Some(work_logs)) => work_logs,
        Ok(None) => return Err(AppError::NotFound("勤怠が見つかりません".to_string())),
        Err(e) => return Err(e),
    };

    let response = WorkLogResponse::try_from(work_logs)
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/api/work-logs/",
    request_body = WorkLogCreate,
    responses(
        (status = 201, description = "勤怠の作成に成功", body = WorkLogCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("/")]
pub async fn create_work_logs(
    usecase: web::Data<Arc<WorkLogUseCase<MongoWorkLogRepository>>>,
    create_dto: web::Json<WorkLogCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_work_logs!!");

    // バリデーションチェック
    create_dto
        .validate_all()
        .map_err(AppError::ValidationError)?;

    let work_logs_id = usecase.create_work_logs(&create_dto.into_inner()).await?;

    Ok(HttpResponse::Created().json(WorkLogCreatedResponse::from(work_logs_id)))
}

#[utoipa::path(
    put,
    path = "/api/work-logs/{id}/",
    request_body = WorkLogUpdate,
    responses(
        (status = 204, description = "勤怠の更新に成功"),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 404, description = "勤怠が見つかりません", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    params(
        ("id" = String, Path, description = "勤怠ID")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/{id}/")]
pub async fn update_work_logs_by_id(
    usecase: web::Data<Arc<WorkLogUseCase<MongoWorkLogRepository>>>,
    path: web::Path<String>,
    update_dto: web::Json<WorkLogUpdate>,
) -> Result<HttpResponse, AppError> {
    info!("called update_work_logs_by_id!!");

    let obj_id = ObjectId::parse_str(path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    // バリデーションチェック
    update_dto
        .validate_all()
        .map_err(AppError::ValidationError)?;

    usecase
        .update_work_logs(&obj_id, &update_dto.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
