use md5::{Digest, Md5};
use reqwest::{Client, ClientBuilder};
use serde_json::Value;
use std::collections::BTreeMap;

use crate::{api::APIResponse, WEATHER_API_URL, WEATHER_DEV_API_URL};

/// 天气API客户端
pub struct QWeatherClient {
    api_host: String,
    /// 客户端
    client: Client,
    /// 基础查询参数
    base_params: BTreeMap<String, String>,
    /// 客户端配置
    client_config: ClientConfig,
}

/// api 客户端配置
pub struct ClientConfig {
    /// 公钥
    pub public_id: String,
    /// 私钥
    pub private_key: String,
    /// 是否订阅
    pub subscription: bool,
    /// 多语言设置，请阅读[多语言](https://dev.qweather.com/docs/resource/language/)文档，了解我们的多语言是如何工作、如何设置以及数据是否支持多语言。
    pub lang: Option<String>,
    /// 数据单位设置，可选值包括unit=m（公制单位，默认）和unit=i（英制单位）。更多选项和说明参考度[量衡单位](https://dev.qweather.com/docs/resource/unit)。
    pub unit: Option<String>,
}

impl ClientConfig {
    /// 创建新的配置
    pub fn new(public_id: impl ToString, private_key: impl ToString) -> Self {
        ClientConfig {
            public_id: public_id.to_string(),
            private_key: private_key.to_string(),
            subscription: false,
            lang: None,
            unit: None,
        }
    }
}

impl QWeatherClient {
    /// 使用配置创建新的客户端
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
            client_config,
        }
    }

    /// 创建新的客户端
    pub fn new(
        public_id: impl ToString,
        private_key: impl ToString,
        subscription: bool,
        lang: impl ToString,
        unit: impl ToString,
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
            client_config: ClientConfig {
                public_id: public_id.to_string(),
                private_key: private_key.to_string(),
                subscription,
                lang: Some(lang.to_string()),
                unit: Some(unit.to_string()),
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
        // 合并参数
        params.extend(self.base_params.clone());
        params.insert(
            "t".to_string(),
            chrono::Local::now().timestamp().to_string(),
        );
        let sign = self.sign_params(&params);
        params.insert("sign".to_string(), sign);
        match self.client.get(&url).query(&params).send().await {
            Ok(response) => {
                let body: Value = response.json().await?;
                match body["code"].as_str() {
                    Some("200") | None => match serde_json::from_value::<T>(body) {
                        Ok(response) => Ok(APIResponse::Success(response)),
                        Err(e) => {
                            log::error!("Failed to parse response: {}", e);
                            Ok(APIResponse::Error("Failed to parse response".to_string()))
                        }
                    },
                    // v1 error
                    Some(code) => Ok(APIResponse::Error(code.to_string())),
                }
            }
            Err(error) => Ok(APIResponse::Error(error.to_string())),
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
        sign.push_str(&self.client_config.private_key);
        let mut hasher = Md5::new();
        hasher.update(&sign);
        let sign = format!("{:x}", hasher.finalize());

        sign
    }
}
