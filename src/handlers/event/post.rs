use axum::{Json, http::StatusCode, response::{IntoResponse, Response}};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Request{
    event_name: String,
    user_name: String,
    description: String,
    priority: String,
    possible_times: Vec<PossibleTimes>
}

#[derive(Debug, Deserialize)]
pub struct PossibleTimes{
    start: String,
    end: String
}

#[derive(Debug)]
pub struct AppError{
    message: String,
    status: StatusCode
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}

fn is_valid_datetime(time: String) -> Result<chrono::NaiveDateTime, AppError>{
    match chrono::NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S"){
        Ok(parsed_datetime) => Ok(parsed_datetime),
        Err(_) => Err(AppError {
            message: "Invalid Date - Time format for {time}".to_string(),
            status: StatusCode::BAD_REQUEST
        })
    }
}

pub async fn post(Json(request): Json<Request>) -> Result<String, AppError> {

    // TODO: Handle apikey validation - if invalid API KEY then you need to ensure the request does not go through

    if request.event_name.is_empty(){
        return Err(AppError {
            message: "Event name is empty".to_string(),
            status: StatusCode::BAD_REQUEST
        });
    }

    if request.user_name.is_empty(){
        return Err(AppError {
            message: "Creator name is empty".to_string(),
            status: StatusCode::BAD_REQUEST
        });
    }

    let priority = request.priority.to_uppercase();

    if &priority != "HIGH" || &priority != "MEDIUM" || &priority != "LOW"{
        return Err(AppError {
            message: "Invalid priority: can only be: High, Medium or Low".to_string(),
            status: StatusCode::BAD_REQUEST
        });
    }

    println!("Event name: {}", request.event_name);
    println!("User name: {}", request.user_name);
    println!("Description provided: {}", request.description);
    println!("Priority: {}", priority);

    for possible_time in request.possible_times {
        let start_time = match is_valid_datetime(possible_time.start) {
            Ok(parsed_time) => parsed_time,
            Err(err) => return Err(err),
        };

        let end_time = match is_valid_datetime(possible_time.end) {
            Ok(parsed_time) => parsed_time,
            Err(err) => return Err(err),
        };

        if start_time > end_time {
            return Err(AppError {
                message: "Start time must be before the end time".to_string(),
                status: StatusCode::BAD_REQUEST
            });
        }

        println!("Possible time: {} - {}", start_time, end_time);
    }

    // TODO: Handle creating event in DB here

    Ok("Success".to_string())
}