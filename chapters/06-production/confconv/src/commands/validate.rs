//! validate 命令实现

use crate::error::{Error, Result};
use crate::format::Format;
use std::fs;

/// 执行验证命令
pub fn run(file: &str, format: Option<Format>, verbose: bool, quiet: bool) -> Result<()> {
    let format = format
        .or_else(|| Format::from_extension(file))
        .ok_or_else(|| Error::UnknownFormat {
            path: file.to_string(),
        })?;

    if verbose {
        eprintln!("验证格式: {}", format.name());
    }

    let content = fs::read_to_string(file).map_err(|e| Error::FileRead {
        path: file.to_string(),
        source: e,
    })?;

    // 尝试解析以验证语法
    match format {
        Format::Json => {
            let _: serde_json::Value = serde_json::from_str(&content).map_err(|e| Error::Parse {
                format: "JSON",
                source: e.to_string(),
            })?;
        }
        Format::Yaml => {
            let _: serde_json::Value = serde_yml::from_str(&content).map_err(|e| Error::Parse {
                format: "YAML",
                source: e.to_string(),
            })?;
        }
        Format::Toml => {
            let _: toml::Value = toml::from_str(&content).map_err(|e| Error::Parse {
                format: "TOML",
                source: e.to_string(),
            })?;
        }
    }

    if !quiet {
        println!("✓ {} 语法正确 ({})", file, format.name());
    }

    Ok(())
}
