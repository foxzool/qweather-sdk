use std::env;

use dotenvy::dotenv;

use qweather_sdk::{
    api::geo::GeoPoiRangeInput,
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

    let geo_poi_range_input = GeoPoiRangeInput {
        location: "116.40528,39.90498",
        type_: "scenic",
        radius: Some(10.0),
        ..Default::default()
    };

    let resp = client.geo_poi_range(geo_poi_range_input).await.unwrap();
    println!("{:#?}", resp);
}
