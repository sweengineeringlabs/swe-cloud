//! Lambda service traits.

use async_trait::async_trait;
use aws_control_spi::CloudResult;
use serde_json::Value;

/// Lambda function service trait.
#[async_trait]
pub trait LambdaService: Send + Sync {
    /// Create a function.
    async fn create_function(&self, input: Value) -> CloudResult<Value>;

    /// Invoke a function.
    async fn invoke(&self, function_name: &str, payload: Value) -> CloudResult<Value>;
}
