//! Message queue trait for queue operations.

use cloudkit_spi::{CloudResult, ResourceId};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// A message in the queue.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    /// Message ID
    pub id: ResourceId,
    /// Message body
    pub body: String,
    /// Receipt handle for deletion
    pub receipt_handle: Option<String>,
    /// Message attributes
    pub attributes: std::collections::HashMap<String, String>,
    /// Approximate receive count
    pub receive_count: u32,
    /// Sent timestamp
    pub sent_at: DateTime<Utc>,
    /// First receive timestamp
    pub first_received_at: Option<DateTime<Utc>>,
}

/// Options for sending messages.
#[derive(Debug, Clone, Default)]
pub struct SendOptions {
    /// Delay before message becomes visible
    pub delay: Option<Duration>,
    /// Message group ID (for FIFO queues)
    pub message_group_id: Option<String>,
    /// Deduplication ID (for FIFO queues)
    pub deduplication_id: Option<String>,
    /// Message attributes
    pub attributes: std::collections::HashMap<String, String>,
}

impl SendOptions {
    /// Create new send options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set delay.
    pub fn delay(mut self, delay: Duration) -> Self {
        self.delay = Some(delay);
        self
    }

    /// Set message group ID.
    pub fn message_group_id(mut self, id: impl Into<String>) -> Self {
        self.message_group_id = Some(id.into());
        self
    }

    /// Add an attribute.
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }
}

/// Options for receiving messages.
#[derive(Debug, Clone, Default)]
pub struct ReceiveOptions {
    /// Maximum number of messages
    pub max_messages: Option<u32>,
    /// Visibility timeout
    pub visibility_timeout: Option<Duration>,
    /// Wait time for long polling
    pub wait_time: Option<Duration>,
}

impl ReceiveOptions {
    /// Create new receive options.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set max messages.
    pub fn max_messages(mut self, max: u32) -> Self {
        self.max_messages = Some(max);
        self
    }

    /// Set visibility timeout.
    pub fn visibility_timeout(mut self, timeout: Duration) -> Self {
        self.visibility_timeout = Some(timeout);
        self
    }

    /// Set wait time for long polling.
    pub fn wait_time(mut self, time: Duration) -> Self {
        self.wait_time = Some(time);
        self
    }
}

/// Message queue service trait.
///
/// This trait abstracts queue operations across cloud providers:
/// - AWS SQS
/// - Azure Service Bus Queue
/// - Google Cloud Tasks / Pub/Sub
/// - Oracle Streaming
///
/// # Example
///
/// ```rust,ignore
/// use cloudkit::api::MessageQueue;
///
/// async fn process_queue<Q: MessageQueue>(queue: &Q) -> CloudResult<()> {
///     // Send a message
///     queue.send("my-queue", "Hello, World!").await?;
///     
///     // Receive messages
///     let messages = queue.receive("my-queue", ReceiveOptions::new().max_messages(10)).await?;
///     
///     for msg in messages {
///         println!("Processing: {}", msg.body);
///         queue.delete("my-queue", &msg).await?;
///     }
///     
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait MessageQueue: Send + Sync {
    // =========================================================================
    // Queue Management
    // =========================================================================

    /// Create a queue.
    async fn create_queue(&self, name: &str) -> CloudResult<String>;

    /// Delete a queue.
    async fn delete_queue(&self, queue_url: &str) -> CloudResult<()>;

    /// Get queue URL by name.
    async fn get_queue_url(&self, name: &str) -> CloudResult<String>;

    /// List queues.
    async fn list_queues(&self, prefix: Option<&str>) -> CloudResult<Vec<String>>;

    // =========================================================================
    // Message Operations
    // =========================================================================

    /// Send a message to the queue.
    async fn send(&self, queue_url: &str, body: &str) -> CloudResult<ResourceId>;

    /// Send a message with options.
    async fn send_with_options(
        &self,
        queue_url: &str,
        body: &str,
        options: SendOptions,
    ) -> CloudResult<ResourceId>;

    /// Send multiple messages.
    async fn send_batch(
        &self,
        queue_url: &str,
        messages: &[&str],
    ) -> CloudResult<Vec<ResourceId>>;

    /// Receive messages from the queue.
    async fn receive(
        &self,
        queue_url: &str,
        options: ReceiveOptions,
    ) -> CloudResult<Vec<Message>>;

    /// Delete a message from the queue.
    async fn delete(&self, queue_url: &str, message: &Message) -> CloudResult<()>;

    /// Delete multiple messages.
    async fn delete_batch(&self, queue_url: &str, messages: &[&Message]) -> CloudResult<()>;

    /// Change message visibility timeout.
    async fn change_visibility(
        &self,
        queue_url: &str,
        message: &Message,
        timeout: Duration,
    ) -> CloudResult<()>;

    /// Get approximate number of messages in queue.
    async fn get_queue_depth(&self, queue_url: &str) -> CloudResult<u64>;

    /// Purge all messages from queue.
    async fn purge(&self, queue_url: &str) -> CloudResult<()>;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_send_options_builder() {
        let options = SendOptions::new()
            .delay(Duration::from_secs(10))
            .message_group_id("group-1")
            .attribute("key", "value");

        assert_eq!(options.delay, Some(Duration::from_secs(10)));
        assert_eq!(options.message_group_id, Some("group-1".to_string()));
        assert_eq!(options.attributes.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_receive_options_builder() {
        let options = ReceiveOptions::new()
            .max_messages(10)
            .visibility_timeout(Duration::from_secs(30))
            .wait_time(Duration::from_secs(20));

        assert_eq!(options.max_messages, Some(10));
        assert_eq!(options.visibility_timeout, Some(Duration::from_secs(30)));
        assert_eq!(options.wait_time, Some(Duration::from_secs(20)));
    }
}

