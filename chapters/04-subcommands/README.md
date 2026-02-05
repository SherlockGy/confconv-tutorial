# 第 4 章：子命令 - 构建多功能工具

## 本章目标

学完本章，你将能够：
- 理解子命令的设计模式
- 使用 `#[derive(Subcommand)]` 定义命令
- 区分全局参数和子命令参数
- 为命令添加别名

---

## CLI 概念讲解

### 什么是子命令？

子命令是 CLI 工具组织多个相关功能的方式。你每天都在使用它们：

```bash
git add file.txt       # git 的 add 子命令
git commit -m "msg"    # git 的 commit 子命令
git push origin main   # git 的 push 子命令

cargo build            # cargo 的 build 子命令
cargo run              # cargo 的 run 子命令
cargo test             # cargo 的 test 子命令
```

### 子命令的结构

```
程序名  子命令   子命令参数
  ↓      ↓        ↓
confconv convert config.json --to yaml
                    ↑           ↑
                位置参数      选项参数
```

### 全局参数 vs 子命令参数

| 类型 | 位置 | 示例 |
|------|------|------|
| 全局参数 | 程序名之后，子命令之前 | `git --version` |
| 子命令参数 | 子命令之后 | `git commit -m "msg"` |

```bash
# 全局参数：--verbose 对所有子命令生效
confconv --verbose convert config.json

# 子命令参数：--pretty 只对 convert 子命令有效
confconv convert config.json --pretty
```

### 为什么使用子命令？

1. **功能分组**：相关功能放在一起
2. **参数隔离**：每个子命令有独立的参数
3. **帮助清晰**：`--help` 显示特定命令的帮助
4. **可扩展**：容易添加新功能

---

## Clap 新特性

### #[derive(Subcommand)]

使用枚举定义子命令：

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 转换格式
    Convert { ... },
    /// 验证语法
    Validate { ... },
}
```

### #[command(alias = "...")]

为命令添加别名，方便快速输入：

```rust
#[derive(Subcommand)]
enum Commands {
    /// 转换格式
    #[command(alias = "c")]  // 可以用 confconv c 代替 confconv convert
    Convert { ... },
}
```

### #[arg(global = true)]

全局参数可以放在子命令前或后：

```rust
#[derive(Parser)]
struct Cli {
    /// 详细输出
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

// 以下两种写法等效：
// confconv --verbose convert config.json
// confconv convert --verbose config.json
```

---

## 项目实战

### 本章代码变化

1. 添加 `cli.rs` 单独管理 CLI 定义
2. 添加 `format.rs` 管理 Format 枚举
3. 实现三个子命令：`convert`, `validate`, `format`

### 新的项目结构

```
confconv/
├── Cargo.toml
└── src/
    ├── main.rs       # 入口点
    ├── cli.rs        # CLI 定义
    ├── convert.rs    # 转换逻辑
    └── format.rs     # Format 枚举
```

### cli.rs

```rust
//! CLI 定义模块

use clap::{Parser, Subcommand};

/// 配置文件格式转换工具
#[derive(Parser)]
#[command(name = "confconv")]
#[command(version = "0.1.0")]
#[command(about = "配置文件格式转换工具")]
#[command(arg_required_else_help = true)]  // 无参数时显示帮助
pub struct Cli {
    /// 显示详细信息
    #[arg(short, long, global = true)]
    pub verbose: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 转换配置文件格式
    #[command(alias = "c")]
    Convert {
        /// 输入文件路径
        input: String,

        /// 输出文件路径
        #[arg(short, long)]
        output: Option<String>,

        /// 目标格式 (json/yaml/toml)
        #[arg(short = 't', long = "to")]
        format: String,

        /// 美化输出
        #[arg(short, long)]
        pretty: bool,
    },

    /// 验证配置文件语法
    #[command(alias = "v")]
    Validate {
        /// 配置文件路径
        file: String,

        /// 指定格式（不指定则从扩展名推断）
        #[arg(short, long)]
        format: Option<String>,
    },

    /// 格式化配置文件
    #[command(alias = "fmt")]
    Format {
        /// 配置文件路径
        file: String,

        /// 缩进空格数
        #[arg(short, long, default_value = "2")]
        indent: usize,

        /// 原地修改文件
        #[arg(short = 'w', long)]
        write: bool,
    },
}
```

---

## 运行效果

```bash
# 查看主帮助
cargo run -- --help
```

输出：
```
配置文件格式转换工具

Usage: confconv [OPTIONS] <COMMAND>

Commands:
  convert   转换配置文件格式 [aliases: c]
  validate  验证配置文件语法 [aliases: v]
  format    格式化配置文件 [aliases: fmt]
  help      Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose  显示详细信息
  -h, --help     Print help
  -V, --version  Print version
```

```bash
# 查看 convert 子命令帮助
cargo run -- convert --help
```

输出：
```
转换配置文件格式

Usage: confconv convert [OPTIONS] --to <FORMAT> <INPUT>

Arguments:
  <INPUT>  输入文件路径

Options:
  -o, --output <OUTPUT>  输出文件路径
  -t, --to <FORMAT>      目标格式 (json/yaml/toml)
  -p, --pretty           美化输出
  -v, --verbose          显示详细信息
  -h, --help             Print help
```

```bash
# 使用别名
cargo run -- c config.json -t yaml      # 等同于 convert
cargo run -- v config.json              # 等同于 validate
cargo run -- fmt config.json -w         # 等同于 format
```

---

## 与 Java 对比

### Java (picocli)

```java
@Command(name = "confconv",
         subcommands = {ConvertCommand.class, ValidateCommand.class})
class App implements Runnable {
    @Option(names = {"-v", "--verbose"})
    boolean verbose;

    public void run() { }
}

@Command(name = "convert", aliases = {"c"})
class ConvertCommand implements Runnable {
    @Parameters(index = "0")
    String input;

    @Option(names = {"-t", "--to"})
    String format;

    public void run() {
        // 转换逻辑
    }
}
```

### Rust (clap)

```rust
#[derive(Parser)]
struct Cli {
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    #[command(alias = "c")]
    Convert { input: String, ... },
}
```

**关键差异**：

| 方面 | Java/picocli | Rust/clap |
|------|-------------|-----------|
| 子命令定义 | 多个类 | 单个枚举 |
| 参数传递 | 通过实例字段 | 通过枚举变体字段 |
| 全局参数 | 需要继承或组合 | `global = true` |

Rust 的枚举方式更紧凑，所有子命令在同一个文件中，便于维护。

---

## 常见陷阱

| 陷阱 | 问题 | 解决方案 |
|------|------|---------|
| 忘记 `pub` | 模块外无法访问 | 结构体和字段都加 `pub` |
| 全局参数不生效 | 没有标记 `global = true` | 添加该属性 |
| 别名冲突 | 两个命令使用相同别名 | 使用不同的别名 |
| 必需参数缺省 | 子命令有必需参数但用户没提供 | clap 会自动报错 |

---

## 要点回顾

1. **子命令**通过枚举定义，每个变体是一个命令
2. `#[derive(Subcommand)]` 用于枚举
3. `#[command(alias = "...")]` 添加命令别名
4. `#[arg(global = true)]` 定义全局参数
5. `arg_required_else_help = true` 无参数时显示帮助

---

## 下一章预告

目前的 `--format` 参数是字符串类型，用户可能输入无效值。下一章将：
- 使用 `ValueEnum` 让格式参数类型安全
- 添加参数验证
- 处理参数之间的约束关系
