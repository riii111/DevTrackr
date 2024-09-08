use crate::dto::response::ApiResponse;
use actix_web::HttpResponse;

pub fn build_response(format: &Option<String>, response: &ApiResponse) -> HttpResponse {
    match format.as_deref() {
        Some("xml") => unimplemented!("XML format is not supported yet"),
        _ => HttpResponse::Ok().json(response),
    }
}
