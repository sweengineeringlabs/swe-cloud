use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; 
use crate::gateway;
use crate::Emulator;
use std::sync::Arc;

#[tokio::test]
async fn test_s3_bucket_lifecycle() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let app = gateway::create_router(emulator);

    // 1. Create Bucket
    let req = Request::builder()
        .method("PUT")
        .uri("/test-bucket")
        .body(Body::empty())
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 2. Put Object
    let req = Request::builder()
        .method("PUT")
        .uri("/test-bucket/hello.txt")
        .body(Body::from("Hello World"))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Get Object
    let req = Request::builder()
        .method("GET")
        .uri("/test-bucket/hello.txt")
        .body(Body::empty())
        .unwrap();
    
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    // Verify body content
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    assert_eq!(&body[..], b"Hello World");

    // 4. Delete Object
    let req = Request::builder()
        .method("DELETE")
        .uri("/test-bucket/hello.txt")
        .body(Body::empty())
        .unwrap();
        
    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}
