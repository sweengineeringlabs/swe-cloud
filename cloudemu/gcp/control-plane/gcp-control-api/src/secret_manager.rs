//! GCP Secret Manager service traits.

use async_trait::async_trait;
use gcp_control_spi::CloudResult;

/// GCP Secret Manager service trait.
#[async_trait]
pub trait SecretManagerService: Send + Sync {
    /// Create a secret.
    async fn create_secret(&self, project_id: &str, secret_id: &str) -> CloudResult<String>;

    /// Add a secret version.
    async fn add_secret_version(&self, secret_name: &str, payload: Vec<u8>) -> CloudResult<String>;

    /// Access a secret version.
    async fn access_secret_version(&self, version_name: &str) -> CloudResult<Vec<u8>>;

    /// Delete a secret.
    async fn delete_secret(&self, secret_name: &str) -> CloudResult<()>;

    /// List secrets.
    async fn list_secrets(&self, project_id: &str) -> CloudResult<Vec<String>>;
}
