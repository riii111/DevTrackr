use crate::errors::app_error::AppError;
use crate::repositories::auth::MongoAuthRepository;
use crate::usecases::auth::AuthUseCase;
use actix_web::http::Method;
use actix_web::{dev::ServiceRequest, web, Error as ActixError};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use log::debug;
use std::sync::Arc;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
) -> Result<ServiceRequest, (ActixError, ServiceRequest)> {
    // OPTIONSリクエストの場合は認証をスキップ
    if req.method() == Method::OPTIONS {
        return Ok(req);
    }

    let token = credentials.token();

    match auth_usecase.verify_access_token(token).await {
        Ok(_) => {
            debug!("Token validation succeeded");
            Ok(req)
        }
        Err(_) => Err((
            AppError::Unauthorized("無効または期限切れのトークンです".to_string()).into(),
            req,
        )),
    }
}
