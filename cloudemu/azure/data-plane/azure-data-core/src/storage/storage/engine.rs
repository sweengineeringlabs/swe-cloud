//! Storage engine implementation

use crate::config::Config;
use crate::error::Result;
use rusqlite::Connection;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use parking_lot::Mutex;
use super::schema::SCHEMA;
use serde::{Serialize, Deserialize};

/// Storage engine with SQLite for metadata and filesystem for objects
#[derive(Clone)]
pub struct StorageEngine {
    /// SQLite connection (wrapped for thread safety)
    pub(crate) db: Arc<Mutex<Connection>>,
    /// Directory for object data
    pub(crate) objects_dir: PathBuf,
}

impl StorageEngine {
    /// Create a new storage engine
    pub fn new(config: &Config) -> Result<Self> {
        // Create data directory
        fs::create_dir_all(&config.data_dir)?;
        
        // Create objects directory
        let objects_dir = config.data_dir.join("objects");
        fs::create_dir_all(&objects_dir)?;
        
        // Open SQLite database
        let db_path = config.data_dir.join("metadata.db");
        let conn = Connection::open(&db_path)?;
        
        // Enable WAL mode for better concurrency
        conn.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=NORMAL;")?;
        
        // Create schema
        conn.execute_batch(SCHEMA)?;
        
        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            objects_dir,
        })
    }
    
    /// Create a new in-memory storage engine (for testing)
    pub fn in_memory() -> Result<Self> {
        let conn = Connection::open_in_memory()?;
        conn.execute_batch(SCHEMA)?;
        
        let temp_dir = std::env::temp_dir().join(format!("cloudemu-{}", uuid::Uuid::new_v4()));
        fs::create_dir_all(&temp_dir)?;
        
        Ok(Self {
            db: Arc::new(Mutex::new(conn)),
            objects_dir: temp_dir,
        })
    }
    
    // ==================== Object Data Storage ====================
    
    /// Store object data to filesystem, returns content hash
    pub(crate) fn store_object_data(&self, data: &[u8]) -> Result<String> {
        use sha2::{Sha256, Digest};
        
        let mut hasher = Sha256::new();
        hasher.update(data);
        let hash = hex::encode(hasher.finalize());
        
        // Content-addressed storage: first 2 chars as directory
        let dir = self.objects_dir.join(&hash[..2]);
        fs::create_dir_all(&dir)?;
        
        let file_path = dir.join(&hash);
        if !file_path.exists() {
            fs::write(&file_path, data)?;
        }
        
        Ok(hash)
    }
    
    /// Read object data from filesystem
    pub(crate) fn read_object_data(&self, content_hash: &str) -> Result<Vec<u8>> {
        if content_hash.is_empty() {
            return Ok(Vec::new());
        }
        
        let file_path = self.objects_dir.join(&content_hash[..2]).join(content_hash);
        fs::read(&file_path).map_err(|e| crate::error::EmulatorError::Internal(e.to_string()))
    }
    
    // Bucket Operations moved to s3.rs
    
    // Object Operations moved to s3.rs
    
    // KMS Operations moved to kms.rs

    // Step Functions Operations moved to workflows.rs

    // Cognito Operations moved to identity.rs

    // CloudWatch Operations moved to monitoring.rs

    // EventBridge Operations moved to events.rs

    // Secrets Operations moved to secrets.rs

    // Object Data Storage helpers moved to s3.rs

    // SQS Operations moved to sqs.rs

    // DynamoDB Operations moved to dynamodb.rs

    // SNS Operations moved to sns.rs

    // Lambda Operations moved to lambda.rs
}

/// Bucket metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BucketMetadata {
    pub name: String,
    pub region: String,
    pub created_at: String,
    pub versioning: String,
    pub policy: Option<String>,
    pub acl: Option<String>,
}

/// Object metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectMetadata {
    pub key: String,
    pub version_id: Option<String>,
    pub etag: String,
    pub size: u64,
    pub last_modified: String,
    pub content_type: String,
    pub storage_class: String,
    pub is_delete_marker: bool,
}

/// List objects result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListObjectsResult {
    pub name: String,
    pub prefix: Option<String>,
    pub delimiter: Option<String>,
    pub max_keys: u32,
    pub is_truncated: bool,
    pub contents: Vec<ObjectMetadata>,
    pub common_prefixes: Vec<String>,
    pub continuation_token: Option<String>,
    pub next_continuation_token: Option<String>,
}

/// Secret metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMetadata {
    pub arn: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: String,
    pub last_changed_date: Option<String>,
    pub tags: Option<String>,
}

/// Secret value
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretValue {
    pub arn: String,
    pub name: String,
    pub version_id: String,
    pub secret_string: Option<String>,
    pub secret_binary: Option<Vec<u8>>,
    pub version_stages: Vec<String>,
    pub created_date: String,
}

/// KMS Key metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KmsKeyMetadata {
    pub id: String,
    pub arn: String,
    pub description: Option<String>,
    pub key_usage: String,
    pub key_state: String,
    pub created_at: String,
    pub tags: Option<String>,
}

/// Event Bus metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventBusMetadata {
    pub name: String,
    pub arn: String,
    pub policy: Option<String>,
}

/// Event Rule metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventRuleMetadata {
    pub name: String,
    pub event_bus_name: String,
    pub arn: String,
    pub event_pattern: Option<String>,
    pub state: String,
    pub description: Option<String>,
    pub schedule_expression: Option<String>,
    pub created_at: String,
}

/// Event Target metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventTargetMetadata {
    pub id: String,
    pub rule_name: String,
    pub event_bus_name: String,
    pub arn: String,
    pub input: Option<String>,
    pub input_path: Option<String>,
}

