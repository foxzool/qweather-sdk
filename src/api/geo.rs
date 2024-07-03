use log::debug;
use url::Url;

use crate::{
    client::QWeatherClient,
    GEO_API_URL,
    model::{DataType, StaticDataResponse},
};

impl QWeatherClient {
    /// 城市搜索
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的名称，支持文字、以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位）、LocationID或Adcode（仅限中国城市）。例如 location=北京 或
    ///   location=116.41,39.92
    ///
    ///     模糊搜索，当location传递的为文字时，支持模糊搜索，
    /// 即用户可以只输入城市名称一部分进行搜索，最少一个汉字或2个字符，
    /// 结果将按照相关性和Rank值进行排列，便于开发或用户进行选择他们需要查看哪个城市的天气。
    /// 例如location=bei，将返回与bei相关性最强的若干结果，包括黎巴嫩的贝鲁特和中国的北京市
    ///
    ///     重名，当location传递的为文字时，可能会出现重名的城市，例如陕西省西安市、
    /// 吉林省辽源市下辖的西安区和黑龙江省牡丹江市下辖的西安区，此时会根据Rank值排序返回所有结果。
    /// 在这种情况下，可以通过adm参数的方式进一步确定需要查询的城市或地区，例如location=西安&
    /// adm=黑龙江
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
    ) -> Result<StaticDataResponse<DataType>, reqwest::Error> {
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
            .json::<StaticDataResponse<DataType>>()
            .await
    }

    /// 热门城市查询
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
    ) -> Result<StaticDataResponse<DataType>, reqwest::Error> {
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
            .json::<StaticDataResponse<DataType>>()
            .await
    }

    /// POI搜索
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
    pub async fn geo_poi_lookup(
        &self,
        location: &str,
        type_: &str,
        city: Option<&str>,
        number: Option<u32>,
    ) -> Result<StaticDataResponse<DataType>, reqwest::Error> {
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
            .json::<StaticDataResponse<DataType>>()
            .await
    }
}
