use std::io;
use std::path::PathBuf;

use luashield::error::{LuaShieldError, Result};

#[test]
fn test_error_display() {
    // 测试 IO 错误
    let io_error = LuaShieldError::IoError(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    assert_eq!(
        io_error.to_string(),
        "IO 错误: file not found"
    );

    // 测试 LLM 错误
    let llm_error = LuaShieldError::LlmError("API request failed".to_string());
    assert_eq!(
        llm_error.to_string(),
        "LLM 错误: API request failed"
    );

    // 测试配置错误
    let config_error = LuaShieldError::ConfigError("invalid configuration".to_string());
    assert_eq!(
        config_error.to_string(),
        "配置错误: invalid configuration"
    );

    // 测试分析错误
    let analysis_error = LuaShieldError::AnalysisError("analysis failed".to_string());
    assert_eq!(
        analysis_error.to_string(),
        "分析错误: analysis failed"
    );

    // 测试输出错误
    let output_error = LuaShieldError::OutputError("output failed".to_string());
    assert_eq!(
        output_error.to_string(),
        "输出错误: output failed"
    );
}

#[test]
fn test_error_from_io_error() {
    let io_error = io::Error::new(io::ErrorKind::NotFound, "file not found");
    let error: LuaShieldError = io_error.into();
    assert_eq!(
        error.to_string(),
        "IO 错误: file not found"
    );
}

#[test]
fn test_error_from_string() {
    let error: LuaShieldError = "test error".to_string().into();
    assert_eq!(
        error.to_string(),
        "配置错误: test error"
    );
}

#[test]
fn test_result_ok() -> Result<()> {
    // 测试成功结果
    let result: Result<()> = Ok(());
    assert!(result.is_ok());
    Ok(())
}

#[test]
fn test_result_err() {
    // 测试错误结果
    let result: Result<()> = Err(LuaShieldError::ConfigError("test error".to_string()));
    assert!(result.is_err());
}

#[test]
fn test_error_kind() {
    // 测试错误类型
    let io_error = LuaShieldError::IoError(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    assert!(matches!(io_error, LuaShieldError::IoError(_)));

    let llm_error = LuaShieldError::LlmError("API request failed".to_string());
    assert!(matches!(llm_error, LuaShieldError::LlmError(_)));

    let config_error = LuaShieldError::ConfigError("invalid configuration".to_string());
    assert!(matches!(config_error, LuaShieldError::ConfigError(_)));

    let analysis_error = LuaShieldError::AnalysisError("analysis failed".to_string());
    assert!(matches!(analysis_error, LuaShieldError::AnalysisError(_)));

    let output_error = LuaShieldError::OutputError("output failed".to_string());
    assert!(matches!(output_error, LuaShieldError::OutputError(_)));
} 