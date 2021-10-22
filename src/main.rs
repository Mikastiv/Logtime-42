use std::fs;
use std::io::Read;

use chrono::DateTime;
use curl::easy::{Easy, List};
use serde::Deserialize;

const URL: &'static str = "https://api.intra.42.fr/";
const BASE_API: &'static str = "https://api.intra.42.fr/v2/";

#[derive(Deserialize)]
struct Auth {
    access_token: String,
}

#[derive(Deserialize)]
struct Config {
    client_id: String,
    secret: String,
    login: String,
    from: String,
    until: String,
}

#[derive(Deserialize, Clone)]
struct User {
    id: u32,
    login: String,
}

#[derive(Deserialize, Debug)]
struct Location {
    begin_at: String,
    end_at: String,
}

fn check_response(easy: &mut Easy) -> Result<(), String> {
    match easy.response_code() {
        Ok(200) => Ok(()),
        Ok(401) => Err(format!("401 Unauthorized")),
        Ok(code) => Err(format!("HTTP return code: {}", code)),
        Err(e) => Err(format!("cURL error code: {}", e)),
    }
}

fn add_authorization(easy: &mut Easy, token: &str) -> Result<(), curl::Error> {
    let mut headers = List::new();

    headers.append(format!("Authorization: Bearer {}", token).as_str())?;
    easy.http_headers(headers)?;

    Ok(())
}

fn send_request(easy: &mut Easy, url: &str) -> Result<Vec<u8>, curl::Error> {
    let mut dst = Vec::new();

    easy.url(url)?;

    {
        let mut transfer = easy.transfer();
        transfer.write_function(|data| {
            dst.extend_from_slice(data);
            Ok(data.len())
        })?;
        transfer.perform()?;
    }

    match check_response(easy) {
        Ok(_) => Ok(dst),
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    }
}

fn authenticate(easy: &mut Easy, config: &Config) -> Result<String, curl::Error> {
    let url = format!(
        "{url}oauth/token?grant_type=client_credentials&client_id={uid}&client_secret={secret}",
        url = URL,
        uid = config.client_id,
        secret = config.secret,
    );

    easy.reset();
    easy.post(true)?;

    let response = send_request(easy, &url)?;
    Ok(serde_json::from_slice::<Auth>(&response)
        .unwrap()
        .access_token)
}

fn get_config() -> Result<Config, String> {
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

fn get_user(easy: &mut Easy, token: &str, login: &str) -> Result<User, curl::Error> {
    let url = format!(
        "{url}users?filter[login]={login}",
        url = BASE_API,
        login = login
    );

    easy.reset();
    add_authorization(easy, token)?;

    let response = send_request(easy, &url)?;
    let users = serde_json::from_slice::<Vec<User>>(&response).unwrap();
    Ok(users.first().unwrap().clone())
}

fn get_locations(
    easy: &mut Easy,
    token: &str,
    user_id: u32,
    start_at: &str,
    end_at: &str,
) -> Result<Vec<Location>, curl::Error> {
    let url = format!(
        "{url}users/{id}/locations?range[begin_at]={start},{end}",
        url = BASE_API,
        id = user_id,
        start = start_at,
        end = end_at
    );

    easy.reset();
    add_authorization(easy, token)?;

    let response = send_request(easy, &url)?;
    let locations = serde_json::from_slice::<Vec<Location>>(&response).unwrap();
    Ok(locations)
}

fn sum_time(locations: &Vec<Location>) -> f64 {
    locations.iter().fold(0.0, |acc, loc: &Location| {
        let start = DateTime::parse_from_rfc3339(&loc.begin_at).unwrap();
        let end = DateTime::parse_from_rfc3339(&loc.end_at).unwrap();

        let time = end.signed_duration_since(start);
        let minutes = time.num_seconds() as f64 / 60.0;

        acc + minutes
    })
}

fn main() {
    let mut easy = Easy::new();
    let config = match get_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    };

    let token = authenticate(&mut easy, &config).unwrap();
    let user = get_user(&mut easy, &token, &config.login).unwrap();
    let start = format!("{}T00%3A00%3A00-05%3A00", &config.from);
    let end = format!("{}T00%3A00%3A00-05%3A00", &config.until);
    let locations = get_locations(&mut easy, &token, user.id, &start, &end).unwrap();

    println!("User: {}", user.login);
    println!("From {} to {}", &config.from, &config.until);
    println!("Time: {:.2} hours", sum_time(&locations) / 60.0);
}
