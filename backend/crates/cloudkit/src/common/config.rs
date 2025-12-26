//! Configuration types for CloudKit.

use super::{CloudResult, Region};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Main configuration for CloudKit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CloudConfig {
    /// Target region
    pub region: Region,
    /// Connection timeout
    pub timeout: Duration,
    /// Request timeout
    pub request_timeout: Duration,
    /// Maximum retry attempts
    pub max_retries: u32,
    /// Enable request tracing
    pub enable_tracing: bool,
    /// Custom endpoint (for local testing or private endpoints)
    pub endpoint: Option<String>,
}

impl Default for CloudConfig {
    fn default() -> Self {
        Self {
            region: Region::default(),
            timeout: Duration::from_secs(30),
            request_timeout: Duration::from_secs(60),
            max_retries: 3,
            enable_tracing: false,
            endpoint: None,
        }
    }
}

impl CloudConfig {
    /// Create a new configuration builder.
    pub fn builder() -> CloudConfigBuilder {
        CloudConfigBuilder::default()
    }
}

/// Builder for [`CloudConfig`].
#[derive(Debug, Default)]
pub struct CloudConfigBuilder {
    region: Option<Region>,
    timeout: Option<Duration>,
    request_timeout: Option<Duration>,
    max_retries: Option<u32>,
    enable_tracing: Option<bool>,
    endpoint: Option<String>,
}

impl CloudConfigBuilder {
    /// Set the target region.
    pub fn region(mut self, region: Region) -> Self {
        self.region = Some(region);
        self
    }

    /// Set the connection timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set the request timeout.
    pub fn request_timeout(mut self, timeout: Duration) -> Self {
        self.request_timeout = Some(timeout);
        self
    }

    /// Set the maximum retry attempts.
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = Some(retries);
        self
    }

    /// Enable or disable request tracing.
    pub fn enable_tracing(mut self, enabled: bool) -> Self {
        self.enable_tracing = Some(enabled);
        self
    }

    /// Set a custom endpoint.
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    /// Build the configuration.
    pub fn build(self) -> CloudResult<CloudConfig> {
        let default = CloudConfig::default();
        
        Ok(CloudConfig {
            region: self.region.unwrap_or(default.region),
            timeout: self.timeout.unwrap_or(default.timeout),
            request_timeout: self.request_timeout.unwrap_or(default.request_timeout),
            max_retries: self.max_retries.unwrap_or(default.max_retries),
            enable_tracing: self.enable_tracing.unwrap_or(default.enable_tracing),
            endpoint: self.endpoint.or(default.endpoint),
        })
    }
}

/// Credentials for cloud authentication.
#[derive(Debug, Clone)]
pub struct Credentials {
    /// Access key or client ID
    pub access_key: String,
    /// Secret key or client secret
    pub secret_key: String,
    /// Optional session token (for temporary credentials)
    pub session_token: Option<String>,
}

impl Credentials {
    /// Create new credentials.
    pub fn new(access_key: impl Into<String>, secret_key: impl Into<String>) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
            session_token: None,
        }
    }

    /// Create credentials with a session token.
    pub fn with_session_token(
        access_key: impl Into<String>,
        secret_key: impl Into<String>,
        session_token: impl Into<String>,
    ) -> Self {
        Self {
            access_key: access_key.into(),
            secret_key: secret_key.into(),
            session_token: Some(session_token.into()),
        }
    }

    /// Load credentials from environment variables.
    ///
    /// # Environment Variables
    /// - `CLOUD_ACCESS_KEY` or `AWS_ACCESS_KEY_ID`
    /// - `CLOUD_SECRET_KEY` or `AWS_SECRET_ACCESS_KEY`
    /// - `CLOUD_SESSION_TOKEN` or `AWS_SESSION_TOKEN` (optional)
    pub fn from_env() -> CloudResult<Self> {
        let access_key = std::env::var("CLOUD_ACCESS_KEY")
            .or_else(|_| std::env::var("AWS_ACCESS_KEY_ID"))
            .map_err(|_| super::CloudError::Auth(
                super::AuthError::MissingCredentials("Access key not found in environment".to_string())
            ))?;

        let secret_key = std::env::var("CLOUD_SECRET_KEY")
            .or_else(|_| std::env::var("AWS_SECRET_ACCESS_KEY"))
            .map_err(|_| super::CloudError::Auth(
                super::AuthError::MissingCredentials("Secret key not found in environment".to_string())
            ))?;

        let session_token = std::env::var("CLOUD_SESSION_TOKEN")
            .or_else(|_| std::env::var("AWS_SESSION_TOKEN"))
            .ok();

        Ok(Self {
            access_key,
            secret_key,
            session_token,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = CloudConfig::builder()
            .region(Region::aws_eu_west_1())
            .timeout(Duration::from_secs(60))
            .max_retries(5)
            .build()
            .unwrap();

        assert_eq!(config.region.code(), "eu-west-1");
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert_eq!(config.max_retries, 5);
    }

    #[test]
    fn test_credentials_new() {
        let creds = Credentials::new("access", "secret");
        assert_eq!(creds.access_key, "access");
        assert_eq!(creds.secret_key, "secret");
        assert!(creds.session_token.is_none());
    }
}
