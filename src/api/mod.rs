use std::{fmt::Display, str::FromStr};

use chrono::{DateTime, FixedOffset};
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use serde_json::Value;

pub mod air_quality;
pub mod geo;
pub mod grid_weather;
pub mod indices;
pub mod minutely;
pub mod tropical_cyclone;
pub mod warning;
pub mod weather;

pub fn decode_datetime<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = DateTime::<FixedOffset>::parse_from_str(&s, "%Y-%m-%dT%H:%M%z").unwrap();
    Ok(dt)
}

pub fn option_decode_datetime<'de, D>(
    deserializer: D,
) -> Result<Option<DateTime<FixedOffset>>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    if s.is_empty() {
        Ok(None)
    } else {
        let dt = DateTime::<FixedOffset>::parse_from_str(&s, "%Y-%m-%dT%H:%M%z").unwrap();
        Ok(Some(dt))
    }
}
/// API响应
#[derive(Debug)]
pub enum APIResponse<T> {
    Success(T),
    Error(String),
}

/// 数据来源
#[derive(Deserialize, Serialize, Debug)]
pub struct Refer {
    /// 原始数据来源，或数据源说明，可能为空
    pub sources: Vec<String>,
    /// 数据许可或版权声明，可能为空
    pub license: Vec<String>,
}

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
        NumericOrNull::Str(s) => match s {
            "" => Ok(None),
            _ => T::from_str(s).map(Some).map_err(Error::custom),
        },
        NumericOrNull::FromStr(i) => Ok(Some(i)),
        NumericOrNull::Null => Ok(None),
        NumericOrNull::SerdeString(value) => match value {
            Value::String(s) => match s.as_str() {
                "" => Ok(None),
                _ => T::from_str(s.as_str()).map(Some).map_err(Error::custom),
            },
            _ => Err(Error::custom(value)),
        },
    }
}
