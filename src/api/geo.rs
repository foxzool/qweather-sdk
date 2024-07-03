use log::debug;
use serde::{Deserialize, Serialize};
use serde_aux::prelude::{deserialize_bool_from_anything, deserialize_number_from_string};
use url::Url;

use crate::{
    client::QWeatherClient,
    GEO_API_URL,
    model::{Refer, StaticDataResponse},
};

impl QWeatherClient {
    /// 城市搜索
    ///
    /// 城市搜索API提供全球地理位位置、全球城市搜索服务，支持经纬度坐标反查、多语言、
    /// 模糊搜索等功能。
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的名称，支持文字、以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）、LocationID或Adcode（仅限中国城市）。例如 location=北京 或
    ///   location=116.41,39.92
    ///
    ///   *模糊搜索*，当location传递的为文字时，支持模糊搜索，
    ///   即用户可以只输入城市名称一部分进行搜索，最少一个汉字或2个字符，
    ///   结果将按照相关性和Rank值进行排列，便于开发或用户进行选择他们需要查看哪个城市的天气。
    ///   例如location=bei，将返回与bei相关性最强的若干结果，包括黎巴嫩的贝鲁特和中国的北京市
    ///
    ///   *重名*，当location传递的为文字时，可能会出现重名的城市，例如陕西省西安市、
    ///   吉林省辽源市下辖的西安区和黑龙江省牡丹江市下辖的西安区，此时会根据Rank值排序返回所有结果。
    ///   在这种情况下，可以通过adm参数的方式进一步确定需要查询的城市或地区，例如location=西安&
    ///   adm=黑龙江
    ///
    /// * adm 城市的上级行政区划，可设定只在某个行政区划范围内进行搜索，
    ///   用于排除重名城市或对结果进行过滤。例如 adm=beijing
    ///
    /// * range 搜索范围，可设定只在某个国家或地区范围内进行搜索，国家和地区名称需使用ISO 3166
    ///   所定义的国家代码。如果不设置此参数，搜索范围将在所有城市。例如 range=cn
    ///
    /// * number返回结果的数量，取值范围1-20，默认返回10个结果。
    pub async fn geo_city_lookup(
        &self,
        location: &str,
        adm: Option<&str>,
        range: Option<&str>,
        number: Option<u32>,
    ) -> Result<CityLookupResponse, reqwest::Error> {
        let url = format!("{}/v2/city/lookup", GEO_API_URL);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);
        if let Some(adm) = adm {
            url.query_pairs_mut().append_pair("adm", adm);
        }
        if let Some(range) = range {
            url.query_pairs_mut().append_pair("range", range);
        }
        if let Some(number) = number {
            url.query_pairs_mut()
                .append_pair("number", &number.to_string());
        }

        debug!("request city_lookup {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json()
            .await
    }

    /// 热门城市查询
    ///
    /// 获取全球各国热门城市列表。
    ///
    /// # Arguments
    ///
    /// * range搜索范围，可设定只在某个国家或地区范围内进行搜索，国家和地区名称需使用ISO 3166
    ///   所定义的国家代码。如果不设置此参数，搜索范围将在所有城市。例如 range=cn
    ///
    /// * number 返回结果的数量，取值范围1-20，默认返回10个结果。
    pub async fn geo_city_top(
        &self,
        range: Option<&str>,
        number: Option<u32>,
    ) -> Result<StaticDataResponse, reqwest::Error> {
        let url = format!("{}/v2/city/top", GEO_API_URL);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        if let Some(range) = range {
            url.query_pairs_mut().append_pair("range", range);
        }
        if let Some(number) = number {
            url.query_pairs_mut()
                .append_pair("number", &number.to_string());
        }

        debug!("request geo_city_top {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<StaticDataResponse>()
            .await
    }

    /// POI搜索
    ///
    /// 使用关键字和坐标查询POI信息（景点、火车站、飞机场、港口等）
    ///
    /// # Arguments
    ///
    /// * location(必选) 需要查询地区的名称，支持文字、以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）、LocationID或Adcode（仅限中国城市）。例如 location=北京 或
    ///   location=116.41,39.92
    ///
    /// * type_ POI类型，可选择搜索某一类型的POI。
    ///     * scenic 景点
    ///     * CSTA 潮流站点
    ///     * TSTA 潮汐站点
    ///
    /// * city 选择POI所在城市，可设定只搜索在特定城市内的POI信息。
    ///   城市名称可以是文字或城市的LocationID。默认不限制特定城市。
    ///
    /// * number 返回结果的数量，取值范围1-20，默认返回10个结果。
    pub async fn geo_poi_lookup(
        &self,
        location: &str,
        type_: &str,
        city: Option<&str>,
        number: Option<u32>,
    ) -> Result<StaticDataResponse, reqwest::Error> {
        let url = format!("{}/v2/poi/lookup", GEO_API_URL);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);
        url.query_pairs_mut().append_pair("type", type_);
        if let Some(city) = city {
            url.query_pairs_mut().append_pair("city", city);
        }
        if let Some(number) = number {
            url.query_pairs_mut()
                .append_pair("number", &number.to_string());
        }

        debug!("request geo_poi_lookup {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<StaticDataResponse>()
            .await
    }

    /// POI范围搜索
    ///
    /// 提供指定区域范围内查询所有POI信息。
    ///
    /// # Arguments
    ///
    /// * location(必选) 需要查询地区的名称，支持文字、以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）、LocationID或Adcode（仅限中国城市）。例如 location=北京 或
    ///   location=116.41,39.92
    ///
    /// * type_ POI类型，可选择搜索某一类型的POI。
    ///     * scenic 景点
    ///     * CSTA 潮流站点
    ///     * TSTA 潮汐站点
    ///
    /// * radius 搜索范围，可设置搜索半径，取值范围1-50，单位：公里。默认5公里。
    ///
    /// * number 返回结果的数量，取值范围1-20，默认返回10个结果。
    pub async fn geo_poi_range(
        &self,
        location: &str,
        type_: &str,
        radius: Option<f32>,
        number: Option<u32>,
    ) -> Result<StaticDataResponse, reqwest::Error> {
        let url = format!("{}/v2/poi/range", GEO_API_URL);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);
        url.query_pairs_mut().append_pair("type", type_);
        if let Some(radius) = radius {
            url.query_pairs_mut()
                .append_pair("radius", &radius.to_string());
        }
        if let Some(number) = number {
            url.query_pairs_mut()
                .append_pair("number", &number.to_string());
        }

        debug!("request geo_poi_range {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<StaticDataResponse>()
            .await
    }
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

/// 城市搜索返回值
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CityLookupResponse {
    /// 请参考[状态码](https://dev.qweather.com/docs/resource/status-code/)
    pub code: String,
    pub location: Vec<Location>,
    pub refer: Refer,
}

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

    let resp = serde_json::from_str::<CityLookupResponse>(json_data).unwrap();
    assert_eq!(resp.code, "200");

}
