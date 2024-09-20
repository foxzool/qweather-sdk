use dotenvy::dotenv;
use std::env;

use qweather_sdk::client::{ClientConfig, QWeatherClient};

/// 空气质量每日预报(new)
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();
    let client_config = ClientConfig::new(id, key);
    let client = QWeatherClient::with_config(client_config);

    let resp = client.air_daily_forecast(39.90, 116.40).await.unwrap();
    println!("{:#?}", resp);
}
