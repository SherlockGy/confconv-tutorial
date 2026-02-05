//! convert 命令实现

use crate::error::{Error, Result};
use crate::format::Format;
use std::fs;
use std::io::{self, Read};

/// 执行转换命令
pub fn run(
    input: &str,
    output: Option<&str>,
    from: Option<Format>,
    to: Format,
    pretty: bool,
    verbose: bool,
) -> Result<()> {
    // 读取输入
    let (content, from_format) = if input == "-" {
        // 从标准输入读取
        let from = from.ok_or_else(|| Error::Convert {
            message: "从标准输入读取时必须指定 --from 参数".to_string(),
        })?;
        let mut content = String::new();
        io::stdin()
            .read_to_string(&mut content)
            .map_err(|e| Error::FileRead {
                path: "stdin".to_string(),
                source: e,
            })?;
        (content, from)
    } else {
        // 从文件读取
        let from = from
            .or_else(|| Format::from_extension(input))
            .ok_or_else(|| Error::UnknownFormat {
                path: input.to_string(),
            })?;
        let content = fs::read_to_string(input).map_err(|e| Error::FileRead {
            path: input.to_string(),
            source: e,
        })?;
        (content, from)
    };

    if verbose {
        eprintln!("源格式: {}", from_format.name());
        eprintln!("目标格式: {}", to.name());
    }

    // 执行转换
    let result = convert(&content, from_format, to, pretty)?;

    // 输出结果
    match output {
        Some(path) => {
            fs::write(path, &result).map_err(|e| Error::FileWrite {
                path: path.to_string(),
                source: e,
            })?;
            if verbose {
                eprintln!("已写入: {}", path);
            }
        }
        None => print!("{}", result),
    }

    Ok(())
}

/// 内部转换函数
fn convert(input: &str, from: Format, to: Format, pretty: bool) -> Result<String> {
    // 解析为 JSON Value
    let value: serde_json::Value = match from {
        Format::Json => serde_json::from_str(input).map_err(|e| Error::Parse {
            format: "JSON",
            source: e.to_string(),
        })?,
        Format::Yaml => serde_yml::from_str(input).map_err(|e| Error::Parse {
            format: "YAML",
            source: e.to_string(),
        })?,
        Format::Toml => {
            let toml_value: toml::Value = toml::from_str(input).map_err(|e| Error::Parse {
                format: "TOML",
                source: e.to_string(),
            })?;
            serde_json::to_value(toml_value).map_err(|e| Error::Convert {
                message: e.to_string(),
            })?
        }
    };

    // 序列化为目标格式
    let output = match to {
        Format::Json => {
            if pretty {
                serde_json::to_string_pretty(&value)
            } else {
                serde_json::to_string(&value)
            }
            .map_err(|e| Error::Convert {
                message: e.to_string(),
            })?
        }
        Format::Yaml => serde_yml::to_string(&value).map_err(|e| Error::Convert {
            message: e.to_string(),
        })?,
        Format::Toml => {
            let json_str = serde_json::to_string(&value).map_err(|e| Error::Convert {
                message: e.to_string(),
            })?;
            let toml_value: toml::Value =
                serde_json::from_str(&json_str).map_err(|e| Error::Convert {
                    message: e.to_string(),
                })?;
            if pretty {
                toml::to_string_pretty(&toml_value)
            } else {
                toml::to_string(&toml_value)
            }
            .map_err(|e| Error::Convert {
                message: e.to_string(),
            })?
        }
    };

    Ok(output)
}
