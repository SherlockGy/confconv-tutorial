# 第 6 章：生产级结构 - 最终版本

## 本章目标

学完本章，你将能够：
- 构建生产级的项目结构
- 实现命令处理模块化
- 处理标准输入输出和管道
- 设计友好的错误消息

---

## 生产级项目结构

### 最终目录结构

```
confconv/
├── Cargo.toml
├── src/
│   ├── main.rs       # 入口点（简洁）
│   ├── cli.rs        # CLI 定义
│   ├── format.rs     # Format 枚举
│   ├── error.rs      # 错误类型
│   └── commands/     # 命令处理模块
│       ├── mod.rs    # 模块导出
│       ├── convert.rs
│       ├── validate.rs
│       └── format.rs
└── README.md
```

### 为什么这样组织？

| 模块 | 职责 |
|------|------|
| `main.rs` | 入口点，只负责解析参数和调用命令 |
| `cli.rs` | CLI 定义，所有 clap 结构体 |
| `commands/` | 命令实现，每个子命令一个文件 |
| `error.rs` | 错误类型定义 |
| `format.rs` | 共享的数据类型 |

**关键原则**：
- **单一职责**：每个模块只做一件事
- **高内聚**：相关代码放在一起
- **低耦合**：模块之间依赖最小化

---

## CLI 用户体验

### 管道支持

好的 CLI 工具应该支持管道：

```bash
# 从标准输入读取
cat config.json | confconv convert --from json --to yaml

# 输出到管道
confconv convert config.json -t yaml | grep "key"
```

使用 `-` 表示标准输入：

```rust
#[derive(Subcommand)]
pub enum Commands {
    Convert {
        /// 输入文件路径（使用 - 表示标准输入）
        #[arg(default_value = "-")]
        input: String,
        // ...
    },
}
```

### 退出码

| 退出码 | 含义 |
|-------|------|
| 0 | 成功 |
| 1 | 一般错误 |
| 2 | 命令行参数错误 |

```rust
fn main() {
    if let Err(e) = run() {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
```

### 错误消息设计

好的错误消息应该：
1. 说明发生了什么
2. 说明为什么发生
3. 建议如何解决

```
错误: 无法读取文件 'config.json'
原因: 文件不存在 (os error 2)
建议: 请检查文件路径是否正确
```

---

## 最佳实践总结

### 参数设计原则

| 原则 | 示例 |
|------|------|
| 主要输入用位置参数 | `confconv convert config.json` |
| 可选配置用选项 | `--output`, `--format` |
| 常用开关用短选项 | `-v`, `-o`, `-p` |
| 布尔功能用标志 | `--verbose`, `--pretty` |
| 互斥选项声明约束 | `conflicts_with` |

### 帮助信息优化

```rust
/// 简短描述（一行）
///
/// 详细描述（多行，在 --help 中完整显示）
///
/// 示例：
///   confconv convert config.json --to yaml
#[command(alias = "c")]
Convert { ... }
```

### 模块 pub use 模式

```rust
// commands/mod.rs
mod convert;
mod validate;
mod format;

pub use convert::run as convert;
pub use validate::run as validate;
pub use format::run as format;

// main.rs
use commands::{convert, validate, format};
```

---

## 完整代码

本章代码较多，请参考 `confconv/` 目录下的完整实现。

关键文件：
- `src/cli.rs`：完整的 CLI 定义
- `src/commands/mod.rs`：命令模块组织
- `src/commands/convert.rs`：转换命令实现
- `src/error.rs`：错误处理

---

## 运行效果

```bash
# 管道输入
echo '{"name":"test"}' | confconv convert --from json --to yaml
```

输出：
```yaml
name: test
```

```bash
# 完整的帮助信息
confconv --help
```

```bash
# 子命令帮助
confconv convert --help
```

```bash
# 验证后格式化
confconv validate config.json && confconv fmt config.json -w
```

---

## 要点回顾

1. **项目结构**：commands 模块化，职责分离
2. **管道支持**：`-` 表示标准输入
3. **错误处理**：友好的错误消息
4. **退出码**：0 成功，非 0 失败
5. **用户体验**：帮助信息、别名、默认值

---

## 总结

恭喜！你已经完成了整个教程。现在你掌握了：

| 章节 | 学到的技能 |
|------|-----------|
| 第1章 | CLI 基础、Parser、位置参数 |
| 第2章 | 选项、标志、默认值 |
| 第3章 | Serde 序列化、模块拆分 |
| 第4章 | 子命令、别名、全局参数 |
| 第5章 | ValueEnum、参数验证、互斥 |
| 第6章 | 生产级结构、用户体验 |

你现在可以：
- 用 clap 构建专业的 CLI 工具
- 用 serde 处理多种数据格式
- 组织生产级的 Rust 项目

继续探索 Rust 生态吧！
