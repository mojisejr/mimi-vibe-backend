## Project Overview

**Project Name**: MiMiVibe Backend (Tarot Reading App)

**Repository**: https://github.com/mojisejr/mimi-vibe-backend 
**Author**: Nonthasak.l <nonthasak.l@gmail.com>

**Description**: High-performance Tarot reading app backend built with Rust + Actix Web, featuring AI-powered card interpretations, user authentication via LINE LIFF, credit system, Stripe payments, and referral rewards. Deployed on serverless architecture (Render, Supabase, Upstash) for solo maintainability.

---

## ⚠️ CRITICAL SAFETY RULES

### 🚨 FORBIDDEN ACTIONS (NEVER ALLOWED)

- ❌ **NEVER merge PRs yourself** - Provide PR link and wait for user instructions
- ❌ **NEVER work on main/staging branches** - Always use feature branches
- ❌ **NEVER delete critical files** (.env, .git/, node_modules/, package.json, lib/database/)
- ❌ **NEVER commit sensitive data** (API keys, passwords, secrets) - Use environment variables
- ❌ **NEVER skip 100% validation** (build, lint, test) - Must pass completely
- ❌ **NEVER use git push --force** - Only use --force-with-lease when absolutely necessary
- ❌ **NEVER implement without task issue** - Must use =plan command first

### 📋 MANDATORY WORKFLOW RULES

- ✅ **ALWAYS** sync main branch before any implementation: `git checkout main && git pull origin main`
- ✅ **ALWAYS** verify task issue exists: `#[issue-number]` before `=impl`
- ✅ **ALWAYS** use feature branch naming: `feature/task-[issue-number]-[description]`
- ✅ **ALWAYS** ensure 100% build success before commit: `cargo build --release`
- ✅ **ALWAYS** ensure 100% clippy pass before commit: `cargo clippy --all-targets --all-features`
- ✅ **ALWAYS** use template-guided workflow with proper context validation
- ✅ **ALWAYS** verify code formatting: `cargo fmt -- --check`

---

## 📋 Workflow System

### Template Integration

**Context Issue Template** - `/docs/ISSUE-TEMP.md`:

- Used for: `=fcs > [topic-name]` or `=fcs > [CONTEXT]`
- **ALWAYS creates GitHub Issue** - Never creates local .md files
- Creates living document for iterative discussion
- Contains: DISCUSSION LOG, ACCUMULATED CONTEXT, PLANNING READINESS CHECKLIST

**Task Issue Template** - `/docs/TASK-ISSUE-TEMP.md`:

- Used for: `=plan > [task description]`
- **ALWAYS creates GitHub Issue** - Never creates local .md files
- Creates atomic tasks based on current mode (MANUAL/COPILOT)
- Contains: EXECUTION MODE field, 100% validation requirements

**Knowledge Issue Template** - `/docs/KNOWLEDGE-TEMP.md`:

- Used for: `=kupdate [category] "[topic]"`
- **ALWAYS creates GitHub Issue** - Never creates local .md files
- Creates structured knowledge entries with AI honest feedback
- Contains: Problem → Solution → Lessons Learned → Links

### Mode-Based Execution System

**Default Mode**: MANUAL (human implementation)

**Mode Commands**:

```bash
=mode manual     # Tasks assigned to human developer
=mode copilot     # Tasks assigned to @copilot
=mode status      # Show current execution mode
```

**Mode-Specific Behavior**:

- **MANUAL Mode**: `=plan` creates tasks assigned to human, `=impl` waits for human implementation
- **COPILOT Mode**: `=plan` creates tasks assigned to @copilot, `=impl` triggers copilot implementation

### Core Commands

**✅ NEW: Claude Code Slash Commands Implemented!**
All workflow commands are now available as proper Claude Code slash commands (markdown files in `.claude/commands/`).

