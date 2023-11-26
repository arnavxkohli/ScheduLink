use axum::{Json, http::StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PostRequest{
    event_name: String,
    user_name: String,
    possible_times: Vec<PossibleTimes>
}

#[derive(Debug, Deserialize)]
pub struct PossibleTimes{
    start: chrono::NaiveDateTime,
    end: chrono::NaiveDateTime
}

pub async fn event_post(request: Json<PostRequest>) -> Result<String, StatusCode> {

    if request.event_name.is_empty() || request.user_name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    } else {
        for possible_time in &request.possible_times {
            if possible_time.start > possible_time.end {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    }

    // Handle creating event in DB here

    Ok("Success".to_string())
}

pub async fn event_get() -> &'static str {
    "Handling GET request for /api/event"
}

pub async fn event_put() -> &'static str {
    "Handling PUT request for /api/event"
}

pub async fn event_delete() -> &'static str {
    "Handling DELETE request for /api/event"
}