use aws_control_core::{Emulator, gateway};
use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use tower::ServiceExt;
use std::sync::Arc;
use serde_json::{json, Value};

async fn get_body_as_json(response: axum::response::Response) -> Value {
    let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
    serde_json::from_slice(&body_bytes).unwrap()
}

#[tokio::test]
async fn test_apigateway_workflow() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = gateway::create_router(emulator.clone());

    // 1. List APIs (should be empty)
    let req = Request::builder()
        .uri("/restapis")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    
    // 2. Create API
    let body = json!({
        "name": "MyAPI",
        "description": "Test API"
    });
    let req = Request::builder()
        .uri("/restapis")
        .method("POST")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let json = get_body_as_json(response).await;
    let api_id = json["id"].as_str().unwrap().to_string();

    // 3. List APIs (should have 1)
    let req = Request::builder()
        .uri("/restapis")
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    let json = get_body_as_json(response).await;
    assert_eq!(json["item"].as_array().unwrap().len(), 1);
    
    // 4. Create resource
    let body = json!({
        "pathPart": "users"
    });
    // Assuming root resource ID needed? Or we use /resources/root/child?
    // My implementation: /restapis/{api_id}/resources/{parent_id}
    // We need to find the root resource ID first.
    // In create_rest_api, I created a root resource "/" but didn't return its ID in the CreateRestApi response (AWS doesn't either, usually separate call).
    // So let's list resources first.
    
    let req = Request::builder()
        .uri(&format!("/restapis/{}/resources", api_id))
        .method("GET")
        .body(Body::empty())
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    let json = get_body_as_json(response).await;
    let root_id = json["item"][0]["id"].as_str().unwrap();

    let req = Request::builder()
        .uri(&format!("/restapis/{}/resources/{}", api_id, root_id))
        .method("POST")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn test_elb_workflow() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = gateway::create_router(emulator);

    // Create LB
    let body = json!({
        "Name": "my-lb",
        "Scheme": "internet-facing"
    });
    let req = Request::builder()
        .uri("/")
        .method("POST")
        .header("x-amz-target", "ElasticLoadBalancing_v20151201.CreateLoadBalancer")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // Describe LBs
    let body = json!({});
    let req = Request::builder()
        .uri("/")
        .method("POST")
        .header("x-amz-target", "ElasticLoadBalancing_v20151201.DescribeLoadBalancers")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    let json = get_body_as_json(response).await;
    let lbs = json["DescribeLoadBalancersResponse"]["DescribeLoadBalancersResult"]["LoadBalancers"].as_array().unwrap();
    assert_eq!(lbs.len(), 1);
    assert_eq!(lbs[0]["LoadBalancerName"], "my-lb");
}

#[tokio::test]
async fn test_ecr_workflow() {
    let emulator = Arc::new(Emulator::in_memory().unwrap());
    let router = gateway::create_router(emulator);

    // Create Repo
    let body = json!({
        "repositoryName": "my-repo"
    });
    let req = Request::builder()
        .uri("/")
        .method("POST")
        .header("x-amz-target", "AmazonEC2ContainerRegistry_V20150921.CreateRepository")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    assert_eq!(response.status(), StatusCode::OK);

    // List Repos
    let body = json!({});
    let req = Request::builder()
        .uri("/")
        .method("POST")
        .header("x-amz-target", "AmazonEC2ContainerRegistry_V20150921.DescribeRepositories")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&body).unwrap()))
        .unwrap();
    let response = router.clone().oneshot(req).await.unwrap();
    let json = get_body_as_json(response).await;
    let repos = json["repositories"].as_array().unwrap();
    assert_eq!(repos.len(), 1);
    assert_eq!(repos[0]["repositoryName"], "my-repo");
}
