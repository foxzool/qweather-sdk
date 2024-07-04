use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

use crate::{
    api::{decode_datetime, option_decode_datetime, Refer},
    client::QWeatherClient,
    APIResult,
};

impl QWeatherClient {
    /// 天气灾害预警
    ///
    /// 天气灾害预警API可以获取中国及全球多个国家或地区官方发布的实时天气灾害预警数据。
    ///
    /// [官方文档](https://dev.qweather.com/docs/api/warning-now/)
    ///
    /// # 参数
    ///
    /// * location 需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    pub async fn weather_warning(&self, location: &str) -> APIResult<WeatherWarningResponse> {
        let url = format!("{}/v7/warning/now", self.get_api_host());

        let mut params = BTreeMap::new();
        params.insert("location".to_string(), location.to_string());

        self.request_api(url, params).await
    }

    /// 天气预警城市列表
    ///
    /// 获取指定国家或地区当前正在发生天气灾害预警的城市列表，
    /// 根据这些城市列表再查询对应城市的天气灾害预警。
    ///
    ///
    /// # 参数
    ///
    /// * range 选择指定的国家或地区，使用ISO
    ///   3166格式。例如range=cn或range=hk。目前该功能仅支持中国（包括港澳台）地区的城市列表，
    ///   其他国家和地区请使用请使用天气灾害预警单独获取
    pub async fn weather_warning_city_list(
        &self,
        range: &str,
    ) -> APIResult<WeatherWarningCityListResponse> {
        let url = format!("{}/v7/warning/list", self.get_api_host());

        let mut params = BTreeMap::new();
        params.insert("range".to_string(), range.to_string());

        self.request_api(url, params).await
    }
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeatherWarning {
    /// 本条预警的唯一标识，可判断本条预警是否已经存在
    pub id: String,
    /// 预警发布单位，可能为空
    pub sender: String,
    /// 预警发布时间
    #[serde(deserialize_with = "decode_datetime")]
    pub pub_time: DateTime<FixedOffset>,
    /// 预警信息标题
    pub title: String,
    /// 预警开始时间，可能为空
    #[serde(deserialize_with = "option_decode_datetime")]
    pub start_time: Option<DateTime<FixedOffset>>,
    /// [预警结束时间](https://dev.qweather.com/docs/resource/warning-info/#expiry-time)，可能为空
    #[serde(deserialize_with = "option_decode_datetime")]
    pub end_time: Option<DateTime<FixedOffset>>,
    /// [预警信息的发布状态](https://dev.qweather.com/docs/resource/warning-info/#status)
    pub status: String,
    /// [预警严重等级](https://dev.qweather.com/docs/resource/warning-info/#severity)
    pub severity: String,
    /// [预警严重等级颜色](https://dev.qweather.com/docs/resource/warning-info/#severity-color)，可能为空
    pub severity_color: String,
    /// [预警类型ID](https://dev.qweather.com/docs/resource/warning-info/#warning-type)
    #[serde(rename = "type")]
    pub type_: String,
    /// [预警类型名称](https://dev.qweather.com/docs/resource/warning-info/#warning-type)
    pub type_name: String,
    /// [预警信息的紧迫程度](https://dev.qweather.com/docs/resource/warning-info/#urgency)，可能为空
    pub urgency: String,
    /// [预警信息的确定性](https://dev.qweather.com/docs/resource/warning-info/#certainty)，可能为空
    pub certainty: String,
    /// 预警详细文字描述
    pub text: String,
    /// 与本条预警相关联的预警ID，当预警状态为cancel或update时返回。可能为空
    pub related: String,
}

/// 天气灾害预警返回数据
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeatherWarningResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    /// 天气灾害预警数据
    pub warning: Vec<WeatherWarning>,
    /// 数据来源
    pub refer: Refer,
}

/// 天气预警城市列表
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct WeatherWarningCityListResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 当前国家预警的LocationID
    pub warning_loc_list: Vec<LocationId>,
    /// 数据来源
    pub refer: Refer,
}

/// LocationID
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LocationId {
    pub location_id: String,
}

