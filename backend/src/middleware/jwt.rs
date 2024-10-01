use crate::repositories::auth::MongoAuthRepository;
use crate::usecases::auth::AuthUseCase;
use actix_web::{dev::ServiceRequest, web, Error as ActixError, HttpMessage};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::sync::Arc;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    let token = credentials.token();

    match auth_usecase.verify_access_token(token).await {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err((
            actix_web::error::ErrorUnauthorized("Invalid or expired token"),
            req,
        )),
    }
}
