//! SNS service traits.

use async_trait::async_trait;
use aws_control_spi::CloudResult;
use serde_json::Value;

/// SNS topic service trait.
#[async_trait]
pub trait SnsService: Send + Sync {
    /// Create a topic.
    async fn create_topic(&self, name: &str) -> CloudResult<String>;

    /// Subscribe to a topic.
    async fn subscribe(&self, topic_arn: &str, protocol: &str, endpoint: &str) -> CloudResult<String>;

    /// Publish a message.
    async fn publish(&self, topic_arn: &str, message: &str) -> CloudResult<String>;

    /// List topics.
    async fn list_topics(&self) -> CloudResult<Value>;
}
