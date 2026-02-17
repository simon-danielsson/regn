use crate::api::request::*;
use home::home_dir;
use std::io;
use std::result::Result::Ok;

#[derive(PartialEq)]
pub enum CurrentCondition {
    Rain,
    Snow,
    Sun,
    Cloud,
    Clear,
    Fog,
    Thunder,
    Unknown,
}

pub struct WeatherAPI {
    pub location: RespLocation,
    pub current_condition: CurrentCondition,
    pub current_condition_as_str: String,
    pub current_temp_c: f64,
    pub forecast_days: Vec<RespForecastDay>,
}

/// this is what gets called from main.rs
pub fn api_main(location: &String, forecast: &i32) -> WeatherAPI {
    let local_key = api_get_local_key();

    let r = api_request(local_key, location.to_string(), forecast)
        .map_err(|_| "Failed to query WeatherAPI. Please check that your API key is valid.")
        .unwrap();

    return WeatherAPI {
        location: r.location,
        current_condition: parse_current_weather(r.current.condition.text.clone()),
        current_condition_as_str: r.current.condition.text,
        current_temp_c: r.current.temp_c,
        forecast_days: r.forecast.forecastday,
    };
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
                "ERROR: No WeatherAPI key was supplied in \"~/.regn\". Please add your accuweather key."
            )
            } else {
                return k;
            }
        }
        _ => {
            panic!(
            "ERROR: \".regn\" does not exist in your home directory. Create this file (\"~/.regn\") and supply your accuweather WeatherAPI key inside it."
        )
        }
    };
}

/// parses the current weather description from the API response
fn parse_current_weather(current: String) -> CurrentCondition {
    let c: &str = current.as_str().trim();
    let cl = c.to_lowercase();

    match cl {
        s if s.contains("sun") => return CurrentCondition::Sun,
        s if s.contains("cloud") => return CurrentCondition::Cloud,
        s if s.contains("snow") | s.contains("blizzard") => return CurrentCondition::Snow,
        s if s.contains("rain") | s.contains("pour") => return CurrentCondition::Rain,
        s if s.contains("clear") => return CurrentCondition::Clear,
        s if s.contains("mist") | s.contains("fog") | s.contains("overcast") => {
            return CurrentCondition::Fog;
        }
        s if s.contains("storm") | s.contains("thunder") => {
            return CurrentCondition::Thunder;
        }
        _ => return CurrentCondition::Unknown,
    }
}