#[test]
fn test_weather_warning() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2023-04-03T14:20+08:00",
  "fxLink": "https://www.qweather.com/severe-weather/shanghai-101020100.html",
  "warning": [
    {
      "id": "10102010020230403103000500681616",
      "sender": "上海中心气象台",
      "pubTime": "2023-04-03T10:30+08:00",
      "title": "上海中心气象台发布大风蓝色预警[Ⅳ级/一般]",
      "startTime": "2023-04-03T10:30+08:00",
      "endTime": "2023-04-04T10:30+08:00",
      "status": "active",
      "level": "",
      "severity": "Minor",
      "severityColor": "Blue",
      "type": "1006",
      "typeName": "大风",
      "urgency": "",
      "certainty": "",
      "text": "上海中心气象台2023年04月03日10时30分发布大风蓝色预警[Ⅳ级/一般]：受江淮气旋影响，预计明天傍晚以前本市大部地区将出现6级阵风7-8级的东南大风，沿江沿海地区7级阵风8-9级，请注意防范大风对高空作业、交通出行、设施农业等的不利影响。",
      "related": ""
    }
  ],
  "refer": {
    "sources": [
      "12379"
    ],
    "license": [
      "QWeather Developers License"
    ]
  }
}"#;

    let resp: WeatherWarningResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(
        resp.fx_link,
        "https://www.qweather.com/severe-weather/shanghai-101020100.html"
    );
    assert_eq!(resp.warning.len(), 1);
    let warning = &resp.warning[0];
    assert_eq!(warning.id, "10102010020230403103000500681616");
    assert_eq!(warning.sender, "上海中心气象台");
    assert_eq!(
        warning.pub_time,
        DateTime::parse_from_rfc3339("2023-04-03T10:30:00+08:00").unwrap()
    );
    assert_eq!(warning.title, "上海中心气象台发布大风蓝色预警[Ⅳ级/一般]");

    assert!(warning.start_time.is_some());
    assert!(warning.end_time.is_some());
    assert_eq!(warning.status, "active");
    assert_eq!(warning.severity, "Minor");
    assert_eq!(warning.severity_color, "Blue");
    assert_eq!(warning.type_, "1006");
    assert_eq!(warning.type_name, "大风");
    assert_eq!(warning.urgency, "");
    assert_eq!(warning.certainty, "");
    assert_eq!(warning.text, "上海中心气象台2023年04月03日10时30分发布大风蓝色预警[Ⅳ级/一般]：受江淮气旋影响，预计明天傍晚以前本市大部地区将出现6级阵风7-8级的东南大风，沿江沿海地区7级阵风8-9级，请注意防范大风对高空作业、交通出行、设施农业等的不利影响。");
    assert_eq!(warning.related, "");
}

#[test]
fn test_weather_warning_city_list() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2020-06-21T05:39+00:00",
  "warningLocList": [
    {
      "locationId": "101010800"
    },
    {
      "locationId": "101011200"
    },
    {
      "locationId": "101011400"
    },
    {
      "locationId": "101020700"
    },
    {
      "locationId": "101040400"
    },
    {
      "locationId": "101041900"
    },
    {
      "locationId": "101043400"
    },
    {
      "locationId": "101043600"
    },
    {
      "locationId": "101050106"
    },
    {
      "locationId": "101050107"
    },
    {
      "locationId": "101050301"
    },
    {
      "locationId": "101050302"
    },
    {
      "locationId": "101050303"
    },
    {
      "locationId": "101130103"
    },
    {
      "locationId": "101130109"
    },
    {
      "locationId": "101130114"
    },
    {
      "locationId": "101130302"
    },
    {
      "locationId": "101130303"
    },
    {
      "locationId": "101130409"
    },
    {
      "locationId": "101130610"
    },
    {
      "locationId": "101130611"
    },
    {
      "locationId": "101130613"
    },
    {
      "locationId": "101130614"
    },
    {
      "locationId": "101131920"
    },
    {
      "locationId": "101221008"
    },
    {
      "locationId": "101230507"
    },
    {
      "locationId": "101132101"
    },
    {
      "locationId": "101132201"
    },
    {
      "locationId": "101132301"
    }
  ],
  "refer": {
    "sources": [
      "12379",
      "QWeather"
    ],
    "license": [
      "QWeather Developers License"
    ]
  }
}"#;

    let resp: WeatherWarningCityListResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(resp.code, "200");
    assert_eq!(
        resp.update_time,
        DateTime::parse_from_rfc3339("2020-06-21T05:39:00+00:00").unwrap()
    );
    assert_eq!(resp.warning_loc_list.len(), 29);
    let location_id = &resp.warning_loc_list[0];
    assert_eq!(location_id.location_id, "101010800");
    let location_id = &resp.warning_loc_list[1];
    assert_eq!(location_id.location_id, "101011200");
    let location_id = &resp.warning_loc_list[2];
    assert_eq!(location_id.location_id, "101011400");
    let location_id = &resp.warning_loc_list[3];
    assert_eq!(location_id.location_id, "101020700");
    let location_id = &resp.warning_loc_list[4];
    assert_eq!(location_id.location_id, "101040400");
    let location_id = &resp.warning_loc_list[5];
    assert_eq!(location_id.location_id, "101041900");
    let location_id = &resp.warning_loc_list[6];
    assert_eq!(location_id.location_id, "101043400");
    let location_id = &resp.warning_loc_list[7];
    assert_eq!(location_id.location_id, "101043600");
    let location_id = &resp.warning_loc_list[8];
    assert_eq!(location_id.location_id, "101050106");
    let location_id = &resp.warning_loc_list[9];
    assert_eq!(location_id.location_id, "101050107");
    let location_id = &resp.warning_loc_list[10];
    assert_eq!(location_id.location_id, "101050301");
    let location_id = &resp.warning_loc_list[11];
    assert_eq!(location_id.location_id, "101050302");
    let location_id = &resp.warning_loc_list[12];
    assert_eq!(location_id.location_id, "101050303");
    let location_id = &resp.warning_loc_list[13];
    assert_eq!(location_id.location_id, "101130103");
    let location_id = &resp.warning_loc_list[14];
    assert_eq!(location_id.location_id, "101130109");
}
