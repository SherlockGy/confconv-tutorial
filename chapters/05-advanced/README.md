# 第 5 章：高级特性 - 类型安全与验证

## 本章目标

学完本章，你将能够：
- 使用 `ValueEnum` 让枚举成为命令行参数
- 自定义参数验证规则
- 定义参数之间的约束关系
- 从环境变量读取默认值

---

## Clap 高级特性

### #[derive(ValueEnum)]

之前的代码中，`--format` 参数是字符串类型：

```rust
#[arg(short = 't', long = "to")]
format: String,  // 用户可能输入任何无效值
```

使用 `ValueEnum` 让参数类型安全：

```rust
use clap::ValueEnum;

#[derive(Clone, Copy, ValueEnum)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

#[arg(short = 't', long = "to")]
format: Format,  // 只接受有效值，否则自动报错
```

**优势**：
- 编译时类型安全
- 自动生成帮助信息（显示可选值）
- 无效输入自动报错

### value_parser - 自定义解析器

验证参数范围：

```rust
/// 缩进空格数（1-8）
#[arg(
    short,
    long,
    default_value = "2",
    value_parser = clap::value_parser!(u8).range(1..=8)
)]
indent: u8,
```

### conflicts_with - 互斥参数

```rust
/// 详细输出
#[arg(short, long, conflicts_with = "quiet")]
verbose: bool,

/// 安静模式
#[arg(short, long, conflicts_with = "verbose")]
quiet: bool,
```

用户不能同时指定 `--verbose` 和 `--quiet`。

### env - 环境变量

```rust
/// 默认输出格式（可通过 CONFCONV_FORMAT 环境变量设置）
#[arg(short, long, env = "CONFCONV_FORMAT", default_value = "json")]
format: Format,
```

---

## 项目实战

### 本章代码变化

1. `Format` 枚举添加 `ValueEnum` 派生
2. 移除字符串参数，改用类型安全的枚举
3. 添加参数验证（缩进范围限制）
4. 添加互斥参数（verbose/quiet）

### format.rs 改进

```rust
use clap::ValueEnum;

/// 支持的配置文件格式
#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum Format {
    /// JSON 格式
    Json,
    /// YAML 格式
    Yaml,
    /// TOML 格式
    Toml,
}
```

### cli.rs 改进

```rust
#[derive(Parser)]
pub struct Cli {
    /// 显示详细信息
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub verbose: bool,

    /// 安静模式（只输出结果）
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Convert {
        // 使用类型安全的 Format 枚举
        #[arg(short = 't', long = "to")]
        format: Format,  // 而不是 String
        // ...
    },
    Format {
        // 带范围验证的参数
        #[arg(short, long, default_value = "2", value_parser = clap::value_parser!(u8).range(1..=8))]
        indent: u8,
        // ...
    },
}
```

---

## 运行效果

```bash
# 类型安全：无效格式自动报错
cargo run -- convert test.json -t invalid
```

输出：
```
error: invalid value 'invalid' for '--to <FORMAT>'
  [possible values: json, yaml, toml]

For more information, try '--help'.
```

```bash
# 帮助信息显示可选值
cargo run -- convert --help
```

输出：
```
...
Options:
  -t, --to <FORMAT>  目标格式 [possible values: json, yaml, toml]
...
```

```bash
# 互斥参数
cargo run -- --verbose --quiet convert test.json -t yaml
```

输出：
```
error: the argument '--verbose' cannot be used with '--quiet'
```

```bash
# 缩进范围验证
cargo run -- fmt test.json --indent 20
```

输出：
```
error: invalid value '20' for '--indent <INDENT>': 20 is not in 1..=8
```

---

## 与 Java 对比

### Java (picocli)

```java
enum Format { JSON, YAML, TOML }

@Command(name = "convert")
class ConvertCommand {
    @Option(names = {"-t", "--to"})
    Format format;  // picocli 也支持枚举

    @Option(names = {"-v", "--verbose"})
    boolean verbose;

    @Option(names = {"-q", "--quiet"})
    boolean quiet;
    // 但互斥关系需要手动检查
}
```

### Rust (clap)

```rust
#[derive(ValueEnum)]
enum Format { Json, Yaml, Toml }

#[derive(Parser)]
struct Cli {
    #[arg(conflicts_with = "quiet")]
    verbose: bool,

    #[arg(conflicts_with = "verbose")]
    quiet: bool,
}
```

**关键差异**：
- 两者都支持枚举参数
- Clap 的 `conflicts_with` 声明式定义互斥关系
- Clap 的 `value_parser` 提供更强的验证能力

---

## 要点回顾

1. `#[derive(ValueEnum)]` 让枚举成为类型安全的参数
2. `value_parser` 自定义参数验证
3. `conflicts_with` 定义互斥参数
4. `env` 从环境变量读取默认值
5. Clap 自动在帮助中显示可选值

---

## 下一章预告

现在代码逻辑都在 `main.rs` 中。下一章将：
- 将命令处理逻辑移到 `commands/` 模块
- 添加自定义错误类型
- 完善用户体验（管道支持、退出码）
