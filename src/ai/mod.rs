use crate::ai::deepseek::DeepSeekClient;
use crate::i18n::Language;

pub mod aliyun;
pub mod deepseek;

pub async fn call_real_deepseek_api(user_input: &str, language: Language) -> String {
    dotenv::dotenv().ok();
    let api_key = match std::env::var("DEEPSEEK_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        Ok(_) => match language {
            Language::Chinese => {
                return "⚠️ DeepSeek API 密钥为空。请检查配置：\n\
                           1. .env 文件中的 DEEPSEEK_API_KEY 不能为空\n\
                           2. 或系统环境变量中的 DEEPSEEK_API_KEY 不能为空"
                    .to_string();
            }
            Language::English => {
                return "⚠️ DeepSeek API key is empty. Please check configuration:\n\
                           1. DEEPSEEK_API_KEY in .env file must not be empty\n\
                           2. Or DEEPSEEK_API_KEY in system environment variables must not be empty"
                    .to_string();
            }
        },
        Err(_) => match language {
            Language::Chinese => {
                return "⚠️ 未找到 DeepSeek API 密钥。请按以下方式配置：\n\
                           1. 在项目根目录创建 .env 文件，内容为：\n\
                              DEEPSEEK_API_KEY=your_api_key_here\n\
                           2. 或在系统环境变量中设置：\n\
                              export DEEPSEEK_API_KEY=your_api_key_here"
                    .to_string();
            }
            Language::English => {
                return "⚠️ DeepSeek API key not found. Please configure as follows:\n\
                           1. Create .env file in project root with content:\n\
                              DEEPSEEK_API_KEY=your_api_key_here\n\
                           2. Or set in system environment variables:\n\
                              export DEEPSEEK_API_KEY=your_api_key_here"
                    .to_string();
            }
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
    dotenv::dotenv().ok();
    let api_key = match std::env::var("ALIYUN_API_KEY") {
        Ok(key) if !key.trim().is_empty() => key,
        Ok(_) => match language {
            Language::Chinese => {
                return "⚠️ 阿里云API密钥为空。请检查配置：\n\
                       1. .env 文件中的 ALIYUN_API_KEY 不能为空\n\
                       2. 或系统环境变量中的 ALIYUN_API_KEY 不能为空"
                    .to_string();
            }
            Language::English => {
                return "⚠️ Aliyun API key is empty. Please check configuration:\n\
                       1. ALIYUN_API_KEY in .env file must not be empty\n\
                       2. Or ALIYUN_API_KEY in system environment variables must not be empty"
                    .to_string();
            }
        },
        Err(_) => match language {
            Language::Chinese => {
                return "⚠️ 未找到阿里云API密钥。请按以下方式配置：\n\
                       1. 在项目根目录创建 .env 文件，内容为：\n\
                          ALIYUN_API_KEY=your_api_key_here\n\
                       2. 或在系统环境变量中设置：\n\
                          export ALIYUN_API_KEY=your_api_key_here"
                    .to_string();
            }
            Language::English => {
                return "⚠️ Aliyun API key not found. Please configure as follows:\n\
                       1. Create .env file in project root with content:\n\
                          ALIYUN_API_KEY=your_api_key_here\n\
                       2. Or set in system environment variables:\n\
                          export ALIYUN_API_KEY=your_api_key_here"
                    .to_string();
            }
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
