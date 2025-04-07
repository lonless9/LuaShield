//! 错误处理模块
//!
//! 负责错误类型定义和处理。

use std::fmt;
use std::io;
use serde_json;

/// 错误类型
#[derive(Debug)]
pub enum LuaShieldError {
    /// 配置错误
    ConfigError(String),
    /// 文件系统错误
    FileSystemError(String),
    /// IO 错误
    IoError(io::Error),
    /// LLM 错误
    LlmError(String),
    /// 分析错误
    AnalysisError(String),
    /// 输出错误
    OutputError(String),
}

impl fmt::Display for LuaShieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigError(msg) => write!(f, "配置错误: {}", msg),
            Self::FileSystemError(msg) => write!(f, "文件系统错误: {}", msg),
            Self::IoError(err) => write!(f, "IO 错误: {}", err),
            Self::LlmError(msg) => write!(f, "LLM 错误: {}", msg),
            Self::AnalysisError(msg) => write!(f, "分析错误: {}", msg),
            Self::OutputError(msg) => write!(f, "输出错误: {}", msg),
        }
    }
}

impl std::error::Error for LuaShieldError {}

impl From<io::Error> for LuaShieldError {
    fn from(err: io::Error) -> Self {
        Self::IoError(err)
    }
}

impl From<String> for LuaShieldError {
    fn from(err: String) -> Self {
        Self::ConfigError(err)
    }
}

// 添加对&str的支持
impl From<&str> for LuaShieldError {
    fn from(err: &str) -> Self {
        Self::ConfigError(err.to_string())
    }
}

// 添加对Infallible的支持
impl From<std::convert::Infallible> for LuaShieldError {
    fn from(_: std::convert::Infallible) -> Self {
        unreachable!("不可能发生的错误")
    }
}

// 实现 From<serde_json::Error> for LuaShieldError
impl From<serde_json::Error> for LuaShieldError {
    fn from(error: serde_json::Error) -> Self {
        LuaShieldError::OutputError(format!("JSON 错误: {}", error))
    }
}

/// 结果类型
pub type Result<T> = std::result::Result<T, LuaShieldError>; 