use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_number_from_string, deserialize_option_number_from_string};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    ///  当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: NaiveDateTime,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    #[serde(flatten)]
    pub data: T,
    pub refer: Refer,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Refer {
    pub sources: Vec<String>,
    pub license: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DataType {
    Now { now: Now },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Now {
    /// 数据观测时间
    #[serde(deserialize_with = "decode_datetime")]
    pub obs_time: NaiveDateTime,
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

fn decode_datetime<'de, D>(deserializer: D) -> Result<NaiveDateTime, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = NaiveDateTime::parse_from_str(&s, "%Y-%m-%dT%H:%M%z").unwrap();
    Ok(dt)
}

#[cfg(test)]
mod test {
    use crate::model::{ApiResponse, DataType};

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

        let resp  = serde_json::from_str::<ApiResponse<DataType>>(json_data);
        assert!(resp.is_ok())
    }
}
