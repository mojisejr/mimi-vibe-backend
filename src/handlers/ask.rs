//! Handler for the /ask LLM endpoint.

use crate::models::{AskRequest, AskResponse};
use crate::services::llm::OpenAiClient;
use actix_web::{HttpResponse, web};
use std::sync::Arc;

/// Shared state for the ask endpoint
#[derive(Clone)]
pub struct AskState {
    pub llm_client: Arc<OpenAiClient>,
}

/// POST /ask handler
async fn ask_handler(state: web::Data<AskState>, req: web::Json<AskRequest>) -> HttpResponse {
    log::info!("Received question: {}", req.question);

    match state.llm_client.ask(&req.question).await {
        Ok((response, raw)) => {
            log::info!("Successfully generated response");
            HttpResponse::Ok().json(AskResponse { response, raw })
        }
        Err(e) => {
            log::error!("Failed to generate response: {}", e);
            HttpResponse::InternalServerError().json(serde_json::json!({
                "error": format!("Failed to generate response: {}", e)
            }))
        }
    }
}

/// Configure the /ask routes
pub fn configure(cfg: &mut web::ServiceConfig) {
    let scope = web::scope("").route("/ask", web::post().to(ask_handler));

    cfg.service(scope);
}
