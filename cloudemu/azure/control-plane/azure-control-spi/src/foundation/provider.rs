//! Cloud provider trait for unified request handling.

use crate::{CloudResult, ServiceType};
use async_trait::async_trait;

/// HTTP request representation (simplified).
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: String,
    /// Request path
    pub path: String,
    /// Headers
    pub headers: std::collections::HashMap<String, String>,
    /// Request body
    pub body: Vec<u8>,
}

/// HTTP response representation (simplified).
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: std::collections::HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
}

impl Response {
    /// Create a successful response (200 OK).
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: std::collections::HashMap::new(),
            body: body.into(),
        }
    }

    /// Create a created response (201 Created).
    pub fn created(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 201,
            headers: std::collections::HashMap::new(),
            body: body.into(),
        }
    }

    /// Create a no content response (204 No Content).
    pub fn no_content() -> Self {
        Self {
            status: 204,
            headers: std::collections::HashMap::new(),
            body: vec![],
        }
    }

    /// Create a not found response (404 Not Found).
    pub fn not_found(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 404,
            headers: std::collections::HashMap::new(),
            body: body.into(),
        }
    }

    /// Create an error response (500 Internal Server Error).
    pub fn error(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 500,
            headers: std::collections::HashMap::new(),
            body: body.into(),
        }
    }

    /// Add a header to the response.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

/// Provider-agnostic cloud service interface.
///
/// Each cloud provider (AWS, Azure, GCP) implements this trait to handle
/// incoming HTTP requests and route them to the appropriate service handlers.
#[async_trait]
pub trait CloudProviderTrait: Send + Sync {
    /// Handle an incoming HTTP request.
    async fn handle_request(&self, req: Request) -> CloudResult<Response>;

    /// Get the list of supported services for this provider.
    fn supported_services(&self) -> Vec<ServiceType>;

    /// Get the default port for this provider.
    fn default_port(&self) -> u16;

    /// Get the provider name.
    fn provider_name(&self) -> &str;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_response_builders() {
        let resp = Response::ok("success");
        assert_eq!(resp.status, 200);
        assert_eq!(resp.body, b"success");

        let resp = Response::created("created");
        assert_eq!(resp.status, 201);

        let resp = Response::not_found("not found");
        assert_eq!(resp.status, 404);

        let resp = Response::error("error");
        assert_eq!(resp.status, 500);
    }

    #[test]
    fn test_response_with_header() {
        let resp = Response::ok("test")
            .with_header("Content-Type", "application/json")
            .with_header("X-Provider", "aws");

        assert_eq!(resp.headers.get("Content-Type"), Some(&"application/json".to_string()));
        assert_eq!(resp.headers.get("X-Provider"), Some(&"aws".to_string()));
    }
}
