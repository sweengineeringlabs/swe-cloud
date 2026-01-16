use aws_control_core::Emulator;
use aws_control_core::gateway::create_router;
use axum::body::Body;
use axum::http::{Request, StatusCode};
use http_body_util::BodyExt; // for collect
use serde_json::{json, Value};
use std::sync::Arc;
use tower::ServiceExt; // for oneshot

#[tokio::test]
async fn test_pricing_get_services() {
    // 1. Setup Emulator
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = create_router(emulator);

    // 2. Create Request
    let request = Request::builder()
        .method("POST")
        .uri("/")
        .header("content-type", "application/x-amz-json-1.1")
        .header("x-amz-target", "AWSPriceListService.GetServices")
        .body(Body::from(json!({}).to_string()))
        .unwrap();

    // 3. Send Request
    let response = router.oneshot(request).await.unwrap();

    // 4. Verify Status
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Verify Body
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

    println!("Response: {}", body_json);

    assert_eq!(body_json["FormatVersion"], "aws_v1");
    let services = body_json["Services"].as_array().unwrap();
    assert!(services.iter().any(|s| s["ServiceCode"] == "AmazonEC2"));
    assert!(services.iter().any(|s| s["ServiceCode"] == "AmazonS3"));
}

#[tokio::test]
async fn test_pricing_get_attribute_values() {
    // 1. Setup Emulator
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = create_router(emulator);

    // 2. Create Request
    let request = Request::builder()
        .method("POST")
        .uri("/")
        .header("content-type", "application/x-amz-json-1.1")
        .header("x-amz-target", "AWSPriceListService.GetAttributeValues")
        .body(Body::from(json!({ "ServiceCode": "AmazonEC2", "AttributeName": "Location" }).to_string()))
        .unwrap();

    // 3. Send Request
    let response = router.oneshot(request).await.unwrap();

    // 4. Verify Status
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Verify Body
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

    println!("Response: {}", body_json);

    let values = body_json["AttributeValues"].as_array().unwrap();
    assert!(values.iter().any(|v| v["Value"] == "US East (N. Virginia)"));
}

#[tokio::test]
async fn test_pricing_get_products() {
    // 1. Setup Emulator
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = create_router(emulator);

    // 2. Create Request (with filter)
    let request = Request::builder()
        .method("POST")
        .uri("/")
        .header("content-type", "application/x-amz-json-1.1")
        .header("x-amz-target", "AWSPriceListService.GetProducts")
        .body(Body::from(json!({ 
            "ServiceCode": "AmazonEC2",
            "Filters": [
                { "Type": "TERM_MATCH", "Field": "location", "Value": "US East (N. Virginia)" }
            ]
        }).to_string()))
        .unwrap();

    // 3. Send Request
    let response = router.oneshot(request).await.unwrap();

    // 4. Verify Status
    assert_eq!(response.status(), StatusCode::OK);

    // 5. Verify Body
    let body_bytes = response.into_body().collect().await.unwrap().to_bytes();
    let body_json: Value = serde_json::from_slice(&body_bytes).unwrap();

    println!("Response: {}", body_json);
    
    // Should have PriceList
    let price_list = body_json["PriceList"].as_array().unwrap();
    assert!(!price_list.is_empty(), "Should return seeded products");
    
    // Check first item structure (it is a JSON string inside a JSON array)
    let item_str = price_list[0].as_str().unwrap();
    let item_json: Value = serde_json::from_str(item_str).unwrap();
    
    assert_eq!(item_json["serviceCode"], "AmazonEC2");
    assert!(item_json["product"]["attributes"]["instanceType"] == "t3.micro");
}