```bash
# Mode Management
/mode [manual|copilot|status]  # Set or show execution mode

# Context Management
/fcs [topic-name]              # Create new Context GitHub Issue
/fcs list                      # Show all active Context Issues

# Task Management
/plan [task description]       # Create Task GitHub Issue using docs/TASK-ISSUE-TEMP.md
/impl [issue-number]           # Implementation workflow for specific GitHub issue
/impl [issue-number] [msg]     # Implementation with additional context
/pr [feedback]                 # Create Pull Request from feature branch (to staging)

# Knowledge Management
/khub                          # 🔍 Read Knowledge Hub #102 (MANDATORY first step)
/kupdate [category] "[topic]"  # Create Knowledge GitHub Issue (CHECK existing numbers!)
/klink [knowledge-issue-number] # Link knowledge entry to Knowledge Hub #102
/ksync                         # Synchronize Knowledge Hub with all entries
/ksearch "[query]"             # Search across all knowledge entries
/krecent                       # Show last 5 knowledge updates
/kcategory [category]          # Show knowledge for specific category

# Other Commands
/rrr [message]                 # Create daily retrospective GitHub Issue

# Legacy = Commands (still supported for backward compatibility)
=fcs > [topic-name]           # Create new Context GitHub Issue
=plan > [task description]    # Create Task GitHub Issue
=impl > [issue-number]        # Implementation workflow
=khub                         # Read Knowledge Hub #102
# ... (all other = commands still work)
```

### Claude Code Slash Command Features

- **Proper Implementation**: Markdown files in `.claude/commands/` directory
- **Claude Integration**: Processed and executed by Claude Code
- **Rich Documentation**: Each command includes comprehensive usage, examples, and implementation details
- **Error Handling**: Clear error messages and helpful suggestions
- **Validation**: Automatic checking of prerequisites and dependencies
- **Help System**: All commands support help via detailed documentation

### Template-Driven Workflow Process

1. **Phase 1**: `/fcs [topic]` → Create initial context **GitHub Issue** (NEVER .md file)
2. **Phase 2**: `/fcs [topic]` → Update context **GitHub Issue** iteratively
3. **Phase 3**: Context reaches `[Ready for Planning]` status → Ready for planning
4. **Phase 4**: `/plan [task]` → Create atomic **GitHub Issues** (NEVER .md files)
5. **Phase 5**: `/impl [issue-number]` → Implement specific GitHub issue based on mode

**💡 Enhanced Workflow with Claude Code Slash Commands:**
- Use `/mode [manual|copilot]` to set execution mode
- Commands processed by Claude Code with intelligent execution
- Rich documentation and help built into each command
- Comprehensive error handling and validation
- All workflows maintain the same template-driven approach
- Legacy `=` commands remain supported for backward compatibility

### Implementation Workflow (MANDATORY)

**Pre-Implementation Checklist**:

1. **Staging Sync**: `git checkout staging && git pull origin staging`
2. **Task Verification**: Confirm Task **GitHub Issue** `#[issue-number]` exists and is [TASK] type
3. **Context Status**: Verify Context **GitHub Issue** is `[Ready for Planning]` or `[Implementation Ready]`
4. **Environment Check**: `git status` - working directory must be clean

**Implementation Steps**:

1. **Create Feature Branch**: `git checkout -b feature/task-[issue-number]-[description]`
2. **Execute Implementation**: Follow task requirements, use TodoWrite for complex tasks
3. **Build Validation**: `cargo build --release` (100% success - zero warnings)
4. **Lint Validation**: `cargo clippy --all-targets --all-features` (100% pass)
5. **Format Check**: `cargo fmt -- --check` (consistent formatting)
6. **Type Check**: `cargo check` (comprehensive type checking)
7. **Run Tests**: `cargo test` (if applicable)
8. **Commit Changes**:

   ```bash
   git add .
   git commit -m "feat: [feature description]

   - Address #[issue-number]: [task title]
   - Build validation: 100% PASS (cargo build --release)
   - Clippy validation: 100% PASS (cargo clippy)
   - Format validation: 100% PASS (cargo fmt)

   🤖 Generated with Claude Code
   Co-Authored-By: Claude <noreply@anthropic.com>"
   ```

9. **Push Branch**: `git push -u origin feature/task-[issue-number]-[description]`

**Post-Implementation**:

- **MANUAL Mode**: User commits and pushes, then uses `=pr` to create PR
- **COPILOT Mode**: Agent handles complete implementation including PR creation via `=pr`

---

## 🧠 Knowledge Management System

### Knowledge Workflow Integration

**Knowledge Capture Points**:

- **After Implementation**: When `=impl` completes successfully, use `=kupdate` to document learnings **(auto-prompts for hub linking)**
- **After Context Discussion**: When `=fcs` reaches key decisions, use `=kupdate` to capture insights **(auto-prompts for hub linking)**
- **After Chat Discoveries**: When breakthrough solutions are found, use `=kupdate` to preserve knowledge **(auto-prompts for hub linking)**

