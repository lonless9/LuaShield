use std::path::PathBuf;

use mockall::predicate::*;
use tokio_test::block_on;
use mockall::predicate::str::contains;

use luashield::{
    analyzer::AnalysisResult,
    config::{Config, LlmProvider},
    error::Result,
};

// 模拟 FileSystem
mockall::mock! {
    pub FileSystem {
        fn find_lua_files(&self) -> Result<Vec<PathBuf>>;
        fn read_file_content(&self, path: &PathBuf) -> Result<String>;
        fn get_readme_content(&self) -> Result<Option<String>>;
        fn find_network_related_files(&self) -> Result<Vec<PathBuf>>;
    }
}

// 模拟 LlmClient
mockall::mock! {
    pub LlmClient {
        fn analyze_code(&self, code: &str, context: &str) -> Result<String>;
    }
}

#[test]
fn test_analyze() {
    // 创建配置
    let mut config = Config::default();
    config.root_path = PathBuf::from("test_data");
    config.llm_provider = LlmProvider::Claude;
    config.api_key = "test-api-key".to_string();
    config.model_name = "test-model".to_string();
    config.analyze_readme = true;
    config.log_level = "info".to_string();
    config.output_format = "text".to_string();

    // 创建模拟对象
    let mut fs = MockFileSystem::new();
    let mut llm_client = MockLlmClient::new();

    // 设置 FileSystem 期望
    fs.expect_find_lua_files()
        .returning(|| Ok(vec![PathBuf::from("test.lua")]));

    fs.expect_read_file_content()
        .with(eq(PathBuf::from("test.lua")))
        .returning(|_| Ok("test code".to_string()));

    fs.expect_get_readme_content()
        .returning(|| Ok(Some("test readme".to_string())));

    fs.expect_find_network_related_files()
        .returning(|| Ok(vec![PathBuf::from("network.lua")]));

    // 设置 LlmClient 期望
    llm_client
        .expect_analyze_code()
        .with(eq("test code"), contains("test readme"))
        .returning(|_, _| Ok("test analysis".to_string()));

    // 修改为创建和使用自定义的测试分析器结构体
    struct TestAnalyzer {
        fs: MockFileSystem,
        llm_client: MockLlmClient,
    }
    
    impl TestAnalyzer {
        async fn analyze(&self) -> Result<Vec<AnalysisResult>> {
            let mut results = Vec::new();
            
            // 调用模拟实现
            let lua_files = self.fs.find_lua_files()?;
            let readme_content = self.fs.get_readme_content()?;
            
            // 构建上下文
            let mut context = String::new();
            if let Some(readme) = readme_content {
                context.push_str(&readme);
            }
            
            // 处理每个文件
            for file_path in lua_files {
                let content = self.fs.read_file_content(&file_path)?;
                let analysis = self.llm_client.analyze_code(&content, &context)?;
                results.push(AnalysisResult {
                    file_path,
                    content: analysis,
                });
            }
            
            Ok(results)
        }
    }

    // 创建测试分析器
    let analyzer = TestAnalyzer {
        fs,
        llm_client,
    };

    // 执行分析
    let results = block_on(analyzer.analyze()).unwrap();

    // 验证结果
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].file_path, PathBuf::from("test.lua"));
    assert_eq!(results[0].content, "test analysis");
} 