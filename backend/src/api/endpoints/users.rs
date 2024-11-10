use crate::dto::responses::users::UserResponse;
use crate::errors::app_error::AppError;
use crate::models::users::UserUpdate;
use crate::repositories::auth::MongoAuthRepository;
use crate::usecases::auth::AuthUseCase;
use actix_web::{get, put, web, HttpRequest, HttpResponse};
use std::sync::Arc;
use validator::Validate;

#[utoipa::path(
    get,
    path = "/api/users/me/",
    responses(
        (status = 200, description = "ユーザー情報の取得に成功", body = UserResponse),
        (status = 401, description = "認証エラー"),
        (status = 500, description = "内部サーバーエラー")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[get("/me/")]
pub async fn get_current_user(
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
    req: HttpRequest,
) -> Result<HttpResponse, AppError> {
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("認証ヘッダーが見つかりません".to_string()))?;

    let token = auth_header.trim_start_matches("Bearer ");
    let user = auth_usecase.get_current_user(token).await?;
    let user_response = UserResponse::from(user);
    Ok(HttpResponse::Ok().json(user_response))
}

#[utoipa::path(
    put,
    path = "/api/users/me/",
    request_body = UserUpdate,
    responses(
        (status = 204, description = "ユーザー情報の更新に成功"),
        (status = 400, description = "無効なリクエストデータ"),
        (status = 401, description = "認証失敗", body = ErrorResponse),
        (status = 404, description = "ユーザーが見つかりません"),
        (status = 500, description = "内部サーバーエラー")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
#[put("/me/")]
pub async fn update_me(
    auth_usecase: web::Data<Arc<AuthUseCase<MongoAuthRepository>>>,
    req: HttpRequest,
    update_dto: web::Json<UserUpdate>,
) -> Result<HttpResponse, AppError> {
    let update_dto = update_dto.into_inner();

    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized("認証ヘッダーが見つかりません".to_string()))?;

    let token = auth_header.trim_start_matches("Bearer ");

    // バリデーションの実行
    update_dto.validate().map_err(AppError::ValidationError)?;

    auth_usecase.update_me(token, &update_dto).await?;
    Ok(HttpResponse::NoContent().finish())
}
