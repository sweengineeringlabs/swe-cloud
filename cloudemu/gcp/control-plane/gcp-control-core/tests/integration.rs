use gcp_control_core::GcpProvider;
use gcp_control_spi::{CloudProviderTrait, Request};
use std::collections::HashMap;
use uuid::Uuid;

fn random_name() -> String {
    format!("res{}", Uuid::new_v4().simple())
}

#[tokio::test]
async fn test_gcp_storage_flow() {
    let provider = GcpProvider::new();
    let bucket_name = random_name();

    // 1. Create Bucket
    let req = Request {
        method: "PUT".to_string(),
        path: format!("/{}", bucket_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);

    // 2. Put Object
    let req = Request {
        method: "PUT".to_string(),
        path: format!("/{}/obj.txt", bucket_name),
        headers: HashMap::new(),
        body: b"content".to_vec(),
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);

    // 3. Get Object
    let req = Request {
        method: "GET".to_string(),
        path: format!("/{}/obj.txt", bucket_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);
    assert_eq!(res.body, b"content");
}

#[tokio::test]
async fn test_gcp_firestore_flow() {
    let provider = GcpProvider::new();
    let collection = random_name();

    // 1. Create Document
    let doc_body = r#"{"name":"test"}"#;
    let req = Request {
        method: "POST".to_string(),
        path: format!("/projects/p/databases/d/documents/{}", collection),
        headers: HashMap::new(),
        body: doc_body.as_bytes().to_vec(),
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);

    // 2. List Documents
    let req = Request {
        method: "GET".to_string(),
        path: format!("/projects/p/databases/d/documents/{}", collection),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);
    let body_str = String::from_utf8(res.body).unwrap();
    assert!(body_str.contains("documents"));
}

#[tokio::test]
async fn test_gcp_pubsub_flow() {
    let provider = GcpProvider::new();
    let topic_name = random_name();

    // 1. Create Topic
    let req = Request {
        method: "PUT".to_string(),
        path: format!("/v1/projects/p/topics/{}", topic_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);

    // 2. Publish Message
    let req = Request {
        method: "POST".to_string(),
        path: format!("/v1/projects/p/topics/{}/publish", topic_name),
        headers: HashMap::new(),
        body: b"message".to_vec(),
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 200);
}

#[tokio::test]
async fn test_gcp_functions_flow() {
    let provider = GcpProvider::new();
    let func_name = random_name();

    // 1. Create Function
    let req = Request {
        method: "POST".to_string(),
        path: format!("/v1/projects/p/locations/l/functions/{}", func_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);
}

#[tokio::test]
async fn test_gcp_secrets_flow() {
    let provider = GcpProvider::new();
    let secret_name = random_name();

    // 1. Create Secret
    let req = Request {
        method: "POST".to_string(),
        path: format!("/v1/projects/p/secrets/{}", secret_name),
        headers: HashMap::new(),
        body: vec![],
    };
    let res = provider.handle_request(req).await.unwrap();
    assert_eq!(res.status, 201);
}
