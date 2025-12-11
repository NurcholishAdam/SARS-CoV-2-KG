// crates/limit-reflection/src/main.rs
use tracing_subscriber;

mod api;
mod engine;
mod govern;
mod model;
mod quantum;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Create router
    let app = api::create_router();

    // Start server
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001")
        .await
        .unwrap();

    tracing::info!(
        "LIMIT Reflection API listening on {}",
        listener.local_addr().unwrap()
    );

    axum::serve(listener, app).await.unwrap();
}
