# 第 1 章：初识命令行 - 从零到第一个参数

## 本章目标

学完本章，你将能够：
- 理解命令行程序的本质
- 创建一个使用 clap 的 Rust 项目
- 定义并解析位置参数
- 自动生成 `--help` 和 `--version`

---

## CLI 概念讲解

### 什么是命令行程序？

命令行程序（CLI，Command Line Interface）是通过终端/控制台与用户交互的程序。与图形界面程序不同，CLI 程序通过文本输入输出来工作。

**Java 对比**：

```java
// Java 的入口点
public class Main {
    public static void main(String[] args) {
        // args 是命令行参数数组
        if (args.length > 0) {
            System.out.println("第一个参数: " + args[0]);
        }
    }
}
```

```rust
// Rust 的入口点
fn main() {
    // 通过 std::env::args() 获取参数
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        println!("第一个参数: {}", args[1]); // 注意：args[0] 是程序名
    }
}
```

**关键差异**：
- Java 的 `args[0]` 是第一个参数
- Rust 的 `args[0]` 是程序名本身，`args[1]` 才是第一个参数

### 参数、选项、标志的区别

命令行程序接收三种输入，理解它们的区别非常重要：

| 类型 | 说明 | 示例 |
|------|------|------|
| **参数**（Argument） | 位置固定的输入值 | `cp source.txt dest.txt` |
| **选项**（Option） | 带名称和值的配置 | `--output file.txt` 或 `-o file.txt` |
| **标志**（Flag） | 布尔开关，不需要值 | `--verbose` 或 `-v` |

#### 详细解释

**1. 参数（Argument）- 也叫"位置参数"**

参数是**按位置传递**的值，没有名字，顺序很重要。

```bash
cp source.txt dest.txt
#  ↑          ↑
#  第1个参数   第2个参数
#  (源文件)    (目标文件)
```

- 你不能写 `cp dest.txt source.txt`，因为顺序会反
- 参数通常是**必需**的（不提供会报错）
- 参数**没有** `--` 或 `-` 前缀

**2. 选项（Option）- 带名字的配置**

选项是**键值对**，有名字，顺序不重要。

```bash
curl --output result.json https://api.example.com
#    ↑        ↑            ↑
#    选项名   选项值        参数（URL）

# 等价写法（顺序可以变）：
curl https://api.example.com --output result.json
```

- 长选项用 `--output`（两个横杠 + 单词）
- 短选项用 `-o`（一个横杠 + 单字母）
- `-o result.json` 和 `--output result.json` 通常等价
- 选项通常是**可选**的（有默认值或可以不提供）

**3. 标志（Flag）- 布尔开关**

标志是**只有名字，没有值**的选项，表示"开启"某个功能。

```bash
ls -l           # 开启"长格式"显示
ls --all        # 开启"显示隐藏文件"
rm -rf folder/  # 开启"递归"和"强制"模式
```

- 出现 = true，不出现 = false
- 不需要写 `--verbose=true`，直接写 `--verbose`

#### 完整示例分析

```bash
git commit -m "message" --amend
```

让我们逐个分析：

| 部分 | 类型 | 说明 |
|------|------|------|
| `git` | 程序名 | 要执行的程序 |
| `commit` | 子命令 | git 的子命令（后面章节讲） |
| `-m` | 选项名 | message 的缩写 |
| `"message"` | 选项值 | `-m` 选项的值 |
| `--amend` | 标志 | 开启"修改上次提交"功能 |

再看一个更复杂的例子：

```bash
ffmpeg -i input.mp4 -c:v libx264 -crf 23 output.mp4
#      ↑  ↑         ↑   ↑        ↑   ↑   ↑
#      选项 选项值   选项 选项值   选项 值  参数（位置参数）
```

#### 为什么要区分？

在 Clap 中，**不同类型的输入用不同的方式定义**：

