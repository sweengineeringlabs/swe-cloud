//! Azure Service Bus service traits.

use async_trait::async_trait;
use azure_control_spi::CloudResult;
use serde_json::Value;

/// Azure Service Bus service trait.
#[async_trait]
pub trait ServiceBusService: Send + Sync {
    /// Create a queue.
    async fn create_queue(&self, queue_name: &str) -> CloudResult<()>;

    /// Send a message to a queue.
    async fn send_message(&self, queue_name: &str, body: &str) -> CloudResult<String>;

    /// Receive messages from a queue.
    async fn receive_messages(&self, queue_name: &str, max_messages: Option<i32>) -> CloudResult<Value>;

    /// Create a topic.
    async fn create_topic(&self, topic_name: &str) -> CloudResult<()>;

    /// Publish to a topic.
    async fn publish(&self, topic_name: &str, body: &str) -> CloudResult<String>;
}
