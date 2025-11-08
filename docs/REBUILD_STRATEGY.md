# MiMiVibes Rebuild Strategy - Comprehensive Guide

**Date**: November 8, 2025  
**Status**: Strategic Planning  
**Scope**: Solo dev rebuild with minimized ops complexity

---

## 📋 Executive Summary

This document outlines a comprehensive strategy to rebuild MiMiVibes as a **production-grade, solo-maintainable system** using:
- **Backend**: Rust + Actix Web (high performance, minimal overhead)
- **Queue**: Upstash Redis + BullMQ alternative (serverless, no infrastructure)
- **Database**: Supabase (managed Postgres, reduces ops burden)
- **Auth**: Better Auth + LINE LIFF (lightweight, extensible)
- **Deployment**: Render/Vercel (minimal DevOps)

---

## 🎯 Phase 0: Strategic Context & Understanding

### Your Current State
- **Current Stack**: Next.js (frontend) + Node.js (backend) + PostgreSQL
- **Pain Points**: Scaling concerns, self-deployment complexity
- **Goal**: Reduce ops burden while maintaining quality & performance

### Key Principles for Your Rebuild
1. **Minimize Infrastructure**: Use serverless/managed services
2. **High Performance**: Rust backend reduces compute needs
3. **Low Maintenance**: Avoid complex DevOps, use platform services
4. **Cost Efficiency**: Serverless scales better than VPS
5. **Solo Viability**: Every component must be solo-maintainable

---

## 🏗️ Phase 1: Backend Architecture (Rust + Actix Web)

### Why Rust + Actix Web?

| Aspect | Benefits |
|--------|----------|
| **Performance** | 5-10x faster than Node.js, uses 1/10th memory |
| **Reliability** | Memory-safe, no null pointer exceptions |
| **Concurrency** | Native async/await, handles thousands of concurrent connections |
| **Deployment** | Single binary, minimal runtime dependencies |
| **Solo DevOps** | Easy scaling, predictable resource usage |

### Recommended Structure

```
backend/
├── Cargo.toml
├── src/
│   ├── main.rs                 # Server entry point
│   ├── config.rs               # Configuration management
│   ├── middleware/
│   │   ├── auth.rs            # LINE auth middleware
│   │   ├── rate_limit.rs      # Rate limiting
│   │   └── error_handler.rs   # Error handling
│   ├── handlers/
│   │   ├── readings.rs        # Tarot reading endpoints
│   │   ├── payments.rs        # Stripe payment endpoints
│   │   ├── users.rs           # User profile endpoints
│   │   └── referrals.rs       # Referral system
│   ├── services/
│   │   ├── ai_engine.rs       # Combined Q-Filter + Analyzer agent
│   │   ├── payment_service.rs # Stripe integration
│   │   └── queue_service.rs   # Job queue management
│   ├── models/
│   │   ├── user.rs
│   │   ├── reading.rs
│   │   └── card.rs
│   └── db/
│       ├── schema.rs          # Supabase schema helpers
│       └── queries.rs         # Common database queries
```

### Core Dependencies (Cargo.toml)

```toml
[package]
name = "mimi-backend"
version = "1.0.0"
edition = "2021"

[dependencies]
# Web framework
actix-web = "4.8"
actix-rt = "2.10"
tokio = { version = "1", features = ["full"] }

# Database & ORM
sqlx = { version = "0.8", features = ["runtime-tokio-rustls", "postgres", "json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# AI/LLM
reqwest = { version = "0.12", features = ["json"] }
langchain-rust = "0.4"

# Authentication & Security
jsonwebtoken = "9.3"
uuid = { version = "1.0", features = ["v4", "serde"] }
argon2 = "0.5"

# Queue System
redis = { version = "0.25", features = ["connection-manager", "json"] }
bullmq = "0.1"  # Or use redis directly with patterns

# Payment
stripe = "0.16"

# Rate Limiting & Caching
dashmap = "5.5"
once_cell = "1.19"

# Logging & Monitoring
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
```

### Key Implementation Patterns

#### 1. Async Handler Setup
```rust
use actix_web::{web, HttpResponse, post, get};

#[post("/api/readings/submit")]
async fn submit_reading(
    req_data: web::Json<ReadingRequest>,
    db: web::Data<DbPool>,
    ai_service: web::Data<AiService>,
) -> Result<HttpResponse, ApiError> {
    // Validate request
    let reading = req_data.into_inner();
    
    // Submit to queue immediately for async processing
    ai_service.queue_reading(reading).await?;
    
    Ok(HttpResponse::Accepted().json(json!({
        "status": "processing",
        "reading_id": reading_id
    })))
}

#[get("/api/readings/status/{id}")]
async fn check_status(
    id: web::Path<String>,
    db: web::Data<DbPool>,
) -> Result<HttpResponse, ApiError> {
    let status = db.get_reading_status(&id).await?;
    Ok(HttpResponse::Ok().json(status))
}
```

