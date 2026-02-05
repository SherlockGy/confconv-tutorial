# Clap 完整教程：confconv

一个通过构建配置文件转换工具来学习 Rust Clap 的实践教程。

## 这是什么

本教程通过循序渐进地构建一个配置文件转换工具（confconv），帮助你掌握 Rust 命令行程序开发。

confconv 可以在 JSON、YAML、TOML 三种格式之间互相转换。

## 适合谁

- 有 Java 或其他语言背景的开发者
- 想学习 Rust CLI 开发的初学者
- 想了解 clap 和 serde 最佳实践的人

## 教程特点

- 循序渐进：从单文件到生产级结构
- 概念驱动：先讲 CLI 基础概念，再引入 clap 特性
- 代码演化：每章都有完整可运行的项目
- 中文注释：所有代码都有详细的中文注释

## 章节目录

| 章节 | 主题 | 学到什么 |
|------|------|----------|
| 01 | 初识命令行 | CLI 基础、位置参数、--help |
| 02 | 选项与标志 | -o/--output、bool 标志、默认值 |
| 03 | 引入 Serde | serde 设计哲学、格式转换 |
| 04 | 子命令 | convert/validate/format 命令 |
| 05 | 高级特性 | ValueEnum、参数验证、环境变量 |
| 06 | 生产级结构 | 模块化、错误处理、最佳实践 |

## 环境要求

- Rust 1.75+（推荐 1.93+）
- Cargo

## 如何使用

1. 按顺序阅读每章的 README.md
2. 进入对应章节的 `confconv/` 目录运行代码
3. 动手修改代码，观察效果

```bash
# 示例：运行第1章代码
cd chapters/01-hello-cli/confconv
cargo run -- test.json
```

## 最终效果

完成教程后，你将得到一个功能完整的配置转换工具：

```bash
# 基本转换
confconv convert config.json --to yaml

# 验证配置
confconv validate config.toml

# 格式化
confconv format config.json --indent 4 -w
```

## 版本信息

- Rust Edition: 2021
- clap: 4.5.x
- serde: 1.x

## 致谢

本教程的项目结构参考了 [SherlockGy/rp](https://github.com/SherlockGy/rp)。

---

*这是一个学习项目，欢迎提出改进建议。*
