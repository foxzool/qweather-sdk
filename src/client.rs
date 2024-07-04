use std::collections::BTreeMap;

use log::debug;
use md5::{Digest, Md5};
use reqwest::{Client, ClientBuilder};
use serde_json::Value;

use crate::{api::APIResponse, WEATHER_API_URL, WEATHER_DEV_API_URL};

/// QWeatherClient
pub struct QWeatherClient {
    api_host: String,
    pub(crate) client: Client,
    /// 基础查询参数
    pub(crate) base_params: BTreeMap<String, String>,
    private_key: String,
    client_config: ClientConfig,
}

/// api 客户端配置
pub struct ClientConfig {
    pub public_id: String,
    pub private_key: String,
    pub subscription: bool,
    pub lang: Option<String>,
}

impl ClientConfig {
    /// 创建新的配置
    pub fn new(public_id: impl ToString, private_key: impl ToString) -> Self {
        ClientConfig {
            public_id: public_id.to_string(),
            private_key: private_key.to_string(),
            subscription: false,
            lang: None,
        }
    }
}

impl QWeatherClient {
    pub fn with_config(client_config: ClientConfig) -> Self {
        let api_host = if client_config.subscription {
            WEATHER_API_URL.to_string()
        } else {
            WEATHER_DEV_API_URL.to_string()
        };

        let client = ClientBuilder::new()
            .gzip(true)
            .build()
            .expect("Failed to create reqwest client");

        let mut base_params = BTreeMap::new();
        base_params.insert("publicid".to_string(), client_config.public_id.to_string());

        QWeatherClient {
            api_host,
            client,
            base_params,
            private_key: client_config.private_key.to_string(),
            client_config,
        }
    }

    pub fn new(
        public_id: impl ToString,
        private_key: impl ToString,
        subscription: bool,
        lang: impl ToString,
    ) -> Self {
        let api_host = if subscription {
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
            api_host,
            client,
            base_params,
            private_key: private_key.to_string(),
            client_config: ClientConfig {
                public_id: public_id.to_string(),
                private_key: private_key.to_string(),
                subscription,
                lang: Some(lang.to_string()),
            },
        }
    }

    /// 获取API Host
    pub fn get_api_host(&self) -> &str {
        &self.api_host
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