```rust
#[derive(Parser)]
struct Cli {
    // 位置参数：没有 #[arg] 属性，或 #[arg] 中没有 short/long
    input: String,                    // 参数

    // 选项：有 #[arg(short, long)]，类型是 Option<T> 或有默认值
    #[arg(short, long)]
    output: Option<String>,           // 选项

    // 标志：有 #[arg(short, long)]，类型是 bool
    #[arg(short, long)]
    verbose: bool,                    // 标志
}
```

下一章我们会详细讲解如何定义选项和标志

### --help 和 --version

几乎所有 CLI 程序都支持：

- `--help` 或 `-h`：显示帮助信息
- `--version` 或 `-V`：显示版本号

这是 Unix 世界的约定俗成。好的 CLI 工具应该提供清晰的帮助信息。

---

## 初识 Clap

### 什么是 Clap？

**Clap**（Command Line Argument Parser）是 Rust 最流行的命令行参数解析库。

**名字由来**：Clap 是 "Command Line Argument Parser" 的首字母缩写，也有"鼓掌"的意思。

**为什么用 Clap 而不是手动解析？**

| 手动解析 | Clap |
|---------|------|
| 需要自己处理各种边界情况 | 自动处理 |
| 需要自己写帮助信息 | 自动生成 |
| 容易出错 | 类型安全 |
| 代码冗长 | 声明式、简洁 |

### #[derive(Parser)]

Clap 提供两种 API：
1. **Builder API**：命令式，灵活但代码冗长
2. **Derive API**：声明式，简洁且类型安全（推荐）

我们使用 Derive API，通过 `#[derive(Parser)]` 宏自动生成解析代码：

```rust
use clap::Parser;

// 这个结构体就是你的 CLI 定义
#[derive(Parser)]
struct Cli {
    // 字段就是参数
    input: String,
}

fn main() {
    // 一行代码完成解析
    let cli = Cli::parse();
    println!("输入: {}", cli.input);
}
```

### #[command(...)] 属性

**这个属性从哪来？**

`#[command(...)]` 不是 Rust 标准库的东西，而是 clap 的 derive 宏定义的**辅助属性**：

```rust
#[derive(Parser)]  // ← 这是 clap 提供的过程宏
//       ↑
//  clap 在处理这个 derive 时，会同时识别：
//  - #[command(...)] 用于配置整个命令
//  - #[arg(...)]     用于配置单个参数
```

`name`、`version`、`about` 等都是 clap 规定的参数名，不是 Rust 语法。

**Java 对比**：
```java
// Java: @Command 注解来自 picocli 库
@Command(name = "confconv", version = "0.1.0")
class Cli { }
```

本质上都是"库定义的元数据标注"，只是语法不同。

**常用参数**：

```rust
#[derive(Parser)]
#[command(name = "confconv")]           // 程序名称（用于 --version 输出）
#[command(bin_name = "confconv")]       // 二进制名称（用于 Usage 行）
#[command(version = "0.1.0")]           // 版本号
#[command(about = "配置文件格式转换")]    // 简短描述
struct Cli {
    // ...
}
```

| 参数 | 作用 | 显示位置 |
|------|------|----------|
| `name` | 程序的"官方名称" | `--version` 输出 |
| `bin_name` | 可执行文件名 | `Usage: xxx` 行 |
| `version` | 版本号 | `--version` 输出 |
| `about` | 简短描述 | `--help` 第一行 |

> **注意**：
> - 如果不设置 `bin_name`，Usage 行会显示实际的可执行文件名（如 `confconv.exe`），而不是 `name` 的值
> - 如果不设置 `version`，程序不会有 `--version` 选项，此时 `name` 基本无处显示——它主要是为 `--version` 服务的

运行 `--help` 时，这些信息会自动显示。

### 位置参数

没有 `#[arg(...)]` 属性的字段默认是位置参数：

```rust
#[derive(Parser)]
struct Cli {
    /// 输入文件路径
    input: String,  // 位置参数，必须提供
}
```

