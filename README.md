[![crates.io](https://img.shields.io/crates/v/qweather-sdk)](https://crates.io/crates/qweather-sdk)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![crates.io](https://img.shields.io/crates/d/qweather-sdk)](https://crates.io/crates/qweather-sdk)
[![CI](https://github.com/foxzool/qweather-sdk/workflows/CI/badge.svg)](https://github.com/foxzool/qweather-sdk/actions)
[![Documentation](https://docs.rs/qweather-sdk/badge.svg)](https://docs.rs/qweather-sdk)

# 和风天气API SDK

## Example

```no_run
use std::env;

use dotenvy::dotenv;

use qweather_sdk::client::{ClientConfig, QWeatherClient};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();
    let client_config = ClientConfig::new(id, key);
    let client = QWeatherClient::with_config(client_config);

    let resp = client.weather_now("101010100").await.unwrap();
    println!("{:#?}", resp);
}

```

## 已完成的API

- GeoAPI
    - [x] 城市搜索
    - [x] 热门城市查询
    - [x] POI搜索
    - [x] POI范围搜索

- 城市天气
    - [x] 实时天气
    - [x] 每日天气预报
    - [x] 逐小时天气预报
- 分钟预报
    - [x] 分钟级降水
- 格点天气
    - [x] 格点实时天气
    - [x] 格点每日天气预报
    - [x] 格点逐小时天气预报
- 预警
    - [x] 天气灾害预警
    - [x] 天气预警城市列表
- 天气指数
    - [x] 天气指数预报
- 空气质量(beta)
    - 实时空气质量(beta)
    - 监测站数据(beta)