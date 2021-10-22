use chrono::DateTime;
use curl::easy::Easy;

use request::Location;

mod config;
mod request;

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

    println!("User: {}", user.login);
    println!("From {} to {}", &config.from, &config.to);
    println!("Time: {:.2} hours", sum_time(&locations) / 60.0);
}
