use chrono::{DateTime, Datelike, Local, LocalResult, TimeZone, Utc};

// Checks for YYYY-MM-DD
pub fn valid_format(date: &str) -> bool {
    let parts: Vec<&str> = date.split('-').collect();

    if parts.len() != 3 {
        return false;
    }

    if parts[0].len() != 4 || parts[1].len() != 2 || parts[2].len() != 2 {
        return false;
    }

    let year = match parts[0].parse() {
        Ok(y) => y,
        Err(_) => return false,
    };

    let month = match parts[1].parse() {
        Ok(m) => m,
        Err(_) => return false,
    };

    let day = match parts[2].parse() {
        Ok(d) => d,
        Err(_) => return false,
    };

    !matches!(Local.ymd_opt(year, month, day), LocalResult::None)
}

fn to_utc_time(local_time: DateTime<Local>) -> DateTime<Utc> {
    DateTime::from(local_time)
}

pub fn current_month_span() -> (String, String) {
    let today = Local::today();
    let (end_month, end_year) = match today.month() {
        12 => (1, today.year() + 1),
        m => (m + 1, today.year()),
    };
    (
        to_utc_time(Local.ymd(today.year(), today.month(), 1).and_hms(0, 0, 0)).to_rfc3339(),
        to_utc_time(Local.ymd(end_year, end_month, 1).and_hms(0, 0, 0)).to_rfc3339(),
    )
}

pub fn current_day_span() -> (String, String) {
    let today = Local::today().and_hms(0, 0, 0);
    let tomorrow = Local::today().succ().and_hms(0, 0, 0);

    (
        to_utc_time(today).to_rfc3339(),
        to_utc_time(tomorrow).to_rfc3339(),
    )
}

pub fn current_week_span() -> (String, String) {
    let days_from_monday = Local::today().weekday().num_days_from_monday();
    let mut week_start = Local::today();
    for _ in 0..days_from_monday {
        week_start = week_start.pred();
    }

    let mut week_end = week_start;
    for _ in 0..7 {
        week_end = week_end.succ();
    }

    (
        to_utc_time(week_start.and_hms(0, 0, 0)).to_rfc3339(),
        to_utc_time(week_end.and_hms(0, 0, 0)).to_rfc3339(),
    )
}

pub fn parse_date(s: &str) -> DateTime<Local> {
    let mut parts = s.split('-');
    let year = parts.next().unwrap();
    let month = parts.next().unwrap();
    let day = parts.next().unwrap();

    Local
        .ymd(
            year.parse().unwrap(),
            month.parse().unwrap(),
            day.parse().unwrap(),
        )
        .and_hms(0, 0, 0)
}
