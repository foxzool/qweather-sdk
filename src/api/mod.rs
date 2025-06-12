use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, FixedOffset, Utc};
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub mod air_quality;
pub mod geo;
pub mod grid_weather;
pub mod indices;
pub mod minutely;
pub mod tropical_cyclone;
pub mod utils;
pub mod warning;
pub mod weather;

/// 解码带时区的日期时间格式
pub fn decode_datetime<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    DateTime::<FixedOffset>::parse_from_str(&s, "%Y-%m-%dT%H:%M%z")
        .map_err(|e| Error::custom(format!("Failed to parse datetime '{}': {}", s, e)))
}

/// 解码ISO 8601格式的UTC日期时间
pub fn decode_iso8601<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: Deserializer<'de>,
{
    let iso8601_str = String::deserialize(deserializer)?;

    // 处理缺少秒数的情况
    let complete_date_str = if iso8601_str.ends_with('Z') {
        format!("{}:00Z", &iso8601_str[..iso8601_str.len() - 1])
    } else {
        iso8601_str.clone()
    };

    DateTime::parse_from_rfc3339(&complete_date_str)
        .map(|datetime| datetime.with_timezone(&Utc))
        .map_err(|e| {
            Error::custom(format!(
                "Failed to parse ISO 8601 string '{}': {}",
                iso8601_str, e
            ))
        })
}

/// 解码可选的日期时间，空字符串返回None
pub fn option_decode_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        DateTime::<FixedOffset>::parse_from_str(&s, "%Y-%m-%dT%H:%M%z")
            .map(Some)
            .map_err(|e| Error::custom(format!("Failed to parse datetime '{}': {}", s, e)))
    }
}

/// API响应封装类型
#[derive(Debug, Clone, PartialEq)]
pub enum APIResponse<T> {
    /// 成功响应
    Success(T),
    /// 错误响应
    Error(String),
}

impl<T> APIResponse<T> {
    /// 检查是否为成功响应
    pub fn is_success(&self) -> bool {
        matches!(self, APIResponse::Success(_))
    }

    /// 检查是否为错误响应
    pub fn is_error(&self) -> bool {
        matches!(self, APIResponse::Error(_))
    }

    /// 获取成功响应的值，如果是错误则panic
    pub fn unwrap(self) -> T {
        match self {
            APIResponse::Success(value) => value,
            APIResponse::Error(err) => panic!("Called unwrap on Error: {}", err),
        }
    }

    /// 获取成功响应的值，如果是错误则返回默认值
    pub fn unwrap_or(self, default: T) -> T {
        match self {
            APIResponse::Success(value) => value,
            APIResponse::Error(_) => default,
        }
    }
}

/// 数据来源信息
#[derive(Deserialize, Serialize, Debug, Clone, Default, PartialEq)]
pub struct Refer {
    /// 原始数据来源，或数据源说明，可能为空
    #[serde(default)]
    pub sources: Vec<String>,
    /// 数据许可或版权声明，可能为空
    #[serde(default)]
    pub license: Vec<String>,
}

/// 从空字符串或null值反序列化为Option<T>
/// 支持多种输入格式：空字符串、null、数值字符串、直接数值
pub fn deserialize_option_number_from_empty_string<'de, T, D>(
    deserializer: D,
) -> Result<Option<T>, D::Error>
where
    D: Deserializer<'de>,
    T: FromStr + serde::Deserialize<'de>,
    <T as FromStr>::Err: Display,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum NumericOrNull<'a, T> {
        Str(&'a str),
        FromStr(T),
        Null,
        SerdeString(Value),
    }

    match NumericOrNull::<T>::deserialize(deserializer)? {
        NumericOrNull::Str(s) => {
            if s.is_empty() {
                Ok(None)
            } else {
                T::from_str(s)
                    .map(Some)
                    .map_err(|e| Error::custom(format!("Failed to parse '{}': {}", s, e)))
            }
        }
        NumericOrNull::FromStr(value) => Ok(Some(value)),
        NumericOrNull::Null => Ok(None),
        NumericOrNull::SerdeString(value) => match value {
            Value::String(s) => {
                if s.is_empty() {
                    Ok(None)
                } else {
                    T::from_str(&s)
                        .map(Some)
                        .map_err(|e| Error::custom(format!("Failed to parse '{}': {}", s, e)))
                }
            }
            Value::Null => Ok(None),
            _ => Err(Error::custom(format!(
                "Expected string or null, got: {}",
                value
            ))),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response() {
        let success: APIResponse<i32> = APIResponse::Success(42);
        assert!(success.is_success());
        assert!(!success.is_error());
        assert_eq!(success.unwrap(), 42);

        let error: APIResponse<i32> = APIResponse::Error("test error".to_string());
        assert!(!error.is_success());
        assert!(error.is_error());
        assert_eq!(error.unwrap_or(0), 0);
    }

    #[test]
    fn test_refer_default() {
        let refer = Refer::default();
        assert!(refer.sources.is_empty());
        assert!(refer.license.is_empty());
    }
}
