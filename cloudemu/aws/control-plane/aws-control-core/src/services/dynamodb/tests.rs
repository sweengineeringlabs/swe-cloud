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
async fn test_dynamodb_json_api() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let app = gateway::create_router(emulator);

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
