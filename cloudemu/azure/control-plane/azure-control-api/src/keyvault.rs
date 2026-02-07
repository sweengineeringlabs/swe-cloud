//! Azure Key Vault service traits.

use async_trait::async_trait;
use azure_control_spi::CloudResult;

/// Azure Key Vault service trait.
#[async_trait]
pub trait KeyVaultService: Send + Sync {
    /// Set a secret.
    async fn set_secret(&self, vault_name: &str, secret_name: &str, value: &str) -> CloudResult<String>;

    /// Get a secret.
    async fn get_secret(&self, vault_name: &str, secret_name: &str) -> CloudResult<String>;

    /// Delete a secret.
    async fn delete_secret(&self, vault_name: &str, secret_name: &str) -> CloudResult<()>;

    /// List secrets.
    async fn list_secrets(&self, vault_name: &str) -> CloudResult<Vec<String>>;
}
