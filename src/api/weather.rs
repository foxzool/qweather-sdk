use chrono::{DateTime, FixedOffset, NaiveDate};
use log::debug;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_number_from_string, deserialize_option_number_from_string};
use url::Url;

use crate::{
    client::QWeatherClient,
    model::{decode_datetime, DynamicDataResponse, Refer},
    SDKResult,
};

impl QWeatherClient {
    /// 实时天气
    ///
    /// 获取中国3000+市县区和海外20万个城市实时天气数据，包括实时温度、体感温度、风力风向、
    /// 相对湿度、大气压强、降水量、能见度、露点温度、云量等。
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    pub async fn weather_now(&self, location: &str) -> SDKResult<WeatherNowResponse> {
        let url = format!("{}/v7/weather/now", self.base_url);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_now {}", url);

        self.client.get(url).send().await?.json().await
    }

    /// 每日天气预报
    ///
    /// 每日天气预报API，提供全球城市未来3-30天天气预报，包括：日出日落、月升月落、最高最低温度、
    /// 天气白天和夜间状况、风力、风速、风向、相对湿度、大气压强、降水量、露点温度、紫外线强度、
    /// 能见度等。
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    ///
    /// * day 天, 只能是 3, 7, 10, 15, 30
    pub async fn weather_daily_forecast(
        &self,
        location: &str,
        day: u8,
    ) -> SDKResult<WeatherDailyForecastResponse> {
        if ![3u8, 7, 10, 15, 30].contains(&day) {
            panic!("invalid day")
        }
        let url = format!("{}/v7/weather/{}d", self.base_url, day);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_daily_forecast {}", url);

        self.client.get(url).send().await?.json().await
    }

    /// 逐小时天气预报
    ///
    /// 逐小时天气预报API，提供全球城市未来24-168小时逐小时天气预报，包括：温度、天气状况、风力、
    /// 风速、风向、相对湿度、大气压强、降水概率、露点温度、云量。
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    ///
    /// * hour 小时, 只能是 24, 72, 168
    pub async fn weather_hourly_forecast(
        &self,
        location: &str,
        hour: u8,
    ) -> SDKResult<DynamicDataResponse> {
        if ![24u8, 72, 168].contains(&hour) {
            panic!("invalid hour")
        }
        let url = format!("{}/v7/weather/{}h", self.base_url, hour);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_hourly_forecast {}", url);

        self.client.get(url).send().await?.json().await
    }
}

/// 实时天气返回值`
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherNow {
    /// 数据观测时间
    #[serde(deserialize_with = "decode_datetime")]
    pub obs_time: DateTime<FixedOffset>,
    /// 温度，默认单位：摄氏度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub temp: f32,
    /// 体感温度，默认单位：摄氏度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub feels_like: f32,
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
    /// 能见度，默认单位：公里
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub vis: f32,
    /// 云量，百分比数值。可能为空
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub cloud: Option<f32>,
    /// 露点温度。可能为空
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub dew: Option<f32>,
}

/// 实时天气返回数据
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherNowResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    /// 实时天气数据
    pub now: WeatherNow,
    /// 数据来源
    pub refer: Refer,
}

/// 每日天气预报
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyForecast {
    /// 预报日期
    pub fx_date: NaiveDate,
    /// [日出时间](https://dev.qweather.com/docs/resource/sun-moon-info/#sunrise-and-sunset)，在高纬度地区可能为空
    pub sunrise: Option<String>,
    /// [日落时间](https://dev.qweather.com/docs/resource/sun-moon-info/#sunrise-and-sunset)，在高纬度地区可能为空
    pub sunset: String,
    /// 当天[月升时间](https://dev.qweather.com/docs/resource/sun-moon-info/#moonrise-and-moonset)，可能为空
    pub moonrise: Option<String>,
    /// 当天[月落时间](https://dev.qweather.com/docs/resource/sun-moon-info/#moonrise-and-moonset)，可能为空
    pub moonset: String,
    /// [月相名称](https://dev.qweather.com/docs/resource/sun-moon-info/#moon-phase)
    pub moon_phase: String,
    /// 月相[图标代码](https://dev.qweather.com/docs/resource/icons/)，另请参考天气[图标项目](https://icons.qweather.com/)
    pub moon_phase_icon: String,
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
    /// 紫外线强度指数
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub uv_index: f32,
    /// 相对湿度，百分比数值
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub humidity: f32,
    /// 大气压强，默认单位：百帕
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub pressure: f32,
    /// 能见度，默认单位：公里
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub vis: f32,
    /// 云量，百分比数值。可能为空
    #[serde(deserialize_with = "deserialize_option_number_from_string")]
    pub cloud: Option<f32>,
}

