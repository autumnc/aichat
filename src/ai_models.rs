use crate::ai::aliyun::{self, AliYunModelType};
use crate::i18n::Language;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum AIModel {
    DeepSeek,
    OpenAI,
    Claude,
    Gemini,
    AliYun(AliYunModelType),
    LocalLLM,
    Custom(String),
}

impl AIModel {
    pub fn all() -> Vec<Self> {
        vec![
            AIModel::DeepSeek,
            AIModel::AliYun(AliYunModelType::QwenTurbo),
            AIModel::AliYun(AliYunModelType::QwenPlus),
            AIModel::AliYun(AliYunModelType::QwenMax),
            AIModel::AliYun(AliYunModelType::QwenMaxLongContext),
            AIModel::OpenAI,
            AIModel::Claude,
            AIModel::Gemini,
            AIModel::LocalLLM,
            AIModel::Custom("Custom Model".to_string()),
        ]
    }

    pub fn name(&self, language: Language) -> String {
        match self {
            AIModel::DeepSeek => match language {
                Language::Chinese => "DeepSeek".to_string(),
                Language::English => "DeepSeek".to_string(),
            },
            AIModel::AliYun(model_type) => model_type.display_name(language),
            AIModel::OpenAI => match language {
                Language::Chinese => "OpenAI GPT".to_string(),
                Language::English => "OpenAI GPT".to_string(),
            },
            AIModel::Claude => match language {
                Language::Chinese => "Claude".to_string(),
                Language::English => "Claude".to_string(),
            },
            AIModel::Gemini => match language {
                Language::Chinese => "Google Gemini".to_string(),
                Language::English => "Google Gemini".to_string(),
            },
            AIModel::LocalLLM => match language {
                Language::Chinese => "本地大模型".to_string(),
                Language::English => "Local LLM".to_string(),
            },
            AIModel::Custom(name) => name.clone(),
        }
    }

    pub fn description(&self, language: Language) -> String {
        match self {
            AIModel::DeepSeek => match language {
                Language::Chinese => "深度求索公司的AI助手，支持128K上下文".to_string(),
                Language::English => {
                    "AI assistant from DeepSeek, supports 128K context".to_string()
                }
            },
            AIModel::AliYun(model_type) => model_type.description(language),
            AIModel::OpenAI => match language {
                Language::Chinese => "OpenAI的GPT系列模型，功能强大".to_string(),
                Language::English => {
                    "OpenAI's GPT series models, powerful capabilities".to_string()
                }
            },
            AIModel::Claude => match language {
                Language::Chinese => "Anthropic的Claude模型，安全可靠".to_string(),
                Language::English => "Anthropic's Claude model, safe and reliable".to_string(),
            },
            AIModel::Gemini => match language {
                Language::Chinese => "Google的Gemini多模态模型".to_string(),
                Language::English => "Google's Gemini multimodal model".to_string(),
            },
            AIModel::LocalLLM => match language {
                Language::Chinese => "本地运行的大语言模型，保护隐私".to_string(),
                Language::English => "Locally running LLM, privacy protected".to_string(),
            },
            AIModel::Custom(_) => match language {
                Language::Chinese => "自定义AI模型".to_string(),
                Language::English => "Custom AI model".to_string(),
            },
        }
    }

    pub fn needs_api_key(&self) -> bool {
        match self {
            AIModel::DeepSeek => true,
            AIModel::AliYun(_) => true,
            AIModel::OpenAI => true,
            AIModel::Claude => true,
            AIModel::Gemini => true,
            AIModel::LocalLLM => false,
            AIModel::Custom(_) => false,
        }
    }

