use log::debug;
use url::Url;

use crate::{client::QWeatherClient, model::DynamicDataResponse};

impl QWeatherClient {
    /// 分钟级降水
    ///
    /// 分钟级降水API（临近预报）支持中国1公里精度的未来2小时每5分钟降雨预报数据。
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    pub async fn minutely_precipitation(
        &self,
        location: &str,
    ) -> Result<DynamicDataResponse, reqwest::Error> {
        let url = format!("{}/v7/minutely/5m", self.base_url);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request minutely_precipitation {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<DynamicDataResponse>()
            .await
    }
}
