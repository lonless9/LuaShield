use std::env;
use std::fs::{File};
use std::io::{Read, Write};
use tempfile::TempDir;
use std::sync::Arc;
use wiremock::{matchers::{header, method, path}, Mock, MockServer, ResponseTemplate};

use luashield::{
    analyzer::Analyzer,
    config::Config,
    error::Result,
    output::{OutputFormat, Outputter},
};

#[tokio::test]
async fn test_full_analysis_workflow() -> Result<()> {
    // 创建 Mock 服务器
    let mock_server = MockServer::start().await;

    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let test_dir = temp_dir.path();

    // 创建测试文件
    let test_file = test_dir.join("test.lua");
    let mut file = File::create(&test_file)?;
    file.write_all(b"local function test()\n  print('test')\nend")?;

    // 创建 README 文件
    let readme_file = test_dir.join("README.md");
    let mut file = File::create(&readme_file)?;
    file.write_all(b"# Test Project\n\nThis is a test project.")?;

    // 配置 Mock 响应
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "这是一个简单的 Lua 函数，没有安全问题。"
                        }
                    }
                ]
            }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // 我们需要确保测试文件被排除测试目录外，因此在排除规则中添加例外
    // 创建 .luashield_exclude 文件以添加自定义排除规则
    std::fs::write(
        test_dir.join(".luashield_exclude"),
        r#"test/*
!test.lua
"#,
    )?;

    // 设置环境变量
    env::set_var("LUASHIELD_LLM_PROVIDER", "claude");
    env::set_var("LUASHIELD_API_KEY", "test-api-key");
    env::set_var("LUASHIELD_MODEL_NAME", "test-model");
    env::set_var("LUASHIELD_BASE_URL", format!("{}/v1", mock_server.uri()));
    env::set_var("LUASHIELD_ROOT_PATH", test_dir.to_str().unwrap());
    env::set_var("LUASHIELD_ANALYZE_README", "true");
    env::set_var("LUASHIELD_LOG_LEVEL", "info");
    env::set_var("LUASHIELD_OUTPUT_FORMAT", "text");

    // 加载配置
    let config = Config::load()?;
    let config_arc = Arc::new(config);

    // 创建分析器
    let analyzer = Analyzer::new(config_arc)?;

    // 执行分析
    let results = analyzer.analyze().await?;

    // 验证结果
    assert!(!results.is_empty());
    assert_eq!(
        results[0].file_path.file_name().unwrap(),
        test_file.file_name().unwrap()
    );
    assert!(!results[0].content.is_empty());

    // 创建输出器
    let output_file = test_dir.join("output.txt");
    let outputter = Outputter::new(OutputFormat::Text, Some(output_file.clone()));

    // 输出结果
    outputter.output(&results)?;

    // 验证输出文件
    let mut file = File::open(output_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    assert!(!content.is_empty());

    // 清理环境变量
    env::remove_var("LUASHIELD_LLM_PROVIDER");
    env::remove_var("LUASHIELD_API_KEY");
    env::remove_var("LUASHIELD_MODEL_NAME");
    env::remove_var("LUASHIELD_BASE_URL");
    env::remove_var("LUASHIELD_ROOT_PATH");
    env::remove_var("LUASHIELD_ANALYZE_README");
    env::remove_var("LUASHIELD_LOG_LEVEL");
    env::remove_var("LUASHIELD_OUTPUT_FORMAT");

    Ok(())
}

