use home::home_dir;
use std::io;

#[derive(PartialEq)]
pub enum CurrentWeather {
    Rain,
    Snow,
    Sun,
    Cloud,
    Clear,
    Fog,
    Thunder,
}

pub struct API {
    pub current: CurrentWeather,
}

impl API {
    fn new(current: CurrentWeather) -> Self {
        Self { current }
    }
}

pub fn api_main(location: String) -> API {
    let local_key = api_get_local_key();
    return API::new(CurrentWeather::Rain);
}

/// helper: api_get_local_key
fn api_read_local_file() -> io::Result<String> {
    let h: String = home_dir().unwrap().display().to_string();
    let d: String = format!("{}/.regn", h);
    std::fs::read_to_string(d)
}

fn api_get_local_key() -> String {
    match api_read_local_file() {
        Ok(key) => {
            let k: String = key.trim().to_string();
            if k.is_empty() {
                panic!(
                "ERROR: No API key was supplied in \"~/.regn\". Please add your accuweather key."
            )
            } else {
                return k;
            }
        }
        _ => {
            panic!(
            "ERROR: \".regn\" does not exist in your home directory. Create this file (\"~/.regn\") and supply your accuweather API key inside it."
        )
        }
    };
}

// use home::home_dir;
// use serde::Deserialize;
// use std::io;
//
// #[derive(PartialEq, Debug, Clone, Copy)]
// pub enum CurrentWeather {
//     Rain,
//     Snow,
//     Sun,
//     Cloud,
//     Clear,
//     Fog,
//     Thunder,
// }
//
// pub struct API {
//     pub current: CurrentWeather,
// }
//
// impl API {
//     fn new(current: CurrentWeather) -> Self {
//         Self { current }
//     }
// }
//
// // --- AccuWeather DTOs (minimal) ---
//
// #[derive(Debug, Deserialize)]
// struct ForecastResponse {
//     #[serde(rename = "DailyForecasts")]
//     daily: Vec<DailyForecastDto>,
// }
//
// #[derive(Debug, Deserialize)]
// struct DailyForecastDto {
//     #[serde(rename = "Day")]
//     day: DayNightDto,
//     #[serde(rename = "Night")]
//     night: DayNightDto,
// }
//
// #[derive(Debug, Deserialize)]
// struct DayNightDto {
//     #[serde(rename = "IconPhrase")]
//     phrase: String,
// }
//
// // --- Public entry point ---
// // Make this async, because it does network I/O.
// pub async fn api_main(location_key: &str) -> anyhow::Result<API> {
//     let api_key: String = api_get_local_key();
//
//     let forecast = api_request_7day(location_key, &api_key).await?;
//
//     // Example: map today's "day phrase" into your enum
//     let today_phrase = forecast
//         .daily
//         .get(0)
//         .map(|d| d.day.phrase.as_str())
//         .unwrap_or("Unknown");
//
//     let current = map_phrase_to_weather(today_phrase);
//
//     Ok(API::new(current))
// }
//
// async fn api_request_7day(location_key: &str, api_key: &str) -> anyhow::Result<ForecastResponse> {
//     let url = format!(
//         "https://dataservice.accuweather.com/forecasts/v1/daily/7day/{}?apikey={}&metric=true",
//         location_key, api_key
//     );
//
//     let resp = reqwest::get(url).await?;
//
//     if !resp.status().is_success() {
//         // Helpful error message if key/quota/etc is wrong
//         let status = resp.status();
//         let body = resp.text().await.unwrap_or_default();
//         anyhow::bail!("AccuWeather request failed: {status}. Body: {body}");
//     }
//
//     let forecast = resp.json::<ForecastResponse>().await?;
//     Ok(forecast)
// }
//
// fn map_phrase_to_weather(phrase: &str) -> CurrentWeather {
//     let p = phrase.to_lowercase();
//
//     if p.contains("thunder") || p.contains("t-storm") {
//         CurrentWeather::Thunder
//     } else if p.contains("snow") || p.contains("flurr") || p.contains("sleet") {
//         CurrentWeather::Snow
//     } else if p.contains("rain") || p.contains("shower") || p.contains("drizzle") {
//         CurrentWeather::Rain
//     } else if p.contains("fog") || p.contains("mist") || p.contains("haze") {
//         CurrentWeather::Fog
//     } else if p.contains("cloud") || p.contains("overcast") {
//         CurrentWeather::Cloud
//     } else if p.contains("sun") {
//         CurrentWeather::Sun
//     } else {
//         CurrentWeather::Clear
//     }
// }
//
// /// helper: api_get_local_key
// fn api_read_local_file() -> io::Result<String> {
//     let h = home_dir().expect("Could not resolve home directory");
//     let d = h.join(".regn");
//     std::fs::read_to_string(d)
// }
//
// fn api_get_local_key() -> String {
//     match api_read_local_file() {
//         Ok(key) => {
//             let k = key.trim().to_string();
//             if k.is_empty() {
//                 panic!(
//                 "ERROR: No API key was supplied in \"~/.regn\". Please add your AccuWeather key."
//             )
//             } else {
//                 k
//             }
//         }
//         _ => {
//             panic!(
//             "ERROR: \".regn\" does not exist in your home directory. Create this file (\"~/.regn\") and supply your AccuWeather API key inside it."
//         )
//         }
//     }
// }
