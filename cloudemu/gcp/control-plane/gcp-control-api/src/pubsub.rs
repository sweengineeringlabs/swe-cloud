//! GCP Pub/Sub service traits.

use async_trait::async_trait;
use gcp_control_spi::CloudResult;
use serde_json::Value;

/// GCP Pub/Sub service trait.
#[async_trait]
pub trait PubSubService: Send + Sync {
    /// Create a topic.
    async fn create_topic(&self, project_id: &str, topic_id: &str) -> CloudResult<()>;

    /// Delete a topic.
    async fn delete_topic(&self, project_id: &str, topic_id: &str) -> CloudResult<()>;

    /// Publish a message.
    async fn publish(&self, project_id: &str, topic_id: &str, data: Vec<u8>) -> CloudResult<String>;

    /// Create a subscription.
    async fn create_subscription(&self, project_id: &str, subscription_id: &str, topic_id: &str) -> CloudResult<()>;

    /// Pull messages.
    async fn pull(&self, project_id: &str, subscription_id: &str, max_messages: i32) -> CloudResult<Value>;
}
