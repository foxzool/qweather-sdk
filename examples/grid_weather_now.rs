use dotenvy::dotenv;
use qweather_sdk::client::QWeatherClient;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(key, false);
    let weather_now = client.grid_weather_now("116.41,39.92").await.unwrap();
    println!("{:#?}", weather_now);
}
