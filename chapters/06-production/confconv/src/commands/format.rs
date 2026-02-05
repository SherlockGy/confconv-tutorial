//! format 命令实现

use crate::error::{Error, Result};
use crate::format::Format;
use std::fs;

/// 执行格式化命令
pub fn run(file: &str, indent: u8, write: bool, verbose: bool) -> Result<()> {
    let format = Format::from_extension(file).ok_or_else(|| Error::UnknownFormat {
        path: file.to_string(),
    })?;

    if verbose {
        eprintln!("格式: {}", format.name());
        eprintln!("缩进: {} 空格", indent);
    }

    let content = fs::read_to_string(file).map_err(|e| Error::FileRead {
        path: file.to_string(),
        source: e,
    })?;

    let result = format_content(&content, format, indent)?;

    if write {
        fs::write(file, &result).map_err(|e| Error::FileWrite {
            path: file.to_string(),
            source: e,
        })?;
        if verbose {
            eprintln!("已更新: {}", file);
        }
    } else {
        print!("{}", result);
    }

    Ok(())
}

/// 格式化内容
fn format_content(input: &str, format: Format, indent: u8) -> Result<String> {
    match format {
        Format::Json => {
            let value: serde_json::Value =
                serde_json::from_str(input).map_err(|e| Error::Parse {
                    format: "JSON",
                    source: e.to_string(),
                })?;

            let mut buf = Vec::new();
            let indent_str = " ".repeat(indent as usize).into_bytes();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_str);
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            serde::Serialize::serialize(&value, &mut ser).map_err(|e| Error::Convert {
                message: e.to_string(),
            })?;

            String::from_utf8(buf).map_err(|e| Error::Convert {
                message: e.to_string(),
            })
        }
        Format::Yaml => {
            let value: serde_json::Value = serde_yml::from_str(input).map_err(|e| Error::Parse {
                format: "YAML",
                source: e.to_string(),
            })?;
            serde_yml::to_string(&value).map_err(|e| Error::Convert {
                message: e.to_string(),
            })
        }
        Format::Toml => {
            let value: toml::Value = toml::from_str(input).map_err(|e| Error::Parse {
                format: "TOML",
                source: e.to_string(),
            })?;
            toml::to_string_pretty(&value).map_err(|e| Error::Convert {
                message: e.to_string(),
            })
        }
    }
}
