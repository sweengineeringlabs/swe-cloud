use azure_control_core::AzureProvider;
use azure_control_spi::{CloudProviderTrait, Request};
use std::collections::HashMap;

#[tokio::test]
async fn test_azure_pricing_get_retail_prices() {
    let provider = AzureProvider::in_memory();

    // 1. Get Retail Prices
    let req = Request {
        method: "GET".to_string(),
        // Normal Azure connection string or CLI would hit https://prices.azure.com/api/retail/prices
        path: "/api/retail/prices?$filter=serviceName eq 'Virtual Machines'".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };

    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);

    let body_str = String::from_utf8(res.body).unwrap();
    let body_json: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    // Verify structure
    assert!(body_json.get("Items").is_some());
    let items = body_json["Items"].as_array().unwrap();
    assert!(!items.is_empty());

    // Verify content of first item (Seeded data)
    let item = &items[0];
    assert_eq!(item["serviceName"], "Virtual Machines");
    assert_eq!(item["skuName"], "Standard_D2s_v3");
    assert!(item["retailPrice"].as_f64().unwrap() > 0.0);
}
