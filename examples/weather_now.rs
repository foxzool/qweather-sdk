use std::env;

use qweather_sdk::client::QWeatherClient;

#[tokio::main]
async fn main() {
    env_logger::init();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(key, false);
    let weather_now = client.weather_now("101010100").await.unwrap();
    println!("{:#?}", weather_now.data);
}
