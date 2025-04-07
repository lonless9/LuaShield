//! LuaShield
//!
//! 一个用于分析 Lua 代码安全漏洞的工具。

pub mod analyzer;
pub mod cli;
pub mod config;
pub mod error;
pub mod fs;
pub mod llm;
pub mod logging;
pub mod output;

pub use analyzer::{Analyzer, AnalysisResult};
pub use config::{Config, LlmProvider};
pub use error::{LuaShieldError, Result};
pub use fs::FileSystem;
pub use llm::LlmClient;
pub use output::{OutputFormat, Outputter}; 