use oracle_control_core::OracleProvider;
use oracle_control_spi::{CloudProviderTrait, Request};
use oracle_data_core::StorageEngine;
use std::sync::Arc;
use std::collections::HashMap;

#[tokio::test]
async fn test_oracle_pricing_list_prices() {
    // 1. Setup (Using a temp dir)
    let temp_dir = tempfile::tempdir().unwrap();
    let storage = Arc::new(StorageEngine::new(temp_dir.path().to_path_buf()).unwrap());
    let provider = OracleProvider::new(storage);

    // 2. Request
    let req = Request {
        method: "GET".to_string(),
        path: "/metering/api/v1/prices".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };

    // 3. Handle
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);

    // 4. Verify
    let body_str = String::from_utf8(res.body).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    
    // Should have seeded items
    let items = body_json["items"].as_array().unwrap();
    assert!(!items.is_empty(), "Pricing items should be seeded");
    
    // Check for VM.Standard2.1
    let vm = items.iter().find(|i| i["partNumber"] == "B9F0-5A32-9D1C").expect("VM SKU not found");
    assert_eq!(vm["service"], "Compute");
    
    // Check price
    let price = vm["price"]["unitPrice"].as_f64().unwrap();
    assert_eq!(price, 0.0638);
}
