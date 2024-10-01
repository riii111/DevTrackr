use crate::utils::jwt;
use actix_web::dev::ServiceRequest;
use actix_web::error::ErrorUnauthorized;
use actix_web::Error;
use actix_web::HttpMessage;
use actix_web_httpauth::extractors::bearer::BearerAuth;

pub async fn validator(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<actix_web::web::Data<Config>>()
        .map(|data| data.get_ref().clone())
        .unwrap_or_else(Default::default);

    match jwt::verify_token(credentials.token(), config.jwt_secret.as_bytes()) {
        Ok(claims) => {
            req.extensions_mut().insert(claims);
            Ok(req)
        }
        Err(_) => Err(ErrorUnauthorized("Invalid token")),
    }
}

#[derive(Clone)]
pub struct Config {
    pub jwt_secret: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            jwt_secret: std::env::var("JWT_SECRET").expect("JWT_SECRET must be set"),
        }
    }
}
