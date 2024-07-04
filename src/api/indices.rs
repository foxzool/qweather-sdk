use chrono::{DateTime, FixedOffset, NaiveDate};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use crate::{
    api::{decode_datetime, Refer},
    client::QWeatherClient,
    APIResult,
};

impl QWeatherClient {
    /// 天气指数预报
    ///
    /// 获取中国和全球城市天气生活指数预报数据。
    ///
    /// * 中国天气生活指数 ：舒适度指数、洗车指数、穿衣指数、感冒指数、运动指数、旅游指数、
    ///   紫外线指数、空气污染扩散条件指数、空调开启指数、过敏指数、太阳镜指数、化妆指数、晾晒指数、
    ///   交通指数、钓鱼指数、防晒指数
    /// * 海外天气生活指数 ：运动指数、洗车指数、紫外线指数、钓鱼指数
    ///
    /// # 参数
    /// * location : 地区/城市ID
    /// * type_ : 指数类型
    /// * day : 预报天数，1天或者3天
    pub async fn indices_forecast(
        &self,
        location: &str,
        type_: &str,
        day: i32,
    ) -> APIResult<IndicesForecastResponse> {
        let url = format!("{}/v7/indices/{}d", self.get_api_host(), day);

        let mut params = self.base_params.clone();
        params.insert("location".to_string(), location.to_string());
        params.insert("type".to_string(), type_.to_string());

        self.request_api(url, params).await
    }
}

/// 天气指数预报
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DailyIndices {
    /// 预报日期
    pub date: NaiveDate,
    /// 生活指数类型ID
    #[serde(deserialize_with = "deserialize_number_from_string")]
    #[serde(rename = "type")]
    pub type_: i32,
    /// 生活指数类型的名称
    pub name: String,
    /// 生活指数预报等级
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub level: i32,
    /// 生活指数预报级别名称
    pub category: String,
    /// 生活指数预报的详细描述，可能为空
    pub text: String,
}

/// 天气指数预报返回数据
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IndicesForecastResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    /// 当前国家预警的LocationID
    pub daily: Vec<DailyIndices>,
    /// 数据来源
    pub refer: Refer,
}

#[test]
fn test_indices_forecast() {
    let json_dat = r#"{
  "code": "200",
  "updateTime": "2021-12-16T18:35+08:00",
  "fxLink": "http://hfx.link/2ax2",
  "daily": [
    {
      "date": "2021-12-16",
      "type": "1",
      "name": "运动指数",
      "level": "3",
      "category": "较不宜",
      "text": "天气较好，但考虑天气寒冷，风力较强，推荐您进行室内运动，若户外运动请注意保暖并做好准备活动。"
    },
    {
      "date": "2021-12-16",
      "type": "2",
      "name": "洗车指数",
      "level": "3",
      "category": "较不宜",
      "text": "较不宜洗车，未来一天无雨，风力较大，如果执意擦洗汽车，要做好蒙上污垢的心理准备。"
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

    let resp: IndicesForecastResponse = serde_json::from_str(json_dat).unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(resp.daily.len(), 2);
    assert_eq!(resp.daily[0].name, "运动指数");
    assert_eq!(resp.daily[0].level, 3);
    assert_eq!(resp.daily[0].category, "较不宜");
    assert_eq!(resp.daily[0].text, "天气较好，但考虑天气寒冷，风力较强，推荐您进行室内运动，若户外运动请注意保暖并做好准备活动。");
    assert_eq!(resp.refer.sources[0], "QWeather");
    assert_eq!(resp.refer.license[0], "QWeather Developers License");
    assert_eq!(resp.update_time.to_rfc3339(), "2021-12-16T18:35:00+08:00");
    assert_eq!(resp.fx_link, "http://hfx.link/2ax2");
}
