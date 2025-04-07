use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use tempfile::TempDir;

use luashield::{
    analyzer::AnalysisResult,
    error::Result,
    output::{OutputFormat, Outputter},
};

use serde_json;

#[test]
fn test_format_text() -> Result<()> {
    // 创建输出器
    let outputter = Outputter::new(OutputFormat::Text, None);

    // 创建分析结果
    let results = vec![AnalysisResult {
        file_path: PathBuf::from("test.lua"),
        content: "test analysis".to_string(),
    }];

    // 格式化文本
    let text = outputter.format_text(&results);

    // 验证结果
    assert_eq!(
        text,
        "文件: test.lua\ntest analysis\n\n"
    );

    Ok(())
}

#[test]
fn test_format_json() -> Result<()> {
    // 创建输出器
    let outputter = Outputter::new(OutputFormat::Json, None);

    // 创建分析结果
    let results = vec![AnalysisResult {
        file_path: PathBuf::from("test.lua"),
        content: "test analysis".to_string(),
    }];

    // 格式化 JSON
    let json = outputter.format_json(&results)?;

    // 使用 serde_json 将结果解析为值，以避免空格和格式化差异
    let actual_json: serde_json::Value = serde_json::from_str(&json)?;
    let expected_json: serde_json::Value = serde_json::from_str(
        r#"[{"file_path":"test.lua","content":"test analysis"}]"#
    )?;

    // 验证结果
    assert_eq!(actual_json, expected_json);

    Ok(())
}

#[test]
fn test_format_markdown() -> Result<()> {
    // 创建输出器
    let outputter = Outputter::new(OutputFormat::Markdown, None);

    // 创建分析结果
    let results = vec![AnalysisResult {
        file_path: PathBuf::from("test.lua"),
        content: "test analysis".to_string(),
    }];

    // 格式化 Markdown
    let markdown = outputter.format_markdown(&results);

    // 验证结果
    assert_eq!(
        markdown,
        "# Lua 代码安全分析报告\n\n## test.lua\n\ntest analysis\n\n"
    );

    Ok(())
}

#[test]
fn test_output_to_file() -> Result<()> {
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let output_file = temp_dir.path().join("output.txt");

    // 创建输出器
    let outputter = Outputter::new(OutputFormat::Text, Some(output_file.clone()));

    // 创建分析结果
    let results = vec![AnalysisResult {
        file_path: PathBuf::from("test.lua"),
        content: "test analysis".to_string(),
    }];

    // 输出到文件
    outputter.output(&results)?;

    // 读取文件内容
    let mut file = File::open(output_file)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;

    // 验证结果
    assert_eq!(
        content,
        "文件: test.lua\ntest analysis\n\n"
    );

    Ok(())
}

#[test]
fn test_output_to_stdout() -> Result<()> {
    // 创建输出器
    let outputter = Outputter::new(OutputFormat::Text, None);

    // 创建分析结果
    let results = vec![AnalysisResult {
        file_path: PathBuf::from("test.lua"),
        content: "test analysis".to_string(),
    }];

    // 输出到标准输出
    outputter.output(&results)?;

    Ok(())
}

#[test]
fn test_output_format_from_str() -> Result<()> {
    // 测试有效的输出格式
    assert_eq!("text".parse::<OutputFormat>()?, OutputFormat::Text);
    assert_eq!("json".parse::<OutputFormat>()?, OutputFormat::Json);
    assert_eq!("markdown".parse::<OutputFormat>()?, OutputFormat::Markdown);

    // 测试无效的输出格式
    assert!("invalid".parse::<OutputFormat>().is_err());

    Ok(())
} 