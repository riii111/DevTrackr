use crate::repositories::working_time::WorkingTimeRepository;
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
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