/// 每日天气预报返回数据
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WeatherDailyForecastResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    /// 实时天气数据
    pub daily: Vec<DailyForecast>,
    /// 数据来源
    pub refer: Refer,
}

#[test]
fn test_weather_now() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2020-06-30T22:00+08:00",
  "fxLink": "http://hfx.link/2ax1",
  "now": {
    "obsTime": "2020-06-30T21:40+08:00",
    "temp": "24",
    "feelsLike": "26",
    "icon": "101",
    "text": "多云",
    "wind360": "123",
    "windDir": "东南风",
    "windScale": "1",
    "windSpeed": "3",
    "humidity": "72",
    "precip": "0.0",
    "pressure": "1003",
    "vis": "16",
    "cloud": "10",
    "dew": "21"
  },
  "refer": {
    "sources": [
      "QWeather",
      "NMC",
      "ECMWF"
    ],
    "license": [
      "QWeather Developers License"
    ]
  }
}"#;

    let resp = serde_json::from_str::<WeatherNowResponse>(json_data).unwrap();
    assert_eq!(resp.now.temp, 24.0)
}

#[test]
fn test_weather_daily_forecast() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2021-11-15T16:35+08:00",
  "fxLink": "http://hfx.link/2ax1",
  "daily": [
    {
      "fxDate": "2021-11-15",
      "sunrise": "06:58",
      "sunset": "16:59",
      "moonrise": "15:16",
      "moonset": "03:40",
      "moonPhase": "盈凸月",
      "moonPhaseIcon": "803",
      "tempMax": "12",
      "tempMin": "-1",
      "iconDay": "101",
      "textDay": "多云",
      "iconNight": "150",
      "textNight": "晴",
      "wind360Day": "45",
      "windDirDay": "东北风",
      "windScaleDay": "1-2",
      "windSpeedDay": "3",
      "wind360Night": "0",
      "windDirNight": "北风",
      "windScaleNight": "1-2",
      "windSpeedNight": "3",
      "humidity": "65",
      "precip": "0.0",
      "pressure": "1020",
      "vis": "25",
      "cloud": "4",
      "uvIndex": "3"
    },
    {
      "fxDate": "2021-11-16",
      "sunrise": "07:00",
      "sunset": "16:58",
      "moonrise": "15:38",
      "moonset": "04:40",
      "moonPhase": "盈凸月",
      "moonPhaseIcon": "803",
      "tempMax": "13",
      "tempMin": "0",
      "iconDay": "100",
      "textDay": "晴",
      "iconNight": "101",
      "textNight": "多云",
      "wind360Day": "225",
      "windDirDay": "西南风",
      "windScaleDay": "1-2",
      "windSpeedDay": "3",
      "wind360Night": "225",
      "windDirNight": "西南风",
      "windScaleNight": "1-2",
      "windSpeedNight": "3",
      "humidity": "74",
      "precip": "0.0",
      "pressure": "1016",
      "vis": "25",
      "cloud": "1",
      "uvIndex": "3"
    },
    {
      "fxDate": "2021-11-17",
      "sunrise": "07:01",
      "sunset": "16:57",
      "moonrise": "16:01",
      "moonset": "05:41",
      "moonPhase": "盈凸月",
      "moonPhaseIcon": "803",
      "tempMax": "13",
      "tempMin": "0",
      "iconDay": "100",
      "textDay": "晴",
      "iconNight": "150",
      "textNight": "晴",
      "wind360Day": "225",
      "windDirDay": "西南风",
      "windScaleDay": "1-2",
      "windSpeedDay": "3",
      "wind360Night": "225",
      "windDirNight": "西南风",
      "windScaleNight": "1-2",
      "windSpeedNight": "3",
      "humidity": "56",
      "precip": "0.0",
      "pressure": "1009",
      "vis": "25",
      "cloud": "0",
      "uvIndex": "3"
    }
  ],
  "refer": {
    "sources": [
      "QWeather",
      "NMC",
      "ECMWF"
    ],
    "license": [
      "QWeather Developers License"
    ]
  }
}"#;

    let resp = serde_json::from_str::<WeatherDailyForecastResponse>(json_data).unwrap();
    assert_eq!(resp.daily.len(), 3)
}
