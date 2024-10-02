use actix_web::cookie::Cookie;
use actix_web::HttpResponse;
use std::env;

/// リフレッシュトークンをクッキーとしてセットする関数
pub fn set_refresh_token_cookie(response: &mut HttpResponse, refresh_token: &str) {
    let secure_mode = env::var("SECURE_MODE").unwrap_or_else(|_| "false".to_string()) == "true";
    let cookie = Cookie::build("refresh_token", refresh_token.to_owned())
        .path("/auth/refresh")
        .secure(secure_mode)
        .http_only(true)
        .same_site(actix_web::cookie::SameSite::Strict)
        .finish();
    response.add_cookie(&cookie).unwrap();
}
