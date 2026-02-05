//! 配置文件格式定义

use clap::ValueEnum;

/// 支持的配置文件格式
#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum Format {
    /// JSON 格式
    Json,
    /// YAML 格式
    Yaml,
    /// TOML 格式
    Toml,
}

impl Format {
    /// 从文件扩展名推断格式
    pub fn from_extension(path: &str) -> Option<Self> {
        let ext = path.rsplit('.').next()?.to_lowercase();
        match ext.as_str() {
            "json" => Some(Format::Json),
            "yaml" | "yml" => Some(Format::Yaml),
            "toml" => Some(Format::Toml),
            _ => None,
        }
    }

    /// 获取格式名称
    pub fn name(&self) -> &'static str {
        match self {
            Format::Json => "JSON",
            Format::Yaml => "YAML",
            Format::Toml => "TOML",
        }
    }
}
