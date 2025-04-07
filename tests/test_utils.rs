use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::PathBuf;
use tempfile::TempDir;

use luashield::{
    config::{Config, LlmProvider},
    error::Result,
};

/// 创建测试目录结构
pub fn create_test_directory() -> Result<(TempDir, PathBuf)> {
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let test_dir = temp_dir.path().to_path_buf();

    // 创建测试文件
    let test_file = test_dir.join("test.lua");
    let mut file = File::create(&test_file)?;
    file.write_all(b"local function test()\n  print('test')\nend")?;

    // 创建 README 文件
    let readme_file = test_dir.join("README.md");
    let mut file = File::create(&readme_file)?;
    file.write_all(b"# Test Project\n\nThis is a test project.")?;

    // 创建网络相关文件
    let network_file = test_dir.join("network.lua");
    let mut file = File::create(&network_file)?;
    file.write_all(b"local http = require('socket.http')\nlocal function test()\n  http.request('http://example.com')\nend")?;

    // 创建子目录
    let sub_dir = test_dir.join("subdir");
    fs::create_dir(&sub_dir)?;

    // 创建子目录中的文件
    let sub_file = sub_dir.join("sub.lua");
    let mut file = File::create(&sub_file)?;
    file.write_all(b"local function sub()\n  print('sub')\nend")?;

    Ok((temp_dir, test_dir))
}

/// 设置测试环境变量
pub fn set_test_env_vars(test_dir: &PathBuf) {
    env::set_var("LUASHIELD_LLM_PROVIDER", "claude");
    env::set_var("LUASHIELD_API_KEY", "test-api-key");
    env::set_var("LUASHIELD_MODEL_NAME", "test-model");
    env::set_var("LUASHIELD_ROOT_PATH", test_dir.to_str().unwrap());
    env::set_var("LUASHIELD_ANALYZE_README", "true");
    env::set_var("LUASHIELD_LOG_LEVEL", "info");
    env::set_var("LUASHIELD_OUTPUT_FORMAT", "text");
}

/// 清理测试环境变量
pub fn cleanup_test_env_vars() {
    env::remove_var("LUASHIELD_LLM_PROVIDER");
    env::remove_var("LUASHIELD_API_KEY");
    env::remove_var("LUASHIELD_MODEL_NAME");
    env::remove_var("LUASHIELD_ROOT_PATH");
    env::remove_var("LUASHIELD_ANALYZE_README");
    env::remove_var("LUASHIELD_LOG_LEVEL");
    env::remove_var("LUASHIELD_OUTPUT_FORMAT");
}

/// 创建测试配置
pub fn create_test_config(test_dir: &PathBuf) -> Result<Config> {
    let mut config = Config::default();
    config.llm_provider = LlmProvider::Claude;
    config.api_key = "test-api-key".to_string();
    config.model_name = "test-model".to_string();
    config.root_path = test_dir.clone();
    config.analyze_readme = true;
    config.log_level = "info".to_string();
    config.output_format = "text".to_string();
    Ok(config)
}

/// 创建测试文件
pub fn create_test_file(path: &PathBuf, content: &[u8]) -> Result<()> {
    let mut file = File::create(path)?;
    file.write_all(content)?;
    Ok(())
}

/// 读取测试文件内容
pub fn read_test_file(path: &PathBuf) -> Result<String> {
    let mut file = File::open(path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// 创建测试目录
pub fn create_test_directory_with_files(
    base_dir: &PathBuf,
    files: &[(&str, &[u8])],
) -> Result<()> {
    for (name, content) in files {
        let path = base_dir.join(name);
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        create_test_file(&path, content)?;
    }
    Ok(())
}

/// 验证文件内容
pub fn assert_file_content(path: &PathBuf, expected_content: &str) -> Result<()> {
    let content = read_test_file(path)?;
    assert_eq!(content, expected_content);
    Ok(())
}

/// 验证文件存在
pub fn assert_file_exists(path: &PathBuf) {
    assert!(path.exists(), "File does not exist: {:?}", path);
}

/// 验证文件不存在
pub fn assert_file_not_exists(path: &PathBuf) {
    assert!(!path.exists(), "File exists but should not: {:?}", path);
}

/// 验证目录存在
pub fn assert_dir_exists(path: &PathBuf) {
    assert!(path.exists() && path.is_dir(), "Directory does not exist: {:?}", path);
}

/// 验证目录不存在
pub fn assert_dir_not_exists(path: &PathBuf) {
    assert!(!path.exists() || !path.is_dir(), "Directory exists but should not: {:?}", path);
} 