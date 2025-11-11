//! MiMi Vibe Backend library - exposes modules for testing and reuse.

pub mod db;
pub mod handlers;
pub mod middleware;
pub mod models;
pub mod services;

use actix_web::{App, HttpResponse, Responder, web};

async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({ "status": "ok" }))
}

pub fn create_app() -> App<
    impl actix_web::dev::ServiceFactory<
        actix_web::dev::ServiceRequest,
        Config = (),
        Response = actix_web::dev::ServiceResponse,
        Error = actix_web::Error,
        InitError = (),
    >,
> {
    App::new().route("/health", web::get().to(health))
}
