//! Pub/Sub trait for publish-subscribe messaging.

use crate::common::{CloudResult, ResourceId};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};

/// A published message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PubSubMessage {
    /// Message ID
    pub id: ResourceId,
    /// Message data
    pub data: Vec<u8>,
    /// Message attributes
    pub attributes: std::collections::HashMap<String, String>,
    /// Publish timestamp
    pub publish_time: chrono::DateTime<chrono::Utc>,
}

/// Subscription configuration.
#[derive(Debug, Clone, Default)]
pub struct SubscriptionConfig {
    /// Push endpoint URL (for push subscriptions)
    pub push_endpoint: Option<String>,
    /// Filter policy
    pub filter_policy: Option<std::collections::HashMap<String, Vec<String>>>,
    /// Dead letter topic
    pub dead_letter_topic: Option<String>,
    /// Max delivery attempts before dead lettering
    pub max_delivery_attempts: Option<u32>,
}

/// Pub/Sub service trait.
///
/// This trait abstracts publish-subscribe messaging across cloud providers:
/// - AWS SNS
/// - Azure Event Grid / Service Bus Topics
/// - Google Cloud Pub/Sub
/// - Oracle Streaming
///
/// # Example
///
/// ```rust,ignore
/// use cloudkit::api::PubSub;
///
/// async fn notify<P: PubSub>(pubsub: &P) -> CloudResult<()> {
///     pubsub.publish("my-topic", b"Event occurred!").await?;
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait PubSub: Send + Sync {
    // =========================================================================
    // Topic Management
    // =========================================================================

    /// Create a topic.
    async fn create_topic(&self, name: &str) -> CloudResult<String>;

    /// Delete a topic.
    async fn delete_topic(&self, topic_arn: &str) -> CloudResult<()>;

    /// List topics.
    async fn list_topics(&self) -> CloudResult<Vec<String>>;

    /// Get topic ARN by name.
    async fn get_topic_arn(&self, name: &str) -> CloudResult<String>;

    // =========================================================================
    // Subscription Management
    // =========================================================================

    /// Subscribe to a topic.
    async fn subscribe(
        &self,
        topic_arn: &str,
        protocol: &str,
        endpoint: &str,
    ) -> CloudResult<String>;

    /// Subscribe with configuration.
    async fn subscribe_with_config(
        &self,
        topic_arn: &str,
        config: SubscriptionConfig,
    ) -> CloudResult<String>;

    /// Unsubscribe.
    async fn unsubscribe(&self, subscription_arn: &str) -> CloudResult<()>;

    /// List subscriptions for a topic.
    async fn list_subscriptions(&self, topic_arn: &str) -> CloudResult<Vec<String>>;

    // =========================================================================
    // Publishing
    // =========================================================================

    /// Publish a message to a topic.
    async fn publish(&self, topic_arn: &str, message: &[u8]) -> CloudResult<ResourceId>;

    /// Publish a message with attributes.
    async fn publish_with_attributes(
        &self,
        topic_arn: &str,
        message: &[u8],
        attributes: std::collections::HashMap<String, String>,
    ) -> CloudResult<ResourceId>;

    /// Publish multiple messages.
    async fn publish_batch(
        &self,
        topic_arn: &str,
        messages: &[&[u8]],
    ) -> CloudResult<Vec<ResourceId>>;

    /// Publish a JSON message.
    async fn publish_json<T: Serialize + Send + Sync>(
        &self,
        topic_arn: &str,
        message: &T,
    ) -> CloudResult<ResourceId>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subscription_config() {
        let config = SubscriptionConfig {
            push_endpoint: Some("https://example.com/webhook".to_string()),
            max_delivery_attempts: Some(5),
            ..Default::default()
        };

        assert_eq!(config.push_endpoint, Some("https://example.com/webhook".to_string()));
        assert_eq!(config.max_delivery_attempts, Some(5));
    }
}
