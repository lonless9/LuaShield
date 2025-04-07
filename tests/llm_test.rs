use std::sync::Arc;

use tokio_test::block_on;
use wiremock::matchers::{body_json, header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use luashield::{
    config::{Config, LlmProvider},
    error::Result,
    llm::{LlmClient, Message},
};

#[test]
fn test_send_request_claude() -> Result<()> {
    // 创建 wiremock 服务器
    let mock_server = block_on(MockServer::start());

    // 创建配置
    let mut config = Config::default();
    config.llm_provider = LlmProvider::Claude;
    config.api_key = "test-api-key".to_string();
    config.model_name = "test-model".to_string();
    config.base_url = format!("{}/v1", mock_server.uri()); // 确保基础 URL 包含 /v1

    // 配置模拟响应
    block_on(Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .and(body_json(serde_json::json!({
            "model": "test-model",
            "messages": [{
                "role": "user",
                "content": "test message"
            }],
            "temperature": 0.7,
            "max_tokens": 4096
        })))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "test response"
                        }
                    }
                ]
            }))
        )
        .mount(&mock_server));

    // 创建 LLM 客户端
    let llm_client = LlmClient::new(Arc::new(config))?;

    // 创建消息
    let messages = vec![Message {
        role: "user".to_string(),
        content: "test message".to_string(),
    }];

    // 发送请求
    let response = block_on(llm_client.send_request(messages))?;

    // 验证结果
    assert_eq!(response, "test response");

    Ok(())
}

#[test]
fn test_send_request_openai() -> Result<()> {
    // 创建 wiremock 服务器
    let mock_server = block_on(MockServer::start());

    // 创建配置
    let mut config = Config::default();
    config.llm_provider = LlmProvider::OpenAI;
    config.api_key = "test-api-key".to_string();
    config.model_name = "test-model".to_string();
    config.base_url = format!("{}/v1", mock_server.uri()); // 确保基础 URL 包含 /v1

    // 配置模拟响应
    block_on(Mock::given(method("POST"))
        .and(path("/v1/chat/completions"))
        .and(header("Authorization", "Bearer test-api-key"))
        .and(body_json(serde_json::json!({
            "model": "test-model",
            "messages": [{
                "role": "user",
                "content": "test message"
            }],
            "temperature": 0.7,
            "max_tokens": 4096
        })))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "test response"
                        }
                    }
                ]
            }))
        )
        .mount(&mock_server));

    // 创建 LLM 客户端
    let llm_client = LlmClient::new(Arc::new(config))?;

    // 创建消息
    let messages = vec![Message {
        role: "user".to_string(),
        content: "test message".to_string(),
    }];

    // 发送请求
    let response = block_on(llm_client.send_request(messages))?;

    // 验证结果
    assert_eq!(response, "test response");

    Ok(())
}

#[test]
fn test_send_request_ollama() -> Result<()> {
    // 创建 wiremock 服务器
    let mock_server = block_on(MockServer::start());

    // 创建配置
    let mut config = Config::default();
    config.llm_provider = LlmProvider::Ollama;
    config.api_key = "test-api-key".to_string();
    config.model_name = "test-model".to_string();
    config.base_url = mock_server.uri(); // Ollama 不需要添加 /v1 路径

    // 配置模拟响应
    block_on(Mock::given(method("POST"))
        .and(path("/api/chat"))
        .and(body_json(serde_json::json!({
            "model": "test-model",
            "messages": [{
                "role": "user",
                "content": "test message"
            }],
            "temperature": 0.7,
            "max_tokens": 4096
        })))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "test response"
                        }
                    }
                ]
            }))
        )
        .mount(&mock_server));

    // 创建 LLM 客户端
    let llm_client = LlmClient::new(Arc::new(config))?;

    // 创建消息
    let messages = vec![Message {
        role: "user".to_string(),
        content: "test message".to_string(),
    }];

    // 发送请求
    let response = block_on(llm_client.send_request(messages))?;

    // 验证结果
    assert_eq!(response, "test response");

    Ok(())
}

#[test]
fn test_analyze_code() -> Result<()> {
    // 创建 wiremock 服务器
    let mock_server = block_on(MockServer::start());

    // 创建配置
    let mut config = Config::default();
    config.llm_provider = LlmProvider::Claude;
    config.api_key = "test-api-key".to_string();
    config.model_name = "test-model".to_string();
    config.base_url = format!("{}/v1", mock_server.uri()); // 确保基础 URL 包含 /v1

    // 配置模拟响应
    block_on(Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "test analysis"
                        }
                    }
                ]
            }))
        )
        .mount(&mock_server));

    // 创建 LLM 客户端
    let llm_client = LlmClient::new(Arc::new(config))?;

    // 分析代码
    let result = block_on(llm_client.analyze_code("test code", "test context"))?;

    // 验证结果
    assert_eq!(result, "test analysis");

    Ok(())
} 