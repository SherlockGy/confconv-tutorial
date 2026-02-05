# 第 2 章：选项与标志 - 让程序更灵活

## 本章目标

学完本章，你将能够：
- 区分位置参数、选项、标志
- 使用 `#[arg(short, long)]` 定义选项
- 处理可选参数 `Option<T>`
- 设置默认值

---

## CLI 概念讲解

### 短选项与长选项

Unix/Linux 世界有一个约定俗成的命令行风格：

| 类型 | 格式 | 示例 |
|------|------|------|
| 短选项 | 单横线 + 单字母 | `-o file.txt` |
| 长选项 | 双横线 + 单词 | `--output file.txt` |

**为什么有两种？**

- **短选项**：输入快，适合常用选项
- **长选项**：可读性好，适合脚本和文档

```bash
# 这两个命令等价
ls -l -a -h
ls --long --all --human-readable

# 短选项可以合并
ls -lah
```

### 标志 vs 选项

| 类型 | 是否需要值 | 示例 |
|------|-----------|------|
| 标志（Flag） | 不需要 | `--verbose` 或 `-v` |
| 选项（Option） | 需要 | `--output file.txt` 或 `-o file.txt` |

```bash
# 标志：只需要出现，表示"开启"某功能
grep --ignore-case pattern file.txt

# 选项：需要跟随一个值
curl --output response.json http://api.example.com
```

### Unix 传统的选项设计

好的 CLI 工具遵循这些原则：

1. **常用功能用短选项**：`-v`, `-o`, `-h`
2. **所有选项都有长选项版本**：便于阅读和记忆
3. **布尔功能用标志**：不需要 `--verbose=true`
4. **避免短选项冲突**：`-h` 通常是 `--help`

---

## Clap 新特性

### #[arg(short, long)]

这是最常用的属性组合，让字段同时支持短选项和长选项：

```rust
#[derive(Parser)]
struct Cli {
    /// 输出文件路径
    #[arg(short, long)]  // 自动生成 -o 和 --output
    output: Option<String>,
}
```

`short` 默认取字段名首字母，`long` 默认取完整字段名。

### 自定义选项名

有时需要手动指定选项名：

```rust
#[derive(Parser)]
struct Cli {
    /// 目标格式
    #[arg(short = 't', long = "to")]  // -t 和 --to
    format: String,
}
```

### Option<T> - 可选参数

用 `Option<T>` 表示参数可以不提供：

```rust
#[derive(Parser)]
struct Cli {
    /// 输出文件（可选，不指定则输出到标准输出）
    #[arg(short, long)]
    output: Option<String>,  // 可能是 Some("file.txt") 或 None
}
```

### bool - 自动成为标志

`bool` 类型字段自动成为标志（不需要值）：

```rust
#[derive(Parser)]
struct Cli {
    /// 启用美化输出
    #[arg(short, long)]
    pretty: bool,  // 使用 --pretty 时为 true，否则为 false
}
```

### default_value - 默认值

```rust
#[derive(Parser)]
struct Cli {
    /// 输出格式
    #[arg(short = 't', long = "to", default_value = "json")]
    format: String,  // 不指定时默认为 "json"
}
```

---

## 项目实战

### 本章代码变化

相比第 1 章，我们添加了：
- `-o`/`--output`：可选的输出文件路径
- `-t`/`--to`：目标格式（默认 json）
- `-p`/`--pretty`：美化输出标志
- `-v`/`--verbose`：详细信息标志

### 完整代码

```rust
// src/main.rs

use clap::Parser;

/// 配置文件格式转换工具
#[derive(Parser)]
#[command(name = "confconv")]
#[command(version = "0.1.0")]
#[command(about = "配置文件格式转换工具")]
struct Cli {
    /// 输入文件路径
    input: String,

    /// 输出文件路径
    ///
    /// 不指定则输出到标准输出（stdout）
    #[arg(short, long)]
    output: Option<String>,

    /// 目标格式
    ///
    /// 支持: json, yaml, toml
    #[arg(short = 't', long = "to", default_value = "json")]
    format: String,

    /// 美化输出（带缩进和换行）
    #[arg(short, long)]
    pretty: bool,

    /// 显示详细信息
    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let cli = Cli::parse();

    // 如果开启了 verbose，打印所有参数信息
    if cli.verbose {
        println!("=== 参数信息 ===");
        println!("输入文件: {}", cli.input);
        println!("输出文件: {:?}", cli.output);
        println!("目标格式: {}", cli.format);
        println!("美化输出: {}", cli.pretty);
        println!("================\n");
    }

    // 根据是否指定输出文件，决定输出方式
    match &cli.output {
        Some(path) => println!("将写入文件: {}", path),
        None => println!("将输出到标准输出"),
    }

    println!("\n功能开发中...");
}
```

