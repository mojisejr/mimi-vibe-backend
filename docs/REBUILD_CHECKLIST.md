# MiMiVibes Rebuild - Complete Checklist

Use this as your project management document. Check items as you complete them.

---

## 📋 PRE-REBUILD PHASE (This Week)

### Understanding & Decision
- [ ] Read REBUILD_EXECUTIVE_SUMMARY.md (15 min)
- [ ] Review cost analysis
- [ ] Understand timeline (5 weeks)
- [ ] Confirm: This is the right approach ✅
- [ ] Read REBUILD_STRATEGY.md Phase 0-2 (optional, deeper dive)

### Environment Setup
- [ ] Install Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- [ ] Verify Rust: `rustc --version && cargo --version`
- [ ] Install VS Code (if not already)
- [ ] Install VS Code extension: `rust-analyzer`
- [ ] Install PostgreSQL locally (or use Docker): `docker run postgres:15`

### Account Creation & Configuration
- [ ] Create Supabase account (https://supabase.com)
  - [ ] Create new project
  - [ ] Note: Project URL, Anon Key, Service Key
  - [ ] Get connection string (PostgreSQL format)
- [ ] Create Upstash account (https://upstash.com)
  - [ ] Create Redis database
  - [ ] Get REST URL & Token
- [ ] Create LINE Developers account (https://developers.line.biz/ja/)
  - [ ] Create Channel
  - [ ] Get Channel ID & Channel Secret
  - [ ] Set Redirect URI: `https://your-domain/api/auth/line/callback`
- [ ] Create Stripe account (https://stripe.com)
  - [ ] Get Secret Key
  - [ ] Get Webhook Secret
- [ ] Create Render.com account (https://render.com)
  - [ ] Note: Will use later
- [ ] Create OpenAI account (https://platform.openai.com)
  - [ ] Get API Key

### GitHub & Version Control
- [ ] Create GitHub repository: `mimi-backend`
- [ ] Clone locally: `git clone <repo>`
- [ ] Create `.env.example` file (no secrets)
- [ ] Create `.env` file (with your secrets, in .gitignore)
- [ ] First commit: "Initial setup"

### Environment Variables (.env)
```
# Database
DATABASE_URL=postgresql://user:password@localhost:5432/mimi

# Redis
UPSTASH_REDIS_REST_URL=https://...
UPSTASH_REDIS_REST_TOKEN=...

# LINE
LINE_CHANNEL_ID=...
LINE_CHANNEL_SECRET=...

# Stripe
STRIPE_SECRET_KEY=...
STRIPE_WEBHOOK_SECRET=...

# JWT
JWT_SECRET=your-very-secure-random-secret

# OpenAI
OPENAI_API_KEY=...

# Environment
ENVIRONMENT=development
```
- [ ] All variables filled in
- [ ] `.env` in `.gitignore`
- [ ] `.env.example` in git (without values)

---

## 🏗️ WEEK 1: FOUNDATION

### Day 1-2: Project Setup
- [ ] Create new Rust project: `cargo new mimi-backend`
- [ ] Copy Cargo.toml from REBUILD_CODE_TEMPLATES.md
- [ ] Update dependencies: `cargo build` (will take time)
- [ ] Verify no errors
- [ ] Create project structure:
  - [ ] `src/main.rs`
  - [ ] `src/config.rs`
  - [ ] `src/handlers/mod.rs`
  - [ ] `src/services/mod.rs`
  - [ ] `src/models.rs`
  - [ ] `src/errors.rs`
  - [ ] `src/db.rs`
  - [ ] `src/middleware.rs`
  - [ ] `src/middleware/mod.rs` (if needed)
- [ ] Copy main.rs template (simplified version to start)
- [ ] First commit: "Project scaffold"

### Day 3: Database Connection
- [ ] Create `config.rs` from template
- [ ] Create `db.rs` with pool creation
- [ ] Test local PostgreSQL connection:
  - [ ] `psql postgresql://...`
  - [ ] Create test database
- [ ] Update `.env` with correct DATABASE_URL
- [ ] Create first Rust connection test
- [ ] Verify: `cargo run` should connect without error
- [ ] Commit: "Database connection"

### Day 4-5: Health Endpoint
- [ ] Create basic main.rs with Actix server
- [ ] Create `/api/health` endpoint
- [ ] Test locally: `curl http://localhost:8080/api/health`
- [ ] Expect response: `{"status":"ok","timestamp":"..."}`
- [ ] Add logging with tracing
- [ ] Verify: Can see logs in terminal
- [ ] Commit: "Health endpoint working"

### End of Week 1
- [ ] **CHECKPOINT**: Health endpoint responds to requests
- [ ] **DEPLOYMENT**: Can build release binary: `cargo build --release`
- [ ] **GIT**: All changes committed
- [ ] Decision: Ready for Week 2? YES ✓ / NO ✗

---

## 🔐 WEEK 2: AUTHENTICATION

### Day 1: LINE OAuth Setup
- [ ] Create `handlers/auth.rs`
- [ ] Implement `/api/auth/line/login` endpoint
- [ ] Returns LINE authorization URL
- [ ] Test in browser: Can redirect to LINE login
- [ ] Commit: "LINE login redirect"

### Day 2-3: JWT Implementation
- [ ] Implement `/api/auth/line/callback` endpoint
- [ ] Exchange LINE code for ID token
- [ ] Parse LINE JWT token
- [ ] Create JWT token for your app
- [ ] Test locally with LINE app or simulator
- [ ] Verify: Token is generated correctly
- [ ] Commit: "LINE callback and JWT generation"

### Day 4: User Endpoints
- [ ] Create `handlers/users.rs`
- [ ] Create user schema in Supabase:
  ```sql
  CREATE TABLE users (
      id UUID PRIMARY KEY,
      line_id VARCHAR NOT NULL UNIQUE,
      name VARCHAR NOT NULL,
      avatar VARCHAR,
      stars INTEGER DEFAULT 0,
      coins INTEGER DEFAULT 100,
      ...
  );
  ```
- [ ] Create `db/queries.rs` with user queries
- [ ] Implement `/api/users/profile` endpoint
- [ ] Implement user creation on first login
- [ ] Test: Can retrieve user profile
- [ ] Commit: "User endpoints"

### Day 5: Auth Middleware
- [ ] Create `middleware/auth.rs`
- [ ] Implement JWT verification
- [ ] Create middleware to protect endpoints
- [ ] Apply middleware to protected routes
- [ ] Test: Protected endpoint rejects request without token
- [ ] Test: Protected endpoint accepts request with valid token
- [ ] Commit: "Auth middleware"

### End of Week 2
- [ ] **CHECKPOINT**: Can login with LINE, get token, access protected endpoint
- [ ] **USERS**: At least 3 test users created
- [ ] **GIT**: All changes committed
- [ ] Decision: Ready for Week 3? YES ✓ / NO ✗

---

## 📖 WEEK 3: CORE READINGS FEATURE

### Day 1: Reading Submission
- [ ] Create `handlers/readings.rs`
- [ ] Create reading schema in Supabase:
  ```sql
  CREATE TABLE readings (
      id UUID PRIMARY KEY,
      user_id UUID REFERENCES users(id),
      question TEXT NOT NULL,
      cards INTEGER[] NOT NULL,
      status VARCHAR DEFAULT 'processing',
      ...
  );
  ```
- [ ] Implement `/api/readings/submit` POST endpoint
- [ ] Validate input (question length, card count)
- [ ] Create reading record (status: 'processing')
- [ ] Deduct 1 star from user
- [ ] Test: Can submit reading
- [ ] Commit: "Reading submission"

### Day 2: Redis Queue Integration
- [ ] Create `services/queue.rs`
- [ ] Initialize Redis connection (Upstash)
- [ ] Implement job enqueueing
- [ ] Enqueue reading after submission
- [ ] Verify: Job appears in Redis
- [ ] Test: Queue maintains order
- [ ] Commit: "Redis queue integration"

### Day 3: OpenAI Integration
- [ ] Create `services/ai_engine.rs`
- [ ] Implement reading generation (optimized single-prompt)
- [ ] Call OpenAI API
- [ ] Parse response
- [ ] Handle errors gracefully
- [ ] Test locally with dummy prompt
- [ ] Verify: Response is parsed correctly
- [ ] Commit: "OpenAI integration"

### Day 4: Background Processing
- [ ] Create async job processor
- [ ] Implement BRPOP loop (reads from queue)
- [ ] For each job:
  - [ ] Call AI engine
  - [ ] Store result in database
  - [ ] Update reading status to 'complete'
- [ ] Error handling: retry logic
- [ ] Test: Submit reading, wait for processing
- [ ] Verify: Result stored in database
- [ ] Commit: "Background job processing"

### Day 5: Status Checking
- [ ] Implement `/api/readings/status/{id}` GET endpoint
- [ ] Implement `/api/readings/history` GET endpoint
- [ ] Test: Can check reading status while processing
- [ ] Test: Can retrieve reading history
- [ ] Test: Can see completed reading with interpretation
- [ ] Commit: "Reading status and history"

### End of Week 3
- [ ] **CHECKPOINT**: Complete reading flow working end-to-end
- [ ] **PROCESSED**: At least 5 readings generated
- [ ] **DATABASE**: All results stored correctly
- [ ] **GIT**: All changes committed
- [ ] Decision: Ready for Week 4? YES ✓ / NO ✗

---

## 💳 WEEK 4: PAYMENTS & POLISH

### Day 1-2: Stripe Integration
- [ ] Create `handlers/payments.rs`
- [ ] Implement `/api/payments/create-intent` POST endpoint
- [ ] Create PaymentIntent with Stripe
- [ ] Test: Can create payment intent
- [ ] Implement `/api/payments/webhook` POST endpoint
- [ ] Handle Stripe webhook events
- [ ] Verify: Payment completion grants stars
- [ ] Commit: "Stripe payment flow"

### Day 3: Credit System
- [ ] Define credit packages (99/199/399 THB)
- [ ] Implement `/api/payments/packages` GET endpoint
- [ ] Implement `/api/credits/exchange` POST endpoint
- [ ] 100 coins = 1 star exchange
- [ ] Test: Can exchange coins for stars
- [ ] Update user credits on purchase
- [ ] Commit: "Credit system"

### Day 4: Testing & Fixes
- [ ] Test all endpoints with curl/Postman
- [ ] Fix any bugs discovered
- [ ] Verify error messages are clear
- [ ] Test edge cases:
  - [ ] Insufficient credits
  - [ ] Invalid input
  - [ ] Concurrent requests
- [ ] Commit: "Bug fixes and testing"

### Day 5: Final Validation
- [ ] Run full manual test suite:
  - [ ] Login → Submit reading → Check status
  - [ ] Purchase credits → Check balance
  - [ ] Exchange coins for stars
- [ ] Check all response formats
- [ ] Verify database consistency
- [ ] Review code for security issues
- [ ] Commit: "Final MVP validation"

### End of Week 4
- [ ] **CHECKPOINT**: All MVP features working
- [ ] **TESTED**: Manual testing complete
- [ ] **DATABASE**: Data integrity verified
- [ ] **GIT**: All changes committed
- [ ] Decision: Ready for deployment? YES ✓ / NO ✗

---

## 🚀 WEEK 5: DEPLOYMENT

### Day 1: Render Setup
- [ ] Push code to GitHub (main branch)
- [ ] Create Render.com account (if not done)
- [ ] Create new Web Service on Render
- [ ] Choose: Build from GitHub repo
- [ ] Connect GitHub repository
- [ ] Set build command: `cargo build --release`
- [ ] Set start command: `./target/release/mimi-backend`
- [ ] Commit: "Render deployment config"

### Day 2: Production Database
- [ ] Create production Supabase project (separate from dev)
- [ ] Run migrations on production
- [ ] Verify: Tables created
- [ ] Set production DATABASE_URL in Render environment
- [ ] Test: Render can connect to production DB
- [ ] Verify: Health endpoint works in production
- [ ] Commit: "Production database ready"

### Day 3: Production Secrets
- [ ] Add all environment variables to Render:
  - [ ] DATABASE_URL (production)
  - [ ] UPSTASH_REDIS_REST_URL
  - [ ] UPSTASH_REDIS_REST_TOKEN
  - [ ] LINE_CHANNEL_ID / SECRET
  - [ ] STRIPE_SECRET_KEY
  - [ ] STRIPE_WEBHOOK_SECRET
  - [ ] JWT_SECRET
  - [ ] OPENAI_API_KEY
- [ ] Verify: All secrets marked as private
- [ ] Trigger deploy: Push empty commit or redeploy
- [ ] Wait: ~5-10 minutes for build
- [ ] Check: Build log for errors

### Day 4: Verification & Testing
- [ ] Get production URL from Render
- [ ] Test health endpoint: `curl https://your-url/api/health`
- [ ] Test LINE login flow
- [ ] Test reading submission in production
- [ ] Test payment webhook
- [ ] Verify: All databases updated
- [ ] Check Render logs for errors
- [ ] Monitor performance metrics

### Day 5: Go Live! 🚀
- [ ] Update frontend to point to production API
- [ ] Test end-to-end flow
- [ ] Announce: "MiMiVibes is live!"
- [ ] Monitor: Check logs for first hour
- [ ] Celebrate: You did it! 🎉
- [ ] Commit: "Production deployment"

### Post-Deployment
- [ ] Monitor uptime dashboard
- [ ] Check error tracking (Sentry/etc)
- [ ] Monitor database usage
- [ ] Monitor Redis usage
- [ ] Track monthly costs
- [ ] Set up backups
- [ ] Document deployment process

### End of Week 5
- [ ] ✅ **LIVE IN PRODUCTION**
- [ ] ✅ **FIRST USERS ONBOARDED**
- [ ] ✅ **READINGS GENERATING**
- [ ] ✅ **PAYMENTS WORKING**
- [ ] ✅ **MONITORED & STABLE**

---

## 📊 Ongoing Monitoring (Post-MVP)

### Daily Checks (2 min)
- [ ] Check Render dashboard for errors
- [ ] Verify: No error spikes
- [ ] Check: API response times normal

### Weekly Checks (15 min)
- [ ] Review error logs
- [ ] Check database size
- [ ] Monitor cost trends
- [ ] Review user feedback

### Monthly Checks (1 hour)
- [ ] Full system review
- [ ] Performance optimization
- [ ] Security audit
- [ ] Plan next features

---

## 🎯 Feature Expansion (Post-MVP)

### Month 2: Referral System
- [ ] [ ] Create referral link generation
- [ ] [ ] Track referral rewards
- [ ] [ ] Distribute referral coins
- [ ] [ ] Test: Referral flow

### Month 3: Gamification
- [ ] [ ] Achievement system
- [ ] [ ] Level progression
- [ ] [ ] Daily login rewards
- [ ] [ ] Leaderboard

### Month 4: Analytics
- [ ] [ ] Dashboard for users
- [ ] [ ] Admin analytics
- [ ] [ ] Revenue tracking
- [ ] [ ] User insights

---

## 📈 Success Metrics to Track

### Performance Metrics
- [ ] API Response Time (target: < 200ms)
- [ ] Queue Processing Time (target: < 2 min average)
- [ ] Error Rate (target: < 0.1%)
- [ ] Uptime (target: > 99.5%)

### Business Metrics
- [ ] User Signups (target: track weekly)
- [ ] Active Users (target: > 100 in first month)
- [ ] Readings Generated (target: > 500 in first month)
- [ ] Revenue (target: > $200 in first month)

### Cost Metrics
- [ ] Render costs (budget: $25/month)
- [ ] Supabase costs (budget: $25/month)
- [ ] Upstash costs (budget: $10/month)
- [ ] OpenAI costs (budget: $50/month per 1000 readings)

### Operational Metrics
- [ ] Deployment time (target: < 10 min)
- [ ] Time spent on ops (target: < 2 hrs/month)
- [ ] Bug fix time (target: < 1 hr)
- [ ] Feature add time (target: < 4 hrs)

---

## 🎓 Learning Checkpoints

### After Week 1
- [ ] Understand Rust ownership
- [ ] Can write basic Actix handlers
- [ ] Know how to configure environment

### After Week 2
- [ ] Understand OAuth flow
- [ ] Can implement JWT
- [ ] Can create middleware

### After Week 3
- [ ] Understand async/await
- [ ] Can work with Redis
- [ ] Can integrate external APIs

### After Week 4
- [ ] Understand payment flows
- [ ] Can handle webhooks
- [ ] Can test complex systems

### After Week 5
- [ ] Can deploy to production
- [ ] Can monitor applications
- [ ] Ready to scale!

---

## ✨ Final Notes

### If You Get Stuck
1. Check documentation in REBUILD_STRATEGY.md
2. Search for exact error message
3. Ask in Rust Discord or Stack Overflow
4. Re-read the relevant section
5. Take a break and come back

### Tips for Success
- ✅ Commit after each working feature
- ✅ Test each endpoint with curl
- ✅ Read error messages carefully
- ✅ Start simple, add complexity
- ✅ Don't skip the documentation
- ✅ Celebrate small wins!

### When to Seek Help
- ❌ Stuck for more than 30 min on one issue
- ❌ Error message completely unclear
- ❌ Unexpected behavior
- ❌ Need second opinion on architecture

---

## 🎉 You've Got This!

Print this checklist. Check items off as you go. This is your road map to production.

**Good luck! You're about to build something amazing. 🚀**

---

**Last Updated**: November 8, 2025  
**Status**: Ready to begin  
**Estimated Completion**: November 29, 2025  
**Confidence**: 95% 🎯
