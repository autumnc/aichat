use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub deepseek: Option<ApiSection>,
    pub aliyun: Option<ApiSection>,
    pub openai: Option<ApiSection>,
    pub claude: Option<ApiSection>,
    pub gemini: Option<ApiSection>,
}

#[derive(Debug, Deserialize)]
pub struct ApiSection {
    pub api_key: Option<String>,
}

impl Config {
    pub fn load() -> Option<Self> {
        let path = config_path();
        let content = std::fs::read_to_string(&path).ok()?;
        toml::from_str(&content).ok()
    }

    pub fn get_api_key(&self, provider: &str) -> Option<&str> {
        match provider {
            "deepseek" => self.deepseek.as_ref()?.api_key.as_deref(),
            "aliyun" => self.aliyun.as_ref()?.api_key.as_deref(),
            "openai" => self.openai.as_ref()?.api_key.as_deref(),
            "claude" => self.claude.as_ref()?.api_key.as_deref(),
            "gemini" => self.gemini.as_ref()?.api_key.as_deref(),
            _ => None,
        }
    }
}

pub fn config_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/home".to_string());
    PathBuf::from(home).join(".config/aichat/config.toml")
}
