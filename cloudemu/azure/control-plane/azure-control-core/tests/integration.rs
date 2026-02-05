use azure_control_core::AzureProvider;
use azure_control_spi::{CloudProviderTrait, Request, Response};
use std::collections::HashMap;
use uuid::Uuid;

fn random_name() -> String {
    format!("res{}", Uuid::new_v4().simple())
}

#[tokio::test]
async fn test_azure_cosmos_flow() {
    let provider = AzureProvider::in_memory();
    let db_name = "db1"; // cosmos.rs currently hardcodes return id as db1
    let coll_name = "coll1"; // cosmos.rs hardcodes coll1

    // 1. Create Database: POST /dbs
    // Body should contain {"id": "..."}
    let req = Request {
        method: "POST".to_string(),
        path: "/dbs".to_string(),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);

    // 2. Create Collection: POST /dbs/{db}/colls
    let req = Request {
        method: "POST".to_string(),
        path: format!("/dbs/{}/colls", db_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);

    // 3. Create Document: POST /dbs/{db}/colls/{coll}/docs
    let doc_body = r#"{"id":"doc1","value":"test"}"#;
    let req = Request {
        method: "POST".to_string(),
        path: format!("/dbs/{}/colls/{}/docs", db_name, coll_name),
        headers: HashMap::new(),
        body: doc_body.as_bytes().to_vec(),
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);
}

#[tokio::test]
async fn test_azure_servicebus_flow() {
    let provider = AzureProvider::in_memory();
    let queue_name = random_name();

    // 1. Create Queue: PUT /queue/{name}
    let req = Request {
        method: "PUT".to_string(),
        path: format!("/queue/{}", queue_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);

    // 2. Send Message: POST /{queue}/messages
    let req = Request {
        method: "POST".to_string(),
        path: format!("/{}/messages", queue_name),
        headers: HashMap::new(),
        body: b"hello world".to_vec(),
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);
}

#[tokio::test]
async fn test_azure_functions_flow() {
    let provider = AzureProvider::in_memory();
    let func_name = random_name();

    // 1. Create Function: POST /admin/functions/{name}
    let req = Request {
        method: "POST".to_string(),
        path: format!("/admin/functions/{}", func_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);
}

#[tokio::test]
async fn test_azure_keyvault_flow() {
    let provider = AzureProvider::in_memory();
    let secret_name = random_name();

    // 1. Set Secret: PUT /secrets/{name}
    let req = Request {
        method: "PUT".to_string(),
        path: format!("/secrets/{}", secret_name),
        headers: HashMap::new(),
        body: b"supersecret".to_vec(),
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);

    // 2. Get Secret: GET /secrets/{name}
    let req = Request {
        method: "GET".to_string(),
        path: format!("/secrets/{}", secret_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);
    let body_str = String::from_utf8(res.body).unwrap();
    assert!(body_str.contains("supersecret"));
}
