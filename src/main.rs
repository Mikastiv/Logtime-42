use ansi_term::Color;
use chrono::DateTime;
use clap::{App, Arg, ArgMatches};
use config::Config;
use curl::easy::Easy;

use request::Location;

mod config;
mod request;

const LINE_LEN: usize = 29;
const CLAP_CONFIG: &str = "config";
const CLAP_LOGIN: &str = "user";

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

    Ok(sum_time(&locations) / 60.0)
}

// Checks for YYYY-MM-DD
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

fn print_blue_line(len: usize) {
    println!("{}", Color::Blue.bold().paint("─".repeat(len)));
}

fn print_header(config: &Config) {
    let text = format!(
        "From {} to {}",
        Color::Yellow.paint(&config.from),
        Color::Yellow.paint(&config.to)
    );
    print_blue_line(LINE_LEN);
    println!("{}", &text);
    print_blue_line(LINE_LEN);
}

fn print_user_logtime(easy: &mut Easy, login: &str, config: &Config) {
    if let Ok(token) = request::authenticate(easy, &config) {
        print_header(&config);

        match get_user_logtime(easy, &config, &token, login) {
            Ok(time) => {
                let time = format!("{:01.0}h{:02.0}", time.trunc(), time.fract() * 60.0);
                println!(
                    "{:<width$} ➜  🕑 {}",
                    login,
                    Color::Green.bold().paint(&time),
                    width = login.len(),
                );
            }
            Err(e) => {
                // If curl error is set to 0 (curl success code), bad login
                if e.code() == 0 {
                    eprintln!(
                        "{:<width$} ➜  ❌ {}",
                        login,
                        Color::Red.bold().paint("bad login"),
                        width = login.len()
                    );
                }
            }
        }

        print_blue_line(LINE_LEN);
    }
}

fn clap_matches() -> ArgMatches<'static> {
    App::new("42 Gettime")
        .version("0.1.0")
        .author("Mikastiv <mleblanc@student.42quebec.com>")
        .about("View logtime of 42 school users")
        .arg(
            Arg::with_name(CLAP_CONFIG)
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Explicit path of config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(CLAP_LOGIN)
                .short("l")
                .long("login")
                .value_name("LOGIN")
                .help("42 login of the user")
                .takes_value(true),
        )
        .get_matches()
}

fn main() {
    let matches = clap_matches();

    let login = matches.value_of(CLAP_LOGIN);
    let config_file = matches.value_of(CLAP_CONFIG);

    let config = match config::get_config(config_file) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("{}", e);
            std::process::exit(1);
        }
    };

    let login = match login {
        Some(l) => l,
        None => match &config.login {
            Some(l) => l.as_str(),
            None => {
                eprintln!("No login found in config file or options, try --help");
                std::process::exit(1);
            }
        },
    };

    if let Err(msg) = validate_config_dates(&config) {
        eprintln!("{}", msg);
        std::process::exit(1);
    }

    let mut easy = Easy::new();
    print_user_logtime(&mut easy, login, &config);
}
