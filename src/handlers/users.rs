//! Placeholder for users endpoints.

use actix_web::{HttpResponse, Responder};

pub async fn get_profile() -> impl Responder {
    HttpResponse::Ok().body("get_profile placeholder")
}
