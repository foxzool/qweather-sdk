use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};

pub mod geo;
pub mod grid_weather;
pub mod minutely;
pub mod warning;
pub mod weather;
pub mod indices;
pub mod air_quality;

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

/// 数据来源
#[derive(Deserialize, Serialize, Debug)]
pub struct Refer {
    /// 原始数据来源，或数据源说明，可能为空
    pub sources: Vec<String>,
    /// 数据许可或版权声明，可能为空
    pub license: Vec<String>,
}
