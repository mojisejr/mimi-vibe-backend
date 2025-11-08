# MiMiVibes Rust Backend - Starter Templates

## Project Setup

### Cargo.toml Template

```toml
[package]
name = "mimi-backend"
version = "1.0.0"
edition = "2021"

[dependencies]
# Web framework
actix-web = "4.8"
actix-rt = "2.10"
actix-cors = "0.7"
actix-middleware = "0.1"

# Async runtime
tokio = { version = "1.40", features = ["full"] }
futures-util = "0.3"

# Database
sqlx = { version = "0.8", features = [
    "runtime-tokio-rustls", 
    "postgres", 
    "json",
    "uuid",
    "chrono"
] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Authentication & Security
jsonwebtoken = "9.3"
uuid = { version = "1.0", features = ["v4", "serde"] }
argon2 = "0.5"

# Redis/Queue
redis = { version = "0.25", features = [
    "connection-manager", 
    "json", 
    "aio"
] }

# HTTP Client
reqwest = { version = "0.12", features = ["json"] }

# Stripe
stripe = "0.16"

# AI/LLM
langchain-rust = "0.4"

# Rate limiting
dashmap = "5.5"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }

# Utilities
dotenv = "0.15"
chrono = { version = "0.4", features = ["serde"] }
anyhow = "1.0"
thiserror = "1.0"

[dev-dependencies]
actix-test = "0.1"
tokio-test = "0.4"

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
```

---

## Main.rs Template

```rust
// src/main.rs

use actix_web::{middleware, web, App, HttpServer};
use sqlx::PgPool;
use std::sync::Arc;
use tracing_subscriber;

mod config;
mod handlers;
mod middleware as mw;
mod models;
mod services;
mod db;
mod errors;

use config::Config;
use handlers::{readings, auth, payments, users};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_writer(std::io::stdout)
        .init();

    // Load environment variables
    dotenv::dotenv().ok();
    let config = Config::from_env();

    tracing::info!("Starting MiMiVibes backend...");
    tracing::info!("Environment: {:?}", config.environment);

    // Database pool
    let pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database migrations completed");

    // Create shared state
    let app_state = web::Data::new(AppState {
        pool: pool.clone(),
        config: config.clone(),
    });

    let redis_url = config.upstash_redis_url.clone();
    let queue_service = services::QueueService::new(&redis_url)
        .await
        .expect("Failed to create queue service");
    let queue_service = web::Data::new(queue_service);

    let ai_service = services::AiService::new(&config.openai_api_key);
    let ai_service = web::Data::new(ai_service);

    tracing::info!("Starting HTTP server on 0.0.0.0:8080");

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Middleware
            .wrap(middleware::Logger::default())
            .wrap(middleware::NormalizePath::trim())
            .wrap(mw::CorsMiddleware)
            
            // App state
            .app_data(app_state.clone())
            .app_data(queue_service.clone())
            .app_data(ai_service.clone())

            // Health check
            .route("/api/health", web::get().to(handlers::health_check))
            
            // Auth routes
            .service(
                web::scope("/api/auth")
                    .route("/line/login", web::get().to(auth::line_login))
                    .route("/line/callback", web::post().to(auth::line_callback))
            )

            // Reading routes
            .service(
                web::scope("/api/readings")
                    .route("/submit", web::post().to(readings::submit))
                    .route("/status/{id}", web::get().to(readings::check_status))
                    .route("/history", web::get().to(readings::get_history))
            )

            // Payment routes
            .service(
                web::scope("/api/payments")
                    .route("/create-intent", web::post().to(payments::create_intent))
                    .route("/webhook", web::post().to(payments::webhook))
                    .route("/packages", web::get().to(payments::get_packages))
            )

            // User routes
            .service(
                web::scope("/api/users")
                    .route("/profile", web::get().to(users::get_profile))
                    .route("/credits", web::get().to(users::get_credits))
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}

pub struct AppState {
    pub pool: PgPool,
    pub config: Config,
}
```

---

## Config Module

