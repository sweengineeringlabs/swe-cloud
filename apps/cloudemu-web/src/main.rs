use axum::{routing::get, Router};
use tower_http::services::ServeDir;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    // Serve static files (WASM bundle)
    let app = Router::new()
        .route("/api/health", get(health))
        .fallback_service(ServeDir::new("dist"));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::info!("CloudEmu Web UI listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "OK"
}
