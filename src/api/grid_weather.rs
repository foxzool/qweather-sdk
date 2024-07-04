use std::collections::BTreeMap;
use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

use crate::{
    api::{decode_datetime, deserialize_option_number_from_empty_string, Refer},
    client::QWeatherClient,
    APIResult,
};

impl QWeatherClient {
    /// 格点实时天气
    ///
    /// 基于全球任意坐标的高精度实时天气，精确到3-5公里范围，包括：温度、湿度、大气压、天气状况、
    /// 风力、风向等。
    ///
    /// # Arguments
    ///
    /// * location (必选)需要查询地区的以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）。例如 location=116.41,39.92
    pub async fn grid_weather_now(&self, location: &str) -> APIResult<GridWeatherNowResponse> {
        let url = format!("{}/v7/grid-weather/now", self.get_api_host());

        let mut params = BTreeMap::new();
        params.insert("location".to_string(), location.to_string());

        self.request_api(url, params).await
    }

    /// 格点每日天气预报
    ///
    /// 基于全球任意坐标的高精度每日天气预报，精确到3-5公里范围，包括温度、湿度、大气压、天气状况、
    /// 风力、风向等。
    ///
    /// # Arguments
    ///
    /// * location (必选)需要查询地区的以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）。例如 location=116.41,39.92
    ///
    /// * day (必选)预报天数，取值3天或者7天。
    pub async fn grid_weather_daily_forecast(
        &self,
        location: &str,
        day: i32,
    ) -> APIResult<GridWeatherDailyForecastResponse> {
        let url = format!("{}/v7/grid-weather/{}d", self.get_api_host(), day);
        let mut params = BTreeMap::new();
        params.insert("location".to_string(), location.to_string());

        self.request_api(url, params).await
    }

    /// 格点逐小时天气预报
    ///
    /// 基于全球任意坐标的高精度逐小时天气预报，精确到3-5公里范围，包括温度、湿度、大气压、
    /// 天气状况、风力、风向等。
    ///
    /// # Arguments
    ///
    /// * location (必选)需要查询地区的以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）。例如 location=116.41,39.92。
    ///
    /// * hour (必选)预报小时数，取值24小时或者72小时。
    pub async fn grid_weather_hourly_forecast(
        &self,
        location: &str,
        hour: i32,
    ) -> APIResult<GridWeatherHourlyForecastResponse> {
        let url = format!("{}/v7/grid-weather/{}h", self.get_api_host(), hour);
        let mut params = BTreeMap::new();
        params.insert("location".to_string(), location.to_string());

        self.request_api(url, params).await
    }
}

/// 格点实时天气返回值
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
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
    #[serde(deserialize_with = "deserialize_option_number_from_empty_string")]
    pub cloud: Option<f32>,
    /// 露点温度。可能为空
    #[serde(deserialize_with = "deserialize_option_number_from_empty_string")]
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

/// 格点逐小时天气预报返回数据
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridWeatherHourlyForecastResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    ///  当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    pub hourly: Vec<GridWeatherHourlyForecast>,
    pub refer: Refer,
}

