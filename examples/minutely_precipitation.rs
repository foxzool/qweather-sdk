use std::env;

use dotenvy::dotenv;

use qweather_sdk::client::QWeatherClient;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(id, key, false, "zh");
    let resp = client.minutely_precipitation("116.38,39.91").await.unwrap();
    println!("{:#?}", resp);
}
