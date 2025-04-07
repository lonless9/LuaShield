//! 输出模块
//!
//! 负责结果输出和格式化。

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;

use serde::Serialize;

use crate::analyzer::AnalysisResult;
use crate::error::{LuaShieldError, Result};

/// 输出格式
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    /// 文本格式
    Text,
    /// JSON 格式
    Json,
    /// Markdown 格式
    Markdown,
}

impl std::str::FromStr for OutputFormat {
    type Err = LuaShieldError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "text" => Ok(Self::Text),
            "json" => Ok(Self::Json),
            "markdown" => Ok(Self::Markdown),
            _ => Err(LuaShieldError::OutputError(format!(
                "不支持的输出格式: {}",
                s
            ))),
        }
    }
}

impl std::fmt::Display for OutputFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Text => write!(f, "text"),
            Self::Json => write!(f, "json"),
            Self::Markdown => write!(f, "markdown"),
        }
    }
}

impl OutputFormat {
    /// 获取文件扩展名
    ///
    /// # 返回
    ///
    /// * `&str` - 文件扩展名
    pub fn extension(&self) -> &str {
        match self {
            Self::Text => "txt",
            Self::Json => "json",
            Self::Markdown => "md",
        }
    }
}

/// 输出器
#[derive(Debug, Clone)]
pub struct Outputter {
    /// 输出格式
    format: OutputFormat,
    /// 输出文件
    output_file: Option<PathBuf>,
}

impl Outputter {
    /// 创建输出器
    ///
    /// # 参数
    ///
    /// * `format` - 输出格式
    /// * `output_file` - 输出文件
    ///
    /// # 返回
    ///
    /// * `Self` - 输出器
    pub fn new(format: OutputFormat, output_file: Option<PathBuf>) -> Self {
        Self {
            format,
            output_file,
        }
    }

    /// 输出结果
    ///
    /// # 参数
    ///
    /// * `results` - 分析结果列表
    ///
    /// # 返回
    ///
    /// * `Result<()>` - 输出结果
    pub fn output(&self, results: &[AnalysisResult]) -> Result<()> {
        let content = match self.format {
            OutputFormat::Text => self.format_text(results),
            OutputFormat::Json => self.format_json(results)?,
            OutputFormat::Markdown => self.format_markdown(results),
        };

        if let Some(output_file) = &self.output_file {
            // 创建输出文件
            let mut file = File::create(output_file).map_err(|e| {
                LuaShieldError::OutputError(format!("创建输出文件失败: {} - {}", output_file.display(), e))
            })?;

            // 写入内容
            file.write_all(content.as_bytes()).map_err(|e| {
                LuaShieldError::OutputError(format!("写入输出文件失败: {} - {}", output_file.display(), e))
            })?;
        } else {
            // 输出到标准输出
            println!("{}", content);
        }

        Ok(())
    }

    /// 格式化文本
    ///
    /// # 参数
    ///
    /// * `results` - 分析结果列表
    ///
    /// # 返回
    ///
    /// * `String` - 格式化后的文本
    pub fn format_text(&self, results: &[AnalysisResult]) -> String {
        let mut output = String::new();

        for result in results {
            output.push_str(&format!("文件: {}\n", result.file_path.display()));
            output.push_str(&result.content);
            output.push_str("\n\n");
        }

        output
    }

    /// 格式化 JSON
    ///
    /// # 参数
    ///
    /// * `results` - 分析结果列表
    ///
    /// # 返回
    ///
    /// * `Result<String>` - 格式化后的 JSON
    pub fn format_json(&self, results: &[AnalysisResult]) -> Result<String> {
        #[derive(Serialize)]
        struct JsonResult {
            file_path: String,
            content: String,
        }

        let json_results: Vec<JsonResult> = results
            .iter()
            .map(|result| JsonResult {
                file_path: result.file_path.to_string_lossy().to_string(),
                content: result.content.clone(),
            })
            .collect();

        serde_json::to_string_pretty(&json_results).map_err(|e| {
            LuaShieldError::OutputError(format!("格式化 JSON 失败: {}", e))
        })
    }

    /// 格式化 Markdown
    ///
    /// # 参数
    ///
    /// * `results` - 分析结果列表
    ///
    /// # 返回
    ///
    /// * `String` - 格式化后的 Markdown
    pub fn format_markdown(&self, results: &[AnalysisResult]) -> String {
        let mut output = String::new();

        output.push_str("# Lua 代码安全分析报告\n\n");

        for result in results {
            output.push_str(&format!("## {}\n\n", result.file_path.display()));
            output.push_str(&result.content);
            output.push_str("\n\n");
        }

        output
    }
} 