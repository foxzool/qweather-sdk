use log::debug;
use url::Url;

use crate::{client::QWeatherClient, model::DynamicDataResponse, SDKResult};

impl QWeatherClient {
    /// 实时天气
    ///
    /// 获取中国3000+市县区和海外20万个城市实时天气数据，包括实时温度、体感温度、风力风向、
    /// 相对湿度、大气压强、降水量、能见度、露点温度、云量等。
    ///
    /// # Arguments
    ///
    /// * location(必选)需要查询地区的LocationID或以英文逗号分隔的经度,纬度坐标（十进制，
    ///   最多支持小数点后两位），LocationID可通过GeoAPI获取。例如 location=101010100 或
    ///   location=116.41,39.92
    pub async fn weather_now(&self, location: &str) -> SDKResult<DynamicDataResponse> {
        let url = format!("{}/v7/weather/now", self.base_url);
        let mut url = Url::parse(&url).unwrap();
        url.set_query(Some(&self.query));
        url.query_pairs_mut().append_pair("location", location);

        debug!("request weather_now {}", url);

        self.client
            .get(url)
            .send()
            .await?
            .json::<DynamicDataResponse>()
            .await
    }

    /// 每日天气预报
    ///
    /// 每日天气预报API，提供全球城市未来3-30天天气预报，包括：日出日落、月升月落、最高最低温度、
    /// 天气白天和夜间状况、风力、风速、风向、相对湿度、大气压强、降水量、露点温度、紫外线强度、
    /// 能见度等。
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
    ) -> SDKResult<DynamicDataResponse> {
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
            .json::<DynamicDataResponse>()
            .await
    }

    /// 逐小时天气预报
    ///
    /// 逐小时天气预报API，提供全球城市未来24-168小时逐小时天气预报，包括：温度、天气状况、风力、
    /// 风速、风向、相对湿度、大气压强、降水概率、露点温度、云量。
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
    ) -> SDKResult<DynamicDataResponse> {
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
            .json::<DynamicDataResponse>()
            .await
    }
}
