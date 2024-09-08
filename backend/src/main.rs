use actix_web::cookie::Key;
use actix_web::{middleware::Logger, App, HttpServer};
use env_logger::Env;
use std::io::Result;

mod dto;
mod endpoints;
mod middleware;
mod models;
mod repositories;
mod request_params;
mod routes;
mod usecases;

#[actix_rt::main]
async fn main() -> Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    let key = Key::generate();
    let message_framework = middleware::session::build_flash_messages_framework();

    HttpServer::new(move || {
        App::new()
            .configure(routes::app)
            .wrap(Logger::default())
            .wrap(message_framework.clone())
            .wrap(middleware::session::build_cookie_session_middleware(
                key.clone(),
            ))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
