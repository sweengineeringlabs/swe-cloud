use zero_control_facade::start_server;
use tokio::time::{sleep, Duration};
use serde_json::json;

#[tokio::test]
async fn test_zero_facade_api_integration() {
    let port = 8081; // Use a different port for tests

    // Start server in background
    let server_handle = tokio::spawn(async move {
        start_server(port, false, true).await.unwrap();
    });

    // Wait for server to start
    sleep(Duration::from_millis(500)).await;

    let client = reqwest::Client::new();
    let base_url = format!("http://localhost:{}", port);

    // 1. Test Node List (GET /v1/nodes)
    let resp = client.get(format!("{}/v1/nodes", base_url))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let nodes: serde_json::Value = resp.json().await.unwrap();
    assert!(nodes["nodes"].is_array());

    // 2. Test Create Workload (POST /v1/workloads)
    let resp = client.post(format!("{}/v1/workloads", base_url))
        .json(&json!({ "id": "test-vm", "image": "ubuntu" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let workload: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(workload["id"], "test-vm");
    assert_eq!(workload["state"], "Running");

    // 3. Test Create Network (POST /v1/networks)
    let resp = client.post(format!("{}/v1/networks", base_url))
        .json(&json!({ "id": "test-net", "cidr": "10.0.1.0/24" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);
    let network: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(network["id"], "test-net");

    // 4. Test Delete Workload (DELETE /v1/workloads)
    let resp = client.delete(format!("{}/v1/workloads", base_url))
        .json(&json!({ "id": "test-vm" }))
        .send().await.unwrap();
    assert_eq!(resp.status(), 200);

    // Abort server
    server_handle.abort();
}
