//! 配置模块
//!
//! 负责加载和管理配置信息，包括 API 密钥、基础 URL、模型名称等。

use std::env;
use std::path::PathBuf;

use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

use crate::error::{LuaShieldError, Result};

/// LLM 提供商
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LlmProvider {
    /// Claude
    Claude,
    /// OpenAI
    OpenAI,
    /// Ollama
    Ollama,
}

impl std::str::FromStr for LlmProvider {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "claude" => Ok(LlmProvider::Claude),
            "openai" => Ok(LlmProvider::OpenAI),
            "ollama" => Ok(LlmProvider::Ollama),
            _ => Err(format!("未知的 LLM 提供商: {}", s)),
        }
    }
}

/// 配置结构体
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// LLM 提供商
    pub llm_provider: LlmProvider,
    /// API 密钥
    pub api_key: String,
    /// 基础 URL
    pub base_url: String,
    /// 模型名称
    pub model_name: String,
    /// 仓库根路径
    pub root_path: PathBuf,
    /// 是否分析 README
    pub analyze_readme: bool,
    /// 日志级别
    pub log_level: String,
    /// 输出格式
    pub output_format: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            llm_provider: LlmProvider::Claude,
            api_key: String::new(),
            base_url: String::from("https://api.anthropic.com/v1"),
            model_name: String::from("claude-3-opus-20240229"),
            root_path: PathBuf::from("."),
            analyze_readme: true,
            log_level: String::from("info"),
            output_format: String::from("text"),
        }
    }
}

impl Config {
    /// 加载配置
    pub fn load() -> Result<Self> {
        // 仅当环境变量标记不存在时才加载 .env 文件
        if env::var("LUASHIELD_ENV_LOADED").is_err() {
            // 加载 .env 文件
            dotenv().ok();
            // 设置已加载标记
            env::set_var("LUASHIELD_ENV_LOADED", "true");
        }

        // 创建默认配置
        let mut config = Config::default();

        // 从环境变量加载配置
        if let Ok(llm_provider) = env::var("LUASHIELD_LLM_PROVIDER") {
            config.llm_provider = llm_provider.parse().map_err(|e| {
                LuaShieldError::ConfigError(format!("无效的 LLM 提供商: {}", e))
            })?;
            
            // 根据 LLM 提供商设置默认的基础 URL
            config.base_url = match config.llm_provider {
                LlmProvider::Claude => String::from("https://api.anthropic.com/v1"),
                LlmProvider::OpenAI => String::from("https://api.openai.com/v1"),
                LlmProvider::Ollama => String::from("http://localhost:11434"),
            };
        }

        if let Ok(api_key) = env::var("LUASHIELD_API_KEY") {
            config.api_key = api_key;
        }

        if let Ok(base_url) = env::var("LUASHIELD_BASE_URL") {
            config.base_url = base_url;
        }

        if let Ok(model_name) = env::var("LUASHIELD_MODEL_NAME") {
            config.model_name = model_name;
        }

        if let Ok(root_path) = env::var("LUASHIELD_ROOT_PATH") {
            config.root_path = PathBuf::from(root_path);
        }

        if let Ok(analyze_readme) = env::var("LUASHIELD_ANALYZE_README") {
            config.analyze_readme = analyze_readme.parse().unwrap_or(true);
        }

        if let Ok(log_level) = env::var("LUASHIELD_LOG_LEVEL") {
            config.log_level = log_level;
        }

        if let Ok(output_format) = env::var("LUASHIELD_OUTPUT_FORMAT") {
            config.output_format = output_format;
        }

        // 验证配置
        config.validate()?;

        Ok(config)
    }

    /// 验证配置
    pub fn validate(&self) -> Result<()> {
        // 检查 API 密钥
        if self.api_key.is_empty() {
            return Err(LuaShieldError::ConfigError("API 密钥不能为空".to_string()));
        }

        // 检查根路径
        if !self.root_path.exists() {
            return Err(LuaShieldError::ConfigError(format!(
                "根路径不存在: {}",
                self.root_path.display()
            )));
        }

        // 检查日志级别
        let valid_log_levels = ["error", "warn", "info", "debug", "trace"];
        if !valid_log_levels.contains(&self.log_level.as_str()) {
            return Err(LuaShieldError::ConfigError(format!(
                "无效的日志级别: {}",
                self.log_level
            )));
        }

        // 检查输出格式
        let valid_output_formats = ["text", "json", "html"];
        if !valid_output_formats.contains(&self.output_format.as_str()) {
            return Err(LuaShieldError::ConfigError(format!(
                "无效的输出格式: {}",
                self.output_format
            )));
        }

        Ok(())
    }

    /// 保存配置到环境变量或配置文件
    pub fn save(&self) -> Result<()> {
        // 在实际实现中，这可能会将配置写入 .env 文件或系统环境变量
        // 这里我们简单实现一个将配置写入到 .env 文件的功能
        
        let home_dir = dirs::home_dir().ok_or_else(|| 
            LuaShieldError::ConfigError("无法获取用户主目录".to_string())
        )?;
        
        let config_dir = home_dir.join(".luashield");
        std::fs::create_dir_all(&config_dir).map_err(|e| 
            LuaShieldError::FileSystemError(format!("创建配置目录失败: {}", e))
        )?;
        
        let config_file = config_dir.join("config.json");
        let config_json = serde_json::to_string_pretty(self).map_err(|e| 
            LuaShieldError::ConfigError(format!("序列化配置失败: {}", e))
        )?;
        
        std::fs::write(&config_file, config_json).map_err(|e| 
            LuaShieldError::FileSystemError(format!("写入配置文件失败: {}", e))
        )?;
        
        Ok(())
    }
} 