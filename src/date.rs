use chrono::{Datelike, Local};

use crate::config::Config;

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

pub fn validate_config_dates(config: &Config) -> Result<(), String> {
    if let Some(from) = &config.from {
        if !valid_date_format(&from) {
            return Err(
                "Bad date format in config file: 'from' date format must be YYYY-MM-DD".to_string(),
            );
        }
    }
    if let Some(to) = &config.to {
        if !valid_date_format(&to) {
            return Err(
                "Bad date format in config file: 'to' date format must be YYYY-MM-DD".to_string(),
            );
        }
    }
    Ok(())
}

fn is_leap_year(year: i32) -> bool {
    if year % 400 == 0 {
        return true;
    }

    if year % 100 == 0 {
        return false;
    }

    if year % 4 == 0 {
        return true;
    }

    false
}

pub fn current_month_span() -> (String, String) {
    let today = Local::today();
    let start = format!("{:04}-{:02}-{:02}", today.year(), today.month(), 1);
    let last_day: u32 = match today.month() {
        2 if is_leap_year(today.year()) => 29,
        2 => 28,
        4 | 6 | 9 | 11 => 30,
        _ => 31,
    };
    let end = format!("{:04}-{:02}-{:02}", today.year(), today.month(), last_day);
    (start, end)
}

pub fn current_day_span() -> (String, String) {
    let today = Local::today();
    let tomorrow = today.succ();
    let start = format!(
        "{:04}-{:02}-{:02}",
        today.year(),
        today.month(),
        today.day()
    );
    let end = format!(
        "{:04}-{:02}-{:02}",
        tomorrow.year(),
        tomorrow.month(),
        tomorrow.day()
    );

    (start, end)
}

pub fn current_week_span() -> (String, String) {
    let days_from_monday = Local::today().weekday().num_days_from_monday();
    let mut week_start = Local::today();
    for _ in 0..days_from_monday {
        week_start = week_start.pred();
    }

    let mut week_end = week_start.clone();
    for _ in 0..7 {
        week_end = week_end.succ();
    }

    let start = format!(
        "{:04}-{:02}-{:02}",
        week_start.year(),
        week_start.month(),
        week_start.day()
    );
    let end = format!(
        "{:04}-{:02}-{:02}",
        week_end.year(),
        week_end.month(),
        week_end.day()
    );

    (start, end)
}
