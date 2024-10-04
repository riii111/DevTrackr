use actix_web::cookie::Cookie;
use actix_web::HttpResponse;
use std::env;

/// アクセストークンをクッキーとしてセットする関数
pub fn set_access_token_cookie(response: &mut HttpResponse, access_token: &str) {
    let secure_mode = env::var("SECURE_MODE").unwrap_or_else(|_| "false".to_string()) == "false";
    let domain = env::var("COOKIE_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let cookie = Cookie::build("access_token", access_token.to_owned())
        .path("/")
        .domain(domain)
        .secure(secure_mode)
        .http_only(false) // JSからアクセスできないようにする
        .same_site(actix_web::cookie::SameSite::Lax)
        .finish();
    response.add_cookie(&cookie).unwrap();
}

/// リフレッシュトークンをクッキーとしてセットする関数
pub fn set_refresh_token_cookie(response: &mut HttpResponse, refresh_token: &str) {
    let secure_mode = env::var("SECURE_MODE").unwrap_or_else(|_| "false".to_string()) == "false";
    let domain = env::var("COOKIE_DOMAIN").unwrap_or_else(|_| "localhost".to_string());
    let cookie = Cookie::build("refresh_token", refresh_token.to_owned())
        .path("/")
        .domain(domain)
        .secure(secure_mode)
        .http_only(false) // JSからアクセスできないようにする
        .same_site(actix_web::cookie::SameSite::Lax)
        .finish();
    response.add_cookie(&cookie).unwrap();
}