/// 格点每日天气预报
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GridWeatherHourlyForecast {
    /// 预报日期
    #[serde(deserialize_with = "decode_datetime")]
    pub fx_time: DateTime<FixedOffset>,
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
    #[serde(deserialize_with = "deserialize_option_number_from_empty_string")]
    pub cloud: Option<f32>,
    /// 露点温度。可能为空
    #[serde(deserialize_with = "deserialize_option_number_from_empty_string")]
    pub dew: Option<f32>,
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

#[test]
fn test_grid_weather_hourly_forecast() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2021-12-16T19:27+08:00",
  "fxLink": "https://www.qweather.com",
  "hourly": [
    {
      "fxTime": "2021-12-16T12:00+00:00",
      "temp": "-2",
      "icon": "150",
      "text": "晴",
      "wind360": "285",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "8",
      "humidity": "30",
      "precip": "0.0",
      "pressure": "1022",
      "cloud": "0",
      "dew": "-17"
    },
    {
      "fxTime": "2021-12-16T13:00+00:00",
      "temp": "-3",
      "icon": "150",
      "text": "晴",
      "wind360": "289",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "8",
      "humidity": "32",
      "precip": "0.0",
      "pressure": "1023",
      "cloud": "0",
      "dew": "-17"
    },
    {
      "fxTime": "2021-12-16T14:00+00:00",
      "temp": "-3",
      "icon": "150",
      "text": "晴",
      "wind360": "293",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "7",
      "humidity": "34",
      "precip": "0.0",
      "pressure": "1024",
      "cloud": "0",
      "dew": "-17"
    },
    {
      "fxTime": "2021-12-16T15:00+00:00",
      "temp": "-4",
      "icon": "150",
      "text": "晴",
      "wind360": "296",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "6",
      "humidity": "35",
      "precip": "0.0",
      "pressure": "1024",
      "cloud": "0",
      "dew": "-17"
    },
    {
      "fxTime": "2021-12-16T16:00+00:00",
      "temp": "-4",
      "icon": "150",
      "text": "晴",
      "wind360": "294",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "6",
      "humidity": "35",
      "precip": "0.0",
      "pressure": "1025",
      "cloud": "0",
      "dew": "-17"
    },
    {
      "fxTime": "2021-12-16T17:00+00:00",
      "temp": "-5",
      "icon": "150",
      "text": "晴",
      "wind360": "290",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "7",
      "humidity": "35",
      "precip": "0.0",
      "pressure": "1025",
      "cloud": "0",
      "dew": "-18"
    },
    {
      "fxTime": "2021-12-16T18:00+00:00",
      "temp": "-5",
      "icon": "150",
      "text": "晴",
      "wind360": "280",
      "windDir": "西风",
      "windScale": "2",
      "windSpeed": "8",
      "humidity": "33",
      "precip": "0.0",
      "pressure": "1025",
      "cloud": "0",
      "dew": "-19"
    },
    {
      "fxTime": "2021-12-16T19:00+00:00",
      "temp": "-6",
      "icon": "154",
      "text": "阴",
      "wind360": "277",
      "windDir": "西风",
      "windScale": "2",
      "windSpeed": "9",
      "humidity": "30",
      "precip": "0.0",
      "pressure": "1026",
      "cloud": "6",
      "dew": "-20"
    },
    {
      "fxTime": "2021-12-16T20:00+00:00",
      "temp": "-6",
      "icon": "150",
      "text": "晴",
      "wind360": "283",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "9",
      "humidity": "29",
      "precip": "0.0",
      "pressure": "1026",
      "cloud": "0",
      "dew": "-21"
    },
    {
      "fxTime": "2021-12-16T21:00+00:00",
      "temp": "-7",
      "icon": "150",
      "text": "晴",
      "wind360": "286",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "9",
      "humidity": "27",
      "precip": "0.0",
      "pressure": "1026",
      "cloud": "0",
      "dew": "-22"
    },
    {
      "fxTime": "2021-12-16T22:00+00:00",
      "temp": "-7",
      "icon": "150",
      "text": "晴",
      "wind360": "287",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "8",
      "humidity": "27",
      "precip": "0.0",
      "pressure": "1026",
      "cloud": "0",
      "dew": "-23"
    },
    {
      "fxTime": "2021-12-16T23:00+00:00",
      "temp": "-7",
      "icon": "150",
      "text": "晴",
      "wind360": "293",
      "windDir": "西北风",
      "windScale": "1",
      "windSpeed": "5",
      "humidity": "28",
      "precip": "0.0",
      "pressure": "1027",
      "cloud": "0",
      "dew": "-23"
    },
    {
      "fxTime": "2021-12-17T00:00+00:00",
      "temp": "-7",
      "icon": "150",
      "text": "晴",
      "wind360": "304",
      "windDir": "西北风",
      "windScale": "1",
      "windSpeed": "3",
      "humidity": "27",
      "precip": "0.0",
      "pressure": "1027",
      "cloud": "0",
      "dew": "-23"
    },
    {
      "fxTime": "2021-12-17T01:00+00:00",
      "temp": "-7",
      "icon": "150",
      "text": "晴",
      "wind360": "335",
      "windDir": "西北风",
      "windScale": "1",
      "windSpeed": "4",
      "humidity": "24",
      "precip": "0.0",
      "pressure": "1028",
      "cloud": "0",
      "dew": "-24"
    },
    {
      "fxTime": "2021-12-17T02:00+00:00",
      "temp": "-6",
      "icon": "150",
      "text": "晴",
      "wind360": "329",
      "windDir": "西北风",
      "windScale": "2",
      "windSpeed": "8",
      "humidity": "20",
      "precip": "0.0",
      "pressure": "1028",
      "cloud": "0",
      "dew": "-25"
    },
    {
      "fxTime": "2021-12-17T03:00+00:00",
      "temp": "-5",
      "icon": "150",
      "text": "晴",
      "wind360": "327",
      "windDir": "西北风",
      "windScale": "3",
      "windSpeed": "14",
      "humidity": "17",
      "precip": "0.0",
      "pressure": "1029",
      "cloud": "0",
      "dew": "-26"
    },
    {
      "fxTime": "2021-12-17T04:00+00:00",
      "temp": "-4",
      "icon": "150",
      "text": "晴",
      "wind360": "325",
      "windDir": "西北风",
      "windScale": "3",
      "windSpeed": "14",
      "humidity": "16",
      "precip": "0.0",
      "pressure": "1028",
      "cloud": "0",
      "dew": "-26"
    },
    {
      "fxTime": "2021-12-17T05:00+00:00",
      "temp": "-3",
      "icon": "150",
      "text": "晴",
      "wind360": "324",
      "windDir": "西北风",
      "windScale": "3",
      "windSpeed": "16",
      "humidity": "15",
      "precip": "0.0",
      "pressure": "1026",
      "cloud": "0",
      "dew": "-26"
    },
    {
      "fxTime": "2021-12-17T06:00+00:00",
      "temp": "-2",
      "icon": "150",
      "text": "晴",
      "wind360": "324",
      "windDir": "西北风",
      "windScale": "3",
      "windSpeed": "15",
      "humidity": "14",
      "precip": "0.0",
      "pressure": "1025",
      "cloud": "0",
      "dew": "-25"
    },
    {
      "fxTime": "2021-12-17T07:00+00:00",
      "temp": "-1",
      "icon": "150",
      "text": "晴",
      "wind360": "325",
      "windDir": "西北风",
      "windScale": "3",
      "windSpeed": "12",
      "humidity": "13",
      "precip": "0.0",
      "pressure": "1025",
      "cloud": "0",
      "dew": "-26"
    },
    {
      "fxTime": "2021-12-17T08:00+00:00",
      "temp": "-1",
      "icon": "150",
      "text": "晴",
      "wind360": "352",
      "windDir": "北风",
      "windScale": "2",
      "windSpeed": "8",
      "humidity": "13",
      "precip": "0.0",
      "pressure": "1024",
      "cloud": "0",
      "dew": "-26"
    },
    {
      "fxTime": "2021-12-17T09:00+00:00",
      "temp": "-2",
      "icon": "150",
      "text": "晴",
      "wind360": "44",
      "windDir": "东北风",
      "windScale": "2",
      "windSpeed": "9",
      "humidity": "19",
      "precip": "0.0",
      "pressure": "1024",
      "cloud": "0",
      "dew": "-22"
    },
    {
      "fxTime": "2021-12-17T10:00+00:00",
      "temp": "-3",
      "icon": "150",
      "text": "晴",
      "wind360": "52",
      "windDir": "东北风",
      "windScale": "2",
      "windSpeed": "8",
      "humidity": "26",
      "precip": "0.0",
      "pressure": "1024",
      "cloud": "0",
      "dew": "-20"
    },
    {
      "fxTime": "2021-12-17T11:00+00:00",
      "temp": "-4",
      "icon": "154",
      "text": "阴",
      "wind360": "48",
      "windDir": "东北风",
      "windScale": "2",
      "windSpeed": "6",
      "humidity": "29",
      "precip": "0.0",
      "pressure": "1023",
      "cloud": "91",
      "dew": "-19"
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

    let resp = serde_json::from_str::<GridWeatherHourlyForecastResponse>(json_data).unwrap();
    assert_eq!(resp.hourly.len(), 24);
    assert_eq!(resp.hourly[0].temp, -2.0);
    assert_eq!(resp.hourly[1].temp, -3.0);
    assert_eq!(resp.hourly[2].temp, -3.0);
    assert_eq!(resp.hourly[3].temp, -4.0);
    assert_eq!(resp.hourly[4].temp, -4.0);
    assert_eq!(resp.hourly[5].temp, -5.0);
    assert_eq!(resp.hourly[6].temp, -5.0);
    assert_eq!(resp.hourly[7].temp, -6.0);
    assert_eq!(resp.hourly[8].temp, -6.0);
    assert_eq!(resp.hourly[9].temp, -7.0);
    assert_eq!(resp.hourly[10].temp, -7.0);
    assert_eq!(resp.hourly[11].temp, -7.0);
    assert_eq!(resp.hourly[12].temp, -7.0);
    assert_eq!(resp.hourly[13].temp, -7.0);
    assert_eq!(resp.hourly[14].temp, -6.0);
    assert_eq!(resp.hourly[15].temp, -5.0);
    assert_eq!(resp.hourly[16].temp, -4.0);
    assert_eq!(resp.hourly[17].temp, -3.0);
    assert_eq!(resp.hourly[18].temp, -2.0);
    assert_eq!(resp.hourly[19].temp, -1.0);
    assert_eq!(resp.hourly[20].temp, -1.0);
    assert_eq!(resp.hourly[21].temp, -2.0);
    assert_eq!(resp.hourly[22].temp, -3.0);
    assert_eq!(resp.hourly[23].temp, -4.0);
}
