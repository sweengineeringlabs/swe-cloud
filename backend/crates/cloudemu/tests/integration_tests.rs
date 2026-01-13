use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use cloudemu::gateway;
use cloudemu::Emulator;
use std::sync::Arc;
use serde_json::json;

// Helper to create the router
fn router() -> axum::Router {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    gateway::create_router(emulator)
}

#[tokio::test]
#[cfg(feature = "s3")]
async fn test_s3_bucket_lifecycle() {
    let app = router();

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

#[tokio::test]
#[cfg(feature = "dynamodb")]
async fn test_dynamodb_json_api() {
    let app = router();

    // 1. Create Table
    // DynamoDB uses POST / with x-amz-target header
    let create_table_body = json!({
        "TableName": "Users",
        "KeySchema": [{"AttributeName": "UserId", "KeyType": "HASH"}],
        "AttributeDefinitions": [{"AttributeName": "UserId", "AttributeType": "S"}],
        "ProvisionedThroughput": {"ReadCapacityUnits": 5, "WriteCapacityUnits": 5}
    });

    let req = Request::builder()
        .method("POST")
        .uri("/")
        .header("x-amz-target", "DynamoDB_20120810.CreateTable")
        .header("content-type", "application/json")
        .body(Body::from(create_table_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 2. Put Item
    let put_item_body = json!({
        "TableName": "Users",
        "Item": {
            "UserId": {"S": "user1"},
            "Name": {"S": "Alice"}
        }
    });

    let req = Request::builder()
        .method("POST")
        .uri("/")
        .header("x-amz-target", "DynamoDB_20120810.PutItem")
        .header("content-type", "application/json")
        .body(Body::from(put_item_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // 3. Get Item
    let get_item_body = json!({
        "TableName": "Users",
        "Key": {
            "UserId": {"S": "user1"}
        }
    });

    let req = Request::builder()
        .method("POST")
        .uri("/")
        .header("x-amz-target", "DynamoDB_20120810.GetItem")
        .header("content-type", "application/json")
        .body(Body::from(get_item_body.to_string()))
        .unwrap();

    let response = app.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    let body = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["Item"]["Name"]["S"], "Alice");
}

#[tokio::test]
#[cfg(feature = "sqs")]
async fn test_sqs_api() {
    let app = router();

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
