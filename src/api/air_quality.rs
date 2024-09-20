use crate::api::decode_iso6801;
use crate::api::utils::{MetaData, RGBA};
use crate::{client::QWeatherClient, APIResult};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::deserialize_number_from_string;
use std::collections::BTreeMap;

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

    /// 空气质量小时预报(new)
    ///
    /// 空气质量小时预报API提供未来24小时空气质量的数据，包括AQI、污染物浓度、分指数以及健康建议。
    ///
    /// 我们推荐阅读空气质量信息文档，以便了解AQI的类型、污染物、支持的国家等信息。
    ///
    /// # Arguments
    ///
    /// * `latitude`: (必选)所需位置的纬度。十进制，最多支持小数点后两位。例如 39.92
    /// * `longitude`: (必选)所需位置的经度。十进制，最多支持小数点后两位。例如 116.41
    ///
    /// returns: Result<APIResponse<AirHourlyForecastResponse>, Error>
    ///
    /// # Examples
    ///
    /// ``` ignore, rust
    ///    let id = env::var("QWEATHER_ID").unwrap();
    ///     let key = env::var("QWEATHER_KEY").unwrap();
    ///     let client_config = ClientConfig::new(id, key);
    ///     let client = QWeatherClient::with_config(client_config);
    ///
    ///     let resp = client.air_hourly_forecast(39.90, 116.40).await.unwrap();
    /// ```
    pub async fn air_hourly_forecast(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> APIResult<AirHourlyForecastResponse> {
        let url = format!(
            "{}/airquality/v1/hourly/{}/{}",
            self.get_api_host(),
            latitude,
            longitude
        );
        let mut params = BTreeMap::new();
        params.insert("latitude".to_string(), latitude.to_string());
        params.insert("longitude".to_string(), longitude.to_string());

        self.request_api(url, params).await
    }

    /// 空气质量每日预报(new)
    ///
    /// 空气质量每日预报API提供未来3天的空气质量（AQI）预报、污染物浓度值和健康建议。
    ///
    /// 我们推荐阅读空气质量信息文档，以便了解AQI的类型、污染物、支持的国家等信息。
    ///
    /// # Arguments
    ///
    /// * `latitude`: (必选)所需位置的纬度。十进制，最多支持小数点后两位。例如 39.92
    /// * `longitude`: (必选)所需位置的经度。十进制，最多支持小数点后两位。例如 116.41
    ///
    /// returns: Result<APIResponse<AirHourlyForecastResponse>, Error>
    ///
    /// # Examples
    ///
    /// ``` ignore, rust
    ///     let id = env::var("QWEATHER_ID").unwrap();
    ///     let key = env::var("QWEATHER_KEY").unwrap();
    ///     let client_config = ClientConfig::new(id, key);
    ///     let client = QWeatherClient::with_config(client_config);
    ///
    ///     let resp = client.air_daily_forecast(39.90, 116.40).await.unwrap();
    /// ```
    pub async fn air_daily_forecast(
        &self,
        latitude: f64,
        longitude: f64,
    ) -> APIResult<AirDailyForecastResponse> {
        let url = format!(
            "{}/airquality/v1/daily/{}/{}",
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

/// 空气质量小时预报(new) 返回值
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AirHourlyForecastResponse {
    /// 数据来源
    pub metadata: MetaData,
    pub hours: Vec<HourlyForecastResponse>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HourlyForecastResponse {
    /// 预报时间，ISO8601格式
    #[serde(deserialize_with = "decode_iso6801")]
    pub forecast_time: DateTime<Utc>,
    /// 空气质量指数
    pub indexes: Vec<AQI>,
    /// 污染物
    pub pollutants: Option<Vec<Pollutant>>,
}

/// 空气质量每日预报(new) 返回值
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AirDailyForecastResponse {
    /// 数据来源
    pub metadata: MetaData,
    pub days: Vec<DailyForecastResponse>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DailyForecastResponse {
    /// 预报数据的开始时间，ISO8601格式
    #[serde(deserialize_with = "decode_iso6801")]
    pub forecast_start_time: DateTime<Utc>,
    /// 预报数据的结束时间，ISO8601格式
    #[serde(deserialize_with = "decode_iso6801")]
    pub forecast_end_time: DateTime<Utc>,
    /// 空气质量指数
    pub indexes: Vec<AQI>,
    /// 污染物
    pub pollutants: Option<Vec<Pollutant>>,
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
    // /// 健康指导意见
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
    pub effect: Option<String>,
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
        indexes[0].health.as_ref().unwrap().effect.as_ref().unwrap(),
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

#[test]
fn test_air_hourly_forecast() {
    let json = serde_json::json!({
      "metadata": {
        "tag": "b1d735802464094bf274fd2165309ddfdab22cec2fa0e644edfcd7f803c2aaad"
      },
      "hours": [
        {
          "forecastTime": "2023-05-17T03:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.4,
              "aqiDisplay": "1.4",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 17.01,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.4,
                  "aqiDisplay": "1.4"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.88,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 4.05,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 6.55,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 49.05,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1,
                  "aqiDisplay": "1"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T04:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.4,
              "aqiDisplay": "1.4",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 16.89,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.4,
                  "aqiDisplay": "1.4"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.84,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 3.91,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 6.21,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 47.75,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1,
                  "aqiDisplay": "1"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T05:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.4,
              "aqiDisplay": "1.4",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 16.56,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.4,
                  "aqiDisplay": "1.4"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.88,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 4.57,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 5.62,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 46.13,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1,
                  "aqiDisplay": "1"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T06:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.4,
              "aqiDisplay": "1.4",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 16.85,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.4,
                  "aqiDisplay": "1.4"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.03,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 5.79,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "o3",
              "name": "O3",
              "fullName": "Ozone",
              "concentration": {
                "value": 3.68,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.1,
                  "aqiDisplay": "0.1"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 45.32,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1,
                  "aqiDisplay": "1"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T07:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.5,
              "aqiDisplay": "1.5",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 17.75,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.5,
                  "aqiDisplay": "1.5"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.06,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 7.12,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "o3",
              "name": "O3",
              "fullName": "Ozone",
              "concentration": {
                "value": 2.66,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.1,
                  "aqiDisplay": "0.1"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 48.14,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1,
                  "aqiDisplay": "1"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T08:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.7,
              "aqiDisplay": "1.7",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 20.19,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.7,
                  "aqiDisplay": "1.7"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.94,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 8.14,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "o3",
              "name": "O3",
              "fullName": "Ozone",
              "concentration": {
                "value": 7.61,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 39.06,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.8,
                  "aqiDisplay": "0.8"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T09:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.6,
              "aqiDisplay": "1.6",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 19.98,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.6,
                  "aqiDisplay": "1.6"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.85,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 6.73,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "o3",
              "name": "O3",
              "fullName": "Ozone",
              "concentration": {
                "value": 25.89,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.6,
                  "aqiDisplay": "0.6"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 32.27,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T10:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.4,
              "aqiDisplay": "1.4",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 16.63,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.4,
                  "aqiDisplay": "1.4"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.92,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 6.03,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "o3",
              "name": "O3",
              "fullName": "Ozone",
              "concentration": {
                "value": 40.21,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.9,
                  "aqiDisplay": "0.9"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 27.15,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.6,
                  "aqiDisplay": "0.6"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T11:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.1,
              "aqiDisplay": "1.1",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 11.35,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1,
                  "aqiDisplay": "1"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.19,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 3.59,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 50.73,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.1,
                  "aqiDisplay": "1.1"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 21.62,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.5,
                  "aqiDisplay": "0.5"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T12:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.3,
              "aqiDisplay": "1.3",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 7.76,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.45,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 2.79,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 60.19,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.3,
                  "aqiDisplay": "1.3"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 13.38,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.3,
                  "aqiDisplay": "0.3"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T13:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.4,
              "aqiDisplay": "1.4",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 6.43,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.6,
                  "aqiDisplay": "0.6"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.37,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 2.08,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 65.1,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.4,
                  "aqiDisplay": "1.4"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 10.41,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.3,
                  "aqiDisplay": "0.3"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T14:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.5,
              "aqiDisplay": "1.5",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 3,
              "aqiDisplay": "3",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 207,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 5.63,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.5,
                  "aqiDisplay": "0.5"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.88,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 1.81,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 74.07,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.5,
                  "aqiDisplay": "1.5"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 9.82,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T15:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.6,
              "aqiDisplay": "1.6",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 3,
              "aqiDisplay": "3",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 207,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 5.49,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.5,
                  "aqiDisplay": "0.5"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.75,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 1.67,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 76.61,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.6,
                  "aqiDisplay": "1.6"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 9.27,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T16:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.6,
              "aqiDisplay": "1.6",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 3,
              "aqiDisplay": "3",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 207,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 5.31,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.5,
                  "aqiDisplay": "0.5"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.55,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 1.6,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 78.22,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.6,
                  "aqiDisplay": "1.6"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 10.08,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.3,
                  "aqiDisplay": "0.3"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T17:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.5,
              "aqiDisplay": "1.5",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 3,
              "aqiDisplay": "3",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 207,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 5.68,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.5,
                  "aqiDisplay": "0.5"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.69,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 1.69,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 74.84,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.5,
                  "aqiDisplay": "1.5"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 12.51,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.3,
                  "aqiDisplay": "0.3"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T18:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.5,
              "aqiDisplay": "1.5",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 3,
              "aqiDisplay": "3",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 207,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 6.61,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.6,
                  "aqiDisplay": "0.6"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.75,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 2.24,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 70.96,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.5,
                  "aqiDisplay": "1.5"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 16.28,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.4,
                  "aqiDisplay": "0.4"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T19:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.3,
              "aqiDisplay": "1.3",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 6.9,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.6,
                  "aqiDisplay": "0.6"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.12,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 3.02,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 61.81,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.3,
                  "aqiDisplay": "1.3"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 23.05,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.5,
                  "aqiDisplay": "0.5"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T20:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.0,
              "aqiDisplay": "1.0",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "o3",
                "name": "O3",
                "fullName": "Ozone"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 8.04,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.21,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 3.71,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 46.33,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1,
                  "aqiDisplay": "1"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 36.13,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.8,
                  "aqiDisplay": "0.8"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T21:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 0.9,
              "aqiDisplay": "0.9",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "so2",
                "name": "SO2",
                "fullName": "Sulfur dioxide"
              },
              "health": {
                "effect": "No health implications.",
                "advice": {
                  "generalPopulation": "Enjoy your outdoor activities.",
                  "sensitivePopulation": "Enjoy your outdoor activities."
                }
              }
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 1,
              "aqiDisplay": "1",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 156,
                "green": 255,
                "blue": 156,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "so2",
                "name": "SO2",
                "fullName": "Sulfur dioxide"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 9.4,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.8,
                  "aqiDisplay": "0.8"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 3.2,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 4.43,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 32.22,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 40.73,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.9,
                  "aqiDisplay": "0.9"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T22:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.1,
              "aqiDisplay": "1.1",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 13.16,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.1,
                  "aqiDisplay": "1.1"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.97,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.2,
                  "aqiDisplay": "0.2"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 4.01,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 30.26,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 39.04,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.8,
                  "aqiDisplay": "0.8"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-17T23:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.2,
              "aqiDisplay": "1.2",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 14.66,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.2,
                  "aqiDisplay": "1.2"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.47,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.1,
                  "aqiDisplay": "0.1"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 3.79,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 31.4,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 38.28,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.8,
                  "aqiDisplay": "0.8"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-18T00:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.2,
              "aqiDisplay": "1.2",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 14.71,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.2,
                  "aqiDisplay": "1.2"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.29,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.1,
                  "aqiDisplay": "0.1"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 3.23,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 30.46,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 38.07,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.8,
                  "aqiDisplay": "0.8"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-18T01:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.2,
              "aqiDisplay": "1.2",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 14.7,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.2,
                  "aqiDisplay": "1.2"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 2.16,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.1,
                  "aqiDisplay": "0.1"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 2.76,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 29.04,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.6,
                  "aqiDisplay": "0.6"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 35.58,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.8,
                  "aqiDisplay": "0.8"
                }
              ]
            }
          ]
        },
        {
          "forecastTime": "2023-05-18T02:00Z",
          "indexes": [
            {
              "code": "qaqi",
              "name": "QAQI",
              "aqi": 1.2,
              "aqiDisplay": "1.2",
              "level": "1",
              "category": "Excellent",
              "color": {
                "red": 195,
                "green": 217,
                "blue": 78,
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
            },
            {
              "code": "gb-defra",
              "name": "DAQI (GB)",
              "aqi": 2,
              "aqiDisplay": "2",
              "level": "1",
              "category": "Low",
              "color": {
                "red": 49,
                "green": 255,
                "blue": 0,
                "alpha": 1
              },
              "primaryPollutant": {
                "code": "pm2p5",
                "name": "PM 2.5",
                "fullName": "Fine particulate matter (<2.5µm)"
              },
              "health": {
                "effect": null,
                "advice": {
                  "generalPopulation": "Enjoy your usual outdoor activities.",
                  "sensitivePopulation": "Enjoy your usual outdoor activities."
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
                "value": 14.18,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 1.2,
                  "aqiDisplay": "1.2"
                }
              ]
            },
            {
              "code": "pm10",
              "name": "PM 10",
              "fullName": "Inhalable particulate matter (<10µm)",
              "concentration": {
                "value": 1.95,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.1,
                  "aqiDisplay": "0.1"
                }
              ]
            },
            {
              "code": "no2",
              "name": "NO2",
              "fullName": "Nitrogen dioxide",
              "concentration": {
                "value": 2.3,
                "unit": "μg/m3"
              },
              "subIndexes": [
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
                "value": 33.35,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            },
            {
              "code": "so2",
              "name": "SO2",
              "fullName": "Sulfur dioxide",
              "concentration": {
                "value": 30.77,
                "unit": "μg/m3"
              },
              "subIndexes": [
                {
                  "code": "qaqi",
                  "aqi": 0.7,
                  "aqiDisplay": "0.7"
                }
              ]
            }
          ]
        }
      ]
    });

    let data: AirHourlyForecastResponse = serde_json::from_value(json).unwrap();

    assert_eq!(
        data.metadata.tag,
        "b1d735802464094bf274fd2165309ddfdab22cec2fa0e644edfcd7f803c2aaad"
    );
}
