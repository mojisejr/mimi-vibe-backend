# MiMiVibes Rebuild - Quick Reference & Decision Matrix

## 🎯 Tech Stack Summary

| Layer | Technology | Why | Cost | DevOps |
|-------|-----------|-----|------|--------|
| **Frontend** | Next.js 14 + React | Kept (works well) | $0-20/mo | Vercel |
| **Backend** | Rust + Actix Web | 5-10x faster, memory safe | $12/mo | Render |
| **Database** | Supabase (Postgres) | Managed, includes backups | $25/mo | 0 DevOps |
| **Queue** | Upstash Redis | Serverless, scales auto | $0-10/mo | 0 DevOps |
| **Auth** | Better Auth + LINE | Lightweight, extensible | $0 | Built-in |
| **Payments** | Stripe | Same as before | Per transaction | Webhooks |

**Total Monthly**: ~$37-50 baseline + stripe fees

---

## ⚡ Performance Gains

```
Current (Node.js Backend):
├─ API Response Time: ~500ms-1s
├─ Memory per instance: ~200-300MB
├─ Concurrent connections: ~100 concurrent
└─ Cost per 1000 requests: ~$0.50

New (Rust Backend):
├─ API Response Time: ~50-100ms
├─ Memory per instance: ~20-40MB
├─ Concurrent connections: ~10,000+ concurrent
└─ Cost per 1000 requests: ~$0.05
```

---

## 🚀 Architecture Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                       Frontend                               │
│        Next.js 14 (Vercel) - LINE LIFF Web App             │
└────────────────────┬────────────────────────────────────────┘
                     │ HTTPS
        ┌────────────┴─────────────────┐
        │                              │
        ▼                              ▼
   ┌──────────────┐            ┌──────────────────┐
   │ Render.com   │────────────│ Supabase Postgres│
   │ Rust Backend │    SQL     │ (PostgreSQL)     │
   │ (Actix Web)  │            │                  │
   └──────────────┘            └──────────────────┘
        │
        │ (Redis)
        ▼
   ┌──────────────────┐
   │ Upstash Redis    │
   │ (Job Queue)      │
   └──────────────────┘

   External Services:
   ├─ LINE OAuth
   ├─ OpenAI / Gemini (AI readings)
   ├─ Stripe (payments)
   └─ Sentry (error tracking)
```

---

## 🎁 MVP Feature Checklist

### Phase 1: Core (Week 1-2)
- [ ] Rust backend scaffold
- [ ] Actix Web server running
- [ ] Supabase Postgres connected
- [ ] User schema created
- [ ] LINE authentication flow
- [ ] JWT token generation

### Phase 2: Readings (Week 3)
- [ ] Reading submission endpoint
- [ ] Redis queue integration
- [ ] Combined AI agent (Filter + Analyzer)
- [ ] OpenAI integration
- [ ] Result storage & retrieval

### Phase 3: Payments (Week 4)
- [ ] Stripe payment intent
- [ ] Credit packages (99/199/399 THB)
- [ ] Webhook handling
- [ ] Star deduction system
- [ ] Payment history tracking

### Phase 4: Gamification (Week 4)
- [ ] Credit system (Stars + Coins)
- [ ] Exchange mechanism (100 coins = 1 star)
- [ ] Referral link generation
- [ ] Referral reward logic
- [ ] User profile endpoints

### Phase 5: Deployment (Week 5)
- [ ] Render.com setup
- [ ] Environment variables
- [ ] Database migrations
- [ ] GitHub integration
- [ ] Production monitoring

---

## 💡 Key Architectural Decisions

### 1. **Why Rust instead of Node.js?**
```
Memory Usage:    Node.js ~300MB    vs    Rust ~30MB (10x less)
Response Time:   Node.js ~500ms    vs    Rust ~50ms (10x faster)
Concurrency:     Node.js ~100      vs    Rust ~10k+ (100x more)
Startup Time:    Node.js ~2s       vs    Rust ~10ms (200x faster)
```
**Impact**: You can run multiple workers in same container, better scaling

### 2. **Why Upstash Redis (not BullMQ)?**
```
BullMQ (Node.js):
├─ Requires Node.js runtime
├─ Adds complexity to Rust backend
└─ Better as separate service

Upstash Redis (HTTP):
├─ Works from any language
├─ Serverless (no infrastructure)
├─ Simple FIFO/BRPOP patterns
└─ Scales automatically
```

### 3. **Why Combined AI Agent?**
```
Old Approach (4 LLM calls):
Question Filter → Analysis → Card Selection → Reading Generator
Cost: $0.0020/reading | Time: 4-5 seconds

