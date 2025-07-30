use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Create router with basic health check endpoint
    let app = Router::new()
        .route("/health", get(|| async { "TMKOC my first Rust Server is Running" }));

    // Bind to 0.0.0.0:8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);
    
    // Create a TcpListener
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Start server using the listener
    axum::serve(listener, app).await.unwrap();
}