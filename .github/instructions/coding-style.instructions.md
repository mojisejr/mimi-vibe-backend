---
applyTo: '**'
---
# MiMiVibe Backend Coding Style & Project Structure Guide

## 1. Project Structure

- `src/handlers/` — REST API endpoint logic (split by domain, e.g., readings, payments, users)
- `src/services/` — Business logic, AI orchestration, workflow, integration (e.g., ai_engine, payment_service, workflow)
- `src/models/` — Data structs, enums, schemas (split by entity, e.g., reading, user, payment, ai)
- `src/middleware/` — Auth, rate limiting, error handler (middleware for Actix)
- `src/db/` — Database schema, SQL queries, migration helpers
- `src/config.rs` — Loads config, env, secrets
- `src/main.rs` — Entry point, route setup, DI, should not exceed 150 lines
- `tests/` — Integration & unit tests

## 2. Naming Conventions

- Files/Folders: snake_case, clear by domain (`handlers/readings.rs`, `services/payment_service.rs`)
- Struct/Enum/Function: PascalCase for types, snake_case for functions/fields
- Modules with multiple files: use `mod.rs` (e.g., `handlers/mod.rs`)

## 3. Patterns & Best Practices

- **Single Responsibility**: 1 file/module = 1 responsibility
- **Service Layer**: handler → service → db (handlers must not call db directly)
- **Error Handling**: Use `Result<T, E>`, custom error types separated in `middleware/error_handler.rs`
- **Config/Secrets**: Load from .env via `config.rs` only
- **Testing**: Add unit tests for critical logic (in `tests/` or #[cfg(test)] in the file)
- **Pipeline/Workflow**: Each agent/service is a separate module, orchestration logic in `services/workflow.rs`
- **Struct/Enum**: Separate in `models/` by entity/domain
- **Reusable Logic**: If there are reusable utils/validation, separate into `utils.rs`

## 4. Additional Conventions

- main.rs should not exceed 150 lines (delegate everything to modules)
- Do not import cross-module unnecessarily (use service layer only)
- All responses must be JSON, use full type safety
- Use async/await, complete error handling
- Comment only on complex logic

## 5. Example Structure

```
src/
  handlers/
    readings.rs
    payments.rs
    users.rs
    mod.rs
  services/
    ai_engine.rs
    payment_service.rs
    queue_service.rs
    workflow.rs
    mod.rs
  models/
    reading.rs
    user.rs
    payment.rs
    ai.rs
    mod.rs
  middleware/
    auth.rs
    rate_limit.rs
    error_handler.rs
    mod.rs
  db/
    schema.rs
    queries.rs
    mod.rs
  config.rs
  main.rs
tests/
```

## 6. Reference
- Based on [AGENTS.md], [CLAUDE.md], and issue #7
- Can be updated as appropriate

---

> Always follow this guideline before adding or modifying code to ensure consistency and maintainability.
