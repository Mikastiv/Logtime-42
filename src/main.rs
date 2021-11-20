use std::{collections::HashMap, thread::sleep, time::Duration};

use chrono::DateTime;
use config::Config;
use curl::easy::Easy;

use request::Location;

mod config;
mod request;

#[allow(dead_code)]
fn parse_duration(str: &str) -> u64 {
    let parts: Vec<&str> = str.split(':').collect();
    let hours: u64 = parts[0].parse().unwrap();
    let minutes: u64 = parts[1].parse().unwrap();

    hours * 60 + minutes
}

#[allow(dead_code)]
fn sum_durations(locations: &HashMap<String, String>) -> f64 {
    locations
        .iter()
        .fold(0.0, |acc, (_, dur): (&String, &String)| {
            let minutes = parse_duration(&dur) as f64;

            acc + minutes
        })
}

fn sum_time(locations: &Vec<Location>) -> f64 {
    locations.iter().fold(0.0, |acc, loc: &Location| {
        let (start, end) = match (&loc.begin_at, &loc.end_at) {
            (Some(ref s), Some(ref e)) => {
                let start = DateTime::parse_from_rfc3339(s).unwrap();
                let end = DateTime::parse_from_rfc3339(e).unwrap();
                (start, end)
            }
            _ => return acc,
        };

        let time = end.signed_duration_since(start);
        let minutes = time.num_seconds() as f64 / 60.0;

        acc + minutes
    })
}

fn get_user_logtime(
    easy: &mut Easy,
    config: &Config,
    token: &str,
    login: &str,
) -> Result<f64, curl::Error> {
    let user = request::get_user(easy, token, login)?;
    let locations = request::get_locations(easy, token, user.id, &config)?;

    // Bugged API call (Always returns 500)
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // let locations_stats = request::get_locations_stats(easy, &token, user.id, &config).unwrap();
    // println!("Time: {:.2} hours", sum_durations(&locations_stats) / 60.0);

    Ok(sum_time(&locations) / 60.0)
}

fn valid_date_format(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();

    if parts.len() != 3 {
        return false;
    }

    if parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return false;
    }

    if let Err(_) = parts[0].parse::<u64>() {
        return false;
    }

    if let Err(_) = parts[1].parse::<u64>() {
        return false;
    }

    if let Err(_) = parts[2].parse::<u64>() {
        return false;
    }

    true
}

fn validate_config_dates(config: &Config) -> Result<(), String> {
    if !valid_date_format(&config.from) {
        return Err(
            "Bad date format in config file: 'from' date format must be YYYY-MM-DD".to_string(),
        );
    }
    if !valid_date_format(&config.to) {
        return Err(
            "Bad date format in config file: 'to' date format must be YYYY-MM-DD".to_string(),
        );
    }
    Ok(())
}

fn print_header(config: &Config) {
    let text = format!("From {} to {}", &config.from, &config.to);
    let line = "-".repeat(text.len());
    println!("{}", &line);
    println!("{}", &text);
    println!("{}", &line);
}

fn print_users_logtime(easy: &mut Easy, logins: &Vec<String>, config: &Config) {
    let col_len = logins
        .iter()
        .fold(0, |size, login| std::cmp::max(size, login.len()));

    if let Ok(token) = request::authenticate(easy, &config) {
        print_header(&config);

        for (i, login) in logins.iter().enumerate() {
            match get_user_logtime(easy, &config, &token, login) {
                Ok(time) => {
                    println!(
                        "{:<width$} : {:01.0}h{:02.0}",
                        login,
                        time.trunc(),
                        time.fract() * 60.0,
                        width = col_len,
                    );
                }
                Err(e) => {
                    // If curl error is set to 0 (typically success), bad login
                    if e.code() == 0 {
                        eprintln!("{:<width$} : bad login", login, width = col_len);
                    }
                }
            }

            // Sleep a bit to prevent Too Many Requests error
            if i != logins.len() - 1 {
                sleep(Duration::from_secs_f32(1.0));
            }
        }
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: {} <login 1> <login 2> ... <login n>", &args[0]);
        std::process::exit(1);
    }
    let args: Vec<String> = args.into_iter().skip(1).collect();

    let config = match config::get_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    if let Err(msg) = validate_config_dates(&config) {
        eprintln!("{}", msg);
        std::process::exit(1);
    }

    let mut easy = Easy::new();
    print_users_logtime(&mut easy, &args, &config);
}
