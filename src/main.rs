mod handlers;
use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    // Dynamic port allocation for fl{/}
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);

    // Routes defined for the app
    let app =
        Router::new()
            .route("/", get(handlers::base));

    // Server the app asynchronously
    axum::Server::bind(&addr.parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
