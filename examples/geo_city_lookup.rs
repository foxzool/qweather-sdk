use std::env;

use dotenvy::dotenv;

use qweather_sdk::{
    api::geo::CityLookupInput,
    client::{ClientConfig, QWeatherClient},
};

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    env_logger::init();
    let id = env::var("QWEATHER_ID").unwrap();
    let key = env::var("QWEATHER_KEY").unwrap();
    let client_config = ClientConfig::new(id, key);
    let client = QWeatherClient::with_config(client_config);

    let city_lookup_input = CityLookupInput {
        location: "beij",
        ..Default::default()
    };
    let resp = client.geo_city_lookup(city_lookup_input).await.unwrap();
    println!("{:#?}", resp);
}
