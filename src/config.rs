use std::fs;
use std::io::Read;

use serde::Deserialize;

pub const PAGE_SIZE: usize = 10000;
const CONFIG_FILE: &str = "config.json";

#[derive(Deserialize)]
pub struct Config {
    pub client_id: String,
    pub secret: String,
    pub from: String,
    pub to: String,
}

pub fn get_config() -> Result<Config, String> {
    let config_file = match fs::File::open(CONFIG_FILE) {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(_) => {
            return Err(format!("Cannot open file '{}'", CONFIG_FILE));
        }
    };

    match serde_json::from_str(&config_file) {
        Ok(c) => Ok(c),
        Err(e) => {
            return Err(format!("{}: {}", CONFIG_FILE, e));
        }
    }
}
