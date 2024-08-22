use std::collections::BTreeMap;

use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use crate::api::utils::RGBA;
use crate::{api::decode_datetime, client::QWeatherClient, APIResult};

/// 实时空气质量(beta)请求参数
#[derive(Default)]
pub struct AirNowInput<'a> {
    /// (必选)所需查询城市的LocationID，LocationID可通过GeoAPI获取。例如 101010100
    pub location: &'a str,
    /// 返回空气质量中的污染物数值，布尔值，默认false。
    pub pollutant: Option<bool>,
    /// 返回当前城市AQI所参考的监测站ID和名字，布尔值，默认false。
    pub station: Option<bool>,
    /// 多语言设置，请阅读多语言文档，了解我们的多语言是如何工作、如何设置以及数据是否支持多语言。
    pub lang: Option<&'a str>,
}

impl QWeatherClient {
    /// 实时空气质量(beta)
    ///
    /// 全球空气质量实时数据，我们提供了基于各个国家或地区当地规则的AQI以及污染物浓度值，
    /// 你可以查询指定城市的当前小时实时数据。
    ///
    /// # 参数
    ///
    /// * location 所需查询城市的LocationID，LocationID可通过GeoAPI获取。例如 101010100
    /// * pollutant 返回空气质量中的污染物数值，布尔值，默认false。
    /// * station 返回当前城市AQI所参考的监测站ID和名字，布尔值，默认false。
    pub async fn air_now(&self, air_now_input: AirNowInput<'_>) -> APIResult<AirNowResponse> {
        let url = format!(
            "{}/airquality/v1/now/{}",
            self.get_api_host(),
            air_now_input.location
        );
        let mut params = BTreeMap::new();
        params.insert("location".to_string(), air_now_input.location.to_string());
        params.insert(
            "pollutant".to_string(),
            air_now_input.pollutant.unwrap_or_default().to_string(),
        );
        params.insert(
            "station".to_string(),
            air_now_input.station.unwrap_or_default().to_string(),
        );

        self.request_api(url, params).await
    }

    /// 监测站数据(beta)
    ///
    /// 全球空气质量监测站数据，提供各个国家或地区监测站的污染物浓度值。
    ///
    /// # 参数
    ///
    /// * location 空气质量监测站的LocationID，LocationID可通过GeoAPI获取。例如 P58911
    pub async fn air_station(&self, location_id: &str) -> APIResult<AirStationResponse> {
        let url = format!(
            "{}/airquality/v1/station/{}",
            self.get_api_host(),
            location_id
        );
        let mut params = BTreeMap::new();
        params.insert("location".to_string(), location_id.to_string());

        self.request_api(url, params).await
    }
}

/// 实时空气质量返回值
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AirNowResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 空气质量指数
    pub aqi: Vec<AQI>,
    /// 污染物
    pub pollutant: Option<Vec<Pollutant>>,
    /// AQI相关联的监测站
    pub station: Option<Vec<Station>>,
    /// 数据来源
    pub source: Vec<String>,
}

/// 空气质量
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AQI {
    /// [空气质量指数](https://dev.qweather.com/docs/resource/air-info/#supported-aqis)Code
    pub code: String,
    /// 空气质量指数的名字
    pub name: String,
    /// 是否是[默认/推荐的当地AQI](https://dev.qweather.com/docs/resource/air-info/#default-local-aqi)
    pub default_local_aqi: bool,
    /// [空气质量指数的值](https://dev.qweather.com/docs/resource/air-info/#aqi-value)
    pub value: i32,
    /// [空气质量指数的值的文本显示](https://dev.qweather.com/docs/resource/air-info/#aqi-value)
    pub value_display: String,
    /// 空气质量指数等级，可能为空
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub level: i32,
    /// 空气质量指数类别，可能为空
    pub category: String,
    /// 空气质量指数的颜色，RGB格式
    pub color: RGBA,
    /// 首要污染物
    pub primary_pollutant: Option<PrimaryPollutant>,
    /// 健康指导意见
    pub health: Option<Health>,
}

/// 首要污染物
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PrimaryPollutant {
    /// [首要污染物](https://dev.qweather.com/docs/resource/air-info/#primary-pollutant)的Code，可能为空
    pub code: String,
    /// 首要污染物的名字，可能为空
    pub name: String,
    /// 首要污染物的全称，可能为空
    pub full_name: String,
}

/// 健康指导意见
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Health {
    /// [空气质量对健康的影响](https://dev.qweather.com/docs/resource/air-info/#health-effects-and-advice)，可能为空
    pub effect: String,
    /// 健康指导意见
    pub advice: HealthAdvice,
}

/// 健康指导意见
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HealthAdvice {
    /// 对一般人群的健康指导意见，可能为空
    pub general_population: String,
    /// 对敏感人群的健康指导意见，可能为空
    pub sensitive_population: String,
}

