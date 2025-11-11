//! LLM integration module for OpenAI API calls.

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Clone)]
pub struct OpenAiClient {
    client: Client,
    api_key: String,
    model: String,
    mock_mode: bool,
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    max_tokens: u32,
    temperature: f32,
}

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: Message,
}

impl OpenAiClient {
    /// Create a new OpenAI client
    pub fn new(api_key: String, model: String, mock_mode: bool) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(20))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            api_key,
            model,
            mock_mode,
        }
    }

    /// Ask a question and get a response
    pub async fn ask(&self, question: &str) -> Result<(String, Option<serde_json::Value>), String> {
        if self.mock_mode {
            log::info!("Mock mode enabled, returning canned response");
            let mock_response = serde_json::json!({
                "id": "mock-123",
                "object": "chat.completion",
                "model": self.model,
                "choices": [{
                    "message": {
                        "role": "assistant",
                        "content": "This is a mock response for testing purposes."
                    }
                }]
            });
            return Ok((
                "This is a mock response for testing purposes.".to_string(),
                Some(mock_response),
            ));
        }

        log::info!("Making real OpenAI API call");

        let request_body = ChatRequest {
            model: self.model.clone(),
            messages: vec![Message {
                role: "user".to_string(),
                content: question.to_string(),
            }],
            max_tokens: 64,
            temperature: 0.0,
        };

        let response = self
            .client
            .post("https://api.openai.com/v1/chat/completions")
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request_body)
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status();
        let response_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response: {}", e))?;

        if !status.is_success() {
            log::error!("OpenAI API error (status {}): {}", status, response_text);
            return Err(format!("OpenAI API error: {}", status));
        }

        let raw_json: serde_json::Value = serde_json::from_str(&response_text)
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        let chat_response: ChatResponse = serde_json::from_value(raw_json.clone())
            .map_err(|e| format!("Failed to deserialize response: {}", e))?;

        let answer = chat_response
            .choices
            .first()
            .map(|c| c.message.content.clone())
            .ok_or_else(|| "No response from OpenAI".to_string())?;

        Ok((answer, Some(raw_json)))
    }
}
