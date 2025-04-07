//! 文件系统模块
//!
//! 负责文件查找、读取和过滤。

use std::path::{Path, PathBuf};
use std::sync::Arc;

use regex::Regex;
use tokio::fs;
use walkdir::WalkDir;

use crate::error::{LuaShieldError, Result};

/// 文件系统操作
#[derive(Debug, Clone)]
pub struct FileSystem {
    /// 仓库根路径
    root_path: PathBuf,
    /// 排除路径正则表达式
    exclude_paths: Arc<Vec<Regex>>,
    /// 排除文件名正则表达式
    exclude_files: Arc<Vec<Regex>>,
}

impl FileSystem {
    /// 创建文件系统操作
    ///
    /// # 参数
    ///
    /// * `root_path` - 仓库根路径
    ///
    /// # 返回
    ///
    /// * `Self` - 文件系统操作
    pub fn new(root_path: PathBuf) -> Self {
        // 默认排除路径
        let exclude_paths = vec![
            Regex::new(r"(?i)(test|spec|vendor|node_modules)/").unwrap(),
            Regex::new(r"(?i)\.git/").unwrap(),
        ];

        // 默认排除文件名
        let exclude_files = vec![
            Regex::new(r"(?i)_spec\.lua$").unwrap(),
            Regex::new(r"(?i)_test\.lua$").unwrap(),
        ];

        Self {
            root_path,
            exclude_paths: Arc::new(exclude_paths),
            exclude_files: Arc::new(exclude_files),
        }
    }

    /// 查找 Lua 文件
    ///
    /// # 返回
    ///
    /// * `Result<Vec<PathBuf>>` - Lua 文件路径列表
    pub fn find_lua_files(&self) -> Result<Vec<PathBuf>> {
        let mut lua_files = Vec::new();

        for entry in WalkDir::new(&self.root_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();

            // 跳过目录
            if path.is_dir() {
                continue;
            }

            // 检查文件扩展名
            if path.extension().map_or(false, |ext| ext == "lua") {
                // 检查是否应该排除
                if self.should_exclude(path) {
                    continue;
                }

                lua_files.push(path.to_path_buf());
            }
        }

        Ok(lua_files)
    }

    /// 读取文件内容
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    ///
    /// # 返回
    ///
    /// * `Result<String>` - 文件内容
    pub async fn read_file_content(&self, path: &Path) -> Result<String> {
        fs::read_to_string(path).await.map_err(|e| {
            LuaShieldError::FileSystemError(format!("读取文件失败: {} - {}", path.display(), e))
        })
    }

    /// 获取 README 内容
    ///
    /// # 返回
    ///
    /// * `Result<Option<String>>` - README 内容（如果存在）
    pub async fn get_readme_content(&self) -> Result<Option<String>> {
        // 可能的 README 文件名
        let readme_names = ["README.md", "README", "README.txt", "README.lua"];

        for name in readme_names.iter() {
            let path = self.root_path.join(name);

            if path.exists() {
                return Ok(Some(self.read_file_content(&path).await?));
            }
        }

        Ok(None)
    }

    /// 查找网络相关文件
    ///
    /// # 返回
    ///
    /// * `Result<Vec<PathBuf>>` - 网络相关文件路径列表
    pub fn find_network_related_files(&self) -> Result<Vec<PathBuf>> {
        let mut network_files = Vec::new();

        // 网络相关模式
        let network_patterns = [
            r"(?i)http",
            r"(?i)https",
            r"(?i)socket",
            r"(?i)ngx\.req",
            r"(?i)lapis",
            r"(?i)openresty",
            r"(?i)luasocket",
            r"(?i)lua-requests",
            r"(?i)lua-http",
            r"(?i)lua-resty-http",
        ];

        // 编译正则表达式
        let patterns: Vec<Regex> = network_patterns
            .iter()
            .map(|&pattern| Regex::new(pattern).unwrap())
            .collect();

        // 遍历所有 Lua 文件
        for path in self.find_lua_files()? {
            // 读取文件内容 - 使用同步方法避免嵌套运行时
            let content = std::fs::read_to_string(&path).map_err(|e| {
                LuaShieldError::FileSystemError(format!(
                    "读取文件失败: {} - {}",
                    path.display(),
                    e
                ))
            })?;

            // 检查是否包含网络相关模式
            for pattern in &patterns {
                if pattern.is_match(&content) {
                    network_files.push(path.clone());
                    break;
                }
            }
        }

        Ok(network_files)
    }

    /// 检查是否应该排除文件
    ///
    /// # 参数
    ///
    /// * `path` - 文件路径
    ///
    /// # 返回
    ///
    /// * `bool` - 是否应该排除
    pub fn should_exclude(&self, path: &Path) -> bool {
        // 获取相对路径
        let relative_path = path
            .strip_prefix(&self.root_path)
            .unwrap_or(path)
            .to_string_lossy();

        // 检查路径是否匹配排除路径
        for pattern in self.exclude_paths.iter() {
            if pattern.is_match(&relative_path) {
                return true;
            }
        }

        // 检查文件名是否匹配排除文件名
        if let Some(file_name) = path.file_name() {
            let file_name = file_name.to_string_lossy();
            for pattern in self.exclude_files.iter() {
                if pattern.is_match(&file_name) {
                    return true;
                }
            }
        }

        false
    }
} 