```rust
// src/config.rs

use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub upstash_redis_url: String,
    pub upstash_redis_token: String,
    pub line_channel_id: String,
    pub line_channel_secret: String,
    pub stripe_secret_key: String,
    pub stripe_webhook_secret: String,
    pub jwt_secret: String,
    pub openai_api_key: String,
    pub environment: String,
}

impl Config {
    pub fn from_env() -> Self {
        Config {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL not set"),
            upstash_redis_url: env::var("UPSTASH_REDIS_REST_URL")
                .expect("UPSTASH_REDIS_REST_URL not set"),
            upstash_redis_token: env::var("UPSTASH_REDIS_REST_TOKEN")
                .expect("UPSTASH_REDIS_REST_TOKEN not set"),
            line_channel_id: env::var("LINE_CHANNEL_ID")
                .expect("LINE_CHANNEL_ID not set"),
            line_channel_secret: env::var("LINE_CHANNEL_SECRET")
                .expect("LINE_CHANNEL_SECRET not set"),
            stripe_secret_key: env::var("STRIPE_SECRET_KEY")
                .expect("STRIPE_SECRET_KEY not set"),
            stripe_webhook_secret: env::var("STRIPE_WEBHOOK_SECRET")
                .expect("STRIPE_WEBHOOK_SECRET not set"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET not set"),
            openai_api_key: env::var("OPENAI_API_KEY")
                .expect("OPENAI_API_KEY not set"),
            environment: env::var("ENVIRONMENT")
                .unwrap_or_else(|_| "development".to_string()),
        }
    }
}
```

---

## Error Handling

```rust
// src/errors.rs

use actix_web::{error, http::StatusCode, HttpResponse};
use serde_json::json;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    BadRequest(String),
    Unauthorized(String),
    Forbidden(String),
    Conflict(String),
    PaymentFailed(String),
    RateLimited,
    InternalServerError(String),
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::BadRequest(msg) => write!(f, "Bad request: {}", msg),
            ApiError::Unauthorized(msg) => write!(f, "Unauthorized: {}", msg),
            ApiError::Forbidden(msg) => write!(f, "Forbidden: {}", msg),
            ApiError::Conflict(msg) => write!(f, "Conflict: {}", msg),
            ApiError::PaymentFailed(msg) => write!(f, "Payment failed: {}", msg),
            ApiError::RateLimited => write!(f, "Rate limited"),
            ApiError::InternalServerError(msg) => write!(f, "Internal server error: {}", msg),
        }
    }
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            ApiError::Forbidden(_) => StatusCode::FORBIDDEN,
            ApiError::Conflict(_) => StatusCode::CONFLICT,
            ApiError::PaymentFailed(_) => StatusCode::PAYMENT_REQUIRED,
            ApiError::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            ApiError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let status = self.status_code();
        HttpResponse::build(status).json(json!({
            "error": self.to_string(),
            "status": status.as_u16()
        }))
    }
}

// Easy conversions
impl From<sqlx::Error> for ApiError {
    fn from(err: sqlx::Error) -> Self {
        tracing::error!("Database error: {}", err);
        ApiError::InternalServerError("Database error".into())
    }
}

impl From<String> for ApiError {
    fn from(err: String) -> Self {
        ApiError::InternalServerError(err)
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        ApiError::BadRequest(err.to_string())
    }
}
```

---

## Database Module

```rust
// src/db.rs

use sqlx::{postgres::PgPoolOptions, PgPool};

pub async fn create_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(30))
        .connect(database_url)
        .await
}

// Common queries
pub mod queries {
    use sqlx::PgPool;
    use uuid::Uuid;
    use crate::models::User;

    pub async fn get_user_by_id(pool: &PgPool, id: &Uuid) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(pool)
        .await
    }

    pub async fn get_user_by_line_id(
        pool: &PgPool,
        line_id: &str,
    ) -> Result<Option<User>, sqlx::Error> {
        sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE line_id = $1"
        )
        .bind(line_id)
        .fetch_optional(pool)
        .await
    }

    pub async fn create_user(
        pool: &PgPool,
        line_id: String,
        name: String,
        avatar: Option<String>,
    ) -> Result<User, sqlx::Error> {
        sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (id, line_id, name, avatar, stars, coins, level)
            VALUES ($1, $2, $3, $4, 0, 100, 1)
            RETURNING *
            "#
        )
        .bind(Uuid::new_v4())
        .bind(line_id)
        .bind(name)
        .bind(avatar)
        .fetch_one(pool)
        .await
    }
}
```