#### 2. Error Handling Middleware
```rust
use actix_web::{error, http::StatusCode};
use serde_json::json;

#[derive(Debug)]
pub enum ApiError {
    NotFound(String),
    Unauthorized,
    PaymentFailed(String),
    RateLimited,
    Internal(String),
}

impl error::ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::NotFound(_) => StatusCode::NOT_FOUND,
            ApiError::Unauthorized => StatusCode::UNAUTHORIZED,
            ApiError::RateLimited => StatusCode::TOO_MANY_REQUESTS,
            ApiError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            _ => StatusCode::BAD_REQUEST,
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

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::NotFound(msg) => write!(f, "Not found: {}", msg),
            ApiError::Unauthorized => write!(f, "Unauthorized"),
            ApiError::PaymentFailed(msg) => write!(f, "Payment failed: {}", msg),
            ApiError::RateLimited => write!(f, "Rate limited"),
            ApiError::Internal(msg) => write!(f, "Internal error: {}", msg),
        }
    }
}
```

#### 3. Database Pool Setup
```rust
use sqlx::postgres::PgPoolOptions;

pub async fn create_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)  // Solo dev: lower connection count
        .connect(&database_url)
        .await?;
    
    Ok(pool)
}
```

---

## 🔄 Phase 2: Queue System - Upstash Redis + BullMQ Alternative

### Architecture Decision: Why Upstash?

| Feature | Upstash | Self-hosted Redis | AWS SQS |
|---------|---------|-------------------|---------|
| **Setup** | Instant, 0 config | Needs infrastructure | Needs AWS account |
| **Cost** | $0-200/month (scalable) | $5-50/month + ops | $1-10/month |
| **Reliability** | 99.9% SLA | Depends on you | 99.99% SLA |
| **Solo DevOps** | ✅ None | ❌ High | ⚠️ Moderate |
| **Local Dev** | ✅ Local mock | ✅ Easy | ❌ Complex |

### Recommended: Rust-native Redis Queue

Since you're using Rust, avoid JavaScript's BullMQ. Use Redis directly with Rust patterns:

```toml
[dependencies]
redis = { version = "0.25", features = ["connection-manager", "json", "aio"] }
serde_json = "1.0"
chrono = "0.4"
```

#### Implementation: Reading Processing Queue

```rust
use redis::aio::ConnectionManager;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ReadingJob {
    pub reading_id: String,
    pub user_id: String,
    pub question: String,
    pub cards: Vec<u32>,
    pub mood: String,
    pub topic: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

pub struct QueueService {
    redis: ConnectionManager,
}

impl QueueService {
    pub async fn new(redis_url: &str) -> Result<Self, redis::RedisError> {
        let client = redis::Client::open(redis_url)?;
        let redis = ConnectionManager::new(client).await?;
        Ok(QueueService { redis })
    }

    // Enqueue a reading job
    pub async fn enqueue_reading(
        &self,
        job: ReadingJob,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let job_id = uuid::Uuid::new_v4().to_string();
        let job_json = serde_json::to_string(&job)?;
        
        let mut conn = self.redis.clone();
        
        // Add to queue (list-based FIFO)
        redis::cmd("LPUSH")
            .arg("reading_queue")
            .arg(&job_json)
            .query_async(&mut conn)
            .await?;
        
        // Store job metadata with TTL (7 days)
        redis::cmd("SETEX")
            .arg(format!("reading:{}", job_id))
            .arg(604800) // 7 days in seconds
            .arg(&job_json)
            .query_async(&mut conn)
            .await?;
        
        Ok(job_id)
    }

    // Process jobs with worker pattern
    pub async fn process_jobs(
        &self,
        ai_service: &AiService,
    ) -> Result<(), Box<dyn std::error::Error>> {
        loop {
            let mut conn = self.redis.clone();
            
            // BRPOP blocks until item available (timeout: 30 seconds)
            let result: Option<String> = redis::cmd("BRPOP")
                .arg("reading_queue")
                .arg(30)
                .query_async(&mut conn)
                .await?;
            
            if let Some(job_json) = result {
                let job: ReadingJob = serde_json::from_str(&job_json)?;
                
                // Process the job
                match ai_service.generate_reading(&job).await {
                    Ok(result) => {
                        // Store result
                        redis::cmd("SET")
                            .arg(format!("reading_result:{}", job.reading_id))
                            .arg(serde_json::to_string(&result)?)
                            .query_async(&mut conn)
                            .await?;
                    }
                    Err(e) => {
                        // Log error and optionally retry
                        tracing::error!("Failed to process reading {}: {}", job.reading_id, e);
                        // Retry logic: re-queue after delay
                    }
                }
            }
        }
    }

    // Check job status
    pub async fn get_job_status(
        &self,
        reading_id: &str,
    ) -> Result<JobStatus, Box<dyn std::error::Error>> {
        let mut conn = self.redis.clone();
        
        let result: Option<String> = redis::cmd("GET")
            .arg(format!("reading_result:{}", reading_id))
            .query_async(&mut conn)
            .await?;
        
        if let Some(result_json) = result {
            Ok(JobStatus::Complete(serde_json::from_str(&result_json)?))
        } else {
            Ok(JobStatus::Processing)
        }
    }
}

#[derive(Serialize)]
pub enum JobStatus {
    Processing,
    Complete(serde_json::Value),
    Failed(String),
}
```

