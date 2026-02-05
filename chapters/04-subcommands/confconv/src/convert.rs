//! 格式转换模块

use crate::format::Format;
use std::error::Error;

/// 执行格式转换
pub fn convert(
    input: &str,
    from: Format,
    to: Format,
    pretty: bool,
) -> Result<String, Box<dyn Error>> {
    // 解析为 JSON Value
    let value: serde_json::Value = match from {
        Format::Json => serde_json::from_str(input)?,
        Format::Yaml => serde_yml::from_str(input)?,
        Format::Toml => {
            let toml_value: toml::Value = toml::from_str(input)?;
            serde_json::to_value(toml_value)?
        }
    };

    // 序列化为目标格式
    let output = match to {
        Format::Json => {
            if pretty {
                serde_json::to_string_pretty(&value)?
            } else {
                serde_json::to_string(&value)?
            }
        }
        Format::Yaml => serde_yml::to_string(&value)?,
        Format::Toml => {
            let toml_value = json_to_toml(&value)?;
            if pretty {
                toml::to_string_pretty(&toml_value)?
            } else {
                toml::to_string(&toml_value)?
            }
        }
    };

    Ok(output)
}

/// 验证配置文件语法
pub fn validate(input: &str, format: Format) -> Result<(), Box<dyn Error>> {
    match format {
        Format::Json => {
            let _: serde_json::Value = serde_json::from_str(input)?;
        }
        Format::Yaml => {
            let _: serde_json::Value = serde_yml::from_str(input)?;
        }
        Format::Toml => {
            let _: toml::Value = toml::from_str(input)?;
        }
    }
    Ok(())
}

/// 格式化配置文件
pub fn format_file(input: &str, fmt: Format, indent: usize) -> Result<String, Box<dyn Error>> {
    match fmt {
        Format::Json => {
            let value: serde_json::Value = serde_json::from_str(input)?;
            // JSON 美化输出默认用 2 空格，这里我们需要自定义
            // 使用 serde_json 的 Formatter 来控制缩进
            let mut buf = Vec::new();
            let indent_str = " ".repeat(indent).into_bytes();
            let formatter = serde_json::ser::PrettyFormatter::with_indent(&indent_str);
            let mut ser = serde_json::Serializer::with_formatter(&mut buf, formatter);
            serde::Serialize::serialize(&value, &mut ser)?;
            Ok(String::from_utf8(buf)?)
        }
        Format::Yaml => {
            // YAML 本身就是美化格式
            let value: serde_json::Value = serde_yml::from_str(input)?;
            Ok(serde_yml::to_string(&value)?)
        }
        Format::Toml => {
            let value: toml::Value = toml::from_str(input)?;
            Ok(toml::to_string_pretty(&value)?)
        }
    }
}

fn json_to_toml(json: &serde_json::Value) -> Result<toml::Value, Box<dyn Error>> {
    let json_str = serde_json::to_string(json)?;
    let toml_value: toml::Value = serde_json::from_str(&json_str)?;
    Ok(toml_value)
}
