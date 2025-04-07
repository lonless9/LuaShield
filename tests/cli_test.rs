use std::path::PathBuf;

use clap::Parser;
use luashield::cli::{Cli, Commands};

#[test]
fn test_cli_analyze_command() {
    // 测试分析命令
    let args = vec![
        "luashield",
        "analyze",
        "--target",
        "test.lua",
        "--llm-provider",
        "claude",
        "--api-key",
        "test-api-key",
        "--model-name",
        "test-model",
        "--analyze-readme",
        "--log-level",
        "info",
        "--output-format",
        "text",
    ];

    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Analyze(args) => {
            assert_eq!(args.target, PathBuf::from("test.lua"));
            assert_eq!(args.llm_provider, "claude");
            assert_eq!(args.api_key, Some("test-api-key".to_string()));
            assert_eq!(args.model_name, Some("test-model".to_string()));
            assert!(args.analyze_readme);
            assert_eq!(args.log_level, "info");
            assert_eq!(args.output_format, "text");
            assert!(args.output_file.is_none());
        }
        _ => panic!("Expected Analyze command"),
    }
}

#[test]
fn test_cli_config_command() {
    // 测试配置命令
    let args = vec![
        "luashield",
        "config",
        "--key",
        "test-key",
        "--value",
        "test-value",
    ];

    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Config(args) => {
            assert_eq!(args.key, Some("test-key".to_string()));
            assert_eq!(args.value, Some("test-value".to_string()));
            assert!(!args.list);
        }
        _ => panic!("Expected Config command"),
    }
}

#[test]
fn test_cli_config_list_command() {
    // 测试配置列表命令
    let args = vec!["luashield", "config", "--list"];

    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Config(args) => {
            assert!(args.key.is_none());
            assert!(args.value.is_none());
            assert!(args.list);
        }
        _ => panic!("Expected Config command"),
    }
}

#[test]
fn test_cli_version_command() {
    // 测试版本命令
    let args = vec!["luashield", "version"];

    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Version => {}
        _ => panic!("Expected Version command"),
    }
}

#[test]
fn test_cli_analyze_command_with_output_file() {
    // 测试分析命令（带输出文件）
    let args = vec![
        "luashield",
        "analyze",
        "--target",
        "test.lua",
        "--output-file",
        "output.txt",
    ];

    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Analyze(args) => {
            assert_eq!(args.target, PathBuf::from("test.lua"));
            assert_eq!(args.output_file, Some(PathBuf::from("output.txt")));
        }
        _ => panic!("Expected Analyze command"),
    }
}

#[test]
fn test_cli_analyze_command_with_base_url() {
    // 测试分析命令（带基础 URL）
    let args = vec![
        "luashield",
        "analyze",
        "--target",
        "test.lua",
        "--base-url",
        "https://api.example.com",
    ];

    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Analyze(args) => {
            assert_eq!(args.target, PathBuf::from("test.lua"));
            assert_eq!(args.base_url, Some("https://api.example.com".to_string()));
        }
        _ => panic!("Expected Analyze command"),
    }
}

#[test]
fn test_cli_analyze_command_with_log_file() {
    // 测试分析命令（带日志文件）
    let args = vec![
        "luashield",
        "analyze",
        "--target",
        "test.lua",
        "--log-file",
        "test.log",
    ];

    let cli = Cli::parse_from(args);
    match cli.command {
        Commands::Analyze(args) => {
            assert_eq!(args.target, PathBuf::from("test.lua"));
            assert_eq!(args.log_file, Some(PathBuf::from("test.log")));
        }
        _ => panic!("Expected Analyze command"),
    }
} 