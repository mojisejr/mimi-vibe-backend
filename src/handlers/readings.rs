//! Placeholder for readings endpoints.

use actix_web::{HttpResponse, Responder};

pub async fn create_reading() -> impl Responder {
    HttpResponse::Ok().body("create_reading placeholder")
}
