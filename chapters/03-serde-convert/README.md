# 第 3 章：引入 Serde - 实现真正的转换

## 本章目标

学完本章，你将能够：
- 理解 Serde 的设计哲学
- 使用 serde_json、serde_yml、toml 处理三种格式
- 实现真正的格式转换功能
- 学会拆分代码到多个文件

---

## Serde 概念讲解

### 什么是 Serde？

**Serde**（SERialize + DEserialize）是 Rust 生态中下载量最高的 crate 之一，由 David Tolnay 开发。它是一个通用的序列化/反序列化框架。

### 为什么 Serde 是 Rust 独有的设计？

在其他语言中，序列化库通常与格式绑定：

| 语言 | JSON | YAML | TOML |
|------|------|------|------|
| Java | Jackson | SnakeYAML | toml4j |
| Python | json | PyYAML | toml |
| JavaScript | JSON.parse | js-yaml | @iarna/toml |

每个库有不同的 API，切换格式需要学习新库。

**Serde 的革命性设计**：

```
┌─────────────┐     ┌─────────────┐
│ 你的数据结构 │ ←→  │    Serde    │ ←→  具体格式库
│             │     │ (框架协议)   │     serde_json
│ #[derive(   │     │             │     serde_yml
│  Serialize, │     │             │     toml
│  Deserialize│     │             │     ...
│ )]          │     │             │
└─────────────┘     └─────────────┘
```

**核心理念**：框架与格式分离
- `serde` 本身不绑定任何格式
- 通过 trait 定义序列化协议
- 具体格式由独立的 crate 实现

**这能实现的原因是 Rust 的**：
- **Trait 系统**：定义通用协议
- **过程宏**：`#[derive(Serialize, Deserialize)]` 自动实现
- **零成本抽象**：编译期生成高效代码

### serde_json::Value - 动态类型

当你不知道数据结构时，可以用 `Value` 类型：

```rust
use serde_json::Value;

let data: Value = serde_json::from_str(r#"{"name": "Alice", "age": 30}"#)?;

// 像操作 JSON 一样操作
println!("{}", data["name"]);  // "Alice"
```

这正是配置转换器需要的——我们不知道用户的配置结构，但需要完整保留它。

---

## 项目实战

### 本章代码变化

1. 拆分为两个文件：`main.rs` 和 `convert.rs`
2. 添加 serde 相关依赖
3. 实现真正的格式转换

### 新的项目结构

```
confconv/
├── Cargo.toml
└── src/
    ├── main.rs      # CLI 入口
    └── convert.rs   # 转换逻辑
```

### Cargo.toml

```toml
[package]
name = "confconv"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4.5", features = ["derive"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_yml = "0.0.12"
toml = "0.8"
```

> **注意**：`serde_yaml` 已被弃用，我们使用其维护的 fork `serde_yml`

### convert.rs

```rust
// src/convert.rs

//! 格式转换模块
//!
//! 负责在 JSON、YAML、TOML 之间转换

use std::error::Error;

/// 支持的配置格式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

impl Format {
    /// 从字符串解析格式
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Format::Json),
            "yaml" | "yml" => Some(Format::Yaml),
            "toml" => Some(Format::Toml),
            _ => None,
        }
    }

    /// 从文件扩展名推断格式
    pub fn from_extension(path: &str) -> Option<Self> {
        let ext = path.rsplit('.').next()?.to_lowercase();
        Self::from_str(&ext)
    }
}

/// 执行格式转换
pub fn convert(
    input: &str,
    from: Format,
    to: Format,
    pretty: bool,
) -> Result<String, Box<dyn Error>> {
    // 第一步：解析为通用的 JSON Value
    let value: serde_json::Value = match from {
        Format::Json => serde_json::from_str(input)?,
        Format::Yaml => serde_yml::from_str(input)?,
        Format::Toml => {
            let toml_value: toml::Value = toml::from_str(input)?;
            serde_json::to_value(toml_value)?
        }
    };

    // 第二步：序列化为目标格式
    let output = match to {
        Format::Json => {
            if pretty {
                serde_json::to_string_pretty(&value)?
            } else {
                serde_json::to_string(&value)?
            }
        }
        Format::Yaml => serde_yml::to_string(&value)?,
        Format::Toml => {
            let toml_value = json_to_toml(&value)?;
            if pretty {
                toml::to_string_pretty(&toml_value)?
            } else {
                toml::to_string(&toml_value)?
            }
        }
    };

    Ok(output)
}

/// 将 JSON Value 转换为 TOML Value
fn json_to_toml(json: &serde_json::Value) -> Result<toml::Value, Box<dyn Error>> {
    let toml_str = serde_json::to_string(json)?;
    let toml_value: toml::Value = serde_json::from_str(&toml_str)?;
    Ok(toml_value)
}
```

### main.rs

