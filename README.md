# Logtime 42

View logtime of a 42 school student

## Usage
```
USAGE:
    logtime42 [FLAGS] [OPTIONS]

FLAGS:
    -d, --day        Logtime of the current day
    -h, --help       Prints help information
    -m, --month      Logtime of the current month
    -V, --version    Prints version information
    -w, --week       Logtime of the current week

OPTIONS:
    -c, --config <FILE>        Explicit path of config file
    -f, --from <YYYY-MM-DD>    Beginning of date span
    -t, --to <YYYY-MM-DD>      End of date span
    -l, --login <LOGIN>        42 login of the user
```

NOTE: if no date span is found in config file and the --month flag is not used, --month will be used by default

## Install

- Download rust
- Clone project
- ```cargo build --release```
- Create an API key pair ([42 API](https://profile.intra.42.fr/oauth/applications/new))
- Create config.json file

## Config

You will need an API public/private key pair for the [42 API](https://profile.intra.42.fr/oauth/applications/new)

Uses a config.json file. Checks first for the file passed with -c, if none, looks in the current working directory.
```
{
    "client_id": "42 Application UID",
    "secret": "42 Application SECRET_KEY",
    "from": "2021-10-21", // Optional
    "to": "2021-10-22", // Optional
    "login": "your_login" // Optional if passed with -l
}
```

![Screenshot](screenshot.png)
