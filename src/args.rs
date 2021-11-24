use clap::{App, Arg, ArgMatches};

pub const ARG_CONFIG: &str = "config";
pub const ARG_LOGIN: &str = "user";
pub const ARG_CUR_MONTH: &str = "month";
pub const ARG_CUR_WEEK: &str = "week";
pub const ARG_CUR_DAY: &str = "day";

pub fn matches() -> ArgMatches<'static> {
    App::new("42 GetTime")
        .author("Mikastiv <mleblanc@student.42quebec.com>")
        .about("View logtime of a 42 school student")
        .version("0.1.0")
        .arg(
            Arg::with_name(ARG_CONFIG)
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Explicit path of config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_LOGIN)
                .short("l")
                .long("login")
                .value_name("LOGIN")
                .help("42 login of the user")
                .takes_value(true),
        )
        .arg(
            Arg::with_name(ARG_CUR_MONTH)
                .short("m")
                .long("month")
                .help("Logtime of the current month")
                .conflicts_with(ARG_CUR_DAY)
                .conflicts_with(ARG_CUR_WEEK),
        )
        .arg(
            Arg::with_name(ARG_CUR_WEEK)
                .short("w")
                .long("week")
                .help("Logtime of the current week")
                .conflicts_with(ARG_CUR_DAY)
                .conflicts_with(ARG_CUR_MONTH),
        )
        .arg(
            Arg::with_name(ARG_CUR_DAY)
                .short("d")
                .long("day")
                .help("Logtime of the current day")
                .conflicts_with(ARG_CUR_MONTH)
                .conflicts_with(ARG_CUR_WEEK),
        )
        .get_matches()
}
