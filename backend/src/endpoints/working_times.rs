use crate::{
    errors::WorkingTimeError, models::working_times::WorkingTime,
    repositories::working_times::MongoWorkingTimeRepository,
    usecases::working_times::WorkingTimeUseCase,
};
use actix_web::{get, post, put, web, HttpResponse, Responder};

#[get("/{id}")]
pub async fn get_working_time(
    usecase: web::Data<WorkingTimeUseCase<MongoWorkingTimeRepository>>,
    id: web::Path<String>,
) -> impl Responder {
    match usecase.get_working_time_by_id(&id).await {
        Ok(Some(working_time)) => HttpResponse::Ok().json(working_time),
        Ok(None) => HttpResponse::NotFound().finish(), // 仮: 見つからなかった場合も正常系として返す.
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("/")]
pub async fn create_working_time(
    usecase: web::Data<WorkingTimeUseCase<MongoWorkingTimeRepository>>,
    working_time: web::Json<WorkingTime>,
) -> impl Responder {
    match usecase
        .create_working_time(&working_time.into_inner())
        .await
    {
        Ok(working_time_id) => HttpResponse::Created().json(working_time_id),
        Err(WorkingTimeError::InvalidTimeRange) => {
            HttpResponse::BadRequest().json("開始時間は終了時間より前である必要があります")
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}")]
pub async fn update_working_time(
    usecase: web::Data<WorkingTimeUseCase<MongoWorkingTimeRepository>>,
    working_time: web::Json<WorkingTime>,
) -> impl Responder {
    let working_time_inner = working_time.into_inner();
    match usecase
        .update_working_time(&working_time_inner.id, &working_time_inner)
        .await
    {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(WorkingTimeError::InvalidTimeRange) => {
            HttpResponse::BadRequest().json("開始時間は終了時間より前である必要があります")
        }
        Err(WorkingTimeError::NotFound) => {
            HttpResponse::NotFound().json("更新対象のIDが見つかりませんでした")
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
