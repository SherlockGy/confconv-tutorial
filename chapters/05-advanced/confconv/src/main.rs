//! confconv - 配置文件格式转换工具
//!
//! 第 5 章：类型安全与参数验证

mod cli;
mod convert;
mod format;

use clap::Parser;
use cli::{Cli, Commands};
use format::Format;
use std::fs;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Convert {
            input,
            output,
            format,
            pretty,
        } => handle_convert(&input, output.as_deref(), format, pretty, cli.verbose, cli.quiet),

        Commands::Validate { file, format } => {
            handle_validate(&file, format, cli.verbose, cli.quiet)
        }

        Commands::Format {
            file,
            indent,
            write,
        } => handle_format(&file, indent, write, cli.verbose, cli.quiet),
    };

    if let Err(e) = result {
        eprintln!("错误: {}", e);
        std::process::exit(1);
    }
}

fn handle_convert(
    input: &str,
    output: Option<&str>,
    to: Format,  // 现在是类型安全的 Format，不是 String
    pretty: bool,
    verbose: bool,
    _quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let from = Format::from_extension(input)
        .ok_or_else(|| format!("无法从文件扩展名推断格式: {}", input))?;

    if verbose {
        println!("源格式: {:?}", from);
        println!("目标格式: {:?}", to);
    }

    let content = fs::read_to_string(input)?;
    let result = convert::convert(&content, from, to, pretty)?;

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

fn handle_validate(
    file: &str,
    format: Option<Format>,  // 可选的 Format，不是 Option<String>
    verbose: bool,
    quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let format = match format {
        Some(f) => f,
        None => Format::from_extension(file)
            .ok_or_else(|| format!("无法从文件扩展名推断格式: {}", file))?,
    };

    if verbose {
        println!("验证格式: {:?}", format);
    }

    let content = fs::read_to_string(file)?;
    convert::validate(&content, format)?;

    if !quiet {
        println!("✓ {} 语法正确", file);
    }
    Ok(())
}

fn handle_format(
    file: &str,
    indent: u8,  // 已验证在 1-8 范围内
    write: bool,
    verbose: bool,
    _quiet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let format = Format::from_extension(file)
        .ok_or_else(|| format!("无法从文件扩展名推断格式: {}", file))?;

    if verbose {
        println!("格式: {:?}", format);
        println!("缩进: {} 空格", indent);
    }

    let content = fs::read_to_string(file)?;
    let result = convert::format_file(&content, format, indent)?;

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
