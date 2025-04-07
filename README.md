# LuaShield

一个用于分析 Lua 代码安全漏洞的工具。

## 功能特点

- 支持多种 LLM 提供商（Claude、OpenAI、Ollama）
- 自动分析 Lua 代码中的安全漏洞
- 提供详细的漏洞分析和修复建议
- 支持多种输出格式（文本、JSON、Markdown）
- 可配置的日志记录
- 支持分析 README 文件以获取上下文信息
- 自动识别网络相关文件

## 安装

### 从源码安装

1. 克隆仓库：

```bash
git clone https://github.com/yourusername/luashield.git
cd luashield
```

2. 编译安装：

```bash
cargo install --path .
```

### 从 crates.io 安装

```bash
cargo install luashield
```

## 使用方法

### 基本用法

```bash
# 分析单个文件
luashield analyze path/to/file.lua

# 分析整个目录
luashield analyze path/to/directory

# 使用特定 LLM 提供商
luashield analyze --llm-provider claude path/to/file.lua

# 指定输出格式
luashield analyze --output-format json path/to/file.lua

# 输出到文件
luashield analyze --output-file report.md path/to/file.lua
```

### 配置管理

```bash
# 列出当前配置
luashield config --list

# 设置配置项
luashield config --key llm_provider --value claude
luashield config --key api_key --value your-api-key
luashield config --key model_name --value claude-3-opus-20240229
```

### 查看版本

```bash
luashield version
```

## 配置项

| 配置项 | 说明 | 默认值 |
|--------|------|--------|
| llm_provider | LLM 提供商 | claude |
| api_key | API 密钥 | - |
| base_url | 基础 URL | - |
| model_name | 模型名称 | claude-3-opus-20240229 |
| root_path | 根路径 | . |
| analyze_readme | 是否分析 README | true |
| log_level | 日志级别 | info |
| output_format | 输出格式 | text |

## 输出格式

### 文本格式

```
文件: path/to/file.lua

1. 漏洞概述
...

2. 漏洞详情
...

3. 修复建议
...

4. 最佳实践
...
```

### JSON 格式

```json
[
  {
    "file_path": "path/to/file.lua",
    "content": "..."
  }
]
```

### Markdown 格式

```markdown
# Lua 代码安全分析报告

## path/to/file.lua

1. 漏洞概述
...

2. 漏洞详情
...

3. 修复建议
...

4. 最佳实践
...
```

## 环境变量

| 环境变量 | 说明 |
|----------|------|
| LUASHIELD_API_KEY | API 密钥 |
| LUASHIELD_BASE_URL | 基础 URL |
| LUASHIELD_MODEL_NAME | 模型名称 |
| LUASHIELD_LOG_LEVEL | 日志级别 |
| LUASHIELD_OUTPUT_FORMAT | 输出格式 |

## 贡献

欢迎提交 Issue 和 Pull Request！

## 许可证

MIT 