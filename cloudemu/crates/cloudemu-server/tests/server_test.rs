use cloudemu_core::{CloudProviderTrait, Request, Response, CloudResult, ServiceType};
use cloudemu_server::server::start_provider_server;
use std::sync::Arc;
use tokio::net::TcpListener;

struct MockProvider;

#[async_trait::async_trait]
impl CloudProviderTrait for MockProvider {
    async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        Ok(Response::ok(format!("Mock Response: {} {}", req.method, req.path))
            .with_header("X-Mock", "true"))
    }
    
    fn supported_services(&self) -> Vec<ServiceType> { 
        vec![] 
    }
    
    fn default_port(&self) -> u16 { 
        0 
    }
    
    fn provider_name(&self) -> &str { 
        "mock" 
    }
}

#[tokio::test]
async fn test_generic_server_routing() {
    // 1. Find a free port
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    drop(listener); // Release port

    // 2. Start generic server
    let provider = Arc::new(MockProvider);
    let server_handle = tokio::spawn(async move {
        start_provider_server(provider, port, "Mock").await.unwrap();
    });

    // Wait for server to start
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

    // 3. Send request
    let client = reqwest::Client::new();
    let url = format!("http://127.0.0.1:{}/test/path?query=1", port);
    
    let resp = client.get(&url)
        .header("X-Custom", "client-header")
        .send()
        .await
        .expect("Failed to send request");

    // 4. Verify generic routing response
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.headers().get("X-Mock").unwrap(), "true");
    
    let body = resp.text().await.unwrap();
    // Verify path and query were preserved by server.rs generic handler
    assert_eq!(body, "Mock Response: GET /test/path?query=1");

    server_handle.abort();
}
