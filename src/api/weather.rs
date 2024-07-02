use log::debug;
use url::Url;

use crate::{
    client::QWeatherClient,
    model::ApiResponse,
    SDKResult,
};
use crate::model::DataType;

impl QWeatherClient {
    /// 实时天气
    pub async fn weather_now(&self, location: &str) -> SDKResult<ApiResponse<DataType>> {
        let url = format!("{}/v7/weather/now", self.base_url);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_now {}", url);


        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<ApiResponse<DataType>>()
            .await?)
    }

    /// 每日天气预报
    pub async fn weather_daily_forecast(&self, location: &str, day: u8) -> SDKResult<ApiResponse<DataType>> {
        let url = format!("{}/v7/weather/{}d", self.base_url, day);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_daily_forecast {}", url);


        Ok(self
            .client
            .get(url)
            .send()
            .await?
            .json::<ApiResponse<DataType>>()
            .await?)
    }
}
