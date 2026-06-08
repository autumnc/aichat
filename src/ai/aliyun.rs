use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::ai::deepseek::{ChatMessage, StreamResponse};

#[derive(Debug, Clone, PartialEq, Copy, Eq, Hash)]
pub enum AliYunModelType {
    QwenTurbo,
    QwenPlus,
    QwenMax,
    QwenMaxLongContext,
}

impl AliYunModelType {
    pub fn name(&self) -> &'static str {
        match self {
            AliYunModelType::QwenTurbo => "qwen-turbo",
            AliYunModelType::QwenPlus => "qwen-plus",
            AliYunModelType::QwenMax => "qwen-max",
            AliYunModelType::QwenMaxLongContext => "qwen-max-longcontext",
        }
    }

    pub fn display_name(&self, language: crate::i18n::Language) -> String {
        match self {
            AliYunModelType::QwenTurbo => match language {
                crate::i18n::Language::Chinese => "通义千问-Turbo".to_string(),
                crate::i18n::Language::English => "Qwen-Turbo".to_string(),
            },
            AliYunModelType::QwenPlus => match language {
                crate::i18n::Language::Chinese => "通义千问-Plus".to_string(),
                crate::i18n::Language::English => "Qwen-Plus".to_string(),
            },
            AliYunModelType::QwenMax => match language {
                crate::i18n::Language::Chinese => "通义千问-Max".to_string(),
                crate::i18n::Language::English => "Qwen-Max".to_string(),
            },
            AliYunModelType::QwenMaxLongContext => match language {
                crate::i18n::Language::Chinese => "通义千问-长文本".to_string(),
                crate::i18n::Language::English => "Qwen-Max-LongContext".to_string(),
            },
        }
    }

    pub fn description(&self, language: crate::i18n::Language) -> String {
        match self {
            AliYunModelType::QwenTurbo => match language {
                crate::i18n::Language::Chinese => {
                    "轻量版，响应速度快，适合通用对话场景".to_string()
                }
                crate::i18n::Language::English => {
                    "Lightweight version, fast response, suitable for general conversation"
                        .to_string()
                }
            },
            AliYunModelType::QwenPlus => match language {
                crate::i18n::Language::Chinese => "增强版，适合复杂任务和长文本处理".to_string(),
                crate::i18n::Language::English => {
                    "Enhanced version, suitable for complex tasks and long text".to_string()
                }
            },
            AliYunModelType::QwenMax => match language {
                crate::i18n::Language::Chinese => "最强版本，适用于高需求专业任务".to_string(),
                crate::i18n::Language::English => {
                    "Maximum version, strongest capabilities for professional tasks".to_string()
                }
            },
            AliYunModelType::QwenMaxLongContext => match language {
                crate::i18n::Language::Chinese => "支持128K长文本，适合长文档处理".to_string(),
                crate::i18n::Language::English => {
                    "Supports 128K long context, suitable for long documents".to_string()
                }
            },
        }
    }

    pub fn endpoint(&self) -> &'static str {
        "https://dashscope.aliyuncs.com/compatible-mode/v1/chat/completions"
    }

    pub fn max_tokens(&self) -> i32 {
        match self {
            AliYunModelType::QwenTurbo => 2000,
            AliYunModelType::QwenPlus => 6000,
            AliYunModelType::QwenMax => 8000,
            AliYunModelType::QwenMaxLongContext => 8000,
        }
    }

    pub fn all() -> Vec<Self> {
        vec![
            AliYunModelType::QwenTurbo,
            AliYunModelType::QwenPlus,
            AliYunModelType::QwenMax,
            AliYunModelType::QwenMaxLongContext,
        ]
    }
}

#[derive(Debug, Clone)]
pub struct AliYunConfig {
    pub api_key: String,
    pub model_type: AliYunModelType,
    pub timeout_seconds: u64,
    pub base_url: String,
}

impl Default for AliYunConfig {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            model_type: AliYunModelType::QwenTurbo,
            timeout_seconds: 30,
            base_url: "https://dashscope.aliyuncs.com".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct AliYunChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: Option<i32>,
    pub stream: bool,
}

#[derive(Debug, Clone, Deserialize)]
pub struct AliYunChatResponse {
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

#[derive(Debug)]
pub enum AliYunError {
    RequestError(String),
    ParseError(String),
    ApiError(String),
    Timeout(String),
    ConfigError(String),
}

impl std::fmt::Display for AliYunError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AliYunError::RequestError(msg) => write!(f, "Request error: {}", msg),
            AliYunError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            AliYunError::ApiError(msg) => write!(f, "API error: {}", msg),
            AliYunError::Timeout(msg) => write!(f, "Timeout: {}", msg),
            AliYunError::ConfigError(msg) => write!(f, "Config error: {}", msg),
        }
    }
}

