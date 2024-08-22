use actix_session::storage::CookieSessionStore;
use actix_session::SessionMiddleware;
use actix_web::cookie::Key;
use actix_web_flash_messages::storage::SessionMessageStore;
use actix_web_flash_messages::FlashMessagesFramework;

pub fn build_cookie_session_middleware(key: Key) -> SessionMiddleware<CookieSessionStore> {
    SessionMiddleware::builder(CookieSessionStore::default(), key).build()
}

pub fn build_flash_messages_framework() -> FlashMessagesFramework {
    let message_store = SessionMessageStore::default();
    FlashMessagesFramework::builder(message_store).build()
}