    pub fn api_key_env_var(&self) -> Option<&'static str> {
        match self {
            AIModel::DeepSeek => Some("DEEPSEEK_API_KEY"),
            AIModel::AliYun(_) => Some("ALIYUN_API_KEY"),
            AIModel::OpenAI => Some("OPENAI_API_KEY"),
            AIModel::Claude => Some("CLAUDE_API_KEY"),
            AIModel::Gemini => Some("GEMINI_API_KEY"),
            AIModel::LocalLLM => None,
            AIModel::Custom(_) => None,
        }
    }

    pub fn is_real_api(&self) -> bool {
        match self {
            AIModel::DeepSeek => true,
            AIModel::AliYun(_) => true,
            AIModel::OpenAI => false,
            AIModel::Claude => false,
            AIModel::Gemini => false,
            AIModel::LocalLLM => false,
            AIModel::Custom(_) => false,
        }
    }

    pub fn simulate_response(&self, user_input: &str, language: Language) -> String {
        let model_name = self.name(language);
        match self {
            AIModel::OpenAI => match language {
                Language::Chinese => format!(
                    "🤖 {} 回复（模拟）:\n\n您好！我是{}，这是一个模拟回复。\n\n您的问题是：\"{}\"\n\n实际上，如果您配置了真实的API密钥，我可以连接到真实的{} API为您提供智能回复。",
                    model_name, model_name, user_input, model_name
                ),
                Language::English => format!(
                    "🤖 {} Response (Simulated):\n\nHello! I'm {}, this is a simulated response.\n\nYour question: \"{}\"\n\nIn reality, if you configure a real API key, I can connect to the real {} API to provide intelligent responses.",
                    model_name, model_name, user_input, model_name
                ),
            },
            AIModel::Claude => match language {
                Language::Chinese => format!(
                    "🤖 {} 回复（模拟）:\n\n你好！我是{}，这是模拟对话。\n\n你说：\"{}\"\n\n要获得真实回复，请配置相应的API密钥。",
                    model_name, model_name, user_input
                ),
                Language::English => format!(
                    "🤖 {} Response (Simulated):\n\nHello! I'm {}, this is a simulated conversation.\n\nYou said: \"{}\"\n\nTo get real responses, please configure the appropriate API key.",
                    model_name, model_name, user_input
                ),
            },
            AIModel::Gemini => match language {
                Language::Chinese => format!(
                    "🤖 {} 回复（模拟）:\n\n您好！我是Google的{}模型，当前为模拟模式。\n\n您输入的内容：{}\n\n如需真实功能，请设置API密钥。",
                    model_name, model_name, user_input
                ),
                Language::English => format!(
                    "🤖 {} Response (Simulated):\n\nHello! I'm Google's {} model, currently in simulation mode.\n\nYour input: {}\n\nFor real functionality, please set up the API key.",
                    model_name, model_name, user_input
                ),
            },
            AIModel::LocalLLM => match language {
                Language::Chinese => format!(
                    "🤖 {} 回复（模拟）:\n\n这是本地大模型的模拟回复。\n\n您的问题：{}\n\n本地模型运行在您的设备上，保护您的隐私。",
                    model_name, user_input
                ),
                Language::English => format!(
                    "🤖 {} Response (Simulated):\n\nThis is a simulated response from a local LLM.\n\nYour question: {}\n\nLocal models run on your device, protecting your privacy.",
                    model_name, user_input
                ),
            },
            AIModel::Custom(name) => match language {
                Language::Chinese => format!(
                    "🤖 自定义模型『{}』回复：\n\n这是自定义模型的模拟回复。\n\n输入内容：{}",
                    name, user_input
                ),
                Language::English => format!(
                    "🤖 Custom Model『{}』Response:\n\nThis is a simulated response from a custom model.\n\nInput: {}",
                    name, user_input
                ),
            },
            _ => match language {
                Language::Chinese => {
                    format!("错误：{}应该通过API调用，但进入了模拟模式", model_name)
                }
                Language::English => format!(
                    "Error: {} should be called via API, but entered simulation mode",
                    model_name
                ),
            },
        }
    }

    pub fn color(&self) -> ratatui::style::Color {
        match self {
            AIModel::DeepSeek => ratatui::style::Color::Green,
            AIModel::AliYun(_) => ratatui::style::Color::Blue,
            AIModel::OpenAI => ratatui::style::Color::Magenta,
            AIModel::Claude => ratatui::style::Color::Yellow,
            AIModel::Gemini => ratatui::style::Color::Red,
            AIModel::LocalLLM => ratatui::style::Color::Cyan,
            AIModel::Custom(_) => ratatui::style::Color::Gray,
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            AIModel::DeepSeek => "🔍",
            AIModel::AliYun(_) => "☁️",
            AIModel::OpenAI => "⚡",
            AIModel::Claude => "🧠",
            AIModel::Gemini => "💎",
            AIModel::LocalLLM => "💻",
            AIModel::Custom(_) => "🛠️",
        }
    }

    pub fn category(&self) -> &'static str {
        match self {
            AIModel::DeepSeek => "Cloud API",
            AIModel::AliYun(_) => "Cloud API",
            AIModel::OpenAI => "Cloud API",
            AIModel::Claude => "Cloud API",
            AIModel::Gemini => "Cloud API",
            AIModel::LocalLLM => "Local",
            AIModel::Custom(_) => "Custom",
        }
    }

    pub fn is_aliyun_model(&self) -> bool {
        matches!(self, AIModel::AliYun(_))
    }

    pub fn get_aliyun_model_type(&self) -> Option<aliyun::AliYunModelType> {
        match self {
            AIModel::AliYun(model_type) => Some(*model_type),
            _ => None,
        }
    }

    pub fn from_str(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "deepseek" => Some(AIModel::DeepSeek),
            "openaigpt" | "openai" => Some(AIModel::OpenAI),
            "claude" => Some(AIModel::Claude),
            "gemini" => Some(AIModel::Gemini),
            "localllm" | "local" => Some(AIModel::LocalLLM),
            "qwenturbo" => Some(AIModel::AliYun(aliyun::AliYunModelType::QwenTurbo)),
            "qwenplus" => Some(AIModel::AliYun(aliyun::AliYunModelType::QwenPlus)),
            "qwenmax" => Some(AIModel::AliYun(aliyun::AliYunModelType::QwenMax)),
            _ => {
                if name.starts_with("custom:") {
                    let custom_name = name.trim_start_matches("custom:").to_string();
                    Some(AIModel::Custom(custom_name))
                } else {
                    None
                }
            }
        }
    }

    pub fn to_string(&self) -> String {
        match self {
            AIModel::DeepSeek => "deepseek".to_string(),
            AIModel::AliYun(model_type) => match model_type {
                aliyun::AliYunModelType::QwenTurbo => "qwenturbo".to_string(),
                aliyun::AliYunModelType::QwenPlus => "qwenplus".to_string(),
                aliyun::AliYunModelType::QwenMax => "qwenmax".to_string(),
                aliyun::AliYunModelType::QwenMaxLongContext => "qwenmaxlongcontext".to_string(),
            },
            AIModel::OpenAI => "openai".to_string(),
            AIModel::Claude => "claude".to_string(),
            AIModel::Gemini => "gemini".to_string(),
            AIModel::LocalLLM => "localllm".to_string(),
            AIModel::Custom(name) => format!("custom:{}", name),
        }
    }

    pub fn default_model() -> Self {
        AIModel::DeepSeek
    }

    pub fn recommended_models() -> Vec<Self> {
        vec![
            AIModel::DeepSeek,
            AIModel::AliYun(aliyun::AliYunModelType::QwenTurbo),
            AIModel::AliYun(aliyun::AliYunModelType::QwenMax),
            AIModel::OpenAI,
        ]
    }

    pub fn get_model_info(&self, language: Language) -> ModelInfo {
        ModelInfo {
            name: self.name(language),
            description: self.description(language),
            needs_api_key: self.needs_api_key(),
            is_real_api: self.is_real_api(),
            icon: self.icon().to_string(),
            color: self.color(),
            category: self.category().to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct ModelInfo {
    pub name: String,
    pub description: String,
    pub needs_api_key: bool,
    pub is_real_api: bool,
    pub icon: String,
    pub color: ratatui::style::Color,
    pub category: String,
}

impl ModelInfo {
    pub fn display_text(&self, language: Language) -> String {
        match language {
            Language::Chinese => format!(
                "{} {} {}",
                self.icon,
                self.name,
                if self.needs_api_key { "🔑" } else { "" }
            ),
            Language::English => format!(
                "{} {} {}",
                self.icon,
                self.name,
                if self.needs_api_key { "🔑" } else { "" }
            ),
        }
    }

    pub fn detailed_info(&self, language: Language) -> String {
        match language {
            Language::Chinese => format!(
                "{} {}\n{}\n{} | {}",
                self.icon,
                self.name,
                self.description,
                if self.needs_api_key {
                    "需要API密钥"
                } else {
                    "无需API密钥"
                },
                if self.is_real_api {
                    "真实API"
                } else {
                    "模拟模式"
                }
            ),
            Language::English => format!(
                "{} {}\n{}\n{} | {}",
                self.icon,
                self.name,
                self.description,
                if self.needs_api_key {
                    "API key required"
                } else {
                    "No API key needed"
                },
                if self.is_real_api {
                    "Real API"
                } else {
                    "Simulation mode"
                }
            ),
        }
    }
}

pub fn create_aliyun_model(model_type: aliyun::AliYunModelType) -> AIModel {
    AIModel::AliYun(model_type)
}

pub fn create_custom_model(name: &str) -> AIModel {
    AIModel::Custom(name.to_string())
}
