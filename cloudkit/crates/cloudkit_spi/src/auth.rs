//! Authentication provider SPI.

use crate::{CloudResult, Credentials};
use async_trait::async_trait;
use std::sync::Arc;

/// Authentication provider trait for custom authentication mechanisms.
///
/// Implement this trait to provide custom credential resolution or
/// authentication flows.
///
/// # Example
///
/// ```rust,ignore
/// use cloudkit::spi::AuthProvider;
///
/// struct VaultAuthProvider {
///     vault_url: String,
/// }
///
/// #[async_trait]
/// impl AuthProvider for VaultAuthProvider {
///     async fn get_credentials(&self) -> CloudResult<Credentials> {
///         // Fetch credentials from HashiCorp Vault
///         todo!()
///     }
/// }
/// ```
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Get current credentials.
    async fn get_credentials(&self) -> CloudResult<Credentials>;

    /// Refresh credentials if expired.
    async fn refresh_credentials(&self) -> CloudResult<Credentials> {
        self.get_credentials().await
    }

    /// Check if credentials are valid.
    async fn is_valid(&self) -> bool {
        self.get_credentials().await.is_ok()
    }
}

/// Default environment-based authentication provider.
#[derive(Debug, Default)]
pub struct EnvAuthProvider;

#[async_trait]
impl AuthProvider for EnvAuthProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        Credentials::from_env()
    }
}

/// Static credentials provider.
#[derive(Debug, Clone)]
pub struct StaticAuthProvider {
    credentials: Credentials,
}

impl StaticAuthProvider {
    /// Create a new static auth provider.
    pub fn new(credentials: Credentials) -> Self {
        Self { credentials }
    }
}

#[async_trait]
impl AuthProvider for StaticAuthProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        Ok(self.credentials.clone())
    }
}

/// Type alias for a boxed auth provider.
pub type BoxedAuthProvider = Arc<dyn AuthProvider>;

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_static_auth_provider() {
        let creds = Credentials::new("access", "secret");
        let provider = StaticAuthProvider::new(creds);
        
        let result = provider.get_credentials().await.unwrap();
        assert_eq!(result.access_key, "access");
        assert_eq!(result.secret_key, "secret");
    }
}