**Enhanced Knowledge Workflow**:

1. **🔍 Pre-Creation Check**: `=khub` → Read Knowledge Hub #102 FIRST to check existing KNOW-[CATEGORY]-XXX numbers
2. **Verify**: Check category section for existing numbers to avoid duplicates (e.g., KNOW-DEVICE-001, KNOW-DEVICE-002)
3. **Create**: `=kupdate [category] "[topic]"` → Creates knowledge issue with next available number
4. **Prompt**: System asks "Link to Knowledge Hub #102? (y/n)"
5. **Link**: If "y" → Automatically runs `=klink`
6. **Sync**: Use `=ksync` to ensure hub is fully synchronized
7. **Discover**: All knowledge accessible through `=khub` navigation

### Knowledge Categories

**Standard Categories**:

- `device` - CU12, KU16, SerialPort, hardware integration
- `database` - SQLite, Sequelize, migrations, queries
- `architecture` - Design patterns, structural decisions
- `debug` - Error solutions, troubleshooting, workarounds
- `workflow` - Process improvements, automation
- `frontend` - React, Electron, UI components
- `backend` - Node.js, APIs, services

### Knowledge ID System

**Format**: `KNOW-[CATEGORY]-[NUMBER]`

- Example: `KNOW-DEVICE-001`, `KNOW-DATABASE-015`
- Auto-increment per category
- Easy reference and cross-linking

### 🔍 Knowledge ID Conflict Prevention (CRITICAL)

**MANDATORY Pre-Creation Checklist**:

1. **ALWAYS run `=khub` first** - Read Knowledge Hub #102 completely
2. **Check existing numbers** in your category section (e.g., "Device Knowledge")
3. **Identify next available number** (if 001, 002 exist, use 003)
4. **Never assume** - always verify existing entries before creating

**Common Mistakes to Avoid**:

- ❌ Creating KNOW-DEVICE-001 when it already exists
- ❌ Not checking Knowledge Hub #102 before creating entries
- ❌ Assuming numbers without verification
- ❌ Creating duplicate knowledge IDs

**Correct Workflow Example**:

```bash
# ❌ WRONG (creates duplicate)
= kupdate device "SHT30 sensor fix"  # Creates KNOW-DEVICE-001 (duplicate!)

# ✅ RIGHT (prevents duplicates)
= khub                              # Read Knowledge Hub #102
# See: KNOW-DEVICE-001, KNOW-DEVICE-002 exist
= kupdate device "SHT30 sensor fix" # Creates KNOW-DEVICE-003 (correct!)
```

### Auto-Label Creation

**System Behavior**:

```bash
# When =kupdate device "CU12 lock-back solution" is used:
# 1. Check if 'knowledge-device' label exists
# 2. If not, create: gh label create knowledge-device --color "1d76db" --description "Device integration knowledge"
# 3. Apply label to knowledge issue
# 4. Auto-generate Knowledge ID: KNOW-DEVICE-001
```

**Knowledge Labels Created Automatically**:

- `knowledge-device` - Device integration knowledge
- `knowledge-database` - Database and persistence knowledge
- `knowledge-architecture` - System design and patterns
- `knowledge-debug` - Debugging and troubleshooting
- `knowledge-workflow` - Development workflow improvements

### Enhanced Knowledge Hub Integration

**New Automated Commands**:

**`=klink [knowledge-issue-number]`**:

- Automatically detects category from knowledge issue labels
- Places knowledge link in appropriate Knowledge Hub section
- Updates statistics counters
- Maintains proper markdown formatting

**`=ksync`**:

- Scans all issues with `knowledge-*` labels
- Synchronizes Knowledge Hub with all existing knowledge entries
- Updates statistics and distribution
- Fixes broken links and formatting
- Ensures hub reflects current knowledge base state

**Enhanced `=kupdate` Workflow**:

1. Creates knowledge GitHub issue ✅
2. **Automatically prompts**: "Link to Knowledge Hub #102? (y/n)"
3. If "y": Runs `=klink` automatically ✨
4. Maintains consistency across knowledge system

**Command Implementation Details**:

**`=klink [issue-number]` Implementation**:

1. **Issue Analysis**: Extract title, labels, and description
2. **Category Detection**: Parse `knowledge-[category]` label
3. **Format Entry**: `**KNOW-[CATEGORY]-[NUMBER]**: [Title](issue-link) - Brief description`
4. **Section Insert**: Add to appropriate "Recent Entries" section
5. **Statistics Update**: Increment total and category counts
6. **Timestamp Update**: Set "Last Updated" to current date

