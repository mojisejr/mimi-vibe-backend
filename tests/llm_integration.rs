//! Integration test for LLM provider abstraction.

use mimi_backend::services::llm::{LlmProvider, OpenAiClient};

#[tokio::test]
async fn test_openai_mock_mode() {
    // Create client in mock mode
    let client = OpenAiClient::new("mock-api-key".to_string(), "gpt-4o-mini".to_string(), true);

    // Test the provider trait implementation
    let result = client.ask("Test question").await;

    // Verify response
    assert!(result.is_ok());
    let (response, raw) = result.unwrap();
    assert_eq!(response, "This is a mock response for testing purposes.");
    assert!(raw.is_some());

    // Verify raw JSON structure
    let raw_json = raw.unwrap();
    assert_eq!(raw_json["id"], "mock-123");
    assert_eq!(raw_json["model"], "gpt-4o-mini");
}

#[tokio::test]
async fn test_provider_trait_object() {
    // Test that we can use Arc<dyn LlmProvider> (trait object)
    use std::sync::Arc;

    let client: Arc<dyn LlmProvider> = Arc::new(OpenAiClient::new(
        "mock-api-key".to_string(),
        "gpt-4o-mini".to_string(),
        true,
    ));

    // Test via trait object
    let result = client.ask("Test via trait object").await;
    assert!(result.is_ok());
    let (response, _) = result.unwrap();
    assert_eq!(response, "This is a mock response for testing purposes.");
}
