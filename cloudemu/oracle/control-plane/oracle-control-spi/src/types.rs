//! Oracle control-plane types.

use std::collections::HashMap;

/// HTTP request for Oracle services.
#[derive(Debug, Clone)]
pub struct Request {
    /// HTTP method
    pub method: String,
    /// Request path
    pub path: String,
    /// Headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Vec<u8>,
}

/// HTTP response for Oracle services.
#[derive(Debug, Clone)]
pub struct Response {
    /// HTTP status code
    pub status: u16,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
}

impl Response {
    /// Create a successful response.
    pub fn ok(body: Vec<u8>) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body,
        }
    }

    /// Create a not found response.
    pub fn not_found(msg: &str) -> Self {
        Self {
            status: 404,
            headers: HashMap::new(),
            body: msg.as_bytes().to_vec(),
        }
    }

    /// Create a JSON response.
    pub fn json(value: serde_json::Value) -> Self {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Self {
            status: 200,
            headers,
            body: serde_json::to_vec(&value).unwrap_or_default(),
        }
    }
}
