use reqwest::{Client, ClientBuilder};

use crate::{WEATHER_API_URL, WEATHER_DEV_API_URL};

/// QWeatherClient
pub struct QWeatherClient {
    pub(crate) base_url: String,
    pub(crate) client: Client,
    pub(crate) lang: Option<String>,
    pub(crate) unit: Option<String>,
    pub(crate) query: String,
}

impl QWeatherClient {
    pub fn new(api_key: impl ToString, subscription: bool) -> Self {
        let base_url = if subscription {
            WEATHER_API_URL.to_string()
        } else {
            WEATHER_DEV_API_URL.to_string()
        };

        let client = ClientBuilder::new()
            .gzip(true)
            .build()
            .expect("Failed to create reqwest client");

        let query = format!("key={}", api_key.to_string());

        QWeatherClient {
            base_url,
            client,
            lang: None,
            unit: None,
            query,
        }
    }

    pub fn set_lang(&mut self, lang: impl ToString) -> &mut Self {
        self.lang = Some(lang.to_string());
        self.query = format!("{}&lang={}", self.query, lang.to_string());
        self
    }

    pub fn set_unit(&mut self, unit: impl ToString) -> &mut Self {
        self.unit = Some(unit.to_string());
        self.query = format!("{}&unit={}", self.query, unit.to_string());
        self
    }
}
