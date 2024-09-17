use crate::{
    errors::WorkingTimeError,
    models::working_time::{self, WorkingTime},
    repositories::working_time::WorkingTimeRepository,
    usecases::working_time::WorkingTimeUseCase,
};
use actix_web::{web, HttpResponse, Responder};
use bson::oid::ObjectId;

pub async fn get_working_time<T: WorkingTimeRepository>(
    repo: web::Data<T>,
    id: web::Path<String>,
) -> impl Responder {
    let object_id = match ObjectId::parse_str(&*id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match repo.find_by_id(&object_id).await {
        Ok(Some(working_time)) => HttpResponse::Ok().json(working_time),
        Ok(None) => HttpResponse::NotFound().finish(), // 仮: 見つからなかった場合も正常系として返す.
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn create_working_time<T: WorkingTimeRepository>(
    usecase: web::Data<WorkingTimeUseCase<impl WorkingTimeRepository>>,
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
