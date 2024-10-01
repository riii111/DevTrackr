use crate::errors::app_error::AppError;
use crate::models::auth::{AuthCreate, AuthLogin, AuthRefresh};
use crate::repositories::auth::MongoAuthRepository;
use crate::usecases::auth::AuthUseCase;
use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    post,
    path = "/login",
    request_body = AuthLogin,
    responses(
        (status = 200, description = "ログインに成功", body = AuthResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("/login")]
async fn login(
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
    login_dto: web::Json<AuthLogin>,
) -> Result<impl Responder, AppError> {
    // バリデーションの実行
    login_dto
        .validate()
        .map_err(|e| AppError::ValidationError(e))?;

    let auth_response = auth_usecase
        .login(&login_dto.email, &login_dto.password)
        .await?;
    Ok(HttpResponse::Ok().json(auth_response))
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = AuthCreate,
    responses(
        (status = 201, description = "ユーザー登録に成功", body = AuthResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 409, description = "既に存在するユーザー", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("/register")]
async fn register(
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
    register_dto: web::Json<AuthCreate>,
) -> Result<impl Responder, AppError> {
    // バリデーションの実行
    register_dto
        .validate()
        .map_err(|e| AppError::ValidationError(e))?;

    let auth_response = auth_usecase
        .register(
            &register_dto.email,
            &register_dto.password,
            &register_dto.name,
        )
        .await?;
    Ok(HttpResponse::Created().json(auth_response))
}

#[utoipa::path(
    post,
    path = "/logout",
    responses(
        (status = 200, description = "ログアウトに成功"),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("/logout")]
async fn logout(
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
    req: HttpRequest,
) -> Result<impl Responder, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .unwrap_or_default(); // ミドルウェアが既に認証済なので、ヘッダーは存在する前提で進める

    auth_usecase.logout(auth_header).await?;
    Ok(HttpResponse::Ok().finish())
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = AuthRefresh,
    responses(
        (status = 200, description = "トークンのリフレッシュに成功", body = AuthResponse),
        (status = 400, description = "無効なリクエストデータ", body = ErrorResponse),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 500, description = "サーバーエラー", body = ErrorResponse)
    )
)]
#[post("/refresh")]
async fn refresh_token(
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
    refresh_token_dto: web::Json<AuthRefresh>,
) -> Result<impl Responder, AppError> {
    // バリデーションの実行
    refresh_token_dto
        .validate()
        .map_err(|e| AppError::ValidationError(e))?;

    let auth_response = auth_usecase
        .refresh_token(&refresh_token_dto.refresh_token)
        .await?;
    Ok(HttpResponse::Ok().json(auth_response))
}
