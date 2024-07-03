use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicDataResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    ///  当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 数据描述
    pub summary: Option<String>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    #[serde(flatten)]
    pub data: DataType,
    pub refer: Refer,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StaticDataResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    #[serde(flatten)]
    pub data: DataType,
    pub refer: Refer,
}

/// 数据来源
#[derive(Deserialize, Serialize, Debug)]
pub struct Refer {
    /// 原始数据来源，或数据源说明，可能为空
    pub sources: Vec<String>,
    /// 数据许可或版权声明，可能为空
    pub license: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DataType {
    Minutely { minutely: Vec<Minutely> },
}

pub fn decode_datetime<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = DateTime::<FixedOffset>::parse_from_str(&s, "%Y-%m-%dT%H:%M%z").unwrap();
    Ok(dt)
}

/// 分钟降水
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Minutely {
    /// 预报时间
    #[serde(deserialize_with = "decode_datetime")]
    pub fx_time: DateTime<FixedOffset>,
    /// 5分钟累计降水量，单位毫米
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub precip: f32,
    /// 降水类型：rain = 雨，snow = 雪
    #[serde(rename = "type")]
    pub type_: String,
}

#[cfg(test)]
mod test {
    use crate::model::DynamicDataResponse;

    #[test]
    fn test_minutely() {
        let json_data = r#"{
  "code": "200",
  "updateTime": "2021-12-16T18:55+08:00",
  "fxLink": "https://www.qweather.com",
  "summary": "95分钟后雨就停了",
  "minutely": [
    {
      "fxTime": "2021-12-16T18:55+08:00",
      "precip": "0.15",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:00+08:00",
      "precip": "0.23",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:05+08:00",
      "precip": "0.21",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:10+08:00",
      "precip": "0.17",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:15+08:00",
      "precip": "0.18",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:20+08:00",
      "precip": "0.24",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:25+08:00",
      "precip": "0.31",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:30+08:00",
      "precip": "0.37",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:35+08:00",
      "precip": "0.41",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:40+08:00",
      "precip": "0.43",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:45+08:00",
      "precip": "0.41",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:50+08:00",
      "precip": "0.36",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:55+08:00",
      "precip": "0.32",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:00+08:00",
      "precip": "0.27",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:05+08:00",
      "precip": "0.22",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:10+08:00",
      "precip": "0.17",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:15+08:00",
      "precip": "0.11",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:20+08:00",
      "precip": "0.06",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:25+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:30+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:35+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:40+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:45+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:50+08:00",
      "precip": "0.0",
      "type": "rain"
    }
  ],
  "refer": {
    "sources": [
      "QWeather"
    ],
    "license": [
      "QWeather Developers License"
    ]
  }
}"#;

        let resp = serde_json::from_str::<DynamicDataResponse>(json_data);
        assert!(resp.is_ok())
    }
}
