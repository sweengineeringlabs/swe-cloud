//! Base cloud client implementation.

use crate::{CloudConfig, CloudResult, Region};
use crate::{AuthProvider, BoxedAuthProvider, EnvAuthProvider, ExponentialBackoff, MetricsCollector, NoopMetrics, RetryPolicy};
use std::sync::Arc;

/// Cloud provider type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    /// Amazon Web Services
    Aws,
    /// Microsoft Azure
    Azure,
    /// Google Cloud Platform
    Gcp,
    /// Oracle Cloud Infrastructure
    Oracle,
}

impl std::fmt::Display for ProviderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProviderType::Aws => write!(f, "aws"),
            ProviderType::Azure => write!(f, "azure"),
            ProviderType::Gcp => write!(f, "gcp"),
            ProviderType::Oracle => write!(f, "oracle"),
        }
    }
}

/// Base cloud client context.
///
/// This struct holds the common configuration and services used by all
/// provider implementations.
pub struct CloudContext {
    /// Provider type
    pub provider: ProviderType,
    /// Configuration
    pub config: CloudConfig,
    /// Authentication provider
    pub auth_provider: BoxedAuthProvider,
    /// Retry policy
    pub retry_policy: Arc<dyn RetryPolicy>,
    /// Metrics collector
    pub metrics: Arc<dyn MetricsCollector>,
}

impl std::fmt::Debug for CloudContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CloudContext")
            .field("provider", &self.provider)
            .field("config", &self.config)
            .finish_non_exhaustive()
    }
}

impl CloudContext {
    /// Create a new builder for the context.
    pub fn builder(provider: ProviderType) -> CloudContextBuilder {
        CloudContextBuilder::new(provider)
    }

    /// Get the provider type.
    pub fn provider(&self) -> ProviderType {
        self.provider
    }

    /// Get the region.
    pub fn region(&self) -> &Region {
        &self.config.region
    }
}

/// Builder for [`CloudContext`].
pub struct CloudContextBuilder {
    provider: ProviderType,
    config: Option<CloudConfig>,
    auth_provider: Option<BoxedAuthProvider>,
    retry_policy: Option<Arc<dyn RetryPolicy>>,
    metrics: Option<Arc<dyn MetricsCollector>>,
}

impl CloudContextBuilder {
    /// Create a new builder.
    pub fn new(provider: ProviderType) -> Self {
        Self {
            provider,
            config: None,
            auth_provider: None,
            retry_policy: None,
            metrics: None,
        }
    }

    /// Set the configuration.
    pub fn config(mut self, config: CloudConfig) -> Self {
        self.config = Some(config);
        self
    }

    /// Set the region.
    pub fn region(mut self, region: Region) -> Self {
        let config = self.config.get_or_insert_with(CloudConfig::default);
        config.region = region;
        self
    }

    /// Set a custom endpoint (often used for emulation).
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        let config = self.config.get_or_insert_with(CloudConfig::default);
        config.endpoint = Some(endpoint.into());
        self
    }

    /// Set the authentication provider.
    pub fn auth_provider<A: AuthProvider + 'static>(mut self, provider: A) -> Self {
        self.auth_provider = Some(Arc::new(provider));
        self
    }

    /// Set the retry policy.
    pub fn retry_policy<R: RetryPolicy + 'static>(mut self, policy: R) -> Self {
        self.retry_policy = Some(Arc::new(policy));
        self
    }

    /// Set the metrics collector.
    pub fn metrics<M: MetricsCollector + 'static>(mut self, metrics: M) -> Self {
        self.metrics = Some(Arc::new(metrics));
        self
    }

    /// Build the context.
    pub async fn build(self) -> CloudResult<CloudContext> {
        Ok(CloudContext {
            provider: self.provider,
            config: self.config.unwrap_or_default(),
            auth_provider: self.auth_provider.unwrap_or_else(|| Arc::new(EnvAuthProvider)),
            retry_policy: self.retry_policy.unwrap_or_else(|| Arc::new(ExponentialBackoff::default())),
            metrics: self.metrics.unwrap_or_else(|| Arc::new(NoopMetrics)),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cloud_context_builder() {
        let context = CloudContext::builder(ProviderType::Aws)
            .region(Region::aws_us_east_1())
            .build()
            .await
            .unwrap();

        assert_eq!(context.provider(), ProviderType::Aws);
        assert_eq!(context.region().code(), "us-east-1");
    }
}