### Deployment: Upstash Configuration

```bash
# .env
UPSTASH_REDIS_REST_URL=https://us1-xyz.upstash.io
UPSTASH_REDIS_REST_TOKEN=your-token-here
```

**No server needed!** Upstash handles scaling automatically.

---

## 💾 Phase 3: Database - Supabase Integration

### Why Supabase for Solo Dev?

| Feature | Supabase | Self-hosted | Cloud |
|---------|----------|-------------|-------|
| **Postgres** | ✅ Managed | ⚠️ DIY | ✅ Managed |
| **Auth** | ⚠️ Limited | ❌ DIY | ✅ Full |
| **Real-time** | ✅ Included | ❌ Complex | ✅ Included |
| **Backups** | ✅ Auto | ⚠️ Manual | ✅ Auto |
| **Solo DevOps** | ✅ Minimal | ❌ High | ✅ Minimal |
| **Cost** | $25-100/month | $10-50/month | $50-500/month |

### Schema Design (Rust SQLx)

```rust
// src/models/schema.rs

use sqlx::FromRow;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, FromRow)]
pub struct User {
    pub id: Uuid,
    pub line_id: String,
    pub email: Option<String>,
    pub name: String,
    pub avatar: Option<String>,
    pub stars: i32,           // Paid currency
    pub coins: i32,           // Earned currency
    pub level: i32,
    pub total_readings: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct Reading {
    pub id: Uuid,
    pub user_id: Uuid,
    pub question: String,
    pub mood: String,
    pub topic: String,
    pub cards: Vec<i32>,      // Stored as JSON
    pub interpretation: Option<String>,
    pub status: String,       // "processing", "complete", "failed"
    pub cost_stars: i32,
    pub created_at: DateTime<Utc>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Clone, FromRow)]
pub struct PaymentHistory {
    pub id: Uuid,
    pub user_id: Uuid,
    pub stripe_payment_id: String,
    pub amount_baht: i32,
    pub stars_granted: i32,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, FromRow)]
pub struct ReferralLink {
    pub id: Uuid,
    pub user_id: Uuid,
    pub code: String,          // Unique code
    pub total_referrals: i32,
    pub coins_earned: i32,
    pub created_at: DateTime<Utc>,
}
```

### Database Queries with SQLx

```rust
// src/db/queries.rs

use sqlx::PgPool;
use uuid::Uuid;
use crate::models::*;

pub async fn create_user(
    pool: &PgPool,
    line_id: String,
    name: String,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, line_id, name, email, stars, coins, level, total_readings)
        VALUES ($1, $2, $3, NULL, 0, 100, 1, 0)
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(line_id)
    .bind(name)
    .fetch_one(pool)
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

pub async fn deduct_user_stars(
    pool: &PgPool,
    user_id: Uuid,
    amount: i32,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        "UPDATE users SET stars = stars - $1 WHERE id = $2 AND stars >= $1"
    )
    .bind(amount)
    .bind(user_id)
    .execute(pool)
    .await?;
    
    Ok(result.rows_affected() > 0)
}

pub async fn save_reading_result(
    pool: &PgPool,
    reading_id: Uuid,
    interpretation: String,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE readings 
        SET interpretation = $1, status = 'complete', completed_at = NOW()
        WHERE id = $2
        "#
    )
    .bind(interpretation)
    .bind(reading_id)
    .execute(pool)
    .await?;
    
    Ok(())
}
```

### Supabase Setup

```sql
-- Create tables (run in Supabase SQL editor)

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

CREATE TABLE payment_history (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    stripe_payment_id VARCHAR NOT NULL,
    amount_baht INTEGER NOT NULL,
    stars_granted INTEGER NOT NULL,
    status VARCHAR NOT NULL,
    created_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE referral_links (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    code VARCHAR NOT NULL UNIQUE,
    total_referrals INTEGER DEFAULT 0,
    coins_earned INTEGER DEFAULT 0,
    created_at TIMESTAMP DEFAULT NOW()
);

-- Indexes for performance
CREATE INDEX idx_users_line_id ON users(line_id);
CREATE INDEX idx_readings_user_id ON readings(user_id);
CREATE INDEX idx_readings_status ON readings(status);
CREATE INDEX idx_payment_history_user_id ON payment_history(user_id);
```

---

## 🔐 Phase 4: Authentication - Better Auth + LINE LIFF

