//! 分析器模块
//!
//! 负责代码分析和漏洞检测。

use std::path::PathBuf;
use std::sync::Arc;

use crate::config::Config;
use crate::error::Result;
use crate::fs::FileSystem;
use crate::llm::LlmClient;

/// 分析器
#[derive(Debug, Clone)]
pub struct Analyzer {
    /// 配置
    config: Arc<Config>,
    /// 文件系统
    fs: FileSystem,
    /// LLM 客户端
    llm_client: LlmClient,
}

/// 分析结果
#[derive(Debug)]
pub struct AnalysisResult {
    /// 文件路径
    pub file_path: PathBuf,
    /// 分析内容
    pub content: String,
}

impl Analyzer {
    /// 创建分析器
    ///
    /// # 参数
    ///
    /// * `config` - 配置
    ///
    /// # 返回
    ///
    /// * `Result<Self>` - 分析器
    pub fn new(config: Arc<Config>) -> Result<Self> {
        let fs = FileSystem::new(config.root_path.clone());
        let llm_client = LlmClient::new(config.clone())?;

        Ok(Self {
            config,
            fs,
            llm_client,
        })
    }

    /// 分析代码
    ///
    /// # 返回
    ///
    /// * `Result<Vec<AnalysisResult>>` - 分析结果列表
    pub async fn analyze(&self) -> Result<Vec<AnalysisResult>> {
        let mut results = Vec::new();

        // 获取 README 内容
        let readme_content = if self.config.analyze_readme {
            self.fs.get_readme_content().await?
        } else {
            None
        };

        // 构建上下文信息
        let mut context = String::new();
        if let Some(readme) = readme_content {
            context.push_str("项目说明：\n");
            context.push_str(&readme);
            context.push_str("\n\n");
        }

        // 查找网络相关文件
        let network_files = self.fs.find_network_related_files()?;
        if !network_files.is_empty() {
            context.push_str("网络相关文件：\n");
            for file in network_files {
                context.push_str(&format!("- {}\n", file.display()));
            }
            context.push_str("\n");
        }

        // 查找所有 Lua 文件
        let lua_files = self.fs.find_lua_files()?;

        // 分析每个文件
        for file_path in lua_files {
            // 读取文件内容
            let content = self.fs.read_file_content(&file_path).await?;

            // 分析代码
            let analysis = self.llm_client.analyze_code(&content, &context).await?;

            // 添加分析结果
            results.push(AnalysisResult {
                file_path,
                content: analysis,
            });
        }

        Ok(results)
    }
} 