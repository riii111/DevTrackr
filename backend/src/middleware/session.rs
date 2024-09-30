/* cookie-sessionを使う場合のコード(featuresにcookie-sessionを指定すること)  */

// use actix_session::storage::CookieSessionStore;
// use actix_session::SessionMiddleware;
// use actix_web::cookie::{Key, SameSite};

// pub fn build_cookie_session_middleware(key: Key) -> SessionMiddleware<CookieSessionStore> {
//     SessionMiddleware::builder(CookieSessionStore::default(), key)
//         .cookie_secure(
//             std::env::var("COOKIE_SECURE")
//                 .expect("COOKIE_SECUREが設定されていません")
//                 .parse()
//                 .unwrap_or(false),
//         )
//         .cookie_same_site(SameSite::Lax)
//         .build()
// }
