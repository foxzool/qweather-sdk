use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;

use crate::api::utils::{MetaData, RGBA};
use crate::{client::QWeatherClient, APIResult};

impl QWeatherClient {
    /// 实时空气质量(new)
    /// 实时空气质量API提供指定地点的实时空气质量数据，精度为1x1公里。
    ///
    /// 基于各个国家或地区当地标准的AQI、AQI等级、颜色和首要污染物
    /// 和风天气通用AQI
    /// 污染物浓度值、分指数
    /// 健康建议
    /// 相关联的监测站信息
    ///
    /// # 参数
    ///
    /// * latitude (必选)所需位置的纬度。十进制，最多支持小数点后两位。例如 39.92
    /// * longitude (必选)所需位置的经度。十进制，最多支持小数点后两位。例如 116.41
    pub async fn air_current(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> APIResult<AirCurrentResponse> {
        let url = format!(
            "{}/airquality/v1/current/{}/{}",
            self.get_api_host(),
            latitude,
            longitude
        );
        let mut params = BTreeMap::new();
        params.insert("latitude".to_string(), latitude.to_string());
        params.insert("longitude".to_string(), longitude.to_string());

        self.request_api(url, params).await
    }

    /// 监测站数据(new)
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

/// 实时空气质量(new)返回值
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AirCurrentResponse {
    /// 数据来源
    pub metadata: MetaData,
    /// 空气质量指数
    pub indexes: Vec<AQI>,
    /// 污染物
    pub pollutants: Option<Vec<Pollutant>>,
    // /// AQI相关联的监测站
    // pub stations: Option<Vec<Station>>,
}

/// 空气质量
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AQI {
    /// [空气质量指数](https://dev.qweather.com/docs/resource/air-info/#supported-aqis)Code
    pub code: String,
    /// 空气质量指数的名字
    pub name: String,
    /// [空气质量指数的值](https://dev.qweather.com/docs/resource/air-info/#aqi-value)
    pub aqi: f64,
    /// [空气质量指数的值的文本显示](https://dev.qweather.com/docs/resource/air-info/#aqi-value)
    pub aqi_display: String,
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
    // /// 污染物的浓度值
    pub concentration: Concentration,
    // /// 污染物的分指数
    pub sub_indexes: Option<Vec<SubIndex>>,
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
    /// 污染物的分指数的Code，可能为空
    pub code: String,
    /// [污染物的分指数的数值](https://dev.qweather.com/docs/resource/air-info/#pollutant-sub-index)，可能为空
    pub aqi: f64,
    /// 污染物的分指数数值的显示名称
    pub aqi_display: String,
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
    /// 数据来源
    pub metadata: MetaData,
    /// 污染物
    pub pollutants: Vec<Pollutant>,
}

#[test]
fn test_air_station() {
    let json_data = r#"{
  "metadata": {
    "sources": [
      "中国环境监测总站 (CNEMC)。数据仅为当天参考值，未经过完整的审核程序进行修订和确认，不适用评价达标状况或任何正式评估。"
    ],
    "tag": "f5306fd35a92320f12995584ac41178d299e0431fc6568387fd0b00dd2b581a0"
  },
  "pollutants": [
    {
      "code": "pm2p5",
      "concentration": {
        "unit": "μg/m3",
        "value": 12.0
      },
      "fullName": "颗粒物（粒径小于等于2.5µm）",
      "name": "PM 2.5"
    },
    {
      "code": "pm10",
      "concentration": {
        "unit": "μg/m3",
        "value": 20.0
      },
      "fullName": "颗粒物（粒径小于等于10µm）",
      "name": "PM 10"
    },
    {
      "code": "no2",
      "concentration": {
        "unit": "μg/m3",
        "value": 11.0
      },
      "fullName": "二氧化氮",
      "name": "NO2"
    },
    {
      "code": "o3",
      "concentration": {
        "unit": "μg/m3",
        "value": 50.0
      },
      "fullName": "臭氧",
      "name": "O3"
    },
    {
      "code": "so2",
      "concentration": {
        "unit": "μg/m3",
        "value": 7.0
      },
      "fullName": "二氧化硫",
      "name": "SO2"
    },
    {
      "code": "co",
      "concentration": {
        "unit": "mg/m3",
        "value": 0.4
      },
      "fullName": "一氧化碳",
      "name": "CO"
    }
  ]
}"#;

    let air_station: AirStationResponse = serde_json::from_str(json_data).unwrap();
    let metadata = air_station.metadata;
    assert_eq!(metadata.sources.unwrap().len(), 1);
    assert_eq!(
        metadata.tag,
        "f5306fd35a92320f12995584ac41178d299e0431fc6568387fd0b00dd2b581a0"
    );
    let pollutant = air_station.pollutants;
    assert_eq!(pollutant.len(), 6);
    assert_eq!(pollutant[0].code, "pm2p5");
    assert_eq!(pollutant[0].name, "PM 2.5");
    assert_eq!(pollutant[0].full_name, "颗粒物（粒径小于等于2.5µm）");
    assert_eq!(pollutant[0].concentration.value, 12.0);
    assert_eq!(pollutant[0].concentration.unit, "μg/m3");
}

#[test]
fn test_air_current() {
    let json_data = r#"{
  "metadata": {
    "tag": "d75a323239766b831889e8020cba5aca9b90fca5080a1175c3487fd8acb06e84"
  },
  "indexes": [
    {
      "code": "us-epa",
      "name": "AQI (US)",
      "aqi": 46,
      "aqiDisplay": "46",
      "level": "1",
      "category": "Good",
      "color": {
        "red": 0,
        "green": 228,
        "blue": 0,
        "alpha": 1
      },
      "primaryPollutant": {
        "code": "pm2p5",
        "name": "PM 2.5",
        "fullName": "Fine particulate matter (<2.5µm)"
      },
      "health": {
        "effect": "No health effects.",
        "advice": {
          "generalPopulation": "Everyone can continue their outdoor activities normally.",
          "sensitivePopulation": "Everyone can continue their outdoor activities normally."
        }
      }
    },
    {
      "code": "qaqi",
      "name": "QAQI",
      "aqi": 0.9,
      "aqiDisplay": "0.9",
      "level": "1",
      "category": "Excellent",
      "color": {
        "red": 80,
        "green": 240,
        "blue": 230,
        "alpha": 1
      },
      "primaryPollutant": {
        "code": "pm2p5",
        "name": "PM 2.5",
        "fullName": "Fine particulate matter (<2.5µm)"
      },
      "health": {
        "effect": "No health implications.",
        "advice": {
          "generalPopulation": "Enjoy your outdoor activities.",
          "sensitivePopulation": "Enjoy your outdoor activities."
        }
      }
    }
  ],
  "pollutants": [
    {
      "code": "pm2p5",
      "name": "PM 2.5",
      "fullName": "Fine particulate matter (<2.5µm)",
      "concentration": {
        "value": 11.0,
        "unit": "μg/m3"
      },
      "subIndexes": [
        {
          "code": "us-epa",
          "aqi": 46,
          "aqiDisplay": "46"
        },
        {
          "code": "qaqi",
          "aqi": 0.9,
          "aqiDisplay": "0.9"
        }
      ]
    },
    {
      "code": "pm10",
      "name": "PM 10",
      "fullName": "Inhalable particulate matter (<10µm)",
      "concentration": {
        "value": 12.0,
        "unit": "μg/m3"
      },
      "subIndexes": [
        {
          "code": "us-epa",
          "aqi": 12,
          "aqiDisplay": "12"
        },
        {
          "code": "qaqi",
          "aqi": 0.5,
          "aqiDisplay": "0.5"
        }
      ]
    },
    {
      "code": "no2",
      "name": "NO2",
      "fullName": "Nitrogen dioxide",
      "concentration": {
        "value": 6.77,
        "unit": "ppb"
      },
      "subIndexes": [
        {
          "code": "us-epa",
          "aqi": 7,
          "aqiDisplay": "7"
        },
        {
          "code": "qaqi",
          "aqi": 0.1,
          "aqiDisplay": "0.1"
        }
      ]
    },
    {
      "code": "o3",
      "name": "O3",
      "fullName": "Ozone",
      "concentration": {
        "value": 0.02,
        "unit": "ppb"
      },
      "subIndexes": [
        {
          "code": "us-epa",
          "aqi": 21,
          "aqiDisplay": "21"
        },
        {
          "code": "qaqi",
          "aqi": 0.2,
          "aqiDisplay": "0.2"
        }
      ]
    },
    {
      "code": "co",
      "name": "CO",
      "fullName": "Carbon monoxide",
      "concentration": {
        "value": 0.25,
        "unit": "ppm"
      },
      "subIndexes": [
        {
          "code": "us-epa",
          "aqi": 3,
          "aqiDisplay": "3"
        },
        {
          "code": "qaqi",
          "aqi": 0.1,
          "aqiDisplay": "0.1"
        }
      ]
    }
  ],
  "stations": [
    {
      "id": "P51762",
      "name": "North Holywood"
    },
    {
      "id": "P58056",
      "name": "Pasadena"
    },
    {
      "id": "P57327",
      "name": "Los Angeles - N. Main Street"
    }
  ]
}"#;

