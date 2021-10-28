use std::fs;
use std::io::Read;

use serde::Deserialize;

pub const PAGE_SIZE: u32 = 500;

#[derive(Deserialize)]
pub struct Config {
    pub client_id: String,
    pub secret: String,
    pub from: String,
    pub to: String,
}

pub fn get_config() -> Result<Config, String> {
    let config_file = match fs::File::open("config.json") {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(_) => {
            return Err(String::from("Cannot open file 'config.json'"));
        }
    };

    match serde_json::from_str(&config_file) {
        Ok(c) => Ok(c),
        Err(e) => {
            return Err(format!("config.json: {}", e));
        }
    }
}
