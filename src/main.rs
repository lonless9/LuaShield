//! LuaShield
//!
//! 一个用于分析 Lua 代码安全漏洞的工具。

mod analyzer;
mod cli;
mod config;
mod error;
mod fs;
mod llm;
mod logging;
mod output;

use std::path::PathBuf;
use std::sync::Arc;

use clap::Parser;

use crate::analyzer::Analyzer;
use crate::cli::{Cli, Commands};
use crate::config::Config;
use crate::error::{LuaShieldError, Result};
use crate::logging::setup_logging;
use crate::output::{OutputFormat, Outputter};

/// 程序入口
#[tokio::main]
async fn main() -> Result<()> {
    // 解析命令行参数
    let cli = Cli::parse();

    // 处理子命令
    match cli.command {
        Commands::Analyze(args) => {
            // 设置日志
            setup_logging(&args.log_level, args.log_file)?;

            // 加载配置
            let mut config = Config::default();
            config.root_path = args.target;
            config.llm_provider = args.llm_provider.parse()?;
            if let Some(api_key) = args.api_key {
                config.api_key = api_key;
            }
            if let Some(base_url) = args.base_url {
                config.base_url = base_url;
            }
            if let Some(model_name) = args.model_name {
                config.model_name = model_name;
            }
            config.analyze_readme = args.analyze_readme;
            config.log_level = args.log_level;
            config.output_format = args.output_format.parse()?;

            // 验证配置
            config.validate()?;

            // 创建分析器
            let analyzer = Analyzer::new(Arc::new(config.clone()))?;

            // 分析代码
            let results = analyzer.analyze().await?;

            // 创建输出器
            let outputter = Outputter::new(config.output_format.parse()?, args.output_file);

            // 输出结果
            outputter.output(&results)?;
        }
        Commands::Config(args) => {
            // 设置日志
            setup_logging("info", None)?;

            // 加载配置
            let mut config = Config::load()?;

            // 处理配置命令
            if args.list {
                // 列出所有配置
                println!("当前配置：");
                println!("  LLM 提供商: {:?}", config.llm_provider);
                println!("  API 密钥: {}", if config.api_key.is_empty() { "未设置" } else { "已设置" });
                println!("  基础 URL: {}", config.base_url);
                println!("  模型名称: {}", config.model_name);
                println!("  根路径: {}", config.root_path.display());
                println!("  分析 README: {}", config.analyze_readme);
                println!("  日志级别: {}", config.log_level);
                println!("  输出格式: {}", config.output_format);
            } else if let (Some(key), Some(value)) = (args.key, args.value) {
                // 设置配置
                match key.as_str() {
                    "llm_provider" => {
                        config.llm_provider = value.parse()?;
                    }
                    "api_key" => {
                        config.api_key = value;
                    }
                    "base_url" => {
                        config.base_url = value;
                    }
                    "model_name" => {
                        config.model_name = value;
                    }
                    "root_path" => {
                        config.root_path = PathBuf::from(value);
                    }
                    "analyze_readme" => {
                        config.analyze_readme = value.parse().map_err(|_| {
                            LuaShieldError::ConfigError("analyze_readme 必须是布尔值".to_string())
                        })?;
                    }
                    "log_level" => {
                        config.log_level = value;
                    }
                    "output_format" => {
                        config.output_format = value.parse()?;
                    }
                    _ => {
                        return Err(LuaShieldError::ConfigError(format!(
                            "未知的配置项: {}",
                            key
                        )));
                    }
                }

                // 验证配置
                config.validate()?;

                // 保存配置
                config.save()?;

                println!("配置已更新");
            } else {
                return Err(LuaShieldError::ConfigError(
                    "必须指定配置项和值，或使用 --list 列出所有配置".to_string(),
                ));
            }
        }
        Commands::Version => {
            // 显示版本信息
            println!("LuaShield {}", env!("CARGO_PKG_VERSION"));
        }
    }

    Ok(())
}
