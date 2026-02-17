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
pub struct RespLocation {
    pub name: String,
    pub country: String,
    pub localtime: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RespCurrent {
    pub temp_c: f64,
    pub condition: RespCondition,
}

#[derive(Debug, Deserialize)]
pub struct RespForecast {
    pub forecastday: Vec<RespForecastDay>,
}

#[derive(Debug, Deserialize)]
pub struct RespForecastDay {
    pub date: String,
    pub day: RespDay,
}

#[derive(Debug, Deserialize)]
pub struct RespDay {
    pub maxtemp_c: f64,
    pub mintemp_c: f64,
    pub condition: RespCondition,
}

#[derive(Debug, Deserialize, Clone)]
pub struct RespCondition {
    pub text: String,
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