/// 污染物
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pollutant {
    /// [污染物](https://dev.qweather.com/docs/resource/air-info/#pollutants)的Code，可能为空
    pub code: String,
    /// 污染物的名字，可能为空
    pub name: String,
    /// 污染物的全称，可能为空
    pub full_name: String,
    /// 污染物的浓度值
    pub concentration: Concentration,
    /// 污染物的分指数
    pub sub_index: Option<SubIndex>,
}

/// 浓度值
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Concentration {
    /// 浓度值
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: f64,
    /// 浓度值的单位
    pub unit: String,
}

/// 分指数
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SubIndex {
    /// [污染物的分指数的数值](https://dev.qweather.com/docs/resource/air-info/#pollutant-sub-index)，可能为空
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub value: f64,
    /// 污染物的分指数数值的显示名称
    pub value_display: String,
}

/// AQI相关联的监测站
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Station {
    /// AQI相关联的监测站Location ID，可能为空
    pub id: String,
    /// AQI相关联的监测站名称
    pub name: String,
}

/// 监测站数据返回值
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AirStationResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    /// 当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 污染物
    pub pollutant: Vec<Pollutant>,
    /// 数据来源
    pub source: Option<Vec<String>>,
}

#[test]
fn test_air_quality() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2023-02-11T10:20+08:00",
  "aqi": [
    {
      "code": "cn-mee-1h",
      "name": "AQI-1H (CN)",
      "defaultLocalAqi": true,
      "value": 37,
      "valueDisplay": "37",
      "level": "1",
      "category": "优",
      "color": "0,228,0",
      "health": {
        "effect": "空气质量令人满意，基本无空气污染。",
        "advice": {
          "generalPopulation": "各类人群可正常活动。",
          "sensitivePopulation": "各类人群可正常活动。"
        }
      }
    },
    {
      "code": "cn-mee",
      "name": "AQI (CN)",
      "defaultLocalAqi": false,
      "value": 55,
      "valueDisplay": "55",
      "level": "2",
      "category": "良",
      "color": {
          "alpha": 255,
          "blue": 0,
          "green": 255,
          "red": 255
        },
      "primaryPollutant": {
        "code": "pm10",
        "name": "PM 10",
        "fullName": "颗粒物（粒径小于等于10µm）"
      },
      "health": {
        "effect": "空气质量可接受，但某些污染物可能对极少数异常敏感人群健康有较弱影响。",
        "advice": {
          "generalPopulation": "一般人群可正常活动。",
          "sensitivePopulation": "极少数异常敏感人群应减少户外活动。"
        }
      }
    }
  ],
  "pollutant": [
    {
      "code": "pm2p5",
      "name": "PM 2.5",
      "fullName": "颗粒物（粒径小于等于2.5µm）",
      "concentration": {
        "value": 25.0,
        "unit": "μg/m3"
      },
      "subIndex": {
        "value": 37,
        "valueDisplay": "37"
      }
    },
    {
      "code": "pm10",
      "name": "PM 10",
      "fullName": "颗粒物（粒径小于等于10µm）",
      "concentration": {
        "value": 36.0,
        "unit": "μg/m3"
      },
      "subIndex": {
        "value": 37,
        "valueDisplay": "37"
      }
    },
    {
      "code": "no2",
      "name": "NO2",
      "fullName": "二氧化氮",
      "concentration": {
        "value": 41.0,
        "unit": "μg/m3"
      },
      "subIndex": {
        "value": 21,
        "valueDisplay": "21"
      }
    },
    {
      "code": "o3",
      "name": "O3",
      "fullName": "臭氧",
      "concentration": {
        "value": 49.0,
        "unit": "μg/m3"
      },
      "subIndex": {
        "value": 16,
        "valueDisplay": "16"
      }
    },
    {
      "code": "so2",
      "name": "SO2",
      "fullName": "二氧化硫",
      "concentration": {
        "value": 6.0,
        "unit": "μg/m3"
      },
      "subIndex": {
        "value": 3,
        "valueDisplay": "3"
      }
    },
    {
      "code": "co",
      "name": "CO",
      "fullName": "一氧化碳",
      "concentration": {
        "value": 0.5,
        "unit": "mg/m3"
      },
      "subIndex": {
        "value": 6,
        "valueDisplay": "6"
      }
    }
  ],
  "station": [
    {
      "id": "P5697",
      "name": "普陀"
    },
    {
      "id": "P54852",
      "name": "十五厂"
    },
    {
      "id": "P57823",
      "name": "虹口"
    },
    {
      "id": "P55605",
      "name": "徐汇上师大"
    },
    {
      "id": "P53057",
      "name": "杨浦四漂"
    },
    {
      "id": "P5663",
      "name": "静安监测站"
    },
    {
      "id": "P53991",
      "name": "浦东川沙"
    },
    {
      "id": "P53763",
      "name": "浦东新区监测站"
    },
    {
      "id": "P5659",
      "name": "浦东张江"
    },
    {
      "id": "P54024",
      "name": "宝山庙行"
    },
    {
      "id": "P51755",
      "name": "崇明上实东滩"
    },
    {
      "id": "P59043",
      "name": "嘉定南翔"
    },
    {
      "id": "P5991",
      "name": "金山新城"
    },
    {
      "id": "P56226",
      "name": "闵行浦江"
    },
    {
      "id": "P56748",
      "name": "青浦徐泾"
    },
    {
      "id": "P56697",
      "name": "松江图书馆"
    },
    {
      "id": "P56700",
      "name": "长宁仙霞"
    },
    {
      "id": "P52014",
      "name": "浦东惠南"
    },
    {
      "id": "P51966",
      "name": "奉贤南桥新城"
    }
  ],
  "source": [
    "中国环境监测总站 (CNEMC)。数据仅为当天参考值，未经过完整的审核程序进行修订和确认，不适用评价达标状况或任何正式评估。"
  ]
}"#;

    let air_now: AirNowResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(air_now.code, "200");
    assert_eq!(
        air_now.update_time.to_string(),
        "2023-02-11 10:20:00 +08:00"
    );
    assert_eq!(air_now.aqi.len(), 2);
    assert_eq!(air_now.pollutant.unwrap().len(), 6);
    assert_eq!(air_now.station.unwrap().len(), 19);
    assert_eq!(air_now.source.len(), 1);
    assert_eq!(air_now.aqi[0].code, "cn-mee-1h");
    assert_eq!(air_now.aqi[0].name, "AQI-1H (CN)");
    assert!(air_now.aqi[0].default_local_aqi);
    assert_eq!(air_now.aqi[0].value, 37);
    assert_eq!(air_now.aqi[0].value_display, "37");
    assert_eq!(air_now.aqi[0].level, 1);
    assert_eq!(air_now.aqi[0].category, "优");
    assert_eq!(air_now.aqi[0].color.red, 255);
    assert_eq!(
        air_now.aqi[0].health.as_ref().unwrap().effect,
        "空气质量令人满意，基本无空气污染。"
    );
    assert_eq!(
        air_now.aqi[0]
            .health
            .as_ref()
            .unwrap()
            .advice
            .general_population,
        "各类人群可正常活动。"
    );
    assert_eq!(
        air_now.aqi[0]
            .health
            .as_ref()
            .unwrap()
            .advice
            .sensitive_population,
        "各类人群可正常活动。"
    );
}

