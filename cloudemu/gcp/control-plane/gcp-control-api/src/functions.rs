//! GCP Cloud Functions service traits.

use async_trait::async_trait;
use gcp_control_spi::CloudResult;
use serde_json::Value;

/// GCP Cloud Functions service trait.
#[async_trait]
pub trait CloudFunctionsService: Send + Sync {
    /// Create a function.
    async fn create_function(&self, input: Value) -> CloudResult<Value>;

    /// Invoke a function.
    async fn invoke_function(&self, name: &str, payload: Value) -> CloudResult<Value>;
}
