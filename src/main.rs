use axum::{routing::get, Router};
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tower_http::services::ServeDir;
use tracing_subscriber;

#[tokio::main]
async fn main() {
    // Initialize logging
    tracing_subscriber::fmt::init();

    // Define our app routes
    let app = Router::new()
        .route("/status", get(|| async { "my first Rust Server (v1.1) is Running" }))
        // Use .fallback_service() instead of .nest_service() for the root
        .fallback_service(ServeDir::new("wwwroot"));


    // Bind to 0.0.0.0:8080
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);

    // Create a TcpListener
    let listener = TcpListener::bind(&addr).await.unwrap();

    // Start server using the listener
    axum::serve(listener, app).await.unwrap();
}