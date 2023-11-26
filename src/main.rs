#![allow(non_snake_case)]
mod handlers;
use axum::{Router, routing::get, routing::post, routing::put, routing::delete};

#[tokio::main]
async fn main() {
    // Dynamic port allocation for fl{/}
    let port = std::env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Routes defined for the app
    let app = Router::new()
    .route("/", get(handlers::base::base))
    .route("/api/event", get(handlers::event::get::get))
    .route("/api/event", post(handlers::event::post::post))
    .route("/api/event", put(handlers::event::put::put))
    .route("/api/event", delete(handlers::event::delete::delete));

    // Serve the app asynchronously
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
