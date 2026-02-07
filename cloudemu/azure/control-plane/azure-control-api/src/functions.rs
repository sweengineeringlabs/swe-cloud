//! Azure Functions service traits.

use async_trait::async_trait;
use azure_control_spi::CloudResult;
use serde_json::Value;

/// Azure Functions service trait.
#[async_trait]
pub trait AzureFunctionsService: Send + Sync {
    /// Create a function app.
    async fn create_function_app(&self, input: Value) -> CloudResult<Value>;

    /// Invoke a function.
    async fn invoke_function(&self, app_name: &str, function_name: &str, payload: Value) -> CloudResult<Value>;
}
