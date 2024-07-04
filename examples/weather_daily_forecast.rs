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

    let resp = client.weather_daily_forecast("101010100", 3).await.unwrap();
    println!("{:#?}", resp);
}
