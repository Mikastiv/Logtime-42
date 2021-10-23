#![allow(dead_code)]

use std::collections::HashMap;

use chrono::DateTime;
use curl::easy::Easy;

use request::Location;

mod config;
mod request;

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

fn parse_duration(str: &str) -> u64 {
    let parts: Vec<&str> = str.split(':').collect();
    let hours: u64 = parts[0].parse().unwrap();
    let minutes: u64 = parts[1].parse().unwrap();

    hours * 60 + minutes
}

fn sum_durations(locations: &HashMap<String, String>) -> f64 {
    locations.iter().fold(0.0, |acc, (_, dur): (&String, &String)| {
        let minutes = parse_duration(&dur) as f64;

        acc + minutes
    })
}

fn main() {
    let mut easy = Easy::new();
    let config = match config::get_config() {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1)
        }
    };

    let token = request::authenticate(&mut easy, &config).unwrap();
    let user = request::get_user(&mut easy, &token, &config.login).unwrap();
    let locations = request::get_locations(&mut easy, &token, user.id, &config).unwrap();
    // Bugged API call (Always returns 500)
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // let locations_stats = request::get_locations_stats(&mut easy, &token, user.id, &config).unwrap();

    println!("User: {}", user.login);
    println!("From {} to {}", &config.from, &config.to);
    println!("Time: {:.2} hours", sum_time(&locations) / 60.0);
    // println!("Time: {:.2} hours", sum_durations(&locations_stats) / 60.0);
}
