use chrono::{DateTime, FixedOffset, NaiveDate};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use url::Url;

use crate::{
    api::{decode_datetime, Refer},
    client::QWeatherClient,
};

impl QWeatherClient {
    /// 格点实时天气
    ///
    /// 基于全球任意坐标的高精度实时天气，精确到3-5公里范围，包括：温度、湿度、大气压、天气状况、
    /// 风力、风向等。
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）。 例如 location=116.41,39.92
    pub async fn grid_weather_now(
        &self,
        location: &str,
    ) -> Result<GridWeatherNowResponse, reqwest::Error> {
        let url = format!("{}/v7/grid-weather/now", self.base_url);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request grid_weather_now {}", url);

        self.client.get(url).send().await?.json().await
    }

    /// 格点每日天气预报
    ///
    /// 基于全球任意坐标的高精度每日天气预报，精确到3-5公里范围，包括温度、湿度、大气压、天气状况、
    /// 风力、风向等。
    ///
    /// # Arguments
    ///
    /// * location (必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）。
    ///
    /// * day (必选)预报天数，取值3天或者7天。
    pub async fn grid_weather_daily_forecast(
        &self,
        location: &str,
        day: i32,
    ) -> Result<GridWeatherDailyForecastResponse, reqwest::Error> {
        let url = format!("{}/v7/grid-weather/{}d", self.base_url, day);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request grid_weather_now {}", url);

        self.client.get(url).send().await?.json().await
    }
}

/// 格点实时天气返回值
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
#[serde(tag = "now")]
pub struct GridWeatherNow {
    /// 数据观测时间
    #[serde(deserialize_with = "decode_datetime")]
    pub obs_time: DateTime<FixedOffset>,
    /// 温度，默认单位：摄氏度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub temp: f32,
    /// 天气状况的[图标代码](https://dev.qweather.com/docs/resource/icons/)，另请参考[天气图标项目](https://icons.qweather.com/)
    pub icon: String,
    /// 天气状况的文字描述，包括阴晴雨雪等天气状态的描述
    pub text: String,
    /// [风向](https://dev.qweather.com/docs/resource/wind-info/#wind-direction)360角度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind360: f32,
    /// [风向](https://dev.qweather.com/docs/resource/wind-info/#wind-direction)
    pub wind_dir: String,
    /// [风力等级](https://dev.qweather.com/docs/resource/wind-info/#wind-scale)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind_scale: f32,
    /// [风速](https://dev.qweather.com/docs/resource/wind-info/#wind-speed)，公里/小时
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind_speed: f32,
    /// 相对湿度，百分比数值
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub humidity: f32,
    /// 当前小时累计降水量，默认单位：毫米
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub precip: f32,
    /// 大气压强，默认单位：百帕
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pressure: f32,
    /// 云量，百分比数值。可能为空
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub cloud: Option<f32>,
    /// 露点温度。可能为空
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub dew: Option<f32>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridWeatherNowResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    ///  当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    pub now: GridWeatherNow,
    pub refer: Refer,
}

/// 格点每日天气预报
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridWeatherDailyForecast {
    /// 预报日期
    pub fx_date: NaiveDate,
    /// 预报当天最高温度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub temp_max: f32,
    /// 预报当天最低温度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub temp_min: f32,
    /// 预报白天天气状况的[图标代码](https://dev.qweather.com/docs/resource/icons/)，另请参考天气[图标项目](https://icons.qweather.com/)
    pub icon_day: String,
    /// 预报白天天气状况文字描述，包括阴晴雨雪等天气状态的描述
    pub text_day: String,
    /// 预报夜间天气状况的[图标代码](https://dev.qweather.com/docs/resource/icons/)，另请参考天气[图标项目](https://icons.qweather.com/)
    pub icon_night: String,
    /// 预报晚间天气状况文字描述，包括阴晴雨雪等天气状态的描述
    pub text_night: String,
    /// 预报白天[风向](https://dev.qweather.com/docs/resource/wind-info/#wind-direction)360角度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind360_day: f32,
    /// 预报白天[风向](https://dev.qweather.com/docs/resource/wind-info/#wind-direction)
    pub wind_dir_day: String,
    /// 预报白天[风力等级](https://dev.qweather.com/docs/resource/wind-info/#wind-scale)
    pub wind_scale_day: String,
    /// 预报白天[风速](https://dev.qweather.com/docs/resource/wind-info/#wind-speed)，公里/小时
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind_speed_day: f32,
    /// 预报晚间[风向](https://dev.qweather.com/docs/resource/wind-info/#wind-direction)360角度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind360_night: f32,
    /// 预报晚间[风向](https://dev.qweather.com/docs/resource/wind-info/#wind-direction)
    pub wind_dir_night: String,
    /// 预报晚间[风力等级](https://dev.qweather.com/docs/resource/wind-info/#wind-scale)
    pub wind_scale_night: String,
    /// 预报晚间[风速](https://dev.qweather.com/docs/resource/wind-info/#wind-speed)，公里/小时
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub wind_speed_night: f32,
    /// 预报当天总降水量，默认单位：毫米
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub precip: f32,
    /// 相对湿度，百分比数值
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub humidity: f32,
    /// 大气压强，默认单位：百帕
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pressure: f32,
}