---

## Models

```rust
// src/models.rs

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub line_id: String,
    pub email: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
    pub stars: i32,
    pub coins: i32,
    pub level: i32,
    pub total_readings: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct Reading {
    pub id: Uuid,
    pub user_id: Uuid,
    pub question: String,
    pub mood: String,
    pub topic: String,
    pub cards: sqlx::types::Json<Vec<i32>>,
    pub interpretation: Option<String>,
    pub status: String,
    pub cost_stars: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadingRequest {
    pub question: String,
    pub cards: Vec<i32>,
    pub mood: Option<String>,
    pub topic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadingResponse {
    pub reading_id: String,
    pub status: String,
}
```

---

## Handlers - Health Check

```rust
// src/handlers/mod.rs

pub mod auth;
pub mod readings;
pub mod payments;
pub mod users;

use actix_web::HttpResponse;
use serde_json::json;

pub async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(json!({
        "status": "ok",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}
```

---

## Handlers - Auth (Basic Template)

```rust
// src/handlers/auth.rs

use actix_web::{web, HttpResponse};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::errors::ApiError;
use crate::models::User;
use crate::{db, config::Config};

pub async fn line_login(config: web::Data<Config>) -> Result<HttpResponse, ApiError> {
    // Generate state for CSRF protection
    let state = Uuid::new_v4().to_string();
    
    // Build LINE authorization URL
    let auth_url = format!(
        "https://web.line.me/web/login?clientId={}&redirectUri={}&state={}&scope=profile+email+openid",
        config.line_channel_id,
        urlencoding::encode(&format!("https://your-domain.com/api/auth/line/callback")),
        state
    );

    Ok(HttpResponse::Ok().json(json!({
        "redirect_url": auth_url,
        "state": state
    })))
}

#[derive(serde::Deserialize)]
pub struct LineCallbackRequest {
    pub code: String,
    pub state: String,
}

pub async fn line_callback(
    req: web::Json<LineCallbackRequest>,
    pool: web::Data<PgPool>,
    config: web::Data<Config>,
) -> Result<HttpResponse, ApiError> {
    // Exchange code for ID token
    let id_token = exchange_line_code(&req.code, &config)
        .await?;

    // Verify token and extract user info
    let line_user = verify_line_token(&id_token, &config)?;

    // Find or create user
    let user = db::queries::get_user_by_line_id(&pool, &line_user.user_id)
        .await?
        .or_else(|| {
            Some(futures::executor::block_on(async {
                db::queries::create_user(
                    &pool,
                    line_user.user_id.clone(),
                    line_user.display_name.clone(),
                    line_user.picture_url.clone(),
                )
                .await
                .ok()
            })?)
        })
        .ok_or(ApiError::InternalServerError("Failed to create user".into()))?;

    // Create JWT token
    let token = create_jwt_token(&user.id, &config)?;

    Ok(HttpResponse::Ok().json(json!({
        "token": token,
        "user": user
    })))
}

// Helper functions
async fn exchange_line_code(code: &str, config: &Config) -> Result<String, ApiError> {
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.line.me/oauth2.0/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", "https://your-domain.com/api/auth/line/callback"),
            ("client_id", &config.line_channel_id),
            ("client_secret", &config.line_channel_secret),
        ])
        .send()
        .await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    let body: serde_json::Value = response.json().await
        .map_err(|e| ApiError::InternalServerError(e.to_string()))?;

    body["id_token"]
        .as_str()
        .ok_or(ApiError::Unauthorized("No id_token in response".into()))
        .map(|s| s.to_string())
}

#[derive(serde::Deserialize)]
struct LineProfile {
    sub: String,
    name: String,
    picture: Option<String>,
}

fn verify_line_token(token: &str, config: &Config) -> Result<LineProfile, ApiError> {
    // Decode JWT - for production, verify signature
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(ApiError::Unauthorized("Invalid token format".into()));
    }

    let claims_json = String::from_utf8(
        base64::decode(parts[1])
            .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?
    )
    .map_err(|_| ApiError::Unauthorized("Invalid token".into()))?;

    serde_json::from_str::<LineProfile>(&claims_json)
        .map_err(|_| ApiError::Unauthorized("Invalid token claims".into()))
}

fn create_jwt_token(user_id: &Uuid, config: &Config) -> Result<String, ApiError> {
    let now = chrono::Utc::now();
    let expiration = now + chrono::Duration::days(7);

    let claims = serde_json::json!({
        "sub": user_id.to_string(),
        "exp": expiration.timestamp(),
        "iat": now.timestamp(),
    });

    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(config.jwt_secret.as_bytes()),
    )
    .map_err(|_| ApiError::InternalServerError("JWT creation failed".into()))
}
```

