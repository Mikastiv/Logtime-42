#![allow(dead_code)]

use std::{collections::HashMap, thread::sleep, time::Duration};

use chrono::DateTime;
use config::Config;
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
    locations
        .iter()
        .fold(0.0, |acc, (_, dur): (&String, &String)| {
            let minutes = parse_duration(&dur) as f64;

            acc + minutes
        })
}

fn print_user_logtime(easy: &mut Easy, config: &Config, login: &str) -> Result<(), curl::Error> {
    let token = request::authenticate(easy, &config)?;
    let user = request::get_user(easy, &token, login)?;
    let locations = request::get_locations(easy, &token, user.id, &config)?;

    // Bugged API call (Always returns 500)
    // std::thread::sleep(std::time::Duration::from_secs(1));
    // let locations_stats = request::get_locations_stats(easy, &token, user.id, &config).unwrap();

    println!("User: {}", user.login);
    println!("From {} to {}", &config.from, &config.to);
    println!("Time: {:.2} hours", sum_time(&locations) / 60.0);
    // println!("Time: {:.2} hours", sum_durations(&locations_stats) / 60.0);

    Ok(())
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
            std::process::exit(1)
        }
    };

    let mut easy = Easy::new();
    for (i, login) in args.iter().enumerate() {
        print_user_logtime(&mut easy, &config, login).unwrap_or_else(|_| {});

        if i != args.len() - 1 {
            println!();
            sleep(Duration::from_secs_f32(0.5));
        }
    }
}
