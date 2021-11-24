use clap::{App, Arg, ArgMatches};

pub const ARG_CONFIG: &str = "config";
pub const ARG_LOGIN: &str = "user";
pub const ARG_CUR_MONTH: &str = "month";

pub fn matches() -> ArgMatches<'static> {
    App::new("42 Gettime")
        .version("0.1.0")
        .author("Mikastiv <mleblanc@student.42quebec.com>")
        .about("View logtime of 42 school users")
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
                .help("Logtime of the current month"),
        )
        .get_matches()
}
