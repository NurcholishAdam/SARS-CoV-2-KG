// crates/limit-hub/src/main.rs
use axum::Router;
use tracing_subscriber;

mod api;
mod governance;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create router
    let app = api::create_router();

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    tracing::info!("LIMIT Hub API listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
