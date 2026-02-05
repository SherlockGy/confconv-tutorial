//! confconv - 配置文件格式转换工具
//!
//! 第 4 章：使用子命令组织功能
//!
//! ## 子命令
//! - convert (c): 格式转换
//! - validate (v): 语法验证
//! - format (fmt): 格式化

mod cli;
mod convert;
mod format;

use clap::Parser;
use cli::{Cli, Commands};
use format::Format;
use std::fs;

fn main() {
    let cli = Cli::parse();

    // 使用 match 处理不同的子命令
    // Rust 编译器会确保我们处理了所有可能的命令
    let result = match cli.command {
        Commands::Convert {
            input,
            output,
            format,
            pretty,
        } => handle_convert(&input, output.as_deref(), &format, pretty, cli.verbose),

        Commands::Validate { file, format } => {
            handle_validate(&file, format.as_deref(), cli.verbose)
        }

        Commands::Format {
            file,
            indent,
            write,
        } => handle_format(&file, indent, write, cli.verbose),
    };

    // 统一的错误处理
    if let Err(e) = result {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

// ============================================================================
// 命令处理函数
// ============================================================================

/// 处理 convert 命令
fn handle_convert(
    input: &str,
    output: Option<&str>,
    to_format: &str,
    pretty: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 解析目标格式
    let to = Format::from_str(to_format).ok_or_else(|| {
        format!(
            "不支持的目标格式 '{}'\n支持的格式: json, yaml, toml",
            to_format
        )
    })?;

    // 从扩展名推断源格式
    let from = Format::from_extension(input)
        .ok_or_else(|| format!("无法从文件扩展名推断格式: {}", input))?;

    if verbose {
        println!("源格式: {:?}", from);
        println!("目标格式: {:?}", to);
    }

    // 读取文件
    let content = fs::read_to_string(input)?;

    // 执行转换
    let result = convert::convert(&content, from, to, pretty)?;

    // 输出结果
    match output {
        Some(path) => {
            fs::write(path, &result)?;
            if verbose {
                println!("已写入: {}", path);
            }
        }
        None => println!("{}", result),
    }

    Ok(())
}

/// 处理 validate 命令
fn handle_validate(
    file: &str,
    format_str: Option<&str>,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 确定格式
    let format = match format_str {
        Some(s) => Format::from_str(s)
            .ok_or_else(|| format!("不支持的格式 '{}'\n支持的格式: json, yaml, toml", s))?,
        None => {
            Format::from_extension(file).ok_or_else(|| format!("无法从文件扩展名推断格式: {}", file))?
        }
    };

    if verbose {
        println!("验证格式: {:?}", format);
    }

    // 读取并验证
    let content = fs::read_to_string(file)?;
    convert::validate(&content, format)?;

    println!("✓ {} 语法正确", file);
    Ok(())
}

/// 处理 format 命令
fn handle_format(
    file: &str,
    indent: usize,
    write: bool,
    verbose: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    // 从扩展名推断格式
    let format =
        Format::from_extension(file).ok_or_else(|| format!("无法从文件扩展名推断格式: {}", file))?;

    if verbose {
        println!("格式: {:?}", format);
        println!("缩进: {} 空格", indent);
    }

    // 读取并格式化
    let content = fs::read_to_string(file)?;
    let result = convert::format_file(&content, format, indent)?;

    // 输出结果
    if write {
        fs::write(file, &result)?;
        if verbose {
            println!("已更新: {}", file);
        }
    } else {
        println!("{}", result);
    }

    Ok(())
}
