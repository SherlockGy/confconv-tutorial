//! 配置文件格式定义
//!
//! 第 5 章：添加 ValueEnum，让 Format 成为类型安全的命令行参数

use clap::ValueEnum;

/// 支持的配置文件格式
///
/// 使用 `#[derive(ValueEnum)]` 让枚举可以直接作为命令行参数。
/// Clap 会自动：
/// - 解析字符串到枚举值（不区分大小写）
/// - 在帮助信息中显示可选值
/// - 对无效值自动报错
#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum Format {
    /// JSON 格式 - 最通用的数据交换格式
    Json,
    /// YAML 格式 - 人类可读性好，常用于配置
    Yaml,
    /// TOML 格式 - Rust 生态首选，Cargo.toml 使用的格式
    Toml,
}

impl Format {
    /// 从文件扩展名推断格式
    ///
    /// 这个方法仍然需要保留，因为有些命令需要从文件名推断格式
    pub fn from_extension(path: &str) -> Option<Self> {
        let ext = path.rsplit('.').next()?.to_lowercase();
        match ext.as_str() {
            "json" => Some(Format::Json),
            "yaml" | "yml" => Some(Format::Yaml),
            "toml" => Some(Format::Toml),
            _ => None,
        }
    }

    /// 获取格式的默认文件扩展名
    #[allow(dead_code)]
    pub fn extension(&self) -> &'static str {
        match self {
            Format::Json => "json",
            Format::Yaml => "yaml",
            Format::Toml => "toml",
        }
    }
}
