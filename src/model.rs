use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DynamicDataResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    ///  当前[API的最近更新时间](https://dev.qweather.com/docs/resource/glossary/#update-time)
    #[serde(deserialize_with = "decode_datetime")]
    pub update_time: DateTime<FixedOffset>,
    /// 数据描述
    pub summary: Option<String>,
    /// 当前数据的响应式页面，便于嵌入网站或应用
    pub fx_link: String,
    #[serde(flatten)]
    pub data: DataType,
    pub refer: Refer,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct StaticDataResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    #[serde(flatten)]
    pub data: DataType,
    pub refer: Refer,
}

/// 数据来源
#[derive(Deserialize, Serialize, Debug)]
pub struct Refer {
    /// 原始数据来源，或数据源说明，可能为空
    pub sources: Vec<String>,
    /// 数据许可或版权声明，可能为空
    pub license: Vec<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum DataType {
    Location {
        location: Vec<Location>,
    },
    TopCityList {
        #[serde(rename = "topCityList")]
        top_city_list: Vec<Location>,
    },
    POI {
        poi: Vec<POI>,
    },
    Minutely {
        minutely: Vec<Minutely>,
    },
}

pub fn decode_datetime<'de, D>(deserializer: D) -> Result<DateTime<FixedOffset>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s = String::deserialize(deserializer)?;
    let dt = DateTime::<FixedOffset>::parse_from_str(&s, "%Y-%m-%dT%H:%M%z").unwrap();
    Ok(dt)
}



/// 地点信息
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    /// 地区/城市名称
    pub name: String,
    /// 地区/城市ID
    pub id: String,
    /// 地区/城市纬度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub lat: f64,
    /// 地区/城市经度
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub lon: f64,
    /// 地区/城市的上级行政区划名称
    pub adm2: String,
    /// 地区/城市所属一级行政区域
    pub adm1: String,
    /// 地区/城市所属国家名称
    pub country: String,
    /// 地区/城市所在[时区](https://dev.qweather.com/docs/resource/glossary/#timezone)
    pub tz: String,
    /// 地区/城市目前与UTC时间偏移的小时数，参考[详细说明](https://dev.qweather.com/docs/resource/glossary/#utc-offset)
    pub utc_offset: String,
    /// 地区/城市是否当前处于[夏令时](https://dev.qweather.com/docs/resource/glossary/#daylight-saving-time)。1 表示当前处于夏令时，0 表示当前不是夏令时。
    #[serde(deserialize_with = "deserialize_bool_from_anything")]
    pub is_dst: bool,
    /// 地区/城市的属性
    pub type_: String,
    /// [地区评分](https://dev.qweather.com/docs/resource/glossary/#rank)
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub rank: i32,
    /// 该地区的天气预报网页链接，便于嵌入你的网站或应用
    pub fx_link: String,
}

/// POI（兴趣点）
pub type POI = Location;

/// 分钟降水
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Minutely {
    /// 预报时间
    #[serde(deserialize_with = "decode_datetime")]
    pub fx_time: DateTime<FixedOffset>,
    /// 5分钟累计降水量，单位毫米
    #[serde(deserialize_with = "deserialize_number_from_string")]
    pub precip: f32,
    /// 降水类型：rain = 雨，snow = 雪
    #[serde(rename = "type")]
    pub type_: String,
}

#[cfg(test)]
mod test {
    use crate::model::{DynamicDataResponse, StaticDataResponse};


    #[test]
    fn test_location() {
        let json_data = r#"{
  "code":"200",
  "location":[
    {
      "name":"北京",
      "id":"101010100",
      "lat":"39.90499",
      "lon":"116.40529",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"10",
      "fxLink":"https://www.qweather.com/weather/beijing-101010100.html"
    },
    {
      "name":"海淀",
      "id":"101010200",
      "lat":"39.95607",
      "lon":"116.31032",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"15",
      "fxLink":"https://www.qweather.com/weather/haidian-101010200.html"
    },
    {
      "name":"朝阳",
      "id":"101010300",
      "lat":"39.92149",
      "lon":"116.48641",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"15",
      "fxLink":"https://www.qweather.com/weather/chaoyang-101010300.html"
    },
    {
      "name":"昌平",
      "id":"101010700",
      "lat":"40.21809",
      "lon":"116.23591",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"23",
      "fxLink":"https://www.qweather.com/weather/changping-101010700.html"
    },
    {
      "name":"房山",
      "id":"101011200",
      "lat":"39.73554",
      "lon":"116.13916",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"23",
      "fxLink":"https://www.qweather.com/weather/fangshan-101011200.html"
    },
    {
      "name":"通州",
      "id":"101010600",
      "lat":"39.90249",
      "lon":"116.65860",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"23",
      "fxLink":"https://www.qweather.com/weather/tongzhou-101010600.html"
    },
    {
      "name":"丰台",
      "id":"101010900",
      "lat":"39.86364",
      "lon":"116.28696",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"25",
      "fxLink":"https://www.qweather.com/weather/fengtai-101010900.html"
    },
    {
      "name":"大兴",
      "id":"101011100",
      "lat":"39.72891",
      "lon":"116.33804",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"25",
      "fxLink":"https://www.qweather.com/weather/daxing-101011100.html"
    },
    {
      "name":"延庆",
      "id":"101010800",
      "lat":"40.46532",
      "lon":"115.98501",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"33",
      "fxLink":"https://www.qweather.com/weather/yanqing-101010800.html"
    },
    {
      "name":"平谷",
      "id":"101011500",
      "lat":"40.14478",
      "lon":"117.11234",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"33",
      "fxLink":"https://www.qweather.com/weather/pinggu-101011500.html"
    }
  ],
  "refer":{
    "sources":[
      "QWeather"
    ],
    "license":[
      "QWeather Developers License"
    ]
  }
}"#;

