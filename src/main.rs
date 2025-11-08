mod db;
mod handlers;
mod llm;
mod middleware;
mod models;
mod services;

use actix_web::{web, App, HttpResponse, HttpServer};
use handlers::AskState;
use llm::OpenAiClient;
use std::env;
use std::sync::Arc;

/// Health check endpoint
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "ok"
    }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment and logging
    dotenv::dotenv().ok();
    env_logger::init();

    log::info!("Starting MiMi Vibe Backend");

    // Read configuration from environment
    let mock_mode = env::var("MOCK_LLM")
        .unwrap_or_else(|_| "false".to_string())
        .parse::<bool>()
        .unwrap_or(false);

    let api_key = env::var("OPENAI_API_KEY").unwrap_or_else(|_| {
        if mock_mode {
            log::info!("MOCK_LLM is enabled, using dummy API key");
            "mock-api-key".to_string()
        } else {
            log::warn!("OPENAI_API_KEY not set and MOCK_LLM not enabled");
            String::new()
        }
    });

    let model = env::var("OPENAI_MODEL").unwrap_or_else(|_| {
        log::info!("OPENAI_MODEL not set, using default: gpt-3.5-turbo");
        "gpt-3.5-turbo".to_string()
    });

    log::info!("Configuration: model={}, mock_mode={}", model, mock_mode);

    // Create LLM client
// Use Arc because the OpenAiClient instance is shared by multiple actix-web
// worker threads and request handlers. Arc<T> provides thread-safe reference
// counting so we can clone cheap handles into app state without moving or
// copying the client. Do NOT use Rc<T> (not Send/Sync). If the client needs
// mutation, combine with Mutex/RwLock (e.g. Arc<Mutex<OpenAiClient>>).
// Note: actix_web::web::Data also wraps values in an Arc internally, so
// storing Arc<...> inside AskState is valid but a bit redundant.
    let llm_client = Arc::new(OpenAiClient::new(api_key, model, mock_mode));

    // Create shared state
    let ask_state = web::Data::new(AskState {
        llm_client: llm_client.clone(),
    });

    let bind_address = env::var("BIND_ADDRESS").unwrap_or_else(|_| "127.0.0.1:8080".to_string());
    log::info!("Starting server on {}", bind_address);

    HttpServer::new(move || {
        App::new()
            .app_data(ask_state.clone())
            .route("/health", web::get().to(health_check))
            .configure(handlers::configure)
    })
    .bind(&bind_address)?
    .run()
    .await
}