/// CloudWatch Metric metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricMetadata {
    pub namespace: String,
    pub metric_name: String,
    pub dimensions: Option<String>,
    pub value: f64,
    pub unit: Option<String>,
    pub timestamp: String,
}

/// CloudWatch Log Group metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogGroupMetadata {
    pub name: String,
    pub arn: String,
    pub retention_days: Option<i32>,
    pub created_at: String,
}

/// CloudWatch Log Stream metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStreamMetadata {
    pub name: String,
    pub log_group_name: String,
    pub arn: String,
    pub created_at: String,
}

/// CloudWatch Log Event metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEventMetadata {
    pub timestamp: String,
    pub message: String,
}

/// Cognito User Pool metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserPoolMetadata {
    pub id: String,
    pub name: String,
    pub arn: String,
    pub created_at: String,
}

/// Cognito Group metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroupMetadata {
    pub user_pool_id: String,
    pub group_name: String,
    pub description: Option<String>,
    pub precedence: Option<i32>,
    pub created_at: String,
}

/// Cognito User metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMetadata {
    pub user_pool_id: String,
    pub username: String,
    pub email: Option<String>,
    pub status: String,
    pub enabled: bool,
    pub created_at: String,
}

/// Step Functions State Machine metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateMachineMetadata {
    pub arn: String,
    pub name: String,
    pub definition: String,
    pub role_arn: String,
    pub machine_type: String,
    pub created_at: String,
}

/// Step Functions Execution metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionMetadata {
    pub arn: String,
    pub state_machine_arn: String,
    pub name: String,
    pub status: String,
    pub input: Option<String>,
    pub output: Option<String>,
    pub start_date: String,
    pub stop_date: Option<String>,
}

/// SQS Queue metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueMetadata {
    pub name: String,
    pub url: String,
    pub arn: String,
    pub created_at: String,
    pub visibility_timeout: i32,
    pub message_retention_period: i32,
    pub delay_seconds: i32,
    pub receive_message_wait_time_seconds: i32,
}

/// SQS Message metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageMetadata {
    pub id: String,
    pub queue_name: String,
    pub body: String,
    pub md5_body: Option<String>,
    pub sent_at: String,
    pub visible_at: String,
    pub receipt_handle: Option<String>,
    pub receive_count: i32,
}

/// DynamoDB Table metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TableMetadata {
    pub name: String,
    pub arn: String,
    pub status: String,
    pub attribute_definitions: String,
    pub key_schema: String,
    pub created_at: String,
}

/// DynamoDB Item metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemMetadata {
    pub table_name: String,
    pub partition_key: String,
    pub sort_key: Option<String>,
    pub item_json: String,
}

/// SNS Topic metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicMetadata {
    pub name: String,
    pub arn: String,
    pub display_name: Option<String>,
    pub created_at: String,
}

/// SNS Subscription metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionMetadata {
    pub arn: String,
    pub topic_arn: String,
    pub protocol: String,
    pub endpoint: String,
    pub created_at: String,
}

/// Lambda metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LambdaMetadata {
    pub name: String,
    pub arn: String,
    pub runtime: String,
    pub handler: String,
    pub last_modified: String,
}

// ==================== Azure Metadata ====================

/// Azure Storage Account metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageAccountMetadata {
    pub name: String,
    pub location: String,
    pub resource_group: String,
    pub sku_name: String,
    pub kind: String,
    pub access_tier: Option<String>,
    pub created_at: String,
}

/// Azure Container metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobContainerMetadata {
    pub name: String,
    pub account_name: String,
    pub public_access: String,
    pub etag: String,
    pub last_modified: String,
}

/// Azure Blob metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlobMetadata {
    pub name: String,
    pub container_name: String,
    pub account_name: String,
    pub blob_type: String,
    pub access_tier: String,
    pub size: u64,
    pub content_type: Option<String>,
    pub etag: String,
    pub last_modified: String,
}

/// Azure Cosmos Account metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosAccountMetadata {
    pub name: String,
    pub location: String,
    pub resource_group: String,
    pub kind: String,
    pub created_at: String,
}

/// Azure Cosmos Database metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosDatabaseMetadata {
    pub name: String,
    pub account_name: String,
    pub throughput: Option<i32>,
    pub etag: String,
}

/// Azure Cosmos Container metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosContainerMetadata {
    pub name: String,
    pub database_name: String,
    pub account_name: String,
    pub partition_key_path: String,
    pub throughput: Option<i32>,
    pub etag: String,
}

/// Azure Cosmos Item metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CosmosItemMetadata {
    pub id: String,
    pub container_name: String,
    pub database_name: String,
    pub account_name: String,
    pub partition_key_value: String,
    pub item_json: String,
    pub last_modified: i64,
    pub etag: String,
}

/// Azure Event Grid Topic metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGridTopicMetadata {
    pub name: String,
    pub location: String,
    pub resource_group: String,
    pub endpoint: String,
    pub created_at: String,
}

/// Azure Event Grid Subscription metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventGridSubscriptionMetadata {
    pub name: String,
    pub topic_name: String,
    pub endpoint: String,
    pub protocol: String,
    pub created_at: String,
}

/// Azure Virtual Machine metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VirtualMachineMetadata {
    pub name: String,
    pub location: String,
    pub resource_group: String,
    pub vm_size: String,
    pub os_type: String,
    pub admin_username: String,
    pub private_ip: Option<String>,
    pub public_ip: Option<String>,
    pub status: String,
    pub created_at: String,
}
