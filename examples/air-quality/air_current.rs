use std::env;

use dotenvy::dotenv;

use qweather_sdk::client::{ClientConfig, QWeatherClient};

/// 实时空气质量(new)
#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();
    let client_config = ClientConfig::new(id, key);
    let client = QWeatherClient::with_config(client_config);

    let resp = client.air_current(39.90, 116.40).await.unwrap();
    println!("{:#?}", resp);
}
