//! CLI 模块
//!
//! 负责命令行参数解析和处理。

use std::path::PathBuf;

use clap::{Parser, Subcommand};

use crate::config::LlmProvider;

/// CLI 参数
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// 子命令
    #[command(subcommand)]
    pub command: Commands,
}

/// 子命令
#[derive(Subcommand, Debug)]
pub enum Commands {
    /// 分析 Lua 代码
    Analyze(AnalyzeArgs),
    /// 配置
    Config(ConfigArgs),
    /// 版本信息
    Version,
}

/// 分析参数
#[derive(Parser, Debug)]
pub struct AnalyzeArgs {
    /// 目标路径（文件或目录）
    #[arg(short, long, value_name = "PATH")]
    pub target: PathBuf,

    /// LLM 提供商
    #[arg(short = 'p', long, value_name = "PROVIDER", default_value = "claude")]
    pub llm_provider: String,

    /// API 密钥
    #[arg(short, long, value_name = "KEY")]
    pub api_key: Option<String>,

    /// 基础 URL
    #[arg(short, long, value_name = "URL")]
    pub base_url: Option<String>,

    /// 模型名称
    #[arg(short, long, value_name = "MODEL")]
    pub model_name: Option<String>,

    /// 是否分析 README
    #[arg(short = 'r', long, default_value = "true")]
    pub analyze_readme: bool,

    /// 日志级别
    #[arg(short, long, value_name = "LEVEL", default_value = "info")]
    pub log_level: String,

    /// 日志文件
    #[arg(short = 'f', long, value_name = "FILE")]
    pub log_file: Option<PathBuf>,

    /// 输出格式
    #[arg(short, long, value_name = "FORMAT", default_value = "text")]
    pub output_format: String,

    /// 输出文件
    #[arg(short = 'w', long, value_name = "FILE")]
    pub output_file: Option<PathBuf>,
}

/// 配置参数
#[derive(Parser, Debug)]
pub struct ConfigArgs {
    /// 配置项
    #[arg(short, long, value_name = "KEY")]
    pub key: Option<String>,

    /// 配置值
    #[arg(short, long, value_name = "VALUE")]
    pub value: Option<String>,

    /// 列出所有配置
    #[arg(short, long)]
    pub list: bool,
}

impl Cli {
    /// 解析命令行参数
    pub fn parse() -> Self {
        <Self as clap::Parser>::parse()
    }

    /// 获取 LLM 提供商
    pub fn get_llm_provider(&self) -> Option<LlmProvider> {
        match &self.command {
            Commands::Analyze(args) => args.llm_provider.parse().ok(),
            _ => None,
        }
    }

    /// 获取 API 密钥
    pub fn get_api_key(&self) -> Option<String> {
        match &self.command {
            Commands::Analyze(args) => args.api_key.clone(),
            _ => None,
        }
    }

    /// 获取基础 URL
    pub fn get_base_url(&self) -> Option<String> {
        match &self.command {
            Commands::Analyze(args) => args.base_url.clone(),
            _ => None,
        }
    }

    /// 获取模型名称
    pub fn get_model_name(&self) -> Option<String> {
        match &self.command {
            Commands::Analyze(args) => args.model_name.clone(),
            _ => None,
        }
    }

    /// 获取目标路径
    pub fn get_target(&self) -> Option<PathBuf> {
        match &self.command {
            Commands::Analyze(args) => Some(args.target.clone()),
            _ => None,
        }
    }

    /// 获取是否分析 README
    pub fn get_analyze_readme(&self) -> bool {
        match &self.command {
            Commands::Analyze(args) => args.analyze_readme,
            _ => true,
        }
    }

    /// 获取日志级别
    pub fn get_log_level(&self) -> String {
        match &self.command {
            Commands::Analyze(args) => args.log_level.clone(),
            _ => "info".to_string(),
        }
    }

    /// 获取日志文件
    pub fn get_log_file(&self) -> Option<PathBuf> {
        match &self.command {
            Commands::Analyze(args) => args.log_file.clone(),
            _ => None,
        }
    }

    /// 获取输出格式
    pub fn get_output_format(&self) -> String {
        match &self.command {
            Commands::Analyze(args) => args.output_format.clone(),
            _ => "text".to_string(),
        }
    }

    /// 获取输出文件
    pub fn get_output_file(&self) -> Option<PathBuf> {
        match &self.command {
            Commands::Analyze(args) => args.output_file.clone(),
            _ => None,
        }
    }
} 