# LLM Provider Integration

## Overview

The MiMi Backend now uses a flexible LLM provider abstraction that allows easy integration with multiple AI providers (OpenAI, Gemini, etc.) without changing business logic.

## Architecture

### Core Components

1. **`LlmProvider` Trait** (`src/services/llm/mod.rs`)
   - Defines the contract for any LLM provider
   - Async trait with `Send + Sync` bounds for thread safety
   - Single method: `ask(&self, question: &str) -> Result<(String, Option<serde_json::Value>), String>`

2. **`OpenAiClient` Implementation** (`src/services/llm/mod.rs`)
   - Concrete implementation for OpenAI's GPT models
   - Supports both real API calls and mock mode for testing
   - Uses `Arc<dyn LlmProvider>` for shared ownership across handlers

3. **Handler Integration** (`src/handlers/ask.rs`)
   - `AskState` struct holds shared `Arc<dyn LlmProvider>`
   - Handler receives trait object, calls provider-agnostic `ask()` method
   - Returns structured JSON response

### Data Flow

```
HTTP POST /ask
    ↓
ask_handler (actix-web)
    ↓
AskState.llm_client (Arc<dyn LlmProvider>)
    ↓
OpenAiClient.ask() [or any other provider]
    ↓
OpenAI API / Mock Response
    ↓
JSON Response to client
```

## Usage

### Starting the Server

**Development (Mock Mode):**
```bash
MOCK_LLM=true cargo run
```

**Production (Real API):**
```bash
export OPENAI_API_KEY="sk-..."
export OPENAI_MODEL="gpt-4o-mini"
cargo run --release
```

### Making Requests

```bash
curl -X POST http://127.0.0.1:8080/ask \
  -H "Content-Type: application/json" \
  -d '{"question":"What is the meaning of life?"}'
```

**Response:**
```json
{
  "response": "This is a mock response for testing purposes.",
  "raw": {
    "id": "mock-123",
    "model": "gpt-4o-mini",
    "object": "chat.completion",
    "choices": [{
      "message": {
        "role": "assistant",
        "content": "This is a mock response for testing purposes."
      }
    }]
  }
}
```

## Configuration

| Environment Variable | Default | Description |
|---------------------|---------|-------------|
| `MOCK_LLM` | `false` | Enable mock mode for testing |
| `OPENAI_API_KEY` | (required) | OpenAI API key (not needed in mock mode) |
| `OPENAI_MODEL` | `gpt-3.5-turbo` | Model to use (e.g., `gpt-4o-mini`) |
| `BIND_ADDRESS` | `127.0.0.1:8080` | Server bind address |

## Testing

### Unit Tests

```bash
cargo test --test llm_integration
```

Tests cover:
- Mock mode responses
- Trait object usage (`Arc<dyn LlmProvider>`)
- JSON structure validation

### Integration Testing

```bash
# Start server in mock mode
MOCK_LLM=true cargo run &

# Test endpoint
curl -X POST http://127.0.0.1:8080/ask \
  -H "Content-Type: application/json" \
  -d '{"question":"test"}'

# Stop server
pkill -f mimi-backend
```

## Adding New Providers

To add a new LLM provider (e.g., Google Gemini):

1. **Create provider struct:**
```rust
pub struct GeminiClient {
    client: Client,
    api_key: String,
    model: String,
}
```

2. **Implement `LlmProvider` trait:**
```rust
#[async_trait]
impl LlmProvider for GeminiClient {
    async fn ask(&self, question: &str) -> Result<(String, Option<serde_json::Value>), String> {
        // Implementation here
    }
}
```

3. **Update main.rs to use new provider:**
```rust
let llm_client: Arc<dyn LlmProvider> = Arc::new(GeminiClient::new(...));
```

No handler or business logic changes required!

## Validation Results

- ✅ **Build validation**: 100% PASS (`cargo build --release`)
- ✅ **Clippy validation**: 100% PASS (`cargo clippy --all-targets --all-features`)
- ✅ **Format validation**: 100% PASS (`cargo fmt -- --check`)
- ✅ **Type check validation**: 100% PASS (`cargo check`)
- ✅ **Test validation**: 100% PASS (`cargo test`)
- ✅ **Runtime validation**: Server starts and responds correctly
- ✅ **Integration validation**: `/ask` endpoint works with mock and real providers

## Notes

- **Thread Safety**: `Arc<dyn LlmProvider>` provides shared ownership across actix-web worker threads
- **Mock Mode**: Essential for development and CI/CD without API costs
- **Type Safety**: Trait abstraction ensures compile-time safety when swapping providers
- **No Workflow Orchestration**: This integration focuses on basic provider connectivity, not multi-step workflows (planned for future tasks)

## Related Issues

- Resolves #11: Integrate and validate system with new LLM provider abstraction
- Based on #10: Implement LLM provider trait
- Part of #7: LLM refactor and workflow implementation plan
