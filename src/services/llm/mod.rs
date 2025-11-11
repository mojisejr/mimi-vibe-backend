//! LLM integration module for OpenAI API calls.

use async_trait::async_trait;

/// Trait for LLM provider abstraction
/// Enables swapping between different LLM backends (OpenAI, Gemini, etc.)
#[async_trait]
pub trait LlmProvider: Send + Sync {
    /// Ask a question and get a response
    /// Returns (answer_text, optional_raw_json)
    async fn ask(&self, question: &str) -> Result<(String, Option<serde_json::Value>), String>;
}

pub mod openai;
pub use openai::OpenAiClient;