---

## Services - Queue

```rust
// src/services/queue.rs

use redis::aio::ConnectionManager;
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingJob {
    pub reading_id: String,
    pub user_id: String,
    pub question: String,
    pub cards: Vec<i32>,
    pub mood: String,
    pub topic: String,
    pub created_at: chrono::DateTime<Utc>,
}

pub struct QueueService {
    redis: ConnectionManager,
}

impl QueueService {
    pub async fn new(redis_url: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(QueueService { redis })
    }

    pub async fn enqueue_reading(
        &self,
        job: ReadingJob,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let job_json = serde_json::to_string(&job)?;
        let mut conn = self.redis.clone();

        redis::cmd("LPUSH")
            .arg("reading_queue")
            .arg(&job_json)
            .query_async::<_, ()>(&mut conn)
            .await?;

        Ok(())
    }

    pub async fn get_job_status(
        &self,
        reading_id: &str,
    ) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let mut conn = self.redis.clone();
        let result: Option<String> = redis::cmd("GET")
            .arg(format!("reading_result:{}", reading_id))
            .query_async(&mut conn)
            .await?;

        Ok(result)
    }
}
```

---

## Docker Setup

```dockerfile
# Dockerfile
FROM rust:1.75 as builder

WORKDIR /app

# Copy manifests
COPY Cargo.* ./

# Copy source
COPY src ./src
COPY migrations ./migrations

# Build for release
RUN cargo build --release

# Final stage
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

COPY --from=builder /app/target/release/mimi-backend /usr/local/bin/

EXPOSE 8080

CMD ["mimi-backend"]
```

---

## Render Configuration

```yaml
# render.yaml
services:
  - type: web
    name: mimi-backend
    runtime: rust
    plan: standard
    buildCommand: cargo build --release
    startCommand: ./target/release/mimi-backend
    
    envVars:
      - key: RUST_LOG
        value: info
      - key: DATABASE_URL
        scope: build,runtime
      - key: UPSTASH_REDIS_REST_URL
        scope: build,runtime
      - key: UPSTASH_REDIS_REST_TOKEN
        scope: build,runtime
        isPrivate: true
      - key: LINE_CHANNEL_ID
        scope: runtime
        isPrivate: true
      - key: LINE_CHANNEL_SECRET
        scope: runtime
        isPrivate: true
      - key: STRIPE_SECRET_KEY
        scope: runtime
        isPrivate: true
      - key: STRIPE_WEBHOOK_SECRET
        scope: runtime
        isPrivate: true
      - key: JWT_SECRET
        scope: runtime
        isPrivate: true
      - key: OPENAI_API_KEY
        scope: runtime
        isPrivate: true
```

---

## SQL Migrations

```sql
-- migrations/001_init_schema.sql

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    line_id VARCHAR NOT NULL UNIQUE,
    email VARCHAR,
    name VARCHAR NOT NULL,
    avatar VARCHAR,
    stars INTEGER DEFAULT 0,
    coins INTEGER DEFAULT 100,
    level INTEGER DEFAULT 1,
    total_readings INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE readings (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    question TEXT NOT NULL,
    mood VARCHAR NOT NULL,
    topic VARCHAR NOT NULL,
    cards INTEGER[] NOT NULL,
    interpretation TEXT,
    status VARCHAR DEFAULT 'processing',
    cost_stars INTEGER DEFAULT 1,
    created_at TIMESTAMP DEFAULT NOW(),
    completed_at TIMESTAMP
);

CREATE INDEX idx_users_line_id ON users(line_id);
CREATE INDEX idx_readings_user_id ON readings(user_id);
CREATE INDEX idx_readings_status ON readings(status);
```

---

These templates give you a solid foundation to start building your Rust backend! Start with main.rs, config, and models, then gradually add handlers and services.
