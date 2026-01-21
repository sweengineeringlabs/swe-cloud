use async_trait::async_trait;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Error {
    Internal(String),
    NotFound(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(msg) => write!(f, "Internal error: {}", msg),
            Error::NotFound(msg) => write!(f, "Not found: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

#[derive(Debug, Clone)]
pub struct Request {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl Response {
    pub fn ok(body: Vec<u8>) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body,
        }
    }
    
    pub fn not_found(msg: &str) -> Self {
        Self {
            status: 404,
            headers: HashMap::new(),
            body: msg.as_bytes().to_vec(),
        }
    }

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

pub type CloudResult<T> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

#[async_trait]
pub trait CloudProviderTrait: Send + Sync {
    async fn handle_request(&self, req: Request) -> CloudResult<Response>;
}
