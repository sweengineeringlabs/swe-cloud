use gcp_control_facade::{router, GcpProvider};
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_server_startup_and_request() {
    // 1. Start Server on random port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    
    let provider = Arc::new(GcpProvider::in_memory());
    let app = router(provider);

    tokio::spawn(async move {
        axum::serve(listener, app).await.unwrap();
    });

    // 2. Make Request
    let client = reqwest::Client::new();
    let url = format!("http://{}/test-bucket", addr);
    
    // Create bucket
    let resp = client.put(&url).send().await.unwrap();
    
    assert_eq!(resp.status(), 201);
}
