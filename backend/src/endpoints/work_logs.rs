use crate::{
    dto::responses::work_logs::{WorkLogsCreatedResponse, WorkLogsResponse},
    errors::app_error::AppError,
    models::work_logs::{WorkLogsCreate, WorkLogsUpdate},
    repositories::work_logs::MongoWorkLogsRepository,
    usecases::work_logs::WorkLogsUseCase,
};
use actix_web::{get, post, put, web, HttpResponse};
use bson::oid::ObjectId;
use log::info;
use std::sync::Arc;

#[utoipa::path(
    get,
    path = "/api/work_logs",
    responses(
        (status = 200, description = "勤怠の取得に成功", body = Vec<WorkLogsResponse>)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("")]
pub async fn get_all_work_logs(
    usecase: web::Data<Arc<WorkLogsUseCase<MongoWorkLogsRepository>>>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_all_work_logs!!");

    let work_logs = match usecase.get_all_work_logs().await {
        Ok(work_logs) => work_logs,
        Err(e) => return Err(e),
    };

    let response = work_logs
        .into_iter()
        .map(WorkLogsResponse::try_from)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    get,
    path = "/api/work_logs/{id}",
    responses(
        (status = 200, description = "勤怠の取得に成功", body = WorkLogsResponse),
        (status = 400, description = "無効なIDです", body = ErrorResponse),
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
#[get("/{id}")]
pub async fn get_work_logs_by_id(
    usecase: web::Data<Arc<WorkLogsUseCase<MongoWorkLogsRepository>>>,
    id: web::Path<String>,
) -> Result<HttpResponse, AppError> {
    info!("called GET get_work_logs_by_id!!");

    let work_logs = match usecase.get_work_logs_by_id(&id).await {
        Ok(Some(work_logs)) => work_logs,
        Ok(None) => return Err(AppError::NotFound("勤怠が見つかりません".to_string())),
        Err(e) => return Err(e),
    };

    let response = WorkLogsResponse::try_from(work_logs)
        .map_err(|e| AppError::InternalServerError(format!("データの変換に失敗しました: {}", e)))?;

    Ok(HttpResponse::Ok().json(response))
}

#[utoipa::path(
    post,
    path = "/api/work_logs",
    request_body = WorkLogsCreate,
    responses(
        (status = 201, description = "勤怠の作成に成功", body = WorkLogsCreatedResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[post("")]
pub async fn create_work_logs(
    usecase: web::Data<Arc<WorkLogsUseCase<MongoWorkLogsRepository>>>,
    work_logs: web::Json<WorkLogsCreate>,
) -> Result<HttpResponse, AppError> {
    info!("called POST create_work_logs!!");

    // バリデーションチェック
    work_logs
        .validate_all()
        .map_err(|e| AppError::ValidationError(e))?;

    let work_logs_id = usecase.create_work_logs(&work_logs.into_inner()).await?;

    Ok(HttpResponse::Created().json(WorkLogsCreatedResponse::from(work_logs_id)))
}

#[utoipa::path(
    put,
    path = "/api/work_logs/{id}",
    request_body = WorkLogsUpdate,
    responses(
        (status = 204, description = "勤怠の更新に成功"),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
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
#[put("/{id}")]
pub async fn update_work_logs_by_id(
    usecase: web::Data<Arc<WorkLogsUseCase<MongoWorkLogsRepository>>>,
    path: web::Path<String>,
    work_logs: web::Json<WorkLogsUpdate>,
) -> Result<HttpResponse, AppError> {
    info!("called update_work_logs_by_id!!");

    let obj_id = ObjectId::parse_str(&path.into_inner())
        .map_err(|_| AppError::BadRequest("無効なIDです".to_string()))?;

    // バリデーションチェック
    work_logs
        .validate_all()
        .map_err(|e| AppError::ValidationError(e))?;

    usecase
        .update_work_logs(&obj_id, &work_logs.into_inner())
        .await?;

    Ok(HttpResponse::NoContent().finish())
}
