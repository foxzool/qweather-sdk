use std::collections::BTreeMap;
use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{
    api::{decode_datetime, deserialize_option_number_from_empty_string, Refer},
    client::QWeatherClient,
    APIResult,
};

impl QWeatherClient {
    /// 台风预报
    ///
    /// 台风预报API提供全球主要海洋流域的台风预测位置、等级、气压、风速等。
    ///
    /// # 参数
    ///
    /// * storm_id : 需要查询的台风ID，StormID可通过台风查询API获取。例如 stormid=NP2018
    pub async fn storm_forecast(&self, storm_id: &str) -> APIResult<StormForecastResponse> {
        let url = "https://api.qweather.com/v7/tropical/storm-forecast".to_string();
        let mut params = BTreeMap::new();
        params.insert("stormid".to_string(), storm_id.to_string());

        self.request_api(url, params).await
    }
}

/// 台风预报返回值
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StormForecastResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    /// 台风预报
    pub forecast: Vec<StormForecast>,
    /// 数据来源
    pub refer: Refer,
}

/// 台风预报
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StormForecast {
    /// 台风预报时间
    #[serde(deserialize_with = "decode_datetime")]
    pub fx_time: DateTime<FixedOffset>,
    /// 台风所处纬度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub lat: f64,
    /// 台风所处经度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub lon: f64,
    /// 台风类型
    pub type_: String,
    /// 台风中心气压
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pressure: f64,
    /// 台风附近最大风速
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind_speed: f64,
    /// 台风移动速度
    #[serde(deserialize_with = "deserialize_option_number_from_empty_string")]
    pub move_speed: Option<f64>,
    /// 台风移动方位
    pub move_dir: String,
    /// 台风移动方位360度方向
    pub move_360: String,
}

#[test]
fn test_store_forecast() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2021-07-27T03:00+00:00",
  "fxLink": "https://www.qweather.com",
  "forecast": [
    {
      "fxTime": "2021-07-27T20:00+08:00",
      "lat": "31.7",
      "lon": "118.4",
      "type": "TS",
      "pressure": "990",
      "windSpeed": "18",
      "moveSpeed": "",
      "moveDir": "",
      "move360": ""
    },
    {
      "fxTime": "2021-07-28T08:00+08:00",
      "lat": "32.5",
      "lon": "117.4",
      "type": "TD",
      "pressure": "992",
      "windSpeed": "15",
      "moveSpeed": "",
      "moveDir": "",
      "move360": ""
    },
    {
      "fxTime": "2021-07-28T20:00+08:00",
      "lat": "33.1",
      "lon": "117.2",
      "type": "TD",
      "pressure": "992",
      "windSpeed": "15",
      "moveSpeed": "",
      "moveDir": "",
      "move360": ""
    },
    {
      "fxTime": "2021-07-29T08:00+08:00",
      "lat": "34.3",
      "lon": "117.2",
      "type": "TD",
      "pressure": "992",
      "windSpeed": "15",
      "moveSpeed": "",
      "moveDir": "",
      "move360": ""
    },
    {
      "fxTime": "2021-07-29T20:00+08:00",
      "lat": "36",
      "lon": "117.8",
      "type": "TD",
      "pressure": "992",
      "windSpeed": "15",
      "moveSpeed": "",
      "moveDir": "",
      "move360": ""
    },
    {
      "fxTime": "2021-07-30T08:00+08:00",
      "lat": "37.1",
      "lon": "118.7",
      "type": "TD",
      "pressure": "995",
      "windSpeed": "15",
      "moveSpeed": "",
      "moveDir": "",
      "move360": ""
    },
    {
      "fxTime": "2021-07-31T08:00+08:00",
      "lat": "38",
      "lon": "119.8",
      "type": "TD",
      "pressure": "995",
      "windSpeed": "15",
      "moveSpeed": "",
      "moveDir": "",
      "move360": ""
    }
  ],
  "refer": {
    "sources": [
      "NMC"
    ],
    "license": [
      "QWeather Developers License"
    ]
  }
}"#;

    let resp: StormForecastResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.forecast.len(), 7);
    assert_eq!(resp.forecast[0].type_, "TS");
    assert_eq!(resp.forecast[0].pressure, 990.0);
    assert_eq!(resp.forecast[0].wind_speed, 18.0);
    assert_eq!(resp.forecast[0].lat, 31.7);
    assert_eq!(resp.forecast[0].lon, 118.4);
    assert_eq!(resp.update_time.to_rfc3339(), "2021-07-27T03:00:00+00:00");
    assert_eq!(resp.fx_link, "https://www.qweather.com");
    assert_eq!(resp.refer.sources[0], "NMC");
    assert_eq!(resp.refer.license[0], "QWeather Developers License");
}