    let air_current: AirCurrentResponse = serde_json::from_str(json_data).unwrap();
    let metadata = air_current.metadata;
    assert_eq!(
        metadata.tag,
        "d75a323239766b831889e8020cba5aca9b90fca5080a1175c3487fd8acb06e84"
    );
    let indexes = air_current.indexes;
    assert_eq!(indexes.len(), 2);
    assert_eq!(indexes[0].code, "us-epa");
    assert_eq!(indexes[0].name, "AQI (US)");
    // assert_eq!(indexes[0].aqi, 46);
    // assert_eq!(indexes[0].aqi_display, "46");
    assert_eq!(indexes[0].level, 1);
    assert_eq!(indexes[0].category, "Good");
    assert_eq!(indexes[0].color.green, 228);
    assert_eq!(indexes[0].primary_pollutant.as_ref().unwrap().code, "pm2p5");
    assert_eq!(
        indexes[0].health.as_ref().unwrap().effect,
        "No health effects."
    );
    assert_eq!(
        indexes[0]
            .health
            .as_ref()
            .unwrap()
            .advice
            .general_population,
        "Everyone can continue their outdoor activities normally."
    );
    assert_eq!(
        indexes[0]
            .health
            .as_ref()
            .unwrap()
            .advice
            .sensitive_population,
        "Everyone can continue their outdoor activities normally."
    );
    let pollutants = air_current.pollutants.unwrap();
    assert_eq!(pollutants.len(), 5);
    assert_eq!(pollutants[0].code, "pm2p5");
    assert_eq!(pollutants[0].name, "PM 2.5");
    assert_eq!(pollutants[0].full_name, "Fine particulate matter (<2.5µm)");
    assert_eq!(pollutants[0].concentration.value, 11.0);
    assert_eq!(pollutants[0].concentration.unit, "μg/m3");
    assert_eq!(pollutants[0].sub_indexes.as_ref().unwrap().len(), 2);
    assert_eq!(
        pollutants[0].sub_indexes.as_ref().unwrap()[0].code,
        "us-epa"
    );
    assert_eq!(pollutants[0].sub_indexes.as_ref().unwrap()[0].aqi, 46.0);
    assert_eq!(
        pollutants[0].sub_indexes.as_ref().unwrap()[0].aqi_display,
        "46"
    );
}
