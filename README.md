# 42 GetTime

Get the logtime for a specific user

Requests are made with page size 1000 to reduce the number of calls to the API. So if the date span is too long the values might be wrong

Uses a config.json file

{  
	"client_id": "42 Application UID",  
	"secret": "42 Application SECRET_KEY",  
	"from": "2021-10-21",  
	"to": "2021-10-22"  
}  
