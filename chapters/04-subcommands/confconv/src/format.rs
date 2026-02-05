//! 配置文件格式定义
//!
//! 从 convert.rs 中提取出来，便于在多个模块中共享

/// 支持的配置文件格式
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Format {
    Json,
    Yaml,
    Toml,
}

impl Format {
    /// 从字符串解析格式
    ///
    /// # 示例
    /// ```
    /// let format = Format::from_str("yaml");
    /// assert_eq!(format, Some(Format::Yaml));
    /// ```
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "json" => Some(Format::Json),
            "yaml" | "yml" => Some(Format::Yaml),
            "toml" => Some(Format::Toml),
            _ => None,
        }
    }

    /// 从文件扩展名推断格式
    pub fn from_extension(path: &str) -> Option<Self> {
        let ext = path.rsplit('.').next()?.to_lowercase();
        Self::from_str(&ext)
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
