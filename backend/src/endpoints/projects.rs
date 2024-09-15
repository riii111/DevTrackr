use crate::repositories::projects::ProjectRepository;
use actix_web::{web, HttpResponse, Responder};
use bson::oid::ObjectId;

pub async fn get_project<T: ProjectRepository>(
    repo: web::Data<T>,
    id: web::Path<String>,
) -> impl Responder {
    //  "&*id"について
    // "*id"でPath<String>から、ラッパー元のStringを取り外す（参照外し）.そのStringに対して参照を作成して暗黙的に&strに変換している.
    let object_id = match ObjectId::parse_str(&*id) {
        Ok(oid) => oid,
        Err(_) => return HttpResponse::BadRequest().finish(),
    };
    match repo.find_by_id(&object_id).await {
        Ok(Some(project)) => HttpResponse::Ok().json(project),
        Ok(None) => HttpResponse::NotFound().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