#[test]
fn test_air_station() {
    let json_data = r#"{
  "code": "200",
  "updateTime": "2023-08-30T09:40+00:00",
  "pollutant": [
    {
      "code": "pm2p5",
      "name": "PM 2.5",
      "fullName": "Fine particulate matter (<2.5µm)",
      "concentration": {
        "value": "19",
        "unit": "μg/m3"
      }
    },
    {
      "code": "pm10",
      "name": "PM 10",
      "fullName": "Inhalable particulate matter (<10µm)",
      "concentration": {
        "value": "26",
        "unit": "μg/m3"
      }
    },
    {
      "code": "no2",
      "name": "NO2",
      "fullName": "Nitrogen dioxide",
      "concentration": {
        "value": "12.3",
        "unit": "ppb"
      }
    },
    {
      "code": "o3",
      "name": "O3",
      "fullName": "Ozone",
      "concentration": {
        "value": "30",
        "unit": "ppb"
      }
    },
    {
      "code": "co",
      "name": "CO",
      "fullName": "Carbon monoxide",
      "concentration": {
        "value": "0.4",
        "unit": "ppm"
      }
    }
  ],
  "source": [
    "EPA"
  ]
}"#;

    let air_station: AirStationResponse = serde_json::from_str(json_data).unwrap();
    assert_eq!(air_station.code, "200");
    assert_eq!(
        air_station.update_time.to_string(),
        "2023-08-30 09:40:00 +00:00"
    );
    let pollutant = air_station.pollutant;
    assert_eq!(pollutant.len(), 5);
    assert_eq!(air_station.source.unwrap().len(), 1);
    assert_eq!(pollutant[0].code, "pm2p5");
    assert_eq!(pollutant[0].name, "PM 2.5");
    assert_eq!(pollutant[0].full_name, "Fine particulate matter (<2.5µm)");
    assert_eq!(pollutant[0].concentration.value, 19.0);
    assert_eq!(pollutant[0].concentration.unit, "μg/m3");
}
