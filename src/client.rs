use std::collections::BTreeMap;

use log::debug;
use md5::{Digest, Md5};
use reqwest::{Client, ClientBuilder};
use serde_json::Value;

use crate::{api::APIResponse, WEATHER_API_URL, WEATHER_DEV_API_URL};

/// QWeatherClient
pub struct QWeatherClient {
    pub(crate) base_url: String,
    pub(crate) client: Client,
    pub(crate) lang: Option<String>,
    pub(crate) unit: Option<String>,
    /// 基础查询参数
    pub(crate) base_params: BTreeMap<String, String>,
    private_key: String,
}

impl QWeatherClient {
    pub fn new(public_id: impl ToString, private_key: impl ToString, subscription: bool) -> Self {
        let base_url = if subscription {
            WEATHER_API_URL.to_string()
        } else {
            WEATHER_DEV_API_URL.to_string()
        };

        let client = ClientBuilder::new()
            .gzip(true)
            .build()
            .expect("Failed to create reqwest client");

        let mut base_params = BTreeMap::new();
        base_params.insert("publicid".to_string(), public_id.to_string());

        QWeatherClient {
            base_url,
            client,
            lang: None,
            unit: None,
            base_params,
            private_key: private_key.to_string(),
        }
    }

    pub fn set_lang(&mut self, lang: impl ToString) -> &mut Self {
        self.lang = Some(lang.to_string());
        self.base_params
            .insert("lang".to_string(), lang.to_string());
        self
    }

    pub fn set_unit(&mut self, unit: impl ToString) -> &mut Self {
        self.unit = Some(unit.to_string());
        self.base_params
            .insert("unit".to_string(), unit.to_string());
        self
    }

    /// 请求API
    pub async fn request_api<T>(
        &self,
        url: String,
        mut params: BTreeMap<String, String>,
    ) -> Result<APIResponse<T>, reqwest::Error>
    where
        T: serde::de::DeserializeOwned,
    {
        params.insert(
            "t".to_string(),
            chrono::Local::now().timestamp().to_string(),
        );
        let sign = self.sign_params(&params);
        params.insert("sign".to_string(), sign);
        let response = self.client.get(&url).query(&params).send().await?;
        let body: Value = response.json().await?;
        debug!("Response: {}", body.to_string());
        let code = body["code"].as_str().unwrap();
        if code == "200" {
            match serde_json::from_value::<T>(body) {
                Ok(response) => Ok(APIResponse::Success(response)),
                Err(e) => {
                    log::error!("Failed to parse response: {}", e);
                    Ok(APIResponse::Error("Failed to parse response".to_string()))
                }
            }
        } else {
            Ok(APIResponse::Error(code.to_string()))
        }
    }

    /// 签名参数
    fn sign_params(&self, params: &BTreeMap<String, String>) -> String {
        let mut sign = String::new();

        for (key, value) in params {
            if key.to_lowercase() == "sign" || key.to_lowercase() == "key" {
                continue;
            }
            if value.is_empty() {
                continue;
            }
            sign.push_str(&format!("{}={}&", key, value));
        }
        sign.pop();
        sign.push_str(&self.private_key);
        let mut hasher = Md5::new();
        hasher.update(&sign);
        let sign = format!("{:x}", hasher.finalize());

        sign
    }
}
