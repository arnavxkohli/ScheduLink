use axum::{http::StatusCode, response::{Response, IntoResponse}};

#[derive(Debug)]
pub struct AppError{
    pub message: String,
    pub status: StatusCode
}

impl IntoResponse for AppError{
    fn into_response(self) -> Response {
        (self.status, self.message).into_response()
    }
}