use dotenvy::dotenv;
use qweather_sdk::client::QWeatherClient;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(id, key, false, "zh");
    let resp = client
        .grid_weather_daily_forecast("116.41,39.92", 3)
        .await
        .unwrap();
    println!("{:#?}", resp);
}
