use actix_csrf::CsrfMiddleware;
use actix_web::cookie::SameSite;
use rand::rngs::StdRng;

// https://kvnallsn.github.io/actix-web-database-identity/actix_web/middleware/csrf/index.html
pub fn csrf_middleware() -> CsrfMiddleware<StdRng> {
    CsrfMiddleware::<StdRng>::new()
        .http_only(true) // JavaScriptからのアクセスを防ぐ
        .secure(true) // HTTPS接続でのみクッキーを送信
        .same_site(Some(SameSite::Strict)) // 同一サイトからのリクエストのみクッキーを送信
}
