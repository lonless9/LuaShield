use std::env;
use tempfile::TempDir;
use std::path::PathBuf;
use dotenvy;

use luashield::{
    config::{Config, LlmProvider},
    error::Result,
};

#[test]
fn test_config_default() {
    // 测试默认配置
    let config = Config::default();
    assert_eq!(config.llm_provider, LlmProvider::Claude);
    assert!(config.api_key.is_empty());
    assert_eq!(config.base_url, "https://api.anthropic.com/v1");
    assert_eq!(config.model_name, "claude-3-opus-20240229");
    assert_eq!(config.root_path.to_string_lossy(), ".");
    assert!(config.analyze_readme);
    assert_eq!(config.log_level, "info");
    assert_eq!(config.output_format, "text");
}

#[test]
fn test_config_load_from_env() -> Result<()> {
    // 获取当前目录
    let current_dir = std::env::current_dir()?;
    
    // 设置环境变量
    env::set_var("LUASHIELD_LLM_PROVIDER", "openai");
    env::set_var("LUASHIELD_API_KEY", "test-api-key");
    env::set_var("LUASHIELD_BASE_URL", "https://api.openai.com/v1");
    env::set_var("LUASHIELD_MODEL_NAME", "gpt-4");
    env::set_var("LUASHIELD_ROOT_PATH", current_dir.to_string_lossy().to_string());
    env::set_var("LUASHIELD_ANALYZE_README", "false");
    env::set_var("LUASHIELD_LOG_LEVEL", "debug");
    env::set_var("LUASHIELD_OUTPUT_FORMAT", "json");

    // 加载配置
    let config = Config::load()?;

    // 验证配置
    assert_eq!(config.llm_provider, LlmProvider::OpenAI);
    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.base_url, "https://api.openai.com/v1");
    assert_eq!(config.model_name, "gpt-4");
    assert_eq!(config.root_path, current_dir);
    assert!(!config.analyze_readme);
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.output_format, "json");

    // 清理环境变量
    env::remove_var("LUASHIELD_LLM_PROVIDER");
    env::remove_var("LUASHIELD_API_KEY");
    env::remove_var("LUASHIELD_BASE_URL");
    env::remove_var("LUASHIELD_MODEL_NAME");
    env::remove_var("LUASHIELD_ROOT_PATH");
    env::remove_var("LUASHIELD_ANALYZE_README");
    env::remove_var("LUASHIELD_LOG_LEVEL");
    env::remove_var("LUASHIELD_OUTPUT_FORMAT");

    Ok(())
}

#[test]
fn test_config_load_from_env_file() -> Result<()> {
    // 获取当前目录
    let current_dir = std::env::current_dir()?;
    
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let env_file = temp_dir.path().join(".env");

    // 写入环境变量文件
    std::fs::write(
        &env_file,
        format!(r#"LUASHIELD_LLM_PROVIDER=openai
LUASHIELD_API_KEY=test-api-key
LUASHIELD_BASE_URL=https://api.openai.com/v1
LUASHIELD_MODEL_NAME=llama2
LUASHIELD_ROOT_PATH={}
LUASHIELD_ANALYZE_README=false
LUASHIELD_LOG_LEVEL=debug
LUASHIELD_OUTPUT_FORMAT=json
"#, current_dir.to_string_lossy()),
    )?;

    // 直接从路径加载 .env 文件
    dotenvy::from_path(&env_file).ok();

    // 加载配置
    let config = Config::load()?;

    // 验证配置
    assert_eq!(config.llm_provider, LlmProvider::OpenAI);
    assert_eq!(config.api_key, "test-api-key");
    assert_eq!(config.base_url, "https://api.openai.com/v1");
    assert_eq!(config.model_name, "llama2");
    assert_eq!(config.root_path, current_dir);
    assert!(!config.analyze_readme);
    assert_eq!(config.log_level, "debug");
    assert_eq!(config.output_format, "json");

    // 清理环境变量，避免影响其他测试
    env::remove_var("LUASHIELD_LLM_PROVIDER");
    env::remove_var("LUASHIELD_API_KEY");
    env::remove_var("LUASHIELD_BASE_URL");
    env::remove_var("LUASHIELD_MODEL_NAME");
    env::remove_var("LUASHIELD_ROOT_PATH");
    env::remove_var("LUASHIELD_ANALYZE_README");
    env::remove_var("LUASHIELD_LOG_LEVEL");
    env::remove_var("LUASHIELD_OUTPUT_FORMAT");

    Ok(())
}

#[test]
fn test_config_validate() -> Result<()> {
    // 测试有效配置
    let mut config = Config::default();
    config.api_key = "test-api-key".to_string();
    // 确保根路径存在
    config.root_path = std::env::current_dir().unwrap();
    assert!(config.validate().is_ok());

    // 测试无效配置 - 缺少 API 密钥
    let mut config = Config::default();
    // 确保根路径存在
    config.root_path = std::env::current_dir().unwrap();
    assert!(config.validate().is_err());

    // 测试无效配置 - 根路径不存在
    let mut config = Config::default();
    config.api_key = "test-api-key".to_string();
    config.root_path = PathBuf::from("/path/that/does/not/exist");
    assert!(config.validate().is_err());

    Ok(())
}

#[test]
fn test_llm_provider_from_str() -> Result<()> {
    // 测试有效的 LLM 提供商
    assert_eq!("claude".parse::<LlmProvider>()?, LlmProvider::Claude);
    assert_eq!("openai".parse::<LlmProvider>()?, LlmProvider::OpenAI);
    assert_eq!("ollama".parse::<LlmProvider>()?, LlmProvider::Ollama);

    // 测试无效的 LLM 提供商
    assert!("invalid".parse::<LlmProvider>().is_err());

    Ok(())
} 