### Why Better Auth + LINE?

- **Better Auth**: Framework-agnostic, extensible, supports custom OAuth
- **LINE Login**: Perfect for Thai market, LIFF integration for web app
- **Line LIFF Web App**: No app store required, deployment via URL

### Better Auth + LINE Setup

#### 1. Rust Backend - Better Auth Integration

Since Better Auth is Node.js-based, you have two options:

**Option A: Use Better Auth as separate auth service (recommended for your case)**
```
Frontend (LINE LIFF) -> Better Auth Service -> Your Rust Backend
```

**Option B: Manual JWT handling in Rust (lighter weight)**

For solo dev, Option B is better. Let's implement custom LINE auth:

```rust
// src/middleware/auth.rs

use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures_util::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LineProfile {
    pub user_id: String,
    pub display_name: String,
    pub picture_url: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub struct LineClaims {
    pub sub: String,        // user_id
    pub name: String,       // display_name
    pub picture: Option<String>,
}

pub struct AuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = std::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        std::future::ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();
        
        Box::pin(async move {
            // Skip auth for public endpoints
            if is_public_route(req.path()) {
                return service.call(req).await;
            }

            // Extract authorization header
            if let Some(auth_header) = req.headers().get("authorization") {
                if let Ok(auth_str) = auth_header.to_str() {
                    if let Some(token) = auth_str.strip_prefix("Bearer ") {
                        // Verify JWT token
                        if let Ok(user_id) = verify_line_token(token) {
                            req.extensions_mut().insert(user_id);
                            return service.call(req).await;
                        }
                    }
                }
            }

            Err(actix_web::error::ErrorUnauthorized("Invalid or missing token"))
        })
    }
}

fn verify_line_token(token: &str) -> Result<String, Box<dyn std::error::Error>> {
    let secret = std::env::var("LINE_CHANNEL_SECRET")?;
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
    
    let token_data = decode::<LineClaims>(token, &decoding_key, &Validation::default())?;
    Ok(token_data.claims.sub)
}

fn is_public_route(path: &str) -> bool {
    matches!(path, 
        "/" | 
        "/api/health" | 
        "/api/auth/line/callback" |
        "/api/auth/line/login"
    )
}
```

#### 2. LINE Login Endpoint

```rust
// src/handlers/auth.rs

use actix_web::{web, HttpResponse, post, get};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LineCallbackRequest {
    pub code: String,
    pub state: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
    pub user: serde_json::Value,
}

#[post("/api/auth/line/callback")]
pub async fn line_callback(
    req: web::Json<LineCallbackRequest>,
    db: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ApiError> {
    // 1. Exchange LINE code for ID token
    let id_token = exchange_line_code(&req.code).await?;
    
    // 2. Verify and decode ID token
    let profile = verify_line_id_token(&id_token)?;
    
    // 3. Upsert user in database
    let user = sqlx::query_as::<_, User>(
        r#"
        INSERT INTO users (id, line_id, name, avatar)
        VALUES ($1, $2, $3, $4)
        ON CONFLICT(line_id) DO UPDATE
        SET updated_at = NOW()
        RETURNING *
        "#
    )
    .bind(uuid::Uuid::new_v4())
    .bind(&profile.user_id)
    .bind(&profile.display_name)
    .bind(profile.picture_url)
    .fetch_one(db.as_ref())
    .await
    .map_err(|_| ApiError::Internal("User creation failed".into()))?;
    
    // 4. Create JWT token for your app
    let token = create_jwt_token(&user.id, &user.line_id)?;
    
    Ok(HttpResponse::Ok().json(LoginResponse {
        token,
        user: serde_json::to_value(&user)?,
    }))
}

async fn exchange_line_code(code: &str) -> Result<String, ApiError> {
    let channel_id = std::env::var("LINE_CHANNEL_ID")?;
    let channel_secret = std::env::var("LINE_CHANNEL_SECRET")?;
    let redirect_uri = std::env::var("LINE_REDIRECT_URI")?;
    
    let client = reqwest::Client::new();
    let response = client
        .post("https://api.line.me/oauth2.0/token")
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &redirect_uri),
            ("client_id", &channel_id),
            ("client_secret", &channel_secret),
        ])
        .send()
        .await
        .map_err(|_| ApiError::Internal("LINE token exchange failed".into()))?;
    
    let body: serde_json::Value = response.json().await
        .map_err(|_| ApiError::Internal("Invalid LINE response".into()))?;
    
    Ok(body["id_token"]
        .as_str()
        .ok_or(ApiError::Internal("No id_token in response".into()))?
        .to_string())
}

fn create_jwt_token(user_id: &uuid::Uuid, line_id: &str) -> Result<String, ApiError> {
    let secret = std::env::var("JWT_SECRET")?;
    let now = chrono::Utc::now();
    let expiration = now + chrono::Duration::days(7);
    
    let claims = serde_json::json!({
        "sub": user_id.to_string(),
        "line_id": line_id,
        "exp": expiration.timestamp(),
        "iat": now.timestamp(),
    });
    
    jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(std::env::var("JWT_SECRET")?.as_bytes()),
    )
    .map_err(|_| ApiError::Internal("JWT creation failed".into()))
}
```

