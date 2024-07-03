use std::env;

use dotenvy::dotenv;

use qweather_sdk::client::QWeatherClient;

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let key = env::var("QWEATHER_KEY").unwrap();

    let client = QWeatherClient::new(key, false);
    let resp = client
        .geo_poi_range("116.40528,39.90498", "scenic", Some(10.0), None)
        .await
        .unwrap();
    println!("{:#?}", resp);
}
