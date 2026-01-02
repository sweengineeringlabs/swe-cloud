//! # Secrets Management API
//!
//! Cross-cloud secret management operations.
//!
//! ## Implementations
//!
//! - **AWS**: Secrets Manager
//! - **Azure**: Key Vault Secrets
//! - **GCP**: Secret Manager

use async_trait::async_trait;
use crate::common::{CloudResult, Metadata};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Secret metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretMetadata {
    /// Secret name/identifier.
    pub name: String,
    /// Secret ARN or full resource path.
    pub arn: Option<String>,
    /// Description of the secret.
    pub description: Option<String>,
    /// When the secret was created.
    pub created_at: Option<DateTime<Utc>>,
    /// When the secret was last updated.
    pub updated_at: Option<DateTime<Utc>>,
    /// When the secret was last accessed.
    pub last_accessed_at: Option<DateTime<Utc>>,
    /// Tags/labels on the secret.
    pub tags: Metadata,
    /// Current version ID.
    pub version_id: Option<String>,
    /// Rotation enabled.
    pub rotation_enabled: bool,
}

impl SecretMetadata {
    /// Create new secret metadata.
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            arn: None,
            description: None,
            created_at: None,
            updated_at: None,
            last_accessed_at: None,
            tags: Metadata::new(),
            version_id: None,
            rotation_enabled: false,
        }
    }
}

/// Secret version information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecretVersion {
    /// Version ID.
    pub version_id: String,
    /// Version stages (e.g., AWSCURRENT, AWSPREVIOUS).
    pub stages: Vec<String>,
    /// When this version was created.
    pub created_at: Option<DateTime<Utc>>,
}

/// Options for creating a secret.
#[derive(Debug, Clone, Default)]
pub struct CreateSecretOptions {
    /// Description of the secret.
    pub description: Option<String>,
    /// Tags to apply to the secret.
    pub tags: Metadata,
    /// KMS key ID for encryption.
    pub kms_key_id: Option<String>,
}

/// Secrets management operations.
#[async_trait]
pub trait SecretsManager: Send + Sync {
    /// Create a new secret.
    async fn create_secret(
        &self,
        name: &str,
        value: &str,
        options: CreateSecretOptions,
    ) -> CloudResult<SecretMetadata>;

    /// Get the current value of a secret.
    async fn get_secret(&self, name: &str) -> CloudResult<String>;

    /// Get a specific version of a secret.
    async fn get_secret_version(&self, name: &str, version_id: &str) -> CloudResult<String>;

    /// Update the value of an existing secret.
    async fn update_secret(&self, name: &str, value: &str) -> CloudResult<SecretMetadata>;

    /// Delete a secret.
    async fn delete_secret(&self, name: &str, force: bool) -> CloudResult<()>;

    /// Restore a deleted secret (if within recovery window).
    async fn restore_secret(&self, name: &str) -> CloudResult<SecretMetadata>;

    /// List all secrets.
    async fn list_secrets(&self) -> CloudResult<Vec<SecretMetadata>>;

    /// Get secret metadata without retrieving the value.
    async fn describe_secret(&self, name: &str) -> CloudResult<SecretMetadata>;

    /// List all versions of a secret.
    async fn list_secret_versions(&self, name: &str) -> CloudResult<Vec<SecretVersion>>;

    /// Rotate a secret (triggers rotation lambda if configured).
    async fn rotate_secret(&self, name: &str) -> CloudResult<()>;

    /// Tag a secret.
    async fn tag_secret(&self, name: &str, tags: Metadata) -> CloudResult<()>;

    /// Remove tags from a secret.
    async fn untag_secret(&self, name: &str, tag_keys: &[&str]) -> CloudResult<()>;
}
