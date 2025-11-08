use actix_web::HttpServer;
use env_logger::Env;
use std::env;

// export library crate functions
use mimi_backend::create_app;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let port: u16 = env::var("PORT")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(8080);
    let bind = format!("0.0.0.0:{}", port);
    log::info!("Starting mimi-backend on http://{}", bind);
    HttpServer::new(create_app).bind(bind)?.run().await
}