New Approach (1 optimized call):
Combined Agent (Filter + Analysis + Interpretation)
Cost: $0.0005/reading | Time: 1-2 seconds (5x cost reduction!)

LLM Used: GPT-4o Mini ($0.00005/1K input tokens)
```

### 4. **Why Better Auth + Manual JWT?**
```
Better Auth:
├─ ✅ Out-of-box OAuth handling
├─ ✅ Multiple social providers
├─ ✅ Database agnostic
└─ ⚠️  Node.js only (need separate service)

Manual LINE Auth (in Rust):
├─ ✅ Lightweight
├─ ✅ No extra service needed
├─ ✅ Full control
└─ ⚠️  More code to write
```
**For MVP**: Use manual LINE auth in Rust backend

---

## 📊 Cost Breakdown (Monthly)

```
Base Infrastructure:
├─ Render (Rust backend):        $12-25/month
├─ Supabase (Postgres):          $25/month
├─ Upstash (Redis):              $0-10/month
└─ Vercel (Frontend):            $0-20/month
─────────────────────────────────────────
Subtotal:                         $37-70/month

Per-User Costs:
├─ Reading generation (GPT-4o):  $0.0005/reading
├─ OpenAI fallback:              negligible
└─ Stripe processing:            2.9% + $0.30

Example: 100 active users, 10 readings/month each:
├─ OpenAI costs: 100 * 10 * $0.0005 = $0.50/month
├─ Stripe (avg $100 purchase): $2.90/month
└─ Total variable:              ~$3.40/month

Total for 100 users: ~$40-70/month baseline + per-user costs
```

---

## 🔧 Development Setup

### Local Rust Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Create project
cargo new mimi-backend
cd mimi-backend

# Add dependencies
cargo add actix-web tokio sqlx stripe redis

# Run locally
cargo run

# Test API
curl http://localhost:8080/api/health
```

### Local Database

```bash
# Option 1: Docker PostgreSQL for local development
docker run --name postgres -e POSTGRES_PASSWORD=password -p 5432:5432 postgres:15

# Option 2: Use Supabase local CLI
supabase start

# Migrations
sqlx migrate run
```

### Environment Setup

```bash
# .env file
DATABASE_URL=postgresql://user:password@localhost:5432/mimi
UPSTASH_REDIS_REST_URL=https://...
UPSTASH_REDIS_REST_TOKEN=...
LINE_CHANNEL_ID=...
LINE_CHANNEL_SECRET=...
STRIPE_SECRET_KEY=...
JWT_SECRET=your-secret-key
OPENAI_API_KEY=...
```

---

## 🎯 Deployment Checklist

### Before First Deploy

- [ ] All tests passing locally
- [ ] Environment variables set in Render
- [ ] Database migrations created
- [ ] Stripe webhook configured
- [ ] LINE login app created
- [ ] Redis connection tested

### Deploy Process

```bash
# 1. Push to GitHub
git push origin main

# 2. Render auto-deploys from main branch
# Check status at: render.com/dashboard

# 3. Run migrations (if needed)
render-cli run --service mimi-backend sqlx migrate run

# 4. Verify health endpoint
curl https://mimi-api.onrender.com/api/health

# 5. Monitor logs
render-cli logs --service mimi-backend
```

---

## 📈 Scaling Strategy

### Phase 1: MVP (Single Instance)
```
Render: 1 instance (0.5 CPU, 512MB RAM)
├─ Can handle: ~1000 concurrent connections
├─ Expected users: 100-500 active
└─ Cost: $12/month
```

### Phase 2: Growing (Multiple Instances)
```
Render: 2-3 instances (load balanced)
├─ Can handle: ~10k concurrent connections
├─ Expected users: 5000-10k active
├─ Auto-scaling: 2-4 instances based on CPU
└─ Cost: $50-100/month
```

### Phase 3: Enterprise (Advanced)
```
Consider: Kubernetes cluster or AWS ECS
├─ Can handle: 100k+ concurrent
├─ Multiple regions
├─ CDN for static assets
└─ Cost: $200+/month
```

---

## ⚠️ Common Pitfalls & Solutions

