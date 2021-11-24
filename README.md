# 42 GetTime

Get the logtime for a specific user

## Usage

View logtime of 42 school users

USAGE:
    gettime [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c, --config <FILE>    Explicit path of config file
    -l, --login <LOGIN>    42 login of the user

## Config

Uses a config.json file
```
{
	"client_id": "42 Application UID",
	"secret": "42 Application SECRET_KEY",
	"from": "2021-10-21",
	"to": "2021-10-22",
    "login": "abcd" // Optional
}
```

![Screenshot](screenshot.png)
