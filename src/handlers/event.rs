use axum::Json;
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

#[derive(Debug, Deserialize)]
pub struct PostResponse{
    status: String
}

pub async fn event_post(Json(request): Json<PostRequest>){
    println!("Event Name: {}", request.event_name);
    println!("User Name: {}", request.user_name);
    for possible_time in request.possible_times{
        println!("Possible Time: {} - {}", possible_time.start, possible_time.end);
    }
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