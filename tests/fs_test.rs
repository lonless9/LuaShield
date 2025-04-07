use std::path::PathBuf;
use tempfile::TempDir;

use luashield::{
    error::Result,
    fs::FileSystem,
};

#[test]
fn test_find_lua_files() -> Result<()> {
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path().to_path_buf();

    // 创建测试文件
    std::fs::write(root_path.join("test1.lua"), "test1")?;
    std::fs::write(root_path.join("test2.lua"), "test2")?;
    std::fs::write(root_path.join("test.txt"), "test")?;
    std::fs::create_dir(root_path.join("test"))?;
    std::fs::write(root_path.join("test/test3.lua"), "test3")?;
    std::fs::create_dir(root_path.join("test_spec"))?;
    std::fs::write(root_path.join("test_spec/test4.lua"), "test4")?;

    // 创建文件系统
    let fs = FileSystem::new(root_path.clone());

    // 查找 Lua 文件
    let lua_files = fs.find_lua_files()?;

    // 打印找到的文件以进行调试
    println!("Found Lua files:");
    for file in &lua_files {
        println!("  {}", file.display());
    }

    // 验证结果
    assert_eq!(lua_files.len(), 2);
    assert!(lua_files.contains(&root_path.join("test1.lua")));
    assert!(lua_files.contains(&root_path.join("test2.lua")));
    assert!(!lua_files.contains(&root_path.join("test/test3.lua")));
    assert!(!lua_files.contains(&root_path.join("test.txt")));
    assert!(!lua_files.contains(&root_path.join("test_spec/test4.lua")));

    Ok(())
}

#[test]
fn test_read_file_content() -> Result<()> {
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path().to_path_buf();

    // 创建测试文件
    let test_file = root_path.join("test.lua");
    std::fs::write(&test_file, "test content")?;

    // 创建文件系统
    let fs = FileSystem::new(root_path.clone());

    // 读取文件内容
    let content = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(fs.read_file_content(&test_file))?;

    // 验证结果
    assert_eq!(content, "test content");

    Ok(())
}

#[test]
fn test_get_readme_content() -> Result<()> {
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path().to_path_buf();

    // 创建测试文件
    std::fs::write(root_path.join("README.md"), "test readme")?;

    // 创建文件系统
    let fs = FileSystem::new(root_path.clone());

    // 读取 README 内容
    let content = tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(fs.get_readme_content())?;

    // 验证结果
    assert_eq!(content, Some("test readme".to_string()));

    Ok(())
}

#[test]
fn test_find_network_related_files() -> Result<()> {
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path().to_path_buf();

    // 创建测试文件
    std::fs::write(
        root_path.join("network1.lua"),
        "local http = require('socket.http')",
    )?;
    std::fs::write(
        root_path.join("network2.lua"),
        "local https = require('ssl.https')",
    )?;
    std::fs::write(root_path.join("normal.lua"), "local x = 1")?;

    // 创建文件系统
    let fs = FileSystem::new(root_path.clone());

    // 查找网络相关文件
    let network_files = fs.find_network_related_files()?;

    // 验证结果
    assert_eq!(network_files.len(), 2);
    assert!(network_files.contains(&root_path.join("network1.lua")));
    assert!(network_files.contains(&root_path.join("network2.lua")));
    assert!(!network_files.contains(&root_path.join("normal.lua")));

    Ok(())
}

#[test]
fn test_should_exclude() -> Result<()> {
    // 创建临时目录
    let temp_dir = TempDir::new()?;
    let root_path = temp_dir.path().to_path_buf();

    // 创建测试文件
    std::fs::create_dir(root_path.join("test"))?;
    std::fs::create_dir(root_path.join("spec"))?;
    std::fs::create_dir(root_path.join("vendor"))?;
    std::fs::create_dir(root_path.join("node_modules"))?;
    std::fs::create_dir(root_path.join(".git"))?;

    // 创建文件系统
    let fs = FileSystem::new(root_path.clone());

    // 验证结果
    assert!(fs.should_exclude(&root_path.join("test/test.lua")));
    assert!(fs.should_exclude(&root_path.join("spec/test.lua")));
    assert!(fs.should_exclude(&root_path.join("vendor/test.lua")));
    assert!(fs.should_exclude(&root_path.join("node_modules/test.lua")));
    assert!(fs.should_exclude(&root_path.join(".git/test.lua")));
    assert!(fs.should_exclude(&root_path.join("test_spec.lua")));
    assert!(fs.should_exclude(&root_path.join("test_test.lua")));
    assert!(!fs.should_exclude(&root_path.join("test.lua")));

    Ok(())
} 