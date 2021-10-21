use std::fs;
use std::io::Read;

use curl::easy::Easy;
use serde::Deserialize;

#[derive(Deserialize)]
struct Auth {
    access_token: String,
}

#[derive(Deserialize)]
struct Config {
    client_id: String,
    secret: String,
}

fn get_token(buf: &[u8]) -> Result<String, serde_json::Error> {
    Ok(serde_json::from_slice::<Auth>(&buf)?.access_token)
}

fn authenticate(easy: &mut Easy, config: &Config) -> Result<Vec<u8>, curl::Error> {
    let url = "https://api.intra.42.fr/";
    let token_uri = format!(
        "{url}oauth/token?grant_type=client_credentials&client_id={uid}&client_secret={secret}",
        url = url,
        uid = config.client_id,
        secret = config.secret,
    );
    let mut dst = Vec::new();

    easy.url(token_uri.as_str())?;
    easy.post(true)?;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    match easy.response_code() {
        Ok(200) => Ok(dst),
        Ok(401) => {
            eprintln!("401 Unauthorized");
            std::process::exit(1);
        }
        Ok(code) => {
            eprintln!("HTTP return code: {}", code);
            std::process::exit(1);
        }
        Err(e) => Err(e),
    }
}

fn main() {
    let mut easy = Easy::new();
    let config_file = match fs::File::open("config.json") {
        Ok(mut file) => {
            let mut content = String::new();
            file.read_to_string(&mut content).unwrap();
            content
        }
        Err(_) => {
            eprintln!("Cannot open file 'config.json'");
            std::process::exit(1);
        }
    };

    let config = match serde_json::from_str(&config_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("config.json: {}", e);
            std::process::exit(1);
        }
    };

    let response = authenticate(&mut easy, &config).unwrap();
    let token = get_token(&response).unwrap();

    println!("Token: {}", token);
}