---

## 运行效果

```bash
# 查看帮助
cargo run -- --help
```

输出：
```
配置文件格式转换工具

Usage: confconv [OPTIONS] <INPUT>

Arguments:
  <INPUT>  输入文件路径

Options:
  -o, --output <OUTPUT>  输出文件路径
  -t, --to <FORMAT>      目标格式 [default: json]
  -p, --pretty           美化输出（带缩进和换行）
  -v, --verbose          显示详细信息
  -h, --help             Print help
  -V, --version          Print version
```

```bash
# 使用各种选项
cargo run -- config.json -t yaml -p -v
```

输出：
```
=== 参数信息 ===
输入文件: config.json
输出文件: None
目标格式: yaml
美化输出: true
================

将输出到标准输出

功能开发中...
```

```bash
# 指定输出文件
cargo run -- config.json -o result.yaml -t yaml
```

输出：
```
将写入文件: result.yaml

功能开发中...
```

---

## 与 Java 对比

### Java (picocli)

```java
@Command(name = "confconv")
class App implements Runnable {
    @Parameters(index = "0")
    private String input;

    @Option(names = {"-o", "--output"})
    private String output;  // null 表示未指定

    @Option(names = {"-t", "--to"}, defaultValue = "json")
    private String format;

    @Option(names = {"-p", "--pretty"})
    private boolean pretty;  // false 表示未指定

    @Option(names = {"-v", "--verbose"})
    private boolean verbose;

    public void run() {
        if (verbose) {
            System.out.println("输入: " + input);
        }
    }
}
```

### Rust (clap)

```rust
#[derive(Parser)]
struct Cli {
    input: String,

    #[arg(short, long)]
    output: Option<String>,  // None 表示未指定

    #[arg(short = 't', long = "to", default_value = "json")]
    format: String,

    #[arg(short, long)]
    pretty: bool,

    #[arg(short, long)]
    verbose: bool,
}
```

**关键差异**：

| 方面 | Java/picocli | Rust/clap |
|------|-------------|-----------|
| 可选值表示 | `null` | `Option<T>` |
| 默认值 | 注解属性 | 注解属性 |
| 短选项 | `names = {"-o"}` | `short` 或 `short = 'o'` |
| 类型安全 | 运行时 | 编译时 |

Rust 的 `Option<T>` 强制你处理"值可能不存在"的情况，避免 `NullPointerException`。

---

## 常见陷阱

| 陷阱 | 问题 | 解决方案 |
|------|------|---------|
| 短选项冲突 | `-h` 被 `--help` 占用 | 使用其他字母，如 `-H` |
| 忘记处理 None | `output.unwrap()` panic | 使用 `match` 或 `if let` |
| 默认值类型不匹配 | `default_value = "123"` 但字段是 `u32` | clap 会自动转换 |

### 关于 -v 和 -V

注意 clap 默认使用：
- `-V`（大写）：`--version`
- `-h`：`--help`

如果你想用 `-v` 作为 `--verbose`，没问题，因为 `-V` 和 `-v` 不冲突。

---

## 要点回顾

1. **短选项** `-o` 和**长选项** `--output` 是 Unix 传统
2. `#[arg(short, long)]` 自动推断选项名
3. `Option<T>` 表示可选参数
4. `bool` 类型自动成为标志
5. `default_value` 设置默认值

---

## 下一章预告

目前我们只是打印参数信息，还没有真正的转换功能。下一章将：
- 引入 Serde，Rust 最强大的序列化库
- 实现 JSON/YAML/TOML 之间的真正转换
- 理解 Serde 独特的设计哲学
