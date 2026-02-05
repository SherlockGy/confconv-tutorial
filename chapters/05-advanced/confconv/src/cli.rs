//! CLI 定义模块
//!
//! 第 5 章：使用 ValueEnum 和高级参数验证
//!
//! ## 本章改进
//! - Format 参数从 String 改为类型安全的枚举
//! - 添加互斥参数（verbose/quiet）
//! - 缩进参数添加范围验证

use clap::{Parser, Subcommand};
use crate::format::Format;

// ============================================================================
// 主命令
// ============================================================================

/// 配置文件格式转换工具
#[derive(Parser)]
#[command(name = "confconv")]
#[command(version = "0.1.0")]
#[command(about = "配置文件格式转换工具")]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// 显示详细信息
    // conflicts_with: 与 quiet 互斥，不能同时使用
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub verbose: bool,

    /// 安静模式（只输出结果，不显示提示信息）
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

// ============================================================================
// 子命令
// ============================================================================

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

        /// 目标格式
        // 现在是类型安全的 Format 枚举！
        // Clap 会自动：
        // - 只接受 json/yaml/toml
        // - 在帮助信息中显示可选值
        // - 对无效值自动报错
        #[arg(short = 't', long = "to")]
        format: Format,

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
        format: Option<Format>,
    },

    /// 格式化配置文件
    #[command(alias = "fmt")]
    Format {
        /// 配置文件路径
        file: String,

        /// 缩进空格数（1-8）
        // value_parser: 自定义验证器
        // clap::value_parser!(u8).range(1..=8) 限制值必须在 1-8 之间
        #[arg(
            short,
            long,
            default_value = "2",
            value_parser = clap::value_parser!(u8).range(1..=8)
        )]
        indent: u8,

        /// 原地修改文件
        #[arg(short = 'w', long)]
        write: bool,
    },
}
