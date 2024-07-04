use dotenvy::dotenv;
use qweather_sdk::client::{ClientConfig, QWeatherClient};
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();
    let client_config = ClientConfig::new(id, key);
    let client = QWeatherClient::with_config(client_config);

    let resp = client
        .grid_weather_daily_forecast("116.41,39.92", 3)
        .await
        .unwrap();
    println!("{:#?}", resp);
}