        let resp = serde_json::from_str::<StaticDataResponse>(json_data);
        assert!(resp.is_ok())
    }

    #[test]
    fn test_city_top() {
        let json_data = r#"{
  "code":"200",
  "topCityList":[
    {
      "name":"北京",
      "id":"101010100",
      "lat":"39.90499",
      "lon":"116.40529",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"10",
      "fxLink":"https://www.qweather.com/weather/beijing-101010100.html"
    },
    {
      "name":"朝阳",
      "id":"101010300",
      "lat":"39.92149",
      "lon":"116.48641",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"15",
      "fxLink":"https://www.qweather.com/weather/chaoyang-101010300.html"
    },
    {
      "name":"海淀",
      "id":"101010200",
      "lat":"39.95607",
      "lon":"116.31032",
      "adm2":"北京",
      "adm1":"北京市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"15",
      "fxLink":"https://www.qweather.com/weather/haidian-101010200.html"
    },
    {
      "name":"深圳",
      "id":"101280601",
      "lat":"22.54700",
      "lon":"114.08595",
      "adm2":"深圳",
      "adm1":"广东省",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"13",
      "fxLink":"https://www.qweather.com/weather/shenzhen-101280601.html"
    },
    {
      "name":"上海",
      "id":"101020100",
      "lat":"31.23171",
      "lon":"121.47264",
      "adm2":"上海",
      "adm1":"上海市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"11",
      "fxLink":"https://www.qweather.com/weather/shanghai-101020100.html"
    },
    {
      "name":"浦东新区",
      "id":"101020600",
      "lat":"31.24594",
      "lon":"121.56770",
      "adm2":"上海",
      "adm1":"上海市",
      "country":"中国",
      "tz":"Asia/Shanghai",
      "utcOffset":"+08:00",
      "isDst":"0",
      "type":"city",
      "rank":"15",
      "fxLink":"https://www.qweather.com/weather/pudong-101020600.html"
    }
  ],
  "refer":{
    "sources":[
      "QWeather"
    ],
    "license":[
      "QWeather Developers License"
    ]
  }
}"#;

        let resp = serde_json::from_str::<StaticDataResponse>(json_data);
        assert!(resp.is_ok())
    }

    #[test]
    fn test_poi_lookup() {
        let json_data = r#"{
  "code": "200",
  "poi": [
    {
      "name": "景山公园",
      "id": "10101010012A",
      "lat": "39.91999",
      "lon": "116.38999",
      "adm2": "北京",
      "adm1": "北京",
      "country": "中国",
      "tz": "Asia/Shanghai",
      "utcOffset": "+08:00",
      "isDst": "0",
      "type": "scenic",
      "rank": "67",
      "fxLink": "https://www.qweather.com"
    },
    {
      "name": "静思园",
      "id": "10119040702A",
      "lat": "31.15999",
      "lon": "120.68000",
      "adm2": "苏州",
      "adm1": "苏州",
      "country": "中国",
      "tz": "Asia/Shanghai",
      "utcOffset": "+08:00",
      "isDst": "0",
      "type": "scenic",
      "rank": "86",
      "fxLink": "https://www.qweather.com"
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

        let resp = serde_json::from_str::<StaticDataResponse>(json_data);
        assert!(resp.is_ok())
    }

    #[test]
    fn test_minutely() {
        let json_data = r#"{
  "code": "200",
  "updateTime": "2021-12-16T18:55+08:00",
  "fxLink": "https://www.qweather.com",
  "summary": "95分钟后雨就停了",
  "minutely": [
    {
      "fxTime": "2021-12-16T18:55+08:00",
      "precip": "0.15",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:00+08:00",
      "precip": "0.23",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:05+08:00",
      "precip": "0.21",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:10+08:00",
      "precip": "0.17",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:15+08:00",
      "precip": "0.18",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:20+08:00",
      "precip": "0.24",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:25+08:00",
      "precip": "0.31",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:30+08:00",
      "precip": "0.37",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:35+08:00",
      "precip": "0.41",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:40+08:00",
      "precip": "0.43",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:45+08:00",
      "precip": "0.41",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:50+08:00",
      "precip": "0.36",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T19:55+08:00",
      "precip": "0.32",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:00+08:00",
      "precip": "0.27",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:05+08:00",
      "precip": "0.22",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:10+08:00",
      "precip": "0.17",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:15+08:00",
      "precip": "0.11",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:20+08:00",
      "precip": "0.06",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:25+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:30+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:35+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:40+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:45+08:00",
      "precip": "0.0",
      "type": "rain"
    },
    {
      "fxTime": "2021-12-16T20:50+08:00",
      "precip": "0.0",
      "type": "rain"
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

        let resp = serde_json::from_str::<DynamicDataResponse>(json_data);
        assert!(resp.is_ok())
    }
}