文档注释 `///` 会成为帮助信息中的参数说明。

---

## 项目实战

### 创建项目

```bash
cd chapters/01-hello-cli
cargo new confconv
cd confconv
```

### 理解 Cargo.toml

`Cargo.toml` 是 Rust 项目的配置文件，类似于 Java 的 `pom.xml` 或 `build.gradle`。

编辑 `Cargo.toml`：

```toml
[package]
name = "confconv"                       # 包名，也是默认的二进制名称
version = "0.1.0"                       # 语义化版本号
edition = "2021"                        # Rust edition（推荐 2021）
description = "配置文件格式转换工具"      # 包描述（发布到 crates.io 时显示）
readme = "README.md"                    # 说明文档路径
license = "MIT"                         # 开源许可证
keywords = ["config", "converter", "json", "yaml", "toml"]  # 搜索关键词（最多5个）
categories = ["command-line-utilities"] # 分类（见 crates.io/category_slugs）

[dependencies]
clap = { version = "4.5", features = ["derive"] }

# 发布构建优化（可选）
[profile.release]
strip = true    # 移除调试符号，减小二进制体积
lto = true      # 链接时优化，提升性能但编译变慢
```

#### 字段说明

| 字段 | 必需 | 说明 |
|------|------|------|
| `name` | 是 | 包名，也是默认的二进制文件名 |
| `version` | 是 | 语义化版本号 (major.minor.patch) |
| `edition` | 是 | Rust 语言版本（2021 或 2024） |
| `description` | 否 | 简短描述，发布时显示 |
| `readme` | 否 | README 文件路径 |
| `license` | 否 | 许可证标识符（MIT / Apache-2.0 等） |
| `keywords` | 否 | 搜索关键词，最多 5 个 |
| `categories` | 否 | crates.io 分类 |

#### [[bin]] 配置（可选）

通常**不需要**手动配置 `[[bin]]`，因为 `[package].name` 就是二进制名称。

只有当你想要**不同于包名的二进制名称**时才需要：

```toml
[[bin]]
name = "my-cli"        # 二进制名称（覆盖 package.name）
path = "src/main.rs"   # 入口文件路径
```

#### 与 clap bin_name 的关系

| 配置位置 | 作用 |
|----------|------|
| `Cargo.toml` 的 `[package].name` | 实际编译出的二进制文件名 |
| clap 的 `#[command(bin_name = "...")]` | `--help` 中 Usage 行显示的名称 |

通常两者保持一致即可。如果不设置 clap 的 `bin_name`，它会自动使用实际运行时的二进制名。

#### 依赖声明

```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
```

**注意**：必须启用 `derive` feature 才能使用 `#[derive(Parser)]`。

### 完整代码

```rust
// src/main.rs

//! confconv - 配置文件格式转换工具
//!
//! 第 1 章：最简单的 CLI 程序
//! 只接收一个位置参数：输入文件路径

use clap::Parser;

/// 配置文件格式转换工具
///
/// 支持在 JSON、YAML、TOML 之间互相转换
#[derive(Parser)]
#[command(name = "confconv")]
#[command(version = "0.1.0")]
#[command(about = "配置文件格式转换工具")]
struct Cli {
    /// 输入文件路径
    ///
    /// 支持 .json, .yaml, .yml, .toml 扩展名
    input: String,
}

fn main() {
    // 解析命令行参数
    // 如果参数不正确，clap 会自动打印错误信息并退出
    let cli = Cli::parse();

    // 目前只是打印收到的参数
    println!("输入文件: {}", cli.input);
    println!("\n功能开发中...");
}
```

---

## 开发与运行

### cargo run 与 `--` 分隔符

开发时我们用 `cargo run` 来编译并运行程序。但这里有个重要的细节：

