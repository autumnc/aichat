use futures::StreamExt;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct DeepSeekConfig {
    pub api_key: String,
    pub base_url: String,
    pub timeout_seconds: u64,
    pub model: String,
}

impl Default for DeepSeekConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            base_url: "https://api.deepseek.com".to_string(),
            timeout_seconds: 30,
            model: "deepseek-chat".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: Option<i32>,
    pub stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Choice {
    pub index: i32,
    pub message: MessageResponse,
    pub finish_reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct MessageResponse {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Usage {
    pub prompt_tokens: i32,
    pub completion_tokens: i32,
    pub total_tokens: i32,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub choices: Vec<StreamChoice>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamChoice {
    pub index: i32,
    pub delta: StreamDelta,
    pub finish_reason: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct StreamDelta {
    pub role: Option<String>,
    pub content: Option<String>,
}

#[derive(Debug)]
pub enum DeepSeekError {
    RequestError(String),
    ParseError(String),
    ApiError(String),
    Timeout(String),
    ConfigError(String),
}

impl std::fmt::Display for DeepSeekError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DeepSeekError::RequestError(msg) => write!(f, "Request error: {}", msg),
            DeepSeekError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            DeepSeekError::ApiError(msg) => write!(f, "API error: {}", msg),
            DeepSeekError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            DeepSeekError::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

impl std::error::Error for DeepSeekError {}

#[derive(Debug, Clone)]
pub struct DeepSeekClient {
    config: DeepSeekConfig,
    client: Client,
}

impl DeepSeekClient {
    pub fn new(config: DeepSeekConfig) -> Result<Self, DeepSeekError> {
        if config.api_key.is_empty() {
            return Err(DeepSeekError::ConfigError(
                "API key cannot be empty".to_string(),
            ));
        }
        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| {
                DeepSeekError::RequestError(format!("Failed to build HTTP client: {}", e))
            })?;
        Ok(Self { config, client })
    }

    pub fn with_api_key(api_key: &str) -> Result<Self, DeepSeekError> {
        let config = DeepSeekConfig {
            api_key: api_key.to_string(),
            ..Default::default()
        };
        Self::new(config)
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, DeepSeekError> {
        self.chat_with_options(messages, None, None, false).await
    }

    pub async fn chat_with_options(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<i32>,
        stream: bool,
    ) -> Result<String, DeepSeekError> {
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature: temperature.unwrap_or(0.7),
            max_tokens,
            stream,
        };
        let endpoint = format!("{}/v1/chat/completions", self.config.base_url);
        if stream {
            return self.chat_stream(&endpoint, &request).await;
        }
        let response = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await
            .map_err(|e| {
                if e.is_timeout() {
                    DeepSeekError::Timeout(format!(
                        "Request timeout after {} seconds",
                        self.config.timeout_seconds
                    ))
                } else {
                    DeepSeekError::RequestError(format!("Failed to send request: {}", e))
                }
            })?;
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(DeepSeekError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }
        let chat_response: ChatResponse = response
            .json()
            .await
            .map_err(|e| DeepSeekError::ParseError(format!("Failed to parse response: {}", e)))?;
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(DeepSeekError::ParseError(
                "No choices in response".to_string(),
            ))
        }
    }

    async fn chat_stream(
        &self,
        endpoint: &str,
        request: &ChatRequest,
    ) -> Result<String, DeepSeekError> {
        let response = self
            .client
            .post(endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .json(request)
            .send()
            .await
            .map_err(|e| {
                DeepSeekError::RequestError(format!("Failed to send stream request: {}", e))
            })?;
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(DeepSeekError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }
        let mut full_response = String::new();
        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk =
                item.map_err(|e| DeepSeekError::RequestError(format!("Stream error: {}", e)))?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        break;
                    }
                    match serde_json::from_str::<StreamResponse>(data) {
                        Ok(stream_response) => {
                            if let Some(choice) = stream_response.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    full_response.push_str(content);
                                }
                            }
                        }
                        Err(_) => {
                            continue;
                        }
                    }
                }
            }
        }
        Ok(full_response)
    }

    pub async fn simple_chat(
        &self,
        user_message: &str,
        system_prompt: Option<&str>,
    ) -> Result<String, DeepSeekError> {
        let mut messages = Vec::new();
        if let Some(prompt) = system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: prompt.to_string(),
            });
        }
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });
        self.chat(messages).await
    }

    pub async fn test_connection(&self) -> Result<bool, DeepSeekError> {
        let test_message = ChatMessage {
            role: "user".to_string(),
            content: "Hello, respond with 'OK' if you can hear me.".to_string(),
        };
        match self.chat(vec![test_message]).await {
            Ok(response) => Ok(response.contains("OK") || !response.is_empty()),
            Err(_) => Ok(false),
        }
    }

    pub fn get_config(&self) -> &DeepSeekConfig {
        &self.config
    }

    pub fn set_model(&mut self, model: &str) {
        self.config.model = model.to_string();
    }

    pub async fn simple_chat_stream<F>(
        &self,
        user_message: &str,
        system_prompt: Option<&str>,
        mut on_chunk: F,
    ) -> Result<String, DeepSeekError>
    where
        F: FnMut(String) + Send + 'static,
    {
        let mut messages = Vec::new();
        if let Some(prompt) = system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: prompt.to_string(),
            });
        }
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });
        let request = ChatRequest {
            model: self.config.model.clone(),
            messages,
            temperature: 0.7,
            max_tokens: None,
            stream: true,
        };
        let endpoint = format!("{}/v1/chat/completions", self.config.base_url);
        let response = self
            .client
            .post(&endpoint)
            .header("Authorization", format!("Bearer {}", self.config.api_key))
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .json(&request)
            .send()
            .await
            .map_err(|e| DeepSeekError::RequestError(format!("Failed to send request: {}", e)))?;
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(DeepSeekError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }
        let mut full_response = String::new();
        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk =
                item.map_err(|e| DeepSeekError::RequestError(format!("Stream error: {}", e)))?;
            let chunk_str = String::from_utf8_lossy(&chunk);
            for line in chunk_str.lines() {
                if line.starts_with("data: ") {
                    let data = &line[6..];
                    if data == "[DONE]" {
                        return Ok(full_response);
                    }
                    if data.trim().is_empty() {
                        continue;
                    }
                    match serde_json::from_str::<StreamResponse>(data) {
                        Ok(stream_response) => {
                            if let Some(choice) = stream_response.choices.first() {
                                if let Some(content) = &choice.delta.content {
                                    full_response.push_str(content);
                                    on_chunk(content.clone());
                                }
                            }
                        }
                        Err(_e) => {
                            if !data.contains(":") {
                                continue;
                            }
                        }
                    }
                }
            }
        }
        Ok(full_response)
    }
}