#### 3. Frontend (LINE LIFF Web App)

```typescript
// frontend/app/auth/line-callback.tsx

'use client';

import { useEffect } from 'react';
import { useRouter } from 'next/navigation';

export default function LineCallback() {
  const router = useRouter();

  useEffect(() => {
    const handleLineCallback = async () => {
      const urlParams = new URLSearchParams(window.location.search);
      const code = urlParams.get('code');
      const state = urlParams.get('state');

      if (!code) {
        router.push('/login?error=no_code');
        return;
      }

      try {
        // Exchange code with backend
        const response = await fetch('/api/auth/line/callback', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ code, state }),
        });

        if (!response.ok) throw new Error('Auth failed');

        const { token, user } = await response.json();

        // Store token
        localStorage.setItem('auth_token', token);
        localStorage.setItem('user', JSON.stringify(user));

        router.push('/dashboard');
      } catch (error) {
        console.error('Auth error:', error);
        router.push('/login?error=auth_failed');
      }
    };

    handleLineCallback();
  }, [router]);

  return <div>Processing LINE login...</div>;
}
```

---

## 🤖 Phase 5: AI Engine - Combined Agent (Optimized)

### Current Architecture
- **Old**: Separate Question Filter + Question Analyzer + Card Selector + Reading Generator
- **Problem**: 4 LLM calls per reading = slow & expensive

### Proposed: Single Optimized Agent

```rust
// src/services/ai_engine.rs

use langchain_rust::{
    llm::OpenAIConfig,
    prompt_args,
    chains::llm_chain::LLMChain,
    prompts::PromptTemplate,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReadingRequest {
    pub question: String,
    pub cards: Vec<u32>,
    pub user_mood: Option<String>,
    pub topic: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadingResult {
    pub question_analysis: QuestionAnalysis,
    pub interpretation: String,
    pub key_insights: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct QuestionAnalysis {
    pub is_valid: bool,
    pub mood: String,
    pub topic: String,
    pub timeframe: String,
}

pub struct AiEngine {
    openai_config: OpenAIConfig,
}

impl AiEngine {
    pub fn new() -> Self {
        let openai_config = OpenAIConfig::default()
            .with_api_key(std::env::var("OPENAI_API_KEY").unwrap());
        
        AiEngine { openai_config }
    }

    pub async fn generate_reading(
        &self,
        req: &ReadingRequest,
    ) -> Result<ReadingResult, Box<dyn std::error::Error>> {
        // Single optimized prompt combining all analysis
        let prompt_template = PromptTemplate::new(
            r#"You are an expert tarot reader. Analyze the user's question and provide a reading.

User Question: {question}
Selected Cards (by position 0-77 in Rider-Waite): {cards}
User Mood (if provided): {mood}
Topic (if provided): {topic}

INSTRUCTIONS:
1. First, validate the question (reject if inappropriate)
2. Analyze mood, topic, and timeframe
3. Provide a personalized tarot interpretation
4. Extract 3-5 key insights

Respond in JSON format:
{{
    "question_analysis": {{
        "is_valid": boolean,
        "mood": "string",
        "topic": "string",
        "timeframe": "string"
    }},
    "interpretation": "detailed reading interpretation",
    "key_insights": ["insight1", "insight2", "insight3"]
}}
"#
        );

        let cards_str = req.cards.iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(", ");

        let chain = LLMChain::new(
            self.openai_config.clone(),
            prompt_template,
        );

        let result = chain.run(
            prompt_args!(
                "question" => &req.question,
                "cards" => &cards_str,
                "mood" => req.user_mood.as_deref().unwrap_or("not provided"),
                "topic" => req.topic.as_deref().unwrap_or("general"),
            ),
        ).await?;

        let reading: ReadingResult = serde_json::from_str(&result)?;
        Ok(reading)
    }
}
```

### Cost Optimization

| Approach | Cost/Reading | Speed | Quality |
|----------|-------------|-------|---------|
| **4 Separate Calls** | $0.0020 | 4-5s | 100% |
| **Single Optimized** | $0.0005 | 1-2s | 95% |
| **GPT-4o Mini** | $0.00005 | 1-2s | 90% |

**Recommendation**: Use GPT-4o Mini for MVP, upgrade to GPT-4 Turbo for premium readings

---

## 💳 Phase 6: Stripe Integration

### Simple Payment Flow