| Pitfall | Solution |
|---------|----------|
| **Rust borrow checker issues** | Use `Arc<Mutex<T>>` for shared state, `web::Data` for DI |
| **Async/await complexity** | Start simple, use `tokio` tasks for background work |
| **Database connection timeouts** | Configure connection pool size: `max_connections(5)` for Render |
| **Redis queue not processing** | Implement consumer loop with `BRPOP` blocking |
| **LINE auth token expired** | Implement token refresh in middleware |
| **Stripe webhook failures** | Store webhook events in DB, process async |
| **Memory leaks** | Use Rust's memory safety + valgrind for profiling |

---

## 🔐 Security Best Practices

### API Security
```rust
// 1. Input validation
use validator::Validate;

#[derive(Validate)]
pub struct ReadingRequest {
    #[validate(length(min = 10, max = 500))]
    pub question: String,
}

// 2. Rate limiting
.wrap(ActixLimiter::default())

// 3. CORS
.wrap(middleware::DefaultHeaders::new()
    .add(("X-Version", "1.0")))

// 4. HTTPS only
// Configure in Render: REDIRECT_TO_HTTPS=true
```

### Data Protection
```rust
// 1. Hash passwords with Argon2
use argon2::{Argon2, PasswordHasher};

// 2. Encrypt sensitive fields
use chacha20poly1305::{ChaCha20Poly1305, Key, Nonce};

// 3. JWT secret rotation
// Rotate every 90 days, store previous key for validation
```

### Monitoring
```rust
// 1. Log all API calls
tracing::info!("API call: {} {}", method, path);

// 2. Track errors
tracing::error!("Database error: {}", error);

// 3. Monitor performance
let start = Instant::now();
// ... operation ...
let elapsed = start.elapsed();
```

---

## 📞 Migration Path (If You Decide to Switch)

If you want to migrate from current Node.js backend:

```
Week 1: Setup & validate
├─ Deploy Rust backend with dummy endpoints
└─ Configure new database

Week 2: Data migration
├─ Export current Postgres data
├─ Import to Supabase
└─ Validate integrity

Week 3: Feature migration
├─ One endpoint at a time
├─ Parallel run (old + new)
└─ A/B test with users

Week 4: Cutover
├─ Switch frontend to new API
├─ Monitor error rates
└─ Keep old backend as backup
```

---

## 🎓 Recommended Learning Order

1. **Day 1-2**: Rust basics + ownership
2. **Day 3-4**: Actix Web hello world
3. **Day 5**: Database integration (SQLx)
4. **Day 6**: Async/await patterns
5. **Day 7**: Error handling middleware
6. **Day 8**: OAuth flow implementation
7. **Day 9**: Queue system
8. **Day 10**: Deployment practice

**Resources**:
- Official Rust Book: https://doc.rust-lang.org/book/
- Actix Web Examples: https://github.com/actix/examples
- Tokio Tutorial: https://tokio.rs/tokio/tutorial

---

## 🚀 Quick Start Commands

```bash
# Clone and setup
git clone <your-repo>
cd mimi-backend

# Install deps and build
cargo build --release

# Run locally
cargo run

# Run tests
cargo test

# Format code
cargo fmt

# Check linting
cargo clippy

# Build Docker image
docker build -t mimi-backend:latest .
docker run -p 8080:8080 mimi-backend:latest

# Deploy to Render
# (automatic on git push)
git push origin main
```

---

## 📊 Success Metrics (Track After Deploy)

| Metric | Target | How to Track |
|--------|--------|--------------|
| API Response Time (p95) | < 200ms | Render dashboard |
| Reading Generation Time | < 3s | Application logs |
| Uptime | > 99.5% | Render monitoring |
| Error Rate | < 0.1% | Sentry or LogRocket |
| Queue Processing Latency | < 2min avg | Redis monitoring |
| Cost per Request | < $0.0001 | AWS Cost Explorer equivalent |

---

## 🎁 Next Actions

**Immediate (This Week)**
1. ✅ Read full REBUILD_STRATEGY.md
2. ⏳ Set up Rust development environment locally
3. ⏳ Create GitHub repository for new backend
4. ⏳ Set up Supabase project and get connection string

**Week 1**
1. ⏳ Scaffold Rust project with Actix
2. ⏳ Create database schema in Supabase
3. ⏳ Implement health check endpoint
4. ⏳ Deploy to Render test environment

**Week 2**
1. ⏳ Implement LINE authentication
2. ⏳ Create user management endpoints
3. ⏳ Set up Redis queue
4. ⏳ Implement reading submission flow

This is your sustainable, solo-maintainable path forward! 🎯