```rust
// src/main.rs

//! confconv - 配置文件格式转换工具
//!
//! 第 3 章：引入 Serde，实现真正的格式转换

mod convert;

use clap::Parser;
use convert::{convert, Format};
use std::fs;

/// 配置文件格式转换工具
#[derive(Parser)]
#[command(name = "confconv")]
#[command(version = "0.1.0")]
#[command(about = "配置文件格式转换工具")]
struct Cli {
    /// 输入文件路径
    input: String,

    /// 输出文件路径
    #[arg(short, long)]
    output: Option<String>,

    /// 目标格式 (json/yaml/toml)
    #[arg(short = 't', long = "to", default_value = "json")]
    format: String,

    /// 美化输出
    #[arg(short, long)]
    pretty: bool,

    /// 显示详细信息
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    // 解析目标格式
    let to_format = match Format::from_str(&cli.format) {
        Some(f) => f,
        None => {
            eprintln!("错误: 不支持的目标格式 '{}'", cli.format);
            eprintln!("支持的格式: json, yaml, toml");
            std::process::exit(1);
        }
    };

    // 从文件扩展名推断源格式
    let from_format = match Format::from_extension(&cli.input) {
        Some(f) => f,
        None => {
            eprintln!("错误: 无法从文件扩展名推断格式: {}", cli.input);
            eprintln!("请使用 .json, .yaml, .yml, 或 .toml 扩展名");
            std::process::exit(1);
        }
    };

    if cli.verbose {
        println!("源格式: {:?}", from_format);
        println!("目标格式: {:?}", to_format);
    }

    // 读取输入文件
    let input_content = match fs::read_to_string(&cli.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("错误: 无法读取文件 '{}': {}", cli.input, e);
            std::process::exit(1);
        }
    };

    // 执行转换
    let output_content = match convert(&input_content, from_format, to_format, cli.pretty) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("错误: 转换失败: {}", e);
            std::process::exit(1);
        }
    };

    // 输出结果
    match &cli.output {
        Some(path) => {
            if let Err(e) = fs::write(path, &output_content) {
                eprintln!("错误: 无法写入文件 '{}': {}", path, e);
                std::process::exit(1);
            }
            if cli.verbose {
                println!("已写入: {}", path);
            }
        }
        None => {
            println!("{}", output_content);
        }
    }
}
```

---

## 运行效果

首先创建测试文件：

```bash
# 创建测试 JSON 文件
echo '{"name": "confconv", "version": "0.1.0", "features": ["json", "yaml", "toml"]}' > test.json
```

```bash
# JSON → YAML
cargo run -- test.json -t yaml -p
```

输出：
```yaml
features:
- json
- yaml
- toml
name: confconv
version: 0.1.0
```

```bash
# JSON → TOML
cargo run -- test.json -t toml -p
```

输出：
```toml
name = "confconv"
version = "0.1.0"
features = ["json", "yaml", "toml"]
```

```bash
# 保存到文件
cargo run -- test.json -t yaml -o result.yaml -v
```

输出：
```
源格式: Json
目标格式: Yaml
已写入: result.yaml
```

---

## 与 Java 对比

### Java 的格式转换

```java
// 需要使用不同的库和 API
import com.fasterxml.jackson.databind.ObjectMapper;
import com.fasterxml.jackson.dataformat.yaml.YAMLFactory;
import com.moandjiezana.toml.Toml;

// JSON → YAML
ObjectMapper jsonMapper = new ObjectMapper();
ObjectMapper yamlMapper = new ObjectMapper(new YAMLFactory());
Object data = jsonMapper.readValue(jsonString, Object.class);
String yaml = yamlMapper.writeValueAsString(data);

// TOML 需要完全不同的 API
Toml toml = new Toml().read(tomlString);
Map<String, Object> map = toml.toMap();
```

### Rust 的格式转换

```rust
// 统一的 API 风格
let json_value: serde_json::Value = serde_json::from_str(&json_str)?;
let yaml_str = serde_yml::to_string(&json_value)?;

let yaml_value: serde_json::Value = serde_yml::from_str(&yaml_str)?;
let toml_str = toml::to_string(&toml_value)?;
```

**关键差异**：
- Java 需要学习多个库的不同 API
- Rust 所有格式库遵循相同的 `from_str` / `to_string` 模式
- Serde 的 Value 类型可以作为中间桥梁

---

## 代码组织：模块系统

### mod 关键字

```rust
// main.rs
mod convert;  // 声明 convert 模块

// 这会寻找以下文件之一：
// - src/convert.rs
// - src/convert/mod.rs
```

### 与 Java 的对比

| Java | Rust |
|------|------|
| `import com.example.Convert;` | `mod convert;` |
| 按包路径寻找类 | 按文件路径寻找模块 |
| `public class` 控制可见性 | `pub` 控制可见性 |

```rust
// convert.rs
pub enum Format { ... }     // pub 使其在模块外可见
pub fn convert(...) { ... } // pub 使函数可被外部调用
fn json_to_toml(...) { ... } // 没有 pub，只能在本模块内使用
```

---

## 常见陷阱

| 陷阱 | 问题 | 解决方案 |
|------|------|---------|
| 使用 `serde_yaml` | 已被弃用 | 改用 `serde_yml` |
| TOML 数组类型 | TOML 数组元素必须同类型 | 检查输入数据 |
| 忘记 `pub` | 模块外无法访问 | 添加 `pub` 关键字 |

### TOML 的特殊限制

TOML 对数据有更严格的要求：
- 数组元素必须是同一类型
- 不支持 null 值
- 键名有限制

如果转换到 TOML 失败，检查源数据是否符合 TOML 规范。

---

## 要点回顾

1. **Serde** 是框架与格式分离的设计典范
2. `serde_json::Value` 可以作为不同格式的桥梁
3. `mod` 声明模块，`pub` 控制可见性
4. `serde_yaml` 已弃用，使用 `serde_yml`
5. 错误处理使用 `Box<dyn Error>` 简化类型

---

## 下一章预告

目前我们的程序只是一个简单的转换工具。下一章将：
- 引入子命令：`convert`, `validate`, `format`
- 学习 Clap 的 `Subcommand` 特性
- 重构为更清晰的模块结构
