//! confconv - 配置文件格式转换工具
//!
//! 第 2 章：选项与标志
//! 学习如何定义 -o/--output 选项、--pretty 标志等

use clap::Parser;

// ============================================================================
// CLI 结构体定义
// ============================================================================

/// 配置文件格式转换工具
///
/// 支持在 JSON、YAML、TOML 之间互相转换
#[derive(Parser)]
#[command(name = "confconv")]
#[command(version = "0.1.0")]
#[command(about = "配置文件格式转换工具")]
struct Cli {
    // ------------------------------------------------------------------------
    // 位置参数（Positional Argument）
    // ------------------------------------------------------------------------

    /// 输入文件路径
    ///
    /// 支持 .json, .yaml, .yml, .toml 扩展名
    // 没有 #[arg(...)] 属性的非 Option 类型是必需的位置参数
    input: String,

    // ------------------------------------------------------------------------
    // 选项参数（Options）
    // ------------------------------------------------------------------------

    /// 输出文件路径
    ///
    /// 不指定则输出到标准输出（stdout）
    // #[arg(short, long)] 同时启用短选项和长选项
    // short 默认取首字母 'o'，long 默认取字段名 "output"
    // Option<String> 表示这个参数是可选的
    #[arg(short, long)]
    output: Option<String>,

    /// 目标格式
    ///
    /// 支持: json, yaml, toml
    // short = 't' 手动指定短选项（因为 'f' 可能与 --from 冲突）
    // long = "to" 手动指定长选项名
    // default_value 当用户不指定时使用的默认值
    #[arg(short = 't', long = "to", default_value = "json")]
    format: String,

    // ------------------------------------------------------------------------
    // 标志（Flags）
    // ------------------------------------------------------------------------

    /// 美化输出（带缩进和换行）
    // bool 类型自动成为标志
    // 使用 --pretty 或 -p 时，值为 true
    // 不使用时，值为 false
    #[arg(short, long)]
    pretty: bool,

    /// 显示详细信息
    // -v 和 --verbose
    #[arg(short, long)]
    verbose: bool,
}

// ============================================================================
// 程序入口
// ============================================================================

fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    // 如果开启了 verbose，打印所有参数信息
    // 这对于调试和理解程序行为很有帮助
    if cli.verbose {
        println!("=== 参数信息 ===");
        println!("输入文件: {}", cli.input);
        // {:?} 是 Debug 格式，会显示 Some("xxx") 或 None
        println!("输出文件: {:?}", cli.output);
        println!("目标格式: {}", cli.format);
        println!("美化输出: {}", cli.pretty);
        println!("================\n");
    }

    // 处理可选参数的方式：使用 match 或 if let
    // 这是 Rust 处理 Option 的惯用方式，避免了 null pointer 问题
    match &cli.output {
        Some(path) => println!("将写入文件: {}", path),
        None => println!("将输出到标准输出"),
    }

    println!("\n功能开发中...");
}

// ============================================================================
// 与 Java 的对比说明
// ============================================================================
//
// Java 中使用 picocli 的方式：
//
// ```java
// @Command(name = "confconv")
// class App implements Runnable {
//     @Option(names = {"-o", "--output"})
//     private String output;  // null 表示未指定
//
//     @Option(names = {"-p", "--pretty"})
//     private boolean pretty;  // false 表示未指定
//
//     public void run() {
//         if (output != null) {
//             System.out.println("写入: " + output);
//         }
//     }
// }
// ```
//
// Rust 的不同之处：
// 1. 使用 Option<String> 而非 nullable String
//    - Java: output == null
//    - Rust: output == None
//
// 2. 编译器强制处理 None 的情况
//    - Java: 可能忘记检查 null 导致 NullPointerException
//    - Rust: 必须显式处理 Option，否则编译错误
//
// 3. 属性语法
//    - Java: @Option(names = {"-o", "--output"})
//    - Rust: #[arg(short, long)]
