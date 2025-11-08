//! Error handling helpers for Actix responses. Keep minimal for now.

use actix_web::HttpResponse;

/// Convert an internal error message into a JSON HTTP response.
pub fn internal_error(msg: &str) -> HttpResponse {
    HttpResponse::InternalServerError().body(msg.to_string())
}
