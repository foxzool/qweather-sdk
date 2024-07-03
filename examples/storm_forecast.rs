use std::env;

use dotenvy::dotenv;

use qweather_sdk::client::QWeatherClient;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(id, key, false);
    let resp = client.storm_forecast("NP_2106").await.unwrap();
    println!("{:#?}", resp);
}
