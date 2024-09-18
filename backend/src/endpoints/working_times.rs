use crate::{
    dto::responses::working_time::{WorkingTimeCreatedResponse, WorkingTimeResponse},
    errors::WorkingTimeError,
    models::working_times::{WorkingTimeCreate, WorkingTimeUpdate},
    repositories::working_times::MongoWorkingTimeRepository,
    usecases::working_times::WorkingTimeUseCase,
};
use actix_web::{get, post, put, web, HttpResponse, Responder};
use bson::oid::ObjectId;
use log::info;
use std::sync::Arc;

#[get("/{id}")]
pub async fn get_working_time(
    usecase: web::Data<WorkingTimeUseCase<MongoWorkingTimeRepository>>,
    id: web::Path<String>,
) -> impl Responder {
    info!("called GET get_working_time!!");

    match usecase.get_working_time_by_id(&id).await {
        Ok(Some(working_time)) => HttpResponse::Ok().json(WorkingTimeResponse::from(working_time)),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[post("")]
pub async fn create_working_time(
    usecase: web::Data<Arc<WorkingTimeUseCase<MongoWorkingTimeRepository>>>,
    working_time: web::Json<WorkingTimeCreate>,
) -> impl Responder {
    info!("called POST create_working_time!!");

    match usecase
        .create_working_time(&working_time.into_inner())
        .await
    {
        Ok(working_time_id) => {
            HttpResponse::Created().json(WorkingTimeCreatedResponse::from(working_time_id))
        }
        Err(WorkingTimeError::InvalidTimeRange) => {
            HttpResponse::BadRequest().json("開始時間は終了時間より前である必要があります")
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[put("/{id}")]
pub async fn update_working_time(
    usecase: web::Data<WorkingTimeUseCase<MongoWorkingTimeRepository>>,
    path: web::Path<String>,
    working_time: web::Json<WorkingTimeUpdate>,
) -> impl Responder {
    info!("called updated_working_time!!");

    let id = path.into_inner();
    match ObjectId::parse_str(&id) {
        Ok(obj_id) => {
            match usecase
                .update_working_time(&obj_id, &working_time.into_inner())
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
        Err(_) => HttpResponse::BadRequest().json("不正なID形式です"),
    }
}