**`=ksync` Implementation**:

1. **Knowledge Discovery**: Scan all issues with `knowledge-*` labels
2. **Category Processing**: Group by label type (device, database, etc.)
3. **Entry Generation**: Create standardized format for each found issue
4. **Hub Reconstruction**: Replace all category sections with complete lists
5. **Statistics Calculation**: Recalculate all counts from scratch
6. **Format Validation**: Ensure proper markdown structure and valid links

**Hub Integration Benefits**:

- ✅ **No more manual linking required**
- ✅ **Automatic statistics updates**
- ✅ **Consistent formatting maintained**
- ✅ **Centralized knowledge discovery**
- ✅ **Real-time hub synchronization**

### Knowledge Search & Retrieval

**Search Capabilities**:

```bash
=ksearch "CU12 lock-back"    # Full-text search across all knowledge
=kcategory device           # Show all device-related knowledge
=krecent                    # Last 5 knowledge entries
=khub                       # Go to main Knowledge Hub issue
=ksync                      # Synchronize hub with all knowledge entries
=klink 116                  # Link knowledge issue #116 to hub
```

**Search Optimization**:

- Knowledge entries include searchable tags
- Problem statements use clear, technical language
- Solutions include specific keywords and technologies
- Cross-references link related knowledge
- Hub ensures all knowledge is discoverable from central location

### Knowledge Structure

**Each Knowledge Entry Contains**:

- **Problem Statement**: Clear description of what was solved
- **Solution Implementation**: Step-by-step working solution
- **AI Honest Feedback**: What worked, what didn't, lessons learned
- **Things to Avoid**: Common pitfalls and their consequences
- **Prerequisites**: What to check before starting
- **AI Self-Improvement**: Insights for future problem-solving
- **Links & References**: Connections to source issues/PRs/code
- **Verification Status**: Testing and validation state

---

## 🏗️ Technical Architecture

### Core Stack

- **Language**: Rust 2021 edition
- **Web Framework**: Actix Web 4.8 + Tokio async runtime
- **Database**: PostgreSQL via Supabase (managed)
- **Cache/Queue**: Redis via Upstash (serverless)
- **Authentication**: LINE LIFF OAuth + JWT
- **AI Engine**: OpenAI GPT-4o Mini (combined single-agent architecture)
- **Payments**: Stripe SDK
- **Deployment**: Render.com (single binary)
- **Frontend**: Next.js with LINE LIFF Web integration

### Project Structure

```
mimi-backend/
├── Cargo.toml                  # Dependencies and project config
├── src/
│   ├── main.rs                 # Server entry point (Actix setup)
│   ├── config.rs               # Configuration management (.env handling)
│   ├── middleware/
│   │   ├── auth.rs            # LINE auth + JWT verification
│   │   ├── rate_limit.rs      # Rate limiting via Redis
│   │   └── error_handler.rs   # Custom error responses
│   ├── handlers/
│   │   ├── readings.rs        # Tarot reading endpoints
│   │   ├── payments.rs        # Stripe payment endpoints
│   │   ├── users.rs           # User profile & auth endpoints
│   │   └── referrals.rs       # Referral system endpoints
│   ├── services/
│   │   ├── ai_engine.rs       # Combined Q-Filter + Analyzer agent
│   │   ├── payment_service.rs # Stripe integration
│   │   └── queue_service.rs   # Redis queue management
│   ├── models/
│   │   ├── user.rs            # User data structure
│   │   ├── reading.rs         # Reading data structure
│   │   └── payment.rs         # Payment data structure
│   └── db/
│       ├── schema.rs          # Supabase schema helpers
│       └── queries.rs         # SQL query helpers
├── migrations/                # Database migrations (SQLx)
└── .env.example              # Environment variables template
```

### Database Schema (Supabase PostgreSQL)

```sql
-- Core Tables
users (id, line_id, name, email, avatar, stars, coins, level, created_at)
readings (id, user_id, question, mood, topic, cards[], interpretation, status, created_at)
payment_history (id, user_id, stripe_id, amount_baht, stars_granted, status, created_at)
referral_links (id, user_id, code, total_referrals, coins_earned, created_at)
```

### Key Features

