//! 格式转换模块
//!
//! 负责在 JSON、YAML、TOML 之间转换
//!
//! ## 设计思路
//!
//! 使用 serde_json::Value 作为中间格式：
//! 1. 将源格式解析为 Value
//! 2. 将 Value 序列化为目标格式
//!
//! 这种设计简单且灵活，不需要知道具体的数据结构。

use std::error::Error;

// ============================================================================
// Format 枚举
// ============================================================================

/// 支持的配置文件格式
///
/// 目前支持三种最常用的配置格式：
/// - JSON: 最通用的数据交换格式
/// - YAML: 人类可读性好，常用于配置文件
/// - TOML: Rust 生态首选，Cargo.toml 就是这种格式
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
        // to_lowercase() 使解析不区分大小写
        match s.to_lowercase().as_str() {
            "json" => Some(Format::Json),
            "yaml" | "yml" => Some(Format::Yaml),  // yml 是 yaml 的常见缩写
            "toml" => Some(Format::Toml),
            _ => None,  // 不支持的格式返回 None
        }
    }

    /// 从文件扩展名推断格式
    ///
    /// # 示例
    /// ```
    /// let format = Format::from_extension("config.yaml");
    /// assert_eq!(format, Some(Format::Yaml));
    /// ```
    pub fn from_extension(path: &str) -> Option<Self> {
        // rsplit('.') 从右边开始分割，取第一个就是扩展名
        // 例如: "config.backup.json" -> "json"
        let ext = path.rsplit('.').next()?.to_lowercase();
        Self::from_str(&ext)
    }
}

// ============================================================================
// 转换函数
// ============================================================================

/// 执行格式转换
///
/// # 参数
/// - `input`: 输入内容（源格式的字符串）
/// - `from`: 源格式
/// - `to`: 目标格式
/// - `pretty`: 是否美化输出
///
/// # 返回
/// - `Ok(String)`: 转换后的内容
/// - `Err(...)`: 转换失败的错误信息
///
/// # 设计说明
/// 使用 `Box<dyn Error>` 作为错误类型是一个简化的做法，
/// 适合小型项目。生产级项目可能会使用自定义错误类型。
pub fn convert(
    input: &str,
    from: Format,
    to: Format,
    pretty: bool,
) -> Result<String, Box<dyn Error>> {
    // ========================================================================
    // 第一步：解析为通用的 JSON Value
    // ========================================================================
    // serde_json::Value 是一个枚举，可以表示任意 JSON 数据：
    // - Value::Null
    // - Value::Bool(bool)
    // - Value::Number(Number)
    // - Value::String(String)
    // - Value::Array(Vec<Value>)
    // - Value::Object(Map<String, Value>)
    //
    // 我们用它作为所有格式的"中间表示"
    let value: serde_json::Value = match from {
        Format::Json => {
            // JSON → Value: 直接解析
            serde_json::from_str(input)?
        }
        Format::Yaml => {
            // YAML → Value: serde_yml 可以直接解析为 serde_json::Value
            // 因为它们都实现了 Serde 的 Deserialize trait
            serde_yml::from_str(input)?
        }
        Format::Toml => {
            // TOML → Value: 需要两步
            // 1. 先解析为 toml::Value
            // 2. 再转换为 serde_json::Value
            let toml_value: toml::Value = toml::from_str(input)?;
            serde_json::to_value(toml_value)?
        }
    };

    // ========================================================================
    // 第二步：序列化为目标格式
    // ========================================================================
    let output = match to {
        Format::Json => {
            if pretty {
                // to_string_pretty 添加缩进和换行
                serde_json::to_string_pretty(&value)?
            } else {
                // to_string 输出紧凑格式
                serde_json::to_string(&value)?
            }
        }
        Format::Yaml => {
            // YAML 本身就是人类可读的格式，没有"紧凑"版本
            serde_yml::to_string(&value)?
        }
        Format::Toml => {
            // Value → TOML 需要先转换为 toml::Value
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

/// 将 serde_json::Value 转换为 toml::Value
///
/// 这是一个辅助函数，通过 JSON 字符串作为中间格式进行转换
fn json_to_toml(json: &serde_json::Value) -> Result<toml::Value, Box<dyn Error>> {
    // 这里使用一个技巧：
    // 1. 将 JSON Value 序列化为 JSON 字符串
    // 2. 将 JSON 字符串反序列化为 TOML Value
    //
    // 这利用了 TOML 库也实现了 serde 的 Deserialize trait
    let json_str = serde_json::to_string(json)?;
    let toml_value: toml::Value = serde_json::from_str(&json_str)?;
    Ok(toml_value)
}

// ============================================================================
// 与 Java 的对比
// ============================================================================
//
// Java 中，不同格式需要不同的库和 API：
//
// ```java
// // JSON (Jackson)
// ObjectMapper jsonMapper = new ObjectMapper();
// JsonNode node = jsonMapper.readTree(jsonString);
//
// // YAML (Jackson + YAML)
// ObjectMapper yamlMapper = new ObjectMapper(new YAMLFactory());
// Object data = yamlMapper.readValue(yamlString, Object.class);
//
// // TOML (toml4j，完全不同的 API)
// Toml toml = new Toml().read(tomlString);
// Map<String, Object> map = toml.toMap();
// ```
//
// Rust + Serde 的优势：
// - 统一的 from_str/to_string API
// - 通过 trait 实现格式无关的序列化
// - 编译时类型检查
