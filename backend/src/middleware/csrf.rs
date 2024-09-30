use actix_csrf::CsrfMiddleware;
use actix_web::cookie::SameSite;
use actix_web::http::Method;
use rand::rngs::StdRng;

pub fn csrf_middleware() -> CsrfMiddleware<StdRng> {
    CsrfMiddleware::<StdRng>::new()
        .set_cookie(Method::GET, "/login") // ログインページでCSRFトークンを設定
        .set_cookie(Method::GET, "/register") // 登録ページでCSRFトークンを設定
        .http_only(true) // JavaScriptからのアクセスを防ぐ
        .secure(true) // HTTPS接続でのみクッキーを送信
        .same_site(Some(SameSite::Strict)) // 同一サイトからのリクエストのみクッキーを送信
}
