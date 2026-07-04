use crate::ai::deepseek::DeepSeekClient;
use crate::i18n::Language;

pub mod aliyun;
pub mod deepseek;

pub async fn call_real_deepseek_api(user_input: &str, language: Language) -> String {
    let config = crate::config::Config::load();
    let config_path = crate::config::config_path();

    let api_key = match config.as_ref().and_then(|c| c.get_api_key("deepseek")) {
        Some(key) if !key.trim().is_empty() => key.to_string(),
        _ => match config {
            Some(_) => match language {
                Language::Chinese => {
                    return format!(
                        "⚠️ DeepSeek API 密钥为空。请检查配置：\n\
                         {path} 中的 deepseek.api_key 不能为空",
                        path = config_path.display()
                    );
                }
                Language::English => {
                    return format!(
                        "⚠️ DeepSeek API key is empty. Please check configuration:\n\
                         deepseek.api_key in {path} must not be empty",
                        path = config_path.display()
                    );
                }
            },
            None => match language {
                Language::Chinese => {
                    return format!(
                        "⚠️ 未找到 DeepSeek API 密钥。请按以下方式配置：\n\
                         在 {path} 中添加：\n\
                         [deepseek]\n\
                         api_key = \"your_api_key_here\"",
                        path = config_path.display()
                    );
                }
                Language::English => {
                    return format!(
                        "⚠️ DeepSeek API key not found. Please configure:\n\
                         Add to {path}:\n\
                         [deepseek]\n\
                         api_key = \"your_api_key_here\"",
                        path = config_path.display()
                    );
                }
            },
        },
    };
    match DeepSeekClient::with_api_key(&api_key) {
        Ok(client) => match client.simple_chat(user_input, None).await {
            Ok(response) => response,
            Err(e) => match language {
                Language::Chinese => format!("⚠️ API调用失败: {}", e),
                Language::English => format!("⚠️ API call failed: {}", e),
            },
        },
        Err(e) => match language {
            Language::Chinese => format!("⚠️ 客户端创建失败: {}", e),
            Language::English => format!("⚠️ Client creation failed: {}", e),
        },
    }
}

pub async fn call_real_aliyun_api(
    user_input: &str,
    language: Language,
    model_type: aliyun::AliYunModelType,
) -> String {
    let config = crate::config::Config::load();
    let config_path = crate::config::config_path();

    let api_key = match config.as_ref().and_then(|c| c.get_api_key("aliyun")) {
        Some(key) if !key.trim().is_empty() => key.to_string(),
        _ => match config {
            Some(_) => match language {
                Language::Chinese => {
                    return format!(
                        "⚠️ 阿里云API密钥为空。请检查配置：\n\
                         {path} 中的 aliyun.api_key 不能为空",
                        path = config_path.display()
                    );
                }
                Language::English => {
                    return format!(
                        "⚠️ Aliyun API key is empty. Please check configuration:\n\
                         aliyun.api_key in {path} must not be empty",
                        path = config_path.display()
                    );
                }
            },
            None => match language {
                Language::Chinese => {
                    return format!(
                        "⚠️ 未找到阿里云API密钥。请按以下方式配置：\n\
                         在 {path} 中添加：\n\
                         [aliyun]\n\
                         api_key = \"your_api_key_here\"",
                        path = config_path.display()
                    );
                }
                Language::English => {
                    return format!(
                        "⚠️ Aliyun API key not found. Please configure:\n\
                         Add to {path}:\n\
                         [aliyun]\n\
                         api_key = \"your_api_key_here\"",
                        path = config_path.display()
                    );
                }
            },
        },
    };
    match aliyun::AliYunClient::with_api_key_and_model(&api_key, model_type) {
        Ok(client) => match client.simple_chat(user_input, None, language).await {
            Ok(response) => response,
            Err(e) => match language {
                Language::Chinese => format!("⚠️ 阿里云API调用失败: {}", e),
                Language::English => format!("⚠️ Aliyun API call failed: {}", e),
            },
        },
        Err(e) => match language {
            Language::Chinese => format!("⚠️ 阿里云客户端创建失败: {}", e),
            Language::English => format!("⚠️ Aliyun client creation failed: {}", e),
        },
    }
}
