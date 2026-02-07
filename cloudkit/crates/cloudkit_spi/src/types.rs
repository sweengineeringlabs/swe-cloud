//! Common types used across cloud operations.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

/// Metadata type for key-value pairs (tags, attributes, etc).
pub type Metadata = HashMap<String, String>;

/// Unique identifier for cloud resources.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ResourceId(String);

impl ResourceId {
    /// Create a new resource ID.
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    /// Generate a new random resource ID.
    pub fn generate() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Get the ID as a string slice.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ResourceId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<String> for ResourceId {
    fn from(s: String) -> Self {
        Self(s)
    }
}

impl From<&str> for ResourceId {
    fn from(s: &str) -> Self {
        Self(s.to_string())
    }
}

/// Metadata about a cloud resource.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceMetadata {
    /// Resource identifier
    pub id: ResourceId,
    /// Resource name
    pub name: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Last modification timestamp
    pub updated_at: DateTime<Utc>,
    /// Resource tags/labels
    pub tags: std::collections::HashMap<String, String>,
    /// Provider-specific metadata
    pub provider_metadata: serde_json::Value,
}

impl ResourceMetadata {
    /// Create new metadata with minimal information.
    pub fn new(id: impl Into<ResourceId>, name: impl Into<String>) -> Self {
        let now = Utc::now();
        Self {
            id: id.into(),
            name: name.into(),
            created_at: now,
            updated_at: now,
            tags: std::collections::HashMap::new(),
            provider_metadata: serde_json::Value::Null,
        }
    }
}

/// Object storage metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetadata {
    /// Object key (path)
    pub key: String,
    /// Size in bytes
    pub size: u64,
    /// Content type (MIME)
    pub content_type: Option<String>,
    /// ETag (entity tag)
    pub etag: Option<String>,
    /// Last modified timestamp
    pub last_modified: DateTime<Utc>,
    /// Storage class
    pub storage_class: Option<String>,
    /// Custom metadata
    pub metadata: std::collections::HashMap<String, String>,
}

/// Bucket/container metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketMetadata {
    /// Bucket name
    pub name: String,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
    /// Region where bucket is located
    pub region: String,
    /// Versioning enabled
    pub versioning_enabled: bool,
}

impl BucketMetadata {
    /// Create new bucket metadata.
    pub fn new(name: impl Into<String>, region: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            created_at: Utc::now(),
            region: region.into(),
            versioning_enabled: false,
        }
    }
}

/// Compute instance metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InstanceMetadata {
    /// Instance ID
    pub id: ResourceId,
    /// Instance type/size
    pub instance_type: String,
    /// Current state (running, stopped, etc)
    pub state: String,
    /// Private IP address
    pub private_ip: Option<String>,
    /// Public IP address
    pub public_ip: Option<String>,
    /// VPC ID
    pub vpc_id: Option<ResourceId>,
    /// Subnet ID
    pub subnet_id: Option<ResourceId>,
    /// Launch timestamp
    pub launch_time: DateTime<Utc>,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Virtual Private Cloud metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpcMetadata {
    /// VPC ID
    pub id: ResourceId,
    /// CIDR block
    pub cidr_block: String,
    /// Current state
    pub state: String,
    /// Whether it is the default VPC
    pub is_default: bool,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Subnet metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubnetMetadata {
    /// Subnet ID
    pub id: ResourceId,
    /// VPC ID
    pub vpc_id: ResourceId,
    /// CIDR block
    pub cidr_block: String,
    /// Availability zone
    pub availability_zone: String,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Security Group metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityGroupMetadata {
    /// Security Group ID
    pub id: ResourceId,
    /// Group name
    pub name: String,
    /// Description
    pub description: Option<String>,
    /// VPC ID
    pub vpc_id: ResourceId,
    /// Tags
    pub tags: HashMap<String, String>,
}

/// Pagination token for list operations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationToken(pub Option<String>);

impl PaginationToken {
    /// Create an empty pagination token.
    pub fn none() -> Self {
        Self(None)
    }

    /// Create a pagination token with a value.
    pub fn some(token: impl Into<String>) -> Self {
        Self(Some(token.into()))
    }

    /// Check if there are more results.
    pub fn has_more(&self) -> bool {
        self.0.is_some()
    }
}

/// List result with pagination.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListResult<T> {
    /// Items in this page
    pub items: Vec<T>,
    /// Token for the next page (if any)
    pub next_token: PaginationToken,
    /// Total count (if available)
    pub total_count: Option<u64>,
}

impl<T> ListResult<T> {
    /// Create a new list result.
    pub fn new(items: Vec<T>, next_token: PaginationToken) -> Self {
        Self {
            items,
            next_token,
            total_count: None,
        }
    }

    /// Check if there are more pages.
    pub fn has_more(&self) -> bool {
        self.next_token.has_more()
    }
}

/// Content type presets.
pub struct ContentType;

impl ContentType {
    /// Application JSON
    pub const JSON: &'static str = "application/json";
    /// Plain text
    pub const TEXT: &'static str = "text/plain";
    /// Binary data
    pub const BINARY: &'static str = "application/octet-stream";
    /// HTML
    pub const HTML: &'static str = "text/html";
    /// XML
    pub const XML: &'static str = "application/xml";
    /// PDF
    pub const PDF: &'static str = "application/pdf";
    /// PNG image
    pub const PNG: &'static str = "image/png";
    /// JPEG image
    pub const JPEG: &'static str = "image/jpeg";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_id() {
        let id = ResourceId::new("test-123");
        assert_eq!(id.as_str(), "test-123");
        assert_eq!(id.to_string(), "test-123");
    }

    #[test]
    fn test_resource_id_generate() {
        let id1 = ResourceId::generate();
        let id2 = ResourceId::generate();
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_pagination() {
        let list = ListResult::new(
            vec![1, 2, 3],
            PaginationToken::some("next-page"),
        );
        assert!(list.has_more());
        assert_eq!(list.items.len(), 3);
    }
}