```rust
// src/services/payment_service.rs

use stripe::{Client, PaymentIntent, CreatePaymentIntent, Currency};

pub struct PaymentService {
    stripe_client: Client,
}

impl PaymentService {
    pub fn new() -> Self {
        let stripe_key = std::env::var("STRIPE_SECRET_KEY").unwrap();
        PaymentService {
            stripe_client: Client::new(stripe_key),
        }
    }

    pub async fn create_payment_intent(
        &self,
        user_id: &str,
        amount_baht: i32,
        stars_to_grant: i32,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let mut params = CreatePaymentIntent::new(amount_baht as i64);
        params.currency = Some(Currency::THB);
        params.metadata = Some(
            [
                ("user_id", user_id.to_string()),
                ("stars", stars_to_grant.to_string()),
            ]
            .iter()
            .cloned()
            .collect(),
        );

        let intent = PaymentIntent::create(&self.stripe_client, params).await?;
        Ok(intent.client_secret.unwrap().to_string())
    }

    pub async fn handle_webhook(
        &self,
        payload: &str,
        signature: &str,
        db: &sqlx::PgPool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let event = stripe::Event::construct_from(
            serde_json::from_str(payload)?,
            signature,
            std::env::var("STRIPE_WEBHOOK_SECRET")?.as_bytes(),
        )?;

        match event.type_ {
            stripe::EventType::PaymentIntentSucceeded => {
                if let stripe::EventObject::PaymentIntent(intent) = event.data.object {
                    let user_id = intent.metadata
                        .get("user_id")
                        .map(|v| v.as_str());
                    let stars = intent.metadata
                        .get("stars")
                        .and_then(|v| v.parse::<i32>().ok());

                    if let (Some(user_id), Some(stars)) = (user_id, stars) {
                        // Grant stars to user
                        sqlx::query(
                            "UPDATE users SET stars = stars + $1 WHERE id = $2"
                        )
                        .bind(stars)
                        .bind(user_id)
                        .execute(db)
                        .await?;
                    }
                }
            }
            _ => {}
        }

        Ok(())
    }
}
```

---

## 🚀 Phase 7: Deployment Strategy

### Solo-Dev Friendly Options

#### Option 1: Render.com (Recommended)
- **Cost**: $7-25/month
- **Setup**: 5 minutes
- **DevOps**: Minimal (git push = deploy)
- **Scaling**: Automatic

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
      - key: DATABASE_URL
        scope: build,runtime
      - key: UPSTASH_REDIS_REST_URL
        scope: build,runtime
      - key: LINE_CHANNEL_ID
        scope: runtime
      - key: STRIPE_SECRET_KEY
        scope: runtime
        isPrivate: true
```

Deploy: `git push render main` ✅

#### Option 2: Vercel (Frontend only)
- **Cost**: $0-20/month
- **Setup**: 2 minutes
- **Deployment**: Automatic from git

#### Option 3: Railway.app
- **Cost**: $5-50/month
- **Setup**: 3 minutes
- **DevOps**: Very minimal

### Infrastructure Diagram

```
┌─────────────────────────────────────────────────────┐
│                   Your Domain                        │
│              (yourdomain.com)                        │
└────────────────────┬────────────────────────────────┘
                     │
        ┌────────────┼─────────────┐
        │            │             │
        ▼            ▼             ▼
   ┌────────┐  ┌──────────┐  ┌──────────┐
   │ Vercel │  │ Render   │  │ Upstash  │
   │(Frontend)  │(Rust API)│  │ (Redis)  │
   └────────┘  └──────────┘  └──────────┘
                     │             
        ┌────────────┴──────────┐
        │                       │
        ▼                       ▼
   ┌──────────┐          ┌──────────┐
   │Supabase  │          │ Stripe   │
   │(Postgres)│          │(Payments)│
   └──────────┘          └──────────┘