- **Tarot Readings**: Question submission → AI processing (1 combined agent) → Result storage
- **Credit System**: Stars (paid currency) + Coins (earned currency), exchangeable 100:1
- **User Authentication**: LINE LIFF OAuth with JWT tokens
- **Payment Processing**: Stripe integration with webhook handling
- **Referral System**: Generate referral codes, earn coins per signup
- **AI Engine**: Single optimized GPT-4o Mini call (combines analysis + interpretation)
- **Queue System**: Upstash Redis for async reading processing
- **Real-time Status**: Check reading processing status via polling or WebSocket

### Development Commands

```bash
cargo run              # Development server (default: http://localhost:8080)
cargo build --release  # Production build (creates optimized binary)
cargo test             # Run all tests
cargo clippy           # Lint checks
cargo fmt              # Code formatting
```

### Performance Metrics

- **API Response Time**: Target < 200ms (p95)
- **Reading Generation**: 1-2 seconds (single optimized AI call)
- **Concurrent Connections**: 5,000+ (Actix-web capable)
- **Memory Usage**: ~3-5MB per request (Rust efficiency)
- **Startup Time**: ~10ms
- **Monthly Cost**: ~$50-75 (Render + Supabase + Upstash)

---

## 🎯 Quality Standards

### Code Quality Requirements

- **Rust**: Edition 2021, strict type system (eliminates entire classes of bugs)
- **Cargo Check**: Zero compiler warnings (enforced)
- **Clippy Lints**: Zero warnings (`cargo clippy`)
- **Formatting**: `cargo fmt` auto-formatting, consistent across project
- **Build**: 100% success rate before commit
- **Tests**: Unit tests for critical paths (payments, auth)
- **Async Safety**: No panics, proper error handling in all async contexts

### API Quality Standards

- **Response Times**: p95 < 200ms for all endpoints
- **Error Handling**: Always return structured JSON errors with status codes
- **Rate Limiting**: Enforce per-user limits via Redis
- **Input Validation**: Validate all user inputs before processing
- **JWT Security**: 7-day token expiration, secure secret management
- **HTTPS Only**: Enforce in production, automatic via Render

### Performance Standards

- **Startup Time**: API ready within 10ms
- **Database Queries**: < 50ms per query (with indexes)
- **Redis Operations**: < 10ms per operation
- **AI Processing**: 1-2 seconds per reading (queue-based async)
- **Concurrent Users**: Handle 100+ concurrent connections
- **Memory Usage**: < 10MB base memory + ~1MB per concurrent request

### Security Standards

- **Secrets Management**: Use .env, never commit sensitive data
- **Database Access**: All queries use parameterized statements (SQLx)
- **Authentication**: JWT tokens with proper expiration
- **CORS**: Configured for frontend domain only
- **Rate Limiting**: Per-user limits on sensitive endpoints
- **Webhook Verification**: Verify Stripe webhook signatures
- **Error Messages**: Never expose sensitive system details

### Template-Guided Quality

- **Context Issues**: Complete PLANNING READINESS CHECKLIST ✅ (Always GitHub Issues)
- **Task Issues**: 100% build/lint/test requirements mandatory (Always GitHub Issues)
- **Mode Execution**: Follow mode-specific behavior exactly
- **Template Consistency**: All issues follow template structures
- **File Policy**: NEVER create local .md files for issues - ALWAYS use GitHub Issues

---

## 📚 Reference Materials

### Templates

- `/docs/ISSUE-TEMP.md` - Context Issue Template for iterative discussions
- `/docs/TASK-ISSUE-TEMP.md` - Atomic Task Template for implementation
- `/docs/KNOWLEDGE-TEMP.md` - Knowledge Issue Template for structured learning

### Performance Metrics

- **Target**: API response time < 200ms (p95)
- **Goal**: 99.9% uptime for Tarot reading service
- **Reliability**: 99.99% accurate reading delivery
- **Database**: PostgreSQL via Supabase with automatic scaling
- **Queue**: Upstash Redis with serverless scaling
- **Cost**: ~$50-75/month for full stack at scale

### Security Notes

- **Input Validation**: Comprehensive validation for all user inputs
- **Authentication**: LINE LIFF OAuth + JWT with 7-day expiration
- **Data Protection**: Encrypted connections, secure token storage
- **Access Control**: Role-based access (user, admin levels)
- **Payment Security**: Stripe webhook verification, idempotent operations
- **Audit Trail**: Complete logs for readings and transactions

---

_This document focuses on agent-critical information for efficient workflow execution and safe development practices._