pub fn create_system_message(content: &str) -> ChatMessage {
    ChatMessage {
        role: "system".to_string(),
        content: content.to_string(),
    }
}

pub fn create_user_message(content: &str) -> ChatMessage {
    ChatMessage {
        role: "user".to_string(),
        content: content.to_string(),
    }
}

pub fn create_assistant_message(content: &str) -> ChatMessage {
    ChatMessage {
        role: "assistant".to_string(),
        content: content.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    fn test_deepseek_client_creation() {
        if let Ok(api_key) = std::env::var("DEEPSEEK_API_KEY") {
            let client = DeepSeekClient::with_api_key(&api_key);
            assert!(client.is_ok());
        }
    }

    #[test]
    fn test_message_creation() {
        let system_msg = create_system_message("You are a helpful assistant.");
        let user_msg = create_user_message("Hello, how are you?");
        let assistant_msg = create_assistant_message("I'm doing well, thank you!");
        assert_eq!(system_msg.role, "system");
        assert_eq!(user_msg.role, "user");
        assert_eq!(assistant_msg.role, "assistant");
    }

    #[ignore]
    #[test]
    fn test_integration_chat() {
        if let Ok(api_key) = std::env::var("DEEPSEEK_API_KEY") {
            let rt = Runtime::new().unwrap();
            rt.block_on(async {
                let client = DeepSeekClient::with_api_key(&api_key).unwrap();
                let connected = client.test_connection().await;
                println!("Connection test: {:?}", connected);
                let response = client.simple_chat("Hello, who are you?", None).await;
                match response {
                    Ok(reply) => {
                        println!("AI Response: {}", reply);
                        assert!(!reply.is_empty());
                    }
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            });
        }
    }
}