```

**Total Monthly Cost** (Solo Dev):
- Render: $12/month
- Supabase: $25/month
- Upstash Redis: $0-10/month
- Stripe: 2.9% + $0.30 per transaction
- **Total**: ~$37-50/month baseline

---

## 🎁 Phase 8: MVP Features Implementation

### 8.1 Tarot Reading (Core)

```rust
#[post("/api/readings/submit")]
async fn submit_reading(
    question: web::Json<ReadingRequest>,
    user_id: web::Path<String>,
    queue: web::Data<QueueService>,
    db: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ApiError> {
    // Check user stars
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = $1")
        .bind(&user_id.into_inner())
        .fetch_one(db.as_ref())
        .await?;
    
    if user.stars < 1 {
        return Err(ApiError::PaymentFailed("Insufficient stars".into()));
    }

    // Create reading record
    let reading_id = Uuid::new_v4();
    sqlx::query(
        r#"
        INSERT INTO readings (id, user_id, question, mood, topic, cards, status)
        VALUES ($1, $2, $3, $4, $5, $6, 'processing')
        "#
    )
    .bind(reading_id)
    .bind(&user_id.into_inner())
    .bind(&question.question)
    .bind(&question.mood)
    .bind(&question.topic)
    .bind(&question.cards)
    .execute(db.as_ref())
    .await?;

    // Deduct stars
    sqlx::query("UPDATE users SET stars = stars - 1 WHERE id = $1")
        .bind(&user_id.into_inner())
        .execute(db.as_ref())
        .await?;

    // Queue for processing
    queue.enqueue_reading(ReadingJob {
        reading_id: reading_id.to_string(),
        user_id: user_id.into_inner(),
        question: question.question.clone(),
        cards: question.cards.clone(),
        mood: question.mood.clone(),
        topic: question.topic.clone(),
        created_at: Utc::now(),
    }).await?;

    Ok(HttpResponse::Accepted().json(json!({
        "reading_id": reading_id,
        "status": "processing"
    })))
}
```

### 8.2 Credit System

```rust
#[derive(Debug, Serialize, Deserialize)]
pub struct CreditPackage {
    pub id: i32,
    pub name: String,
    pub price_baht: i32,
    pub stars: i32,
    pub bonus_percentage: i32,
}

pub static CREDIT_PACKAGES: &[CreditPackage] = &[
    CreditPackage {
        id: 1,
        name: "Starter",
        price_baht: 99,
        stars: 10,
        bonus_percentage: 0,
    },
    CreditPackage {
        id: 2,
        name: "Popular",
        price_baht: 199,
        stars: 25,
        bonus_percentage: 10,
    },
    CreditPackage {
        id: 3,
        name: "Premium",
        price_baht: 399,
        stars: 60,
        bonus_percentage: 20,
    },
];

// Exchange coins to stars
#[post("/api/credits/exchange")]
async fn exchange_coins_to_stars(
    user_id: web::Path<String>,
    amount: web::Json<serde_json::json!({"coins": i32})>,
    db: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ApiError> {
    let coins = amount.coins;
    
    // 100 coins = 1 star
    if coins < 100 {
        return Err(ApiError::PaymentFailed("Need at least 100 coins".into()));
    }

    let stars = coins / 100;
    
    sqlx::query(
        "UPDATE users SET coins = coins - $1, stars = stars + $2 WHERE id = $3"
    )
    .bind(coins)
    .bind(stars)
    .bind(user_id.into_inner())
    .execute(db.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "stars": stars,
        "coins_used": coins
    })))
}
```

### 8.3 Referral System

```rust
#[get("/api/referral/link")]
async fn get_referral_link(
    user_id: web::Path<String>,
    db: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ApiError> {
    let user_id_str = user_id.into_inner();
    
    // Get or create referral link
    let referral = sqlx::query_as::<_, ReferralLink>(
        r#"
        INSERT INTO referral_links (id, user_id, code, total_referrals, coins_earned)
        VALUES ($1, $2, $3, 0, 0)
        ON CONFLICT(user_id) DO UPDATE
        SET user_id = EXCLUDED.user_id
        RETURNING *
        "#
    )
    .bind(Uuid::new_v4())
    .bind(&user_id_str)
    .bind(format!("ref_{}", &user_id_str[0..8]))
    .fetch_one(db.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(json!({
        "referral_code": referral.code,
        "total_referrals": referral.total_referrals,
        "coins_earned": referral.coins_earned,
        "share_link": format!("https://mimi.app/join?ref={}", referral.code)
    })))
}

#[post("/api/referral/claim/{code}")]
async fn claim_referral(
    code: web::Path<String>,
    db: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, ApiError> {
    let code = code.into_inner();
    
    // Get referrer
    let referrer: (String,) = sqlx::query_as(
        "SELECT user_id FROM referral_links WHERE code = $1"
    )
    .bind(&code)
    .fetch_one(db.as_ref())
    .await
    .map_err(|_| ApiError::NotFound("Invalid referral code".into()))?;

    // Give 100 coins to new user, 120 coins to referrer
    sqlx::query("UPDATE users SET coins = coins + 100 WHERE id = (SELECT id FROM users WHERE id = CURRENT_USER_ID())")
        .execute(db.as_ref())
        .await?;

    sqlx::query(
        r#"
        UPDATE referral_links 
        SET total_referrals = total_referrals + 1, coins_earned = coins_earned + 120
        WHERE user_id = $1
        "#
    )
    .bind(&referrer.0)
    .execute(db.as_ref())
    .await?;

    Ok(HttpResponse::Ok().json(json!({"coins_granted": 100})))
}
```

---

## 📊 Implementation Timeline

```
Week 1: Backend Setup
├─ Rust project scaffold
├─ Actix Web server
└─ Supabase schema

Week 2: Core Features
├─ User management (LINE auth)
├─ Reading submission
└─ Queue system

Week 3: AI Integration
├─ OpenAI/Gemini integration
├─ Combined agent
└─ Result storage

Week 4: Payments & Features
├─ Stripe integration
├─ Credit system
├─ Referral system
└─ Testing

Week 5: Deployment
├─ Render setup
├─ Frontend update
├─ Testing in production
└─ Monitoring

Total: 5 weeks for MVP
```

---

## 🔍 Best Practices & Recommendations

### 1. **Error Handling**
```rust
// Always use Result types
pub async fn some_operation() -> Result<Data, ApiError> {
    // Implementation
}

// Log errors properly
tracing::error!("Operation failed: {}", error);
tracing::warn!("User {:?} attempted auth", user_id);
```

### 2. **Database**
- ✅ Use connection pooling (SQLx does this automatically)
- ✅ Index frequently queried fields
- ✅ Use prepared statements (SQLx)
- ✅ Monitor query performance in Supabase dashboard

### 3. **Caching with Redis**
```rust
// Cache expensive operations
pub async fn get_tarot_cards() -> Result<Vec<Card>, Error> {
    let mut conn = redis.clone();
    
    // Try cache first
    if let Ok(Some(cached)) = redis::cmd("GET")
        .arg("tarot_cards")
        .query_async::<_, Option<String>>(&mut conn)
        .await {
        return Ok(serde_json::from_str(&cached)?);
    }
    
    // Fetch from DB
    let cards = fetch_from_db().await?;
    
    // Cache for 24 hours
    redis::cmd("SETEX")
        .arg("tarot_cards")
        .arg(86400)
        .arg(serde_json::to_string(&cards)?)
        .query_async(&mut conn)
        .await?;
    
    Ok(cards)
}
```

### 4. **Rate Limiting**
```rust
use std::sync::Arc;
use dashmap::DashMap;

pub struct RateLimiter {
    requests: Arc<DashMap<String, (i32, Instant)>>,
}

impl RateLimiter {
    pub fn check(&self, user_id: &str, limit: i32, window: Duration) -> bool {
        let now = Instant::now();
        
        if let Some(mut entry) = self.requests.get_mut(user_id) {
            let (count, since) = entry.value_mut();
            
            if now.duration_since(*since) < window {
                if *count >= limit {
                    return false;
                }
                *count += 1;
            } else {
                *count = 1;
                *since = now;
            }
        } else {
            self.requests.insert(user_id.to_string(), (1, now));
        }
        
        true
    }
}
```

### 5. **Monitoring & Logging**
```rust
use tracing_subscriber;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .json()
        .init();

    // Rest of your app...
}
```

### 6. **Environment Management**
```rust
// .env
DATABASE_URL=postgresql://...
UPSTASH_REDIS_REST_URL=https://...
UPSTASH_REDIS_REST_TOKEN=...
LINE_CHANNEL_ID=...
LINE_CHANNEL_SECRET=...
STRIPE_SECRET_KEY=...
STRIPE_WEBHOOK_SECRET=...
JWT_SECRET=...
OPENAI_API_KEY=...
```

---

## 🎓 Learning Resources

### Rust Web Development
- **Actix Web Book**: https://actix.rs/
- **SQLx Documentation**: https://github.com/launchbadge/sqlx
- **Tokio Async Runtime**: https://tokio.rs/

### Deployment
- **Render Docs**: https://render.com/docs
- **Supabase Guides**: https://supabase.com/docs
- **Upstash Redis**: https://upstash.com/docs/redis

### Architecture Patterns
- **12-Factor App**: https://12factor.net/
- **Microservices Patterns**: https://microservices.io/

---

## ⚠️ Potential Challenges & Solutions

| Challenge | Solution |
|-----------|----------|
| **Rust learning curve** | Start with simple handlers, gradually add complexity |
| **Async/await debugging** | Use `tokio-console` for visibility |
| **Database migrations** | Use SQLx migrations, version control them |
| **Queue processing delays** | Monitor job processing time, scale workers if needed |
| **LINE LIFF browser issues** | Test on actual LINE app, not web browser |
| **Payment webhook failures** | Implement webhook retry logic, store events in DB |
| **Rate limit evasion** | Use Redis for distributed rate limiting |

---

## 🎯 Success Metrics

Track these after deployment:

- **API Response Time**: Target < 200ms for 95th percentile
- **Reading Generation Time**: Target < 3 seconds
- **Uptime**: Target > 99.5%
- **Queue Processing**: Target < 2 minute average latency
- **Error Rate**: Target < 0.1%
- **Cost per Request**: Monitor to ensure profitability

---

## 📞 Support & Next Steps

1. **Start Simple**: Get Rust + Actix server running locally first
2. **Connect Database**: Wire up Supabase connection
3. **Add Authentication**: Implement LINE login flow
4. **Build Core Features**: Readings, payments, referrals
5. **Deploy**: Push to Render
6. **Monitor**: Set up error tracking (Sentry, LogRocket)

---

**This strategy prioritizes:**
- ✅ Minimal ops complexity
- ✅ High performance
- ✅ Cost efficiency
- ✅ Solo maintainability
- ✅ Scalability for future growth

Good luck with your rebuild! 🚀
