//! SQS service traits.

use async_trait::async_trait;
use aws_control_spi::CloudResult;
use serde_json::Value;

/// SQS queue service trait.
#[async_trait]
pub trait SqsService: Send + Sync {
    /// Create a queue.
    async fn create_queue(&self, queue_name: &str, attributes: Value) -> CloudResult<String>;

    /// Send a message.
    async fn send_message(&self, queue_url: &str, message_body: &str) -> CloudResult<String>;

    /// Receive messages.
    async fn receive_message(&self, queue_url: &str, max_messages: Option<i32>) -> CloudResult<Value>;

    /// Delete a message.
    async fn delete_message(&self, queue_url: &str, receipt_handle: &str) -> CloudResult<()>;

    /// List queues.
    async fn list_queues(&self) -> CloudResult<Value>;
}