impl std::error::Error for AliYunError {}

#[derive(Debug, Clone)]
pub struct AliYunClient {
    config: AliYunConfig,
    client: Client,
}

impl AliYunClient {
    pub fn new(config: AliYunConfig) -> Result<Self, AliYunError> {
        if config.api_key.is_empty() {
            return Err(AliYunError::ConfigError(
                "API key cannot be empty".to_string(),
            ));
        }

        let client = Client::builder()
            .timeout(Duration::from_secs(config.timeout_seconds))
            .build()
            .map_err(|e| {
                AliYunError::RequestError(format!("Failed to build HTTP client: {}", e))
            })?;

        Ok(Self { config, client })
    }

    pub fn with_api_key(api_key: &str) -> Result<Self, AliYunError> {
        let config = AliYunConfig {
            api_key: api_key.to_string(),
            ..Default::default()
        };
        Self::new(config)
    }

    pub fn with_api_key_and_model(
        api_key: &str,
        model_type: AliYunModelType,
    ) -> Result<Self, AliYunError> {
        let config = AliYunConfig {
            api_key: api_key.to_string(),
            model_type,
            ..Default::default()
        };
        Self::new(config)
    }

    pub async fn chat(&self, messages: Vec<ChatMessage>) -> Result<String, AliYunError> {
        self.chat_with_options(messages, None, None, false).await
    }

    pub async fn chat_with_options(
        &self,
        messages: Vec<ChatMessage>,
        temperature: Option<f32>,
        max_tokens: Option<i32>,
        stream: bool,
    ) -> Result<String, AliYunError> {
        let request = AliYunChatRequest {
            model: self.config.model_type.name().to_string(),
            messages,
            temperature: temperature.unwrap_or(0.7),
            max_tokens,
            stream,
        };
        let endpoint = format!(
            "{}/compatible-mode/v1/chat/completions",
            self.config.base_url
        );
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
                    AliYunError::Timeout(format!(
                        "Request timeout after {} seconds",
                        self.config.timeout_seconds
                    ))
                } else {
                    AliYunError::RequestError(format!("Failed to send request: {}", e))
                }
            })?;
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AliYunError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }
        let chat_response: AliYunChatResponse = response
            .json()
            .await
            .map_err(|e| AliYunError::ParseError(format!("Failed to parse response: {}", e)))?;
        if let Some(choice) = chat_response.choices.first() {
            Ok(choice.message.content.clone())
        } else {
            Err(AliYunError::ParseError(
                "No choices in response".to_string(),
            ))
        }
    }

    async fn chat_stream(
        &self,
        endpoint: &str,
        request: &AliYunChatRequest,
    ) -> Result<String, AliYunError> {
        use futures::StreamExt;
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
                AliYunError::RequestError(format!("Failed to send stream request: {}", e))
            })?;
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response
                .text()
                .await
                .unwrap_or_else(|_| "Unknown error".to_string());
            return Err(AliYunError::ApiError(format!(
                "HTTP {}: {}",
                status, error_text
            )));
        }
        let mut full_response = String::new();
        let mut stream = response.bytes_stream();
        while let Some(item) = stream.next().await {
            let chunk =
                item.map_err(|e| AliYunError::RequestError(format!("Stream error: {}", e)))?;
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
        language: crate::i18n::Language,
    ) -> Result<String, AliYunError> {
        let mut messages = Vec::new();
        if let Some(prompt) = system_prompt {
            messages.push(ChatMessage {
                role: "system".to_string(),
                content: prompt.to_string(),
            });
        }
        let language_prompt = match language {
            crate::i18n::Language::Chinese => "请使用中文回答。",
            crate::i18n::Language::English => "Please respond in English only.",
        };
        messages.push(ChatMessage {
            role: "system".to_string(),
            content: language_prompt.to_string(),
        });
        messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.to_string(),
        });
        self.chat(messages).await
    }

    pub async fn test_connection(&self) -> Result<bool, AliYunError> {
        let test_message = ChatMessage {
            role: "user".to_string(),
            content: "Hello, respond with 'OK' if you can hear me.".to_string(),
        };
        match self.chat(vec![test_message]).await {
            Ok(response) => Ok(response.contains("OK") || !response.is_empty()),
            Err(_) => Ok(false),
        }
    }

    pub fn get_config(&self) -> &AliYunConfig {
        &self.config
    }

    pub fn set_model(&mut self, model_type: AliYunModelType) {
        self.config.model_type = model_type;
    }
}
