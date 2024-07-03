[![crates.io](https://img.shields.io/crates/v/qweather-sdk)](https://crates.io/crates/qweather-sdk)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/Seldom-SE/seldom_pixel#license)
[![crates.io](https://img.shields.io/crates/d/qweather-sdk)](https://crates.io/crates/qweather-sdk)
[![CI](https://github.com/foxzool/qweather-sdk/workflows/CI/badge.svg)](https://github.com/foxzool/qweather-sdk/actions)
[![Documentation](https://docs.rs/qweather-sdk/badge.svg)](https://docs.rs/qweather-sdk)

# 和风天气API SDK

## Example

```no_run
use dotenvy::dotenv;
use qweather_sdk::client::QWeatherClient;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(key, false);
    let weather_now = client.weather_now("101010100").await.unwrap();
    println!("{:#?}", weather_now);
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