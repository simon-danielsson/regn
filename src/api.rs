use std::io;

use home::home_dir;

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
    local_api_key: String,
}

impl API {
    fn new(current: CurrentWeather, local_api_key: String) -> Self {
        Self {
            current,
            local_api_key,
        }
    }
}

pub fn api_main() -> API {
    return API::new(CurrentWeather::Rain, api_get_local_key());
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
