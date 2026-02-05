//! CLI 定义模块

use clap::{Parser, Subcommand};
use crate::format::Format;

/// 配置文件格式转换工具
///
/// 支持在 JSON、YAML、TOML 之间互相转换
#[derive(Parser)]
#[command(name = "confconv")]
#[command(author, version, about, long_about = None)]
#[command(arg_required_else_help = true)]
pub struct Cli {
    /// 显示详细信息
    #[arg(short, long, global = true, conflicts_with = "quiet")]
    pub verbose: bool,

    /// 安静模式
    #[arg(short, long, global = true, conflicts_with = "verbose")]
    pub quiet: bool,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// 转换配置文件格式
    ///
    /// 示例：
    ///   confconv convert config.json --to yaml
    ///   cat config.json | confconv convert --from json --to yaml
    #[command(alias = "c")]
    Convert {
        /// 输入文件路径（使用 - 表示标准输入）
        #[arg(default_value = "-")]
        input: String,

        /// 输出文件路径
        #[arg(short, long)]
        output: Option<String>,

        /// 源格式（从标准输入读取时必需）
        #[arg(short, long)]
        from: Option<Format>,

        /// 目标格式
        #[arg(short = 't', long = "to")]
        to: Format,

        /// 美化输出
        #[arg(short, long)]
        pretty: bool,
    },

    /// 验证配置文件语法
    #[command(alias = "v")]
    Validate {
        /// 配置文件路径
        file: String,

        /// 指定格式
        #[arg(short, long)]
        format: Option<Format>,
    },

    /// 格式化配置文件
    #[command(alias = "fmt")]
    Format {
        /// 配置文件路径
        file: String,

        /// 缩进空格数（1-8）
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
