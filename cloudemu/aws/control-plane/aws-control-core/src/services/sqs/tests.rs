use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; 
use crate::gateway;
use crate::Emulator;
use std::sync::Arc;
use serde_json::json;

#[tokio::test]
async fn test_sqs_api() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let app = gateway::create_router(emulator);

    // 1. Create Queue
    let create_queue_body = json!({
        "QueueName": "MyQueue"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/")
        .header("x-amz-target", "AmazonSQS.CreateQueue")
        .header("content-type", "application/json")
        .body(Body::from(create_queue_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    // Parse response to get QueueUrl? 
    // For now we assume typical mocking: http://localhost:4566/000000000000/MyQueue
    let queue_url = "http://localhost:4566/000000000000/MyQueue";

    // 2. Send Message
    let send_msg_body = json!({
        "QueueUrl": queue_url,
        "MessageBody": "Hello Queue"
    });

    let req = Request::builder()
        .method("POST")
        .uri("/")
        .header("x-amz-target", "AmazonSQS.SendMessage")
        .header("content-type", "application/json")
        .body(Body::from(send_msg_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Receive Message
    let recv_msg_body = json!({
        "QueueUrl": queue_url,
        "MaxNumberOfMessages": 1
    });

    let req = Request::builder()
        .method("POST")
        .uri("/")
        .header("x-amz-target", "AmazonSQS.ReceiveMessage")
        .header("content-type", "application/json")
        .body(Body::from(recv_msg_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    // Verify we got the message back
    let messages = json["Messages"].as_array().expect("Messages array");
    assert!(!messages.is_empty());
    assert_eq!(messages[0]["Body"], "Hello Queue");
}
