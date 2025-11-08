//! Placeholder for payments endpoints (Stripe integration later).

use actix_web::{HttpResponse, Responder};

pub async fn create_payment() -> impl Responder {
    HttpResponse::Ok().body("create_payment placeholder")
}
