//! LLM 模块
//!
//! 负责与 LLM 服务交互。

use std::sync::Arc;
use tokio::sync::Mutex;
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::config::{Config, LlmProvider};
use crate::error::{LuaShieldError, Result};

/// LLM 客户端
#[derive(Debug, Clone)]
pub struct LlmClient {
    /// 配置
    config: Arc<Config>,
    /// HTTP 客户端
    client: Arc<Client>,
    /// 请求锁
    request_lock: Arc<Mutex<()>>,
}

/// LLM 请求
#[derive(Debug, Serialize)]
struct LlmRequest {
    /// 模型名称
    model: String,
    /// 消息列表
    messages: Vec<Message>,
    /// 温度
    temperature: f32,
    /// 最大令牌数
    max_tokens: u32,
}

/// 消息
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Message {
    /// 角色
    pub role: String,
    /// 内容
    pub content: String,
}

/// LLM 响应
#[derive(Debug, Deserialize)]
struct LlmResponse {
    /// 选择列表
    choices: Vec<Choice>,
}

/// 选择
#[derive(Debug, Deserialize)]
struct Choice {
    /// 消息
    message: Message,
}

impl LlmClient {
    /// 创建 LLM 客户端
    ///
    /// # 参数
    ///
    /// * `config` - 配置
    ///
    /// # 返回
    ///
    /// * `Result<Self>` - LLM 客户端
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let client = Client::new();
        let request_lock = Arc::new(Mutex::new(()));

        Ok(Self {
            config,
            client: Arc::new(client),
            request_lock,
        })
    }

    /// 发送请求
    ///
    /// # 参数
    ///
    /// * `messages` - 消息列表
    ///
    /// # 返回
    ///
    /// * `Result<String>` - 响应内容
    pub async fn send_request(&self, messages: Vec<Message>) -> Result<String> {
        // 获取请求锁
        let _lock = self.request_lock.lock().await;

        // 创建请求
        let request = LlmRequest {
            model: self.config.model_name.clone(),
            messages,
            temperature: 0.7,
            max_tokens: 4096,
        };

        // 发送请求
        let response = match self.config.llm_provider {
            LlmProvider::Claude => {
                let url = format!("{}/messages", self.config.base_url);
                self.client
                    .post(url)
                    .header("x-api-key", &self.config.api_key)
                    .header("anthropic-version", "2023-06-01")
                    .json(&request)
                    .send()
                    .await
                    .map_err(|e| LuaShieldError::LlmError(format!("请求失败: {}", e)))?
            }
            LlmProvider::OpenAI => {
                let url = format!("{}/chat/completions", self.config.base_url);
                self.client
                    .post(url)
                    .header("Authorization", format!("Bearer {}", self.config.api_key))
                    .json(&request)
                    .send()
                    .await
                    .map_err(|e| LuaShieldError::LlmError(format!("请求失败: {}", e)))?
            }
            LlmProvider::Ollama => {
                let url = format!("{}/api/chat", self.config.base_url);
                self.client
                    .post(url)
                    .json(&request)
                    .send()
                    .await
                    .map_err(|e| LuaShieldError::LlmError(format!("请求失败: {}", e)))?
            }
        };

        // 检查响应状态
        if !response.status().is_success() {
            let status = response.status();
            let text = response
                .text()
                .await
                .map_err(|e| LuaShieldError::LlmError(format!("读取响应失败: {}", e)))?;

            return Err(LuaShieldError::LlmError(format!(
                "请求失败: {} - {}",
                status, text
            )));
        }

        // 解析响应
        let response: LlmResponse = response
            .json()
            .await
            .map_err(|e| LuaShieldError::LlmError(format!("解析响应失败: {}", e)))?;

        // 获取响应内容
        let content = response
            .choices
            .first()
            .map(|choice| choice.message.content.clone())
            .ok_or_else(|| LuaShieldError::LlmError("响应内容为空".to_string()))?;

        Ok(content)
    }

    /// 分析代码
    ///
    /// # 参数
    ///
    /// * `code` - 代码内容
    /// * `context` - 上下文信息
    ///
    /// # 返回
    ///
    /// * `Result<String>` - 分析结果
    pub async fn analyze_code(&self, code: &str, context: &str) -> Result<String> {
        // 构建系统消息
        let system_message = Message {
            role: "system".to_string(),
            content: format!(
                "你是一个专业的 Lua 代码安全分析专家。请分析以下 Lua 代码中的安全漏洞，并提供详细的修复建议。\n\n上下文信息：\n{}\n\n请按照以下格式输出分析结果：\n\n1. 漏洞概述\n2. 漏洞详情\n3. 修复建议\n4. 最佳实践",
                context
            ),
        };

        // 构建用户消息
        let user_message = Message {
            role: "user".to_string(),
            content: format!("请分析以下 Lua 代码：\n\n```lua\n{}\n```", code),
        };

        // 发送请求
        self.send_request(vec![system_message, user_message]).await
    }
} 