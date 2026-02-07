//! Universal cloud resource types and HTTP primitives.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Cloud provider identifier.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CloudProvider {
    /// Amazon Web Services
    Aws,
    /// Microsoft Azure
    Azure,
    /// Google Cloud Platform
    Gcp,
}

impl CloudProvider {
    /// Get the default port for this provider.
    pub fn default_port(&self) -> u16 {
        match self {
            CloudProvider::Aws => 4566,
            CloudProvider::Azure => 4567,
            CloudProvider::Gcp => 4568,
        }
    }

    /// Get the provider name as a string.
    pub fn as_str(&self) -> &'static str {
        match self {
            CloudProvider::Aws => "aws",
            CloudProvider::Azure => "azure",
            CloudProvider::Gcp => "gcp",
        }
    }
}

impl std::fmt::Display for CloudProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Cloud service type (provider-agnostic).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    /// Object storage (S3, Blob Storage, Cloud Storage)
    ObjectStorage,
    /// Key-value database (DynamoDB, Cosmos DB, Firestore)
    KeyValue,
    /// Message queue (SQS, Service Bus Queue, Pub/Sub)
    MessageQueue,
    /// Pub/Sub messaging (SNS, Event Grid, Pub/Sub)
    PubSub,
    /// Serverless functions (Lambda, Azure Functions, Cloud Functions)
    Functions,
    /// Secret management (Secrets Manager, Key Vault, Secret Manager)
    Secrets,
    /// Key management (KMS, Key Vault, Cloud KMS)
    KeyManagement,
    /// Event routing (EventBridge, Event Grid, Eventarc)
    Events,
    /// Monitoring (CloudWatch, Azure Monitor, Cloud Monitoring)
    Monitoring,
    /// Identity (Cognito, Azure AD, Identity Platform)
    Identity,
    /// Workflows (Step Functions, Logic Apps, Workflows)
    Workflows,
}

/// Universal cloud resource representation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudResource {
    /// Unique resource identifier
    pub id: String,
    /// Cloud provider
    pub provider: CloudProvider,
    /// Service type
    pub service_type: ServiceType,
    /// Resource name
    pub name: String,
    /// Resource metadata
    pub metadata: HashMap<String, String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
}

impl CloudResource {
    /// Create a new cloud resource.
    pub fn new(
        id: impl Into<String>,
        provider: CloudProvider,
        service_type: ServiceType,
        name: impl Into<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            provider,
            service_type,
            name: name.into(),
            metadata: HashMap::new(),
            created_at: now,
            updated_at: now,
        }
    }

    /// Add metadata to the resource.
    pub fn with_metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.metadata.insert(key.into(), value.into());
        self
    }

    /// Update the timestamp.
    pub fn touch(&mut self) {
        self.updated_at = Utc::now();
    }
}

/// Resource filter for querying.
#[derive(Debug, Clone, Default)]
pub struct ResourceFilter {
    /// Filter by provider
    pub provider: Option<CloudProvider>,
    /// Filter by service type
    pub service_type: Option<ServiceType>,
    /// Filter by name prefix
    pub name_prefix: Option<String>,
}

impl ResourceFilter {
    /// Create a new empty filter.
    pub fn new() -> Self {
        Self::default()
    }

    /// Filter by provider.
    pub fn provider(mut self, provider: CloudProvider) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Filter by service type.
    pub fn service_type(mut self, service_type: ServiceType) -> Self {
        self.service_type = Some(service_type);
        self
    }

    /// Filter by name prefix.
    pub fn name_prefix(mut self, prefix: impl Into<String>) -> Self {
        self.name_prefix = Some(prefix.into());
        self
    }

    /// Check if a resource matches this filter.
    pub fn matches(&self, resource: &CloudResource) -> bool {
        if let Some(provider) = self.provider {
            if resource.provider != provider {
                return false;
            }
        }

        if let Some(service_type) = self.service_type {
            if resource.service_type != service_type {
                return false;
            }
        }

        if let Some(prefix) = &self.name_prefix {
            if !resource.name.starts_with(prefix) {
                return false;
            }
        }

        true
    }
}

/// HTTP request representation (simplified).
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

/// HTTP response representation (simplified).
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
    /// Create a successful response (200 OK).
    pub fn ok(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 200,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    /// Create a created response (201 Created).
    pub fn created(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 201,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    /// Create a not found response (404 Not Found).
    pub fn not_found(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 404,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    /// Create an error response (500 Internal Server Error).
    pub fn error(body: impl Into<Vec<u8>>) -> Self {
        Self {
            status: 500,
            headers: HashMap::new(),
            body: body.into(),
        }
    }

    /// Add a header to the response.
    pub fn with_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cloud_provider_port() {
        assert_eq!(CloudProvider::Aws.default_port(), 4566);
        assert_eq!(CloudProvider::Azure.default_port(), 4567);
        assert_eq!(CloudProvider::Gcp.default_port(), 4568);
    }

    #[test]
    fn test_resource_filter() {
        let resource = CloudResource::new(
            "test-id",
            CloudProvider::Aws,
            ServiceType::ObjectStorage,
            "my-bucket",
        );

        let filter = ResourceFilter::new().provider(CloudProvider::Aws);
        assert!(filter.matches(&resource));

        let filter = ResourceFilter::new().provider(CloudProvider::Azure);
        assert!(!filter.matches(&resource));

        let filter = ResourceFilter::new().name_prefix("my-");
        assert!(filter.matches(&resource));

        let filter = ResourceFilter::new().name_prefix("other-");
        assert!(!filter.matches(&resource));
    }

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