#[tokio::test]
async fn test_analysis_with_network_files() -> Result<()> {
    // 创建 Mock 服务器
    let mock_server = MockServer::start().await;

    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let test_dir = temp_dir.path();

    // 创建网络相关文件
    let network_file = test_dir.join("network.lua");
    let mut file = File::create(&network_file)?;
    file.write_all(b"local http = require('socket.http')\nlocal function test()\n  http.request('http://example.com')\nend")?;

    // 创建非网络相关文件
    let non_network_file = test_dir.join("non_network.lua");
    let mut file = File::create(&non_network_file)?;
    file.write_all(b"local function test()\n  print('test')\nend")?;

    // 配置 Mock 响应
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "这是一个网络相关的 Lua 文件，存在潜在安全风险。"
                        }
                    }
                ]
            }))
        )
        .expect(2)
        .mount(&mock_server)
        .await;

    // 添加自定义排除规则
    std::fs::write(
        test_dir.join(".luashield_exclude"),
        r#"test/*
!*.lua
"#,
    )?;

    // 设置环境变量
    env::set_var("LUASHIELD_LLM_PROVIDER", "claude");
    env::set_var("LUASHIELD_API_KEY", "test-api-key");
    env::set_var("LUASHIELD_MODEL_NAME", "test-model");
    env::set_var("LUASHIELD_BASE_URL", format!("{}/v1", mock_server.uri()));
    env::set_var("LUASHIELD_ROOT_PATH", test_dir.to_str().unwrap());
    env::set_var("LUASHIELD_ANALYZE_README", "false");
    env::set_var("LUASHIELD_LOG_LEVEL", "info");
    env::set_var("LUASHIELD_OUTPUT_FORMAT", "text");

    // 加载配置
    let config = Config::load()?;
    let config_arc = Arc::new(config);

    // 创建分析器
    let analyzer = Analyzer::new(config_arc)?;

    // 执行分析
    let results = analyzer.analyze().await?;

    // 验证结果
    assert!(!results.is_empty());
    assert!(results.iter().any(|r| r.file_path == network_file));
    assert!(results.iter().any(|r| r.file_path == non_network_file));

    // 清理环境变量
    env::remove_var("LUASHIELD_LLM_PROVIDER");
    env::remove_var("LUASHIELD_API_KEY");
    env::remove_var("LUASHIELD_MODEL_NAME");
    env::remove_var("LUASHIELD_BASE_URL");
    env::remove_var("LUASHIELD_ROOT_PATH");
    env::remove_var("LUASHIELD_ANALYZE_README");
    env::remove_var("LUASHIELD_LOG_LEVEL");
    env::remove_var("LUASHIELD_OUTPUT_FORMAT");

    Ok(())
}

#[tokio::test]
async fn test_analysis_with_different_output_formats() -> Result<()> {
    // 创建 Mock 服务器
    let mock_server = MockServer::start().await;

    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let test_dir = temp_dir.path();

    // 创建测试文件
    let test_file = test_dir.join("test.lua");
    let mut file = File::create(&test_file)?;
    file.write_all(b"local function test()\n  print('test')\nend")?;

    // 配置 Mock 响应
    Mock::given(method("POST"))
        .and(path("/v1/messages"))
        .and(header("x-api-key", "test-api-key"))
        .and(header("anthropic-version", "2023-06-01"))
        .respond_with(ResponseTemplate::new(200)
            .set_body_json(serde_json::json!({
                "choices": [
                    {
                        "message": {
                            "role": "assistant",
                            "content": "这是一个简单的 Lua 函数，没有安全问题。"
                        }
                    }
                ]
            }))
        )
        .expect(1)
        .mount(&mock_server)
        .await;

    // 我们需要确保测试文件被排除测试目录外，因此在排除规则中添加例外
    // 创建 .luashield_exclude 文件以添加自定义排除规则
    std::fs::write(
        test_dir.join(".luashield_exclude"),
        r#"test/*
!*.lua
"#,
    )?;

    // 设置环境变量
    env::set_var("LUASHIELD_LLM_PROVIDER", "claude");
    env::set_var("LUASHIELD_API_KEY", "test-api-key");
    env::set_var("LUASHIELD_MODEL_NAME", "test-model");
    env::set_var("LUASHIELD_BASE_URL", format!("{}/v1", mock_server.uri()));
    env::set_var("LUASHIELD_ROOT_PATH", test_dir.to_str().unwrap());
    env::set_var("LUASHIELD_ANALYZE_README", "false");
    env::set_var("LUASHIELD_LOG_LEVEL", "info");

    // 测试不同输出格式
    let formats = vec![OutputFormat::Text, OutputFormat::Json, OutputFormat::Markdown];
    
    // 加载配置（在循环外创建一次分析器）
    let config = Config::load()?;
    let config_arc = Arc::new(config);

    // 创建分析器
    let analyzer = Analyzer::new(config_arc.clone())?;

    // 执行分析（只分析一次）
    let results = analyzer.analyze().await?;

    for format in formats {
        // 设置输出格式（不需要重新加载配置）
        env::set_var("LUASHIELD_OUTPUT_FORMAT", format.to_string());

        // 创建输出器
        let output_file = test_dir.join(format!("output.{}", format.extension()));
        let outputter = Outputter::new(format, Some(output_file.clone()));

        // 输出结果
        outputter.output(&results)?;

        // 验证输出文件
        let mut file = File::open(output_file)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        assert!(!content.is_empty());
    }

    // 清理环境变量
    env::remove_var("LUASHIELD_LLM_PROVIDER");
    env::remove_var("LUASHIELD_API_KEY");
    env::remove_var("LUASHIELD_MODEL_NAME");
    env::remove_var("LUASHIELD_BASE_URL");
    env::remove_var("LUASHIELD_ROOT_PATH");
    env::remove_var("LUASHIELD_ANALYZE_README");
    env::remove_var("LUASHIELD_LOG_LEVEL");
    env::remove_var("LUASHIELD_OUTPUT_FORMAT");

    Ok(())
} 