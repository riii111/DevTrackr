use actix_csrf::CsrfMiddleware;
use actix_web::cookie::SameSite;
use actix_web::http::Method;
use rand::rngs::StdRng;

pub fn csrf_middleware() -> CsrfMiddleware<StdRng> {
    CsrfMiddleware::<StdRng>::new()
        .set_cookie(Method::GET, "/login") // クッキーを設定するエンドポイント
        .set_cookie(Method::GET, "/register")
        .http_only(true) // JavaScriptからのアクセスを防ぐ
        .secure(true) // HTTPS接続でのみクッキーを送信
        .same_site(Some(SameSite::Strict)) // 同一サイトからのリクエストのみクッキーを送信
}
