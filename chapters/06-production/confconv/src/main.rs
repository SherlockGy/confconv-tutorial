//! confconv - 配置文件格式转换工具
//!
//! 第 6 章：生产级结构
//!
//! ## 功能
//! - convert: 格式转换
//! - validate: 语法验证
//! - format: 格式化

mod cli;
mod commands;
mod error;
mod format;

use clap::Parser;
use cli::{Cli, Commands};

fn main() {
    // 解析命令行参数
    let cli = Cli::parse();

    // 执行对应的命令
    let result = match cli.command {
        Commands::Convert {
            input,
            output,
            from,
            to,
            pretty,
        } => commands::convert(&input, output.as_deref(), from, to, pretty, cli.verbose),

        Commands::Validate { file, format } => {
            commands::validate(&file, format, cli.verbose, cli.quiet)
        }

        Commands::Format {
            file,
            indent,
            write,
        } => commands::format(&file, indent, write, cli.verbose),
    };

    // 处理错误
    if let Err(e) = result {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}
