use gcp_control_core::GcpProvider;
use gcp_control_spi::{CloudProviderTrait, Request};
use std::collections::HashMap;

#[tokio::test]
async fn test_gcp_billing_list_services() {
    let provider = GcpProvider::in_memory();

    // 1. List Services
    let req = Request {
        method: "GET".to_string(),
        path: "/v1/services".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };

    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);

    let body_str = String::from_utf8(res.body).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert!(body_json.get("services").is_some());
    let services = body_json["services"].as_array().unwrap();
    assert!(!services.is_empty());
    
    let service = &services[0];
    assert_eq!(service["displayName"], "Compute Engine");
}

#[tokio::test]
async fn test_gcp_billing_list_skus() {
    let provider = GcpProvider::in_memory();

    // 2. List SKUs for a service
    let req = Request {
        method: "GET".to_string(),
        path: "/v1/services/6F81-5844-456A/skus".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };

    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);

    let body_str = String::from_utf8(res.body).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    assert!(body_json.get("skus").is_some());
    let skus = body_json["skus"].as_array().unwrap();
    assert!(!skus.is_empty());

    let sku = &skus[0];
    assert!(sku["pricingInfo"].as_array().is_some());
}
