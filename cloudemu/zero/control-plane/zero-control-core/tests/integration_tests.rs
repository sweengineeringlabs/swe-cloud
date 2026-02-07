use zero_control_core::ZeroProvider;
use zero_data_core::ZeroEngine;
use zero_control_spi::{ZeroRequest, ZeroService};
use std::sync::Arc;
use serde_json::json;

#[tokio::test]
async fn test_zero_provider_full_flow() {
    // 1. Setup engine with Mock driver explicitly for environment independence in tests
    let compute = Arc::new(zero_data_core::driver::MockComputeDriver::new());
    let storage = Arc::new(zero_data_core::driver::FileSystemStorage::new(
        tempfile::tempdir().unwrap().path().to_path_buf()
    ));
    let network = Arc::new(zero_data_core::driver::MockNetworkDriver::new());
    let engine = ZeroEngine::new(compute, storage, network).unwrap();
    let provider = ZeroProvider::new(Arc::new(engine));

    // 2. Test Get Nodes
    let req = ZeroRequest {
        method: "GET".into(),
        path: "/v1/nodes".into(),
        headers: std::collections::HashMap::new(),
        body: vec![],
    };
    
    let resp = provider.handle_request(req).await.unwrap();
    assert_eq!(resp.status, 200);
    
    // 3. Test Create Workload
    let req = ZeroRequest {
        method: "POST".into(),
        path: "/v1/workloads".into(),
        headers: std::collections::HashMap::new(),
        body: json!({
            "id": "test-vm-1",
            "image": "ubuntu:latest"
        }).to_string().into_bytes(),
    };
    
    let resp = provider.handle_request(req).await.unwrap();
    assert_eq!(resp.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(body["id"], "test-vm-1");
    assert_eq!(body["state"], "Running");

    // 4. Test Create Volume
    let req = ZeroRequest {
        method: "POST".into(),
        path: "/v1/volumes".into(),
        headers: std::collections::HashMap::new(),
        body: json!({
            "id": "test-vol-1",
            "size_gb": 20
        }).to_string().into_bytes(),
    };
    
    let resp = provider.handle_request(req).await.unwrap();
    assert_eq!(resp.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(body["id"], "test-vol-1");

    // 5. Test Delete Workload
    let req = ZeroRequest {
        method: "DELETE".into(),
        path: "/v1/workloads".into(),
        headers: std::collections::HashMap::new(),
        body: json!({ "id": "test-vm-1" }).to_string().into_bytes(),
    };
    
    let resp = provider.handle_request(req).await.unwrap();
    assert_eq!(resp.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(body["status"], "Deleted");

    // 6. Test Create Network
    let req = ZeroRequest {
        method: "POST".into(),
        path: "/v1/networks".into(),
        headers: std::collections::HashMap::new(),
        body: json!({ "id": "test-net-1", "cidr": "192.168.1.0/24" }).to_string().into_bytes(),
    };
    
    let resp = provider.handle_request(req).await.unwrap();
    assert_eq!(resp.status, 200);
    let body: serde_json::Value = serde_json::from_slice(&resp.body).unwrap();
    assert_eq!(body["id"], "test-net-1");
}
