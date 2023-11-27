use axum::http::StatusCode;
use serde::Deserialize;
use crate::handlers::event::app_error::AppError;

#[derive(Debug, Deserialize)]
pub struct PossibleTimes{
    pub start: String,
    pub end: String
}

pub fn is_valid_datetime(time: String) -> Result<chrono::NaiveDateTime, AppError>{
    match chrono::NaiveDateTime::parse_from_str(&time, "%Y-%m-%dT%H:%M:%S"){
        Ok(parsed_datetime) => Ok(parsed_datetime),
        Err(_) => Err(AppError {
            message: "Invalid Date - Time format for {time}".to_string(),
            status: StatusCode::BAD_REQUEST
        })
    }
}