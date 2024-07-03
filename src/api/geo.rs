use log::debug;
use url::Url;

use crate::{
    client::QWeatherClient,
    GEO_API_URL,
    model::{DataType, StaticDataResponse},
};

impl QWeatherClient {
    /// 城市搜索
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
}