```bash
# ❌ 错误：cargo 会把 --help 当成自己的参数
cargo run --help         # 显示的是 cargo run 的帮助，不是你程序的帮助！

# ✅ 正确：用 -- 分隔 cargo 参数和程序参数
cargo run -- --help      # -- 之后的参数传给你的程序
cargo run -- config.json # 传递 config.json 给程序
```

**`--` 的作用**：告诉 cargo "后面的参数不是给你的，请原样传给程序"。

这是 Unix 命令行的通用约定，不仅限于 cargo。

### 开发阶段 vs 发布阶段

| 阶段 | 命令 | 说明 |
|------|------|------|
| 开发 | `cargo run -- <args>` | 自动编译 + 运行，方便快速测试 |
| 发布 | `confconv <args>` | 使用编译好的二进制文件 |

```bash
# 开发时快速测试
cargo run -- --help

# 编译发布版本
cargo build --release

# 二进制文件位置：target/release/confconv（或 .exe）
# 可以复制到 PATH 目录，然后直接使用：
confconv --help
```

---

## 运行效果

先编译项目：

```bash
cargo build
```

然后运行（以下示例假设 `confconv` 已在 PATH 中，或在 `target/debug/` 目录下运行）：

```bash
# 查看帮助
confconv --help
```

输出：
```
配置文件格式转换工具

Usage: confconv <INPUT>

Arguments:
  <INPUT>  输入文件路径

Options:
  -h, --help     Print help
  -V, --version  Print version
```

```bash
# 查看版本
confconv --version
```

输出：
```
confconv 0.1.0
```

```bash
# 正常使用
confconv config.json
```

输出：
```
输入文件: config.json

功能开发中...
```

```bash
# 不提供参数（会报错）
confconv
```

输出：
```
error: the following required arguments were not provided:
  <INPUT>

Usage: confconv <INPUT>

For more information, try '--help'.
```

---

## 与 Java 对比

### 参数解析对比

**Java（手动解析）**：
```java
public static void main(String[] args) {
    if (args.length < 1) {
        System.out.println("用法: java App <input>");
        System.exit(1);
    }
    String input = args[0];
    System.out.println("输入文件: " + input);
}
```

**Java（使用 picocli）**：
```java
@Command(name = "confconv", version = "0.1.0")
class App implements Runnable {
    @Parameters(index = "0", description = "输入文件路径")
    private String input;

    public void run() {
        System.out.println("输入文件: " + input);
    }
}
```

**Rust（使用 clap）**：
```rust
#[derive(Parser)]
#[command(name = "confconv", version = "0.1.0")]
struct Cli {
    /// 输入文件路径
    input: String,
}

fn main() {
    let cli = Cli::parse();
    println!("输入文件: {}", cli.input);
}
```

**关键差异**：
- Java 的 picocli 需要实现 `Runnable`，逻辑在 `run()` 方法中
- Rust 的 clap 只是解析参数到结构体，逻辑在 `main()` 中
- Rust 的文档注释 `///` 直接成为帮助文本，更简洁

---

## 常见陷阱

| 陷阱 | 解决方案 |
|------|---------|
| 忘记添加 `derive` feature | 确保 `features = ["derive"]` |
| `cargo run config.json` 不工作 | 需要 `cargo run -- config.json`（见上方"开发与运行"） |
| 文档注释用 `//` | 应该用 `///`，单行注释不会成为帮助文本 |

---

## 要点回顾

1. **CLI 程序**通过命令行参数接收输入
2. **Clap** 是 Rust 最流行的参数解析库
3. **Derive API** 使用 `#[derive(Parser)]` 声明式定义 CLI
4. **位置参数**是没有 `--` 前缀的必需输入
5. **文档注释** `///` 会自动成为帮助信息

---

## 下一章预告

目前我们只能接收一个位置参数。下一章将学习：
- 如何添加 `-o`/`--output` 选项
- 如何使用 `--pretty` 标志
- 如何设置默认值

这些都是让程序更灵活的关键特性。
