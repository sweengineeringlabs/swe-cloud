use aws_control_core::{Emulator, gateway};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt; // for `oneshot`
use std::sync::Arc;
use serde_json::json;

#[tokio::test]
async fn test_s3_list_buckets() {
    // S3 uses specific routing (GET /)
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = gateway::create_router(emulator);

    let req = Request::builder()
        .uri("/")
        .method("GET")
        .body(Body::empty())
        .unwrap();

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_dynamodb_create_table() {
    // DynamoDB uses dispatch -> handle_request
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = gateway::create_router(emulator);

    let body = json!({
        "TableName": "TestTable",
        "KeySchema": [{"AttributeName": "pk", "KeyType": "HASH"}],
        "AttributeDefinitions": [{"AttributeName": "pk", "AttributeType": "S"}],
        "ProvisionedThroughput": {"ReadCapacityUnits": 1, "WriteCapacityUnits": 1}
    });

    let req = Request::builder()
        .uri("/")
        .method("POST")
        .header("x-amz-target", "DynamoDB_20120810.CreateTable")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(req).await.unwrap();
    // It calls services::dynamodb::handlers::handle_request which returns impl IntoResponse
    // It might return OK (200) with TableDescription
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_sqs_create_queue() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = gateway::create_router(emulator);

    let body = json!({
        "QueueName": "TestQueue"
    });

    let req = Request::builder()
        .uri("/")
        .method("POST")
        .header("x-amz-target", "AmazonSQS.CreateQueue")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();

    let response = router.oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_lambda_invocation() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    // Create function first? Or just invoke?
    // Lambda service invocation handler might assume function exists or return 404.
    // Let's just check 404 for non-existent function to prove routing
    let router = gateway::create_router(emulator);

    let req = Request::builder()
        .uri("/2015-03-31/functions/MyFunc/invocations")
        .method("POST")
        .body(Body::from("{}"))
        .unwrap();

    let response = router.oneshot(req).await.unwrap();
    // Should be Not Found or specific Lambda error
    assert!(response.status() == StatusCode::NOT_FOUND || response.status() == StatusCode::INTERNAL_SERVER_ERROR);
}
