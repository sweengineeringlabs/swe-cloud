//! Azure Event Grid service traits.

use async_trait::async_trait;
use azure_control_spi::CloudResult;
use serde_json::Value;

/// Azure Event Grid service trait.
#[async_trait]
pub trait EventGridService: Send + Sync {
    /// Create a topic.
    async fn create_topic(&self, topic_name: &str) -> CloudResult<String>;

    /// Publish events.
    async fn publish_events(&self, topic_name: &str, events: Vec<Value>) -> CloudResult<()>;

    /// Create an event subscription.
    async fn create_subscription(&self, topic_name: &str, subscription_name: &str, endpoint: &str) -> CloudResult<String>;
}