/// 格点每日天气预报返回数据
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridWeatherDailyForecastResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    ///  当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    pub daily: Vec<GridWeatherDailyForecast>,
    pub refer: Refer,
}

#[test]
fn test_grid_weather_now() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2021-12-16T18:25+08:00",
  "fxLink": "https://www.qweather.com",
  "now": {
    "obsTime": "2021-12-16T10:00+00:00",
    "temp": "-1",
    "icon": "150",
    "text": "晴",
    "wind360": "287",
    "windDir": "西北风",
    "windScale": "2",
    "windSpeed": "10",
    "humidity": "27",
    "precip": "0.0",
    "pressure": "1021",
    "cloud": "0",
    "dew": "-17"
  },
  "refer": {
    "sources": [
      "QWeather"
    ],
    "license": [
      "QWeather Developers License"
    ]
  }
}"#;

    let resp = serde_json::from_str::<GridWeatherNowResponse>(json_data).unwrap();
    assert_eq!(resp.now.temp, -1.0);
}

#[test]
fn test_grid_weather_daily_forecast() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2021-12-16T18:30+08:00",
  "fxLink": "https://www.qweather.com",
  "daily": [
    {
      "fxDate": "2021-12-16",
      "tempMax": "2",
      "tempMin": "-7",
      "iconDay": "104",
      "iconNight": "154",
      "textDay": "阴",
      "textNight": "阴",
      "wind360Day": "344",
      "windDirDay": "西北风",
      "windScaleDay": "4-5",
      "windSpeedDay": "9",
      "wind360Night": "304",
      "windDirNight": "西北风",
      "windScaleNight": "4-5",
      "windSpeedNight": "6",
      "humidity": "36",
      "precip": "0.0",
      "pressure": "1026"
    },
    {
      "fxDate": "2021-12-17",
      "tempMax": "-1",
      "tempMin": "-8",
      "iconDay": "104",
      "iconNight": "154",
      "textDay": "阴",
      "textNight": "阴",
      "wind360Day": "28",
      "windDirDay": "东北风",
      "windScaleDay": "5-6",
      "windSpeedDay": "15",
      "wind360Night": "55",
      "windDirNight": "东北风",
      "windScaleNight": "4-5",
      "windSpeedNight": "7",
      "humidity": "44",
      "precip": "0.0",
      "pressure": "1028"
    },
    {
      "fxDate": "2021-12-18",
      "tempMax": "4",
      "tempMin": "-8",
      "iconDay": "100",
      "iconNight": "150",
      "textDay": "晴",
      "textNight": "晴",
      "wind360Day": "10",
      "windDirDay": "北风",
      "windScaleDay": "5-6",
      "windSpeedDay": "17",
      "wind360Night": "48",
      "windDirNight": "东北风",
      "windScaleNight": "3-4",
      "windSpeedNight": "5",
      "humidity": "61",
      "precip": "0.0",
      "pressure": "1016"
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

    let resp = serde_json::from_str::<GridWeatherDailyForecastResponse>(json_data).unwrap();
    assert_eq!(resp.daily.len(), 3);
    assert_eq!(resp.daily[0].temp_max, 2.0);
    assert_eq!(resp.daily[1].temp_max, -1.0);
    assert_eq!(resp.daily[2].temp_max, 4.0);
}
