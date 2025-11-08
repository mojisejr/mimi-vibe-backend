//! Placeholder for referral endpoints.

use actix_web::{HttpResponse, Responder};

pub async fn claim_referral() -> impl Responder {
    HttpResponse::Ok().body("claim_referral placeholder")
}
