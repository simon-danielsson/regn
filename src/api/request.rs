use reqwest::Error;
use serde::Deserialize;
use std::result::Result::Ok;

#[derive(Debug, Deserialize)]
pub struct WeatherResponse {
    pub location: RespLocation,
    pub current: RespCurrent,
    pub forecast: RespForecast,
}

#[derive(Debug, Deserialize)]
struct RespLocation {
    name: String,
    country: String,
    localtime: String,
}

#[derive(Debug, Deserialize)]
pub struct RespCurrent {
    pub temp_c: f64,
    pub condition: String,
}

#[derive(Debug, Deserialize)]
pub struct RespForecast {
    forecastday: Vec<RespForecastDay>,
}

#[derive(Debug, Deserialize)]
struct RespForecastDay {
    date: String,
    day: RespDay,
}

#[derive(Debug, Deserialize)]
struct RespDay {
    maxtemp_c: f64,
    mintemp_c: f64,
    condition: String,
}

#[tokio::main]
pub async fn api_request(local_key: String, location: String) -> Result<WeatherResponse, Error> {
    let url = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=3&aqi=no&alerts=no",
        local_key, location
    );
    let r = reqwest::get(&url).await?.json::<WeatherResponse>().await?;
    Ok(r)
}
