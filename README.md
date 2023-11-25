# ScheduLink

## API Docs

1. `/api/event`:
	- `GET`: 
		- Request: `200 OK`
			```javascript
			{
				"event_name": String
			}
			```
		- Response: 
			```javascript
			{
			  "status": String,
			  "data": {
			    "event_name": String,
			    "participants": [
			      {
			        "user_name": String,
			        "availability": [Timestamp]
			      }
			    ]
			  }
			}
			```
	- `POST`:
		- Request: `200 OK`
			```javascript
			{
				"event_name": String,
				"user_name": String,
				"possible_dates": {
					"date": Timestamp,
					"start": Timestamp, 
					"end": Timestamp
				}
			}
			```
		- Response:
			```javascript
			{
				"status": String
			}
			```
	- `PUT`:
		- Request: `200 OK`
			```javascript
			{
				"event_name": String,
				"user_name": String,
				"possible_dates": {
					"date": Timestamp,
					"start": Timestamp, 
					"end": Timestamp
				}
			}
			```
		- Response:
			```javascript
			{
				"status": String
			}
			```
	- `DELETE`:
		- Request: `200 OK`
			```javascript
			{
				"event_name": String,
			}
			```
		- Response:
			```javascript
			{
				"status": String
			}
			```