# 42 GetTime

Get the logtime for a specific user

## Usage
```
gettime [FLAGS] [OPTIONS]

FLAGS:
    -d, --day        Logtime of the current day
    -h, --help       Prints help information
    -m, --month      Logtime of the current month
    -V, --version    Prints version information
    -w, --week       Logtime of the current week

OPTIONS:
    -c, --config <FILE>    Explicit path of config file
    -l, --login <LOGIN>    42 login of the user
```

NOTE: if no date span is found in config file and the --month flag is not used, --month will be used by default

## Config

Uses a config.json file
```
{
    "client_id": "42 Application UID",
    "secret": "42 Application SECRET_KEY",
    "from": "2021-10-21", // Optional if flag -m / -w / -d is used
    "to": "2021-10-22", // Optional if flag -m / -w / -d is used
    "login": "your_login" // Optional if passed with -l
}
```

![Screenshot](screenshot.png)
