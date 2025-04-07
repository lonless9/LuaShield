//! 日志模块
//!
//! 负责日志记录和格式化。

use std::path::PathBuf;
use std::sync::Once;
use tracing::Level;
use tracing_subscriber::{
    fmt::{self, format::FmtSpan},
    EnvFilter,
};

use crate::error::{LuaShieldError, Result};

// 确保只初始化一次
static INIT: Once = Once::new();
static mut INITIALIZED: bool = false;

/// 设置日志系统
///
/// # 参数
///
/// * `log_level` - 日志级别
/// * `log_file` - 日志文件路径（可选）
///
/// # 返回
///
/// * `Result<()>` - 设置结果
pub fn setup_logging(log_level: &str, log_file: Option<PathBuf>) -> Result<()> {
    // 如果已经初始化，则直接返回
    unsafe {
        if INITIALIZED {
            // 已经初始化过日志系统，记录一条提示信息
            tracing::info!("日志系统已经初始化，忽略重复的初始化请求");
            return Ok(());
        }
    }

    // 解析日志级别
    let level = match log_level.to_lowercase().as_str() {
        "error" => Level::ERROR,
        "warn" => Level::WARN,
        "info" => Level::INFO,
        "debug" => Level::DEBUG,
        "trace" => Level::TRACE,
        _ => return Err(LuaShieldError::ConfigError(format!("无效的日志级别: {}", log_level))),
    };

    // 创建环境过滤器
    let filter = EnvFilter::from_default_env()
        .add_directive(level.into())
        .add_directive("luashield=debug".parse().unwrap());

    // 使用 Once 确保只初始化一次
    INIT.call_once(|| {
        // 创建日志构建器
        let builder = fmt::Subscriber::builder()
            .with_env_filter(filter)
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_file(true)
            .with_line_number(true)
            .with_span_events(FmtSpan::CLOSE);

        // 如果指定了日志文件，则添加文件输出
        if let Some(log_file) = log_file {
            // 确保日志目录存在
            if let Some(parent) = log_file.parent() {
                if let Err(e) = std::fs::create_dir_all(parent) {
                    eprintln!("创建日志目录失败: {}", e);
                    return;
                }
            }

            // 创建文件输出
            match std::fs::File::create(log_file) {
                Ok(file) => {
                    // 添加文件输出
                    if let Err(e) = builder
                        .with_writer(file)
                        .with_ansi(false)
                        .json()
                        .try_init() {
                        eprintln!("初始化日志系统失败: {}", e);
                        return;
                    }
                }
                Err(e) => {
                    eprintln!("创建日志文件失败: {}", e);
                    return;
                }
            }
        } else {
            // 使用标准输出
            if let Err(e) = builder.with_ansi(true).try_init() {
                eprintln!("初始化日志系统失败: {}", e);
                return;
            }
        }

        // 标记为已初始化
        unsafe {
            INITIALIZED = true;
        }

        // 记录启动日志
        tracing::info!("日志系统初始化完成，日志级别: {}", level);
    });

    Ok(())
} 