use log::debug;
use url::Url;

use crate::{
    client::QWeatherClient,
    model::{DataType, DynamicDataResponse},
    SDKResult,
};

impl QWeatherClient {
    /// 实时天气
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    pub async fn weather_now(&self, location: &str) -> SDKResult<DynamicDataResponse<DataType>> {
        let url = format!("{}/v7/weather/now", self.base_url);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_now {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<DynamicDataResponse<DataType>>()
            .await
    }

    /// 每日天气预报
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    ///
    /// * day 天, 只能是 3, 7, 10, 15, 30
    pub async fn weather_daily_forecast(
        &self,
        location: &str,
        day: u8,
    ) -> SDKResult<DynamicDataResponse<DataType>> {
        if ![3u8, 7, 10, 15, 30].contains(&day) {
            panic!("invalid day")
        }
        let url = format!("{}/v7/weather/{}d", self.base_url, day);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_daily_forecast {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<DynamicDataResponse<DataType>>()
            .await
    }

    /// 逐小时天气预报
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    ///
    /// * hour 小时, 只能是 24, 72, 168
    pub async fn weather_hourly_forecast(
        &self,
        location: &str,
        hour: u8,
    ) -> SDKResult<DynamicDataResponse<DataType>> {
        if ![24u8, 72, 168].contains(&hour) {
            panic!("invalid hour")
        }
        let url = format!("{}/v7/weather/{}h", self.base_url, hour);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_hourly_forecast {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<DynamicDataResponse<DataType>>()
            .await
    }
}
