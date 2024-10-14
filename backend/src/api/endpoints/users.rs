use crate::dto::responses::users::UserResponse;
use crate::errors::app_error::AppError;
use crate::repositories::auth::MongoAuthRepository;
use crate::usecases::auth::AuthUseCase;
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use std::sync::Arc;

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
) -> Result<impl Responder, AppError> {
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
