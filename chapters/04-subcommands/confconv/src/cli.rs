//! CLI 定义模块
//!
//! 第 4 章：使用子命令组织功能
//!
//! 本模块定义了 confconv 的命令行接口结构，包括：
//! - 主命令及全局参数
//! - 三个子命令：convert, validate, format

use clap::{Parser, Subcommand};

// ============================================================================
// 主命令
// ============================================================================

/// 配置文件格式转换工具
///
/// 支持在 JSON、YAML、TOML 之间互相转换
#[derive(Parser)]
#[command(name = "confconv")]
#[command(version = "0.1.0")]
#[command(about = "配置文件格式转换工具")]
// arg_required_else_help: 如果用户没有提供任何参数，显示帮助信息
// 这比默认的错误信息更友好
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// 显示详细信息
    // global = true: 这个参数可以放在子命令前或后
    // 例如: confconv --verbose convert 或 confconv convert --verbose
    #[arg(short, long, global = true)]
    pub verbose: bool,

    // subcommand: 声明这个字段是子命令
    #[command(subcommand)]
    pub command: Commands,
}

// ============================================================================
// 子命令枚举
// ============================================================================

/// 可用的子命令
///
/// 使用枚举来定义子命令是 Clap 的推荐方式：
/// - 每个枚举变体是一个子命令
/// - 变体的字段是该子命令的参数
/// - 通过 match 来处理不同的命令
#[derive(Subcommand)]
pub enum Commands {
    /// 转换配置文件格式
    ///
    /// 将配置文件从一种格式转换为另一种格式。
    /// 支持 JSON、YAML、TOML 之间的互相转换。
    // alias: 命令的别名，方便快速输入
    #[command(alias = "c")]
    Convert {
        /// 输入文件路径
        input: String,

        /// 输出文件路径（不指定则输出到标准输出）
        #[arg(short, long)]
        output: Option<String>,

        /// 目标格式 (json/yaml/toml)
        // 注意：这里用 "to" 而不是 "format"，避免与子命令 format 混淆
        #[arg(short = 't', long = "to")]
        format: String,

        /// 美化输出（带缩进和换行）
        #[arg(short, long)]
        pretty: bool,
    },

    /// 验证配置文件语法
    ///
    /// 检查配置文件是否是有效的 JSON/YAML/TOML 格式。
    /// 不进行转换，只检查语法正确性。
    #[command(alias = "v")]
    Validate {
        /// 配置文件路径
        file: String,

        /// 指定格式（不指定则从扩展名推断）
        #[arg(short, long)]
        format: Option<String>,
    },

    /// 格式化（美化）配置文件
    ///
    /// 重新格式化配置文件，添加适当的缩进和换行。
    /// 可以原地修改文件或输出到标准输出。
    #[command(alias = "fmt")]
    Format {
        /// 配置文件路径
        file: String,

        /// 缩进空格数（默认 2）
        #[arg(short, long, default_value = "2")]
        indent: usize,

        /// 原地修改文件（不指定则输出到标准输出）
        // 使用 'w' 作为短选项，类似 rustfmt
        #[arg(short = 'w', long)]
        write: bool,
    },
}

// ============================================================================
// 与 Java 的对比说明
// ============================================================================
//
// Java (picocli) 的子命令定义需要多个类：
//
// ```java
// @Command(name = "confconv",
//          subcommands = {ConvertCommand.class, ValidateCommand.class, FormatCommand.class})
// class App implements Runnable {
//     @Option(names = {"-v", "--verbose"})
//     boolean verbose;
//
//     public void run() { }
// }
//
// @Command(name = "convert", aliases = {"c"})
// class ConvertCommand implements Runnable {
//     @Parameters(index = "0")
//     String input;
//
//     @Option(names = {"-t", "--to"})
//     String format;
//
//     public void run() { /* 实现 */ }
// }
// ```
//
// Rust (clap) 的优势：
// 1. 单个枚举定义所有子命令，代码更集中
// 2. 参数定义和命令定义在一起，不需要单独的类
// 3. 通过 match 处理命令，编译器确保处理所有分支
