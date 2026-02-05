//! 错误类型定义
//!
//! 生产级项目应该有清晰的错误类型，而不是到处用 Box<dyn Error>

use std::fmt;
use std::io;

/// confconv 错误类型
#[derive(Debug)]
pub enum Error {
    /// 文件读取错误
    FileRead { path: String, source: io::Error },
    /// 文件写入错误
    FileWrite { path: String, source: io::Error },
    /// 格式解析错误
    Parse { format: &'static str, source: String },
    /// 格式转换错误
    Convert { message: String },
    /// 无法推断格式
    UnknownFormat { path: String },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::FileRead { path, source } => {
                write!(f, "无法读取文件 '{}': {}", path, source)
            }
            Error::FileWrite { path, source } => {
                write!(f, "无法写入文件 '{}': {}", path, source)
            }
            Error::Parse { format, source } => {
                write!(f, "{} 解析失败: {}", format, source)
            }
            Error::Convert { message } => {
                write!(f, "转换失败: {}", message)
            }
            Error::UnknownFormat { path } => {
                write!(
                    f,
                    "无法从文件扩展名推断格式: {}\n支持的扩展名: .json, .yaml, .yml, .toml",
                    path
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// 结果类型别名
pub type Result<T> = std::result::Result<T, Error>;
