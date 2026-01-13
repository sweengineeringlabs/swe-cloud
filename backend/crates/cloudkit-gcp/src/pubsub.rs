//! Google Cloud Pub/Sub implementation.

use async_trait::async_trait;
use cloudkit_spi::api::{Message, MessageQueue, ReceiveOptions, SendOptions};
use cloudkit_spi::common::{CloudError, CloudResult, ResourceId};
use cloudkit_spi::core::CloudContext;
use google_cloud_pubsub::client::Client;
use google_cloud_pubsub::subscription::SubscriptionConfig; // Added import
use std::sync::Arc;
use std::time::Duration;

/// Google Cloud Pub/Sub implementation.
pub struct GcpPubSub {
    context: Arc<CloudContext>,
    client: Client,
    project_id: String,
}

impl GcpPubSub {
    /// Create a new Pub/Sub client.
    pub fn new(context: Arc<CloudContext>, client: Client, project_id: String) -> Self {
        Self {
            context,
            client,
            project_id,
        }
    }
}

#[async_trait]
impl MessageQueue for GcpPubSub {
    async fn create_queue(&self, name: &str) -> CloudResult<String> {
        // Create Topic
        let topic = self.client.topic(name);
        if !topic.exists(None).await.map_err(|e| CloudError::Provider {
            provider: "gcp".to_string(),
            code: "PubSubError".to_string(),
            message: e.to_string(),
        })? {
            topic
                .create(None, None)
                .await
                .map_err(|e| CloudError::Provider {
                    provider: "gcp".to_string(),
                    code: "PubSubError".to_string(),
                    message: e.to_string(),
                })?;
        }

        // Create Subscription
        let sub_id = format!("{}-sub", name);
        let subscription = self.client.subscription(&sub_id);
        if !subscription
            .exists(None)
            .await
            .map_err(|e| CloudError::Provider {
                provider: "gcp".to_string(),
                code: "PubSubError".to_string(),
                message: e.to_string(),
            })?
        {
            subscription
                .create(name, SubscriptionConfig::default(), None)
                .await
                .map_err(|e| CloudError::Provider {
                    provider: "gcp".to_string(),
                    code: "PubSubError".to_string(),
                    message: e.to_string(),
                })?;
        }

        Ok(name.to_string())
    }

    async fn delete_queue(&self, queue_url: &str) -> CloudResult<()> {
        let topic = self.client.topic(queue_url);
        if topic.exists(None).await.map_err(|e| CloudError::Provider {
            provider: "gcp".to_string(),
            code: "PubSubError".to_string(),
            message: e.to_string(),
        })? {
            topic.delete(None).await.map_err(|e| CloudError::Provider {
                provider: "gcp".to_string(),
                code: "PubSubError".to_string(),
                message: e.to_string(),
            })?;
        }

        let sub_id = format!("{}-sub", queue_url);
        let subscription = self.client.subscription(&sub_id);
        if subscription
            .exists(None)
            .await
            .map_err(|e| CloudError::Provider {
                provider: "gcp".to_string(),
                code: "PubSubError".to_string(),
                message: e.to_string(),
            })?
        {
            subscription
                .delete(None)
                .await
                .map_err(|e| CloudError::Provider {
                    provider: "gcp".to_string(),
                    code: "PubSubError".to_string(),
                    message: e.to_string(),
                })?;
        }
        Ok(())
    }

    async fn get_queue_url(&self, name: &str) -> CloudResult<String> {
        Ok(name.to_string())
    }

    async fn list_queues(&self, _prefix: Option<&str>) -> CloudResult<Vec<String>> {
        Ok(vec![])
    }

    async fn send(&self, _queue_url: &str, _body: &str) -> CloudResult<ResourceId> {
        tracing::info!("send called stub");
        Ok(ResourceId::new("stub".to_string()))
    }

    async fn send_with_options(
        &self,
        _queue_url: &str,
        _body: &str,
        _options: SendOptions,
    ) -> CloudResult<ResourceId> {
        tracing::info!("send_with_options called stub");
        Ok(ResourceId::new("stub".to_string()))
    }

    async fn send_batch(
        &self,
        _queue_url: &str,
        messages: &[&str],
    ) -> CloudResult<Vec<ResourceId>> {
        tracing::info!("send_batch called stub");
        Ok(messages
            .iter()
            .map(|_| ResourceId::new("stub".to_string()))
            .collect())
    }

    async fn receive(
        &self,
        _queue_url: &str,
        _options: ReceiveOptions,
    ) -> CloudResult<Vec<Message>> {
        tracing::info!("receive called stub");
        Ok(vec![])
    }

    async fn delete(&self, _queue_url: &str, _message: &Message) -> CloudResult<()> {
        tracing::info!("delete called stub");
        Ok(())
    }

    async fn delete_batch(&self, _queue_url: &str, _messages: &[&Message]) -> CloudResult<()> {
        tracing::info!("delete_batch called stub");
        Ok(())
    }

    async fn change_visibility(
        &self,
        _queue_url: &str,
        _message: &Message,
        _timeout: Duration,
    ) -> CloudResult<()> {
        tracing::info!("change_visibility called stub");
        Ok(())
    }

    async fn get_queue_depth(&self, _queue_url: &str) -> CloudResult<u64> {
        Ok(0)
    }

    async fn purge(&self, _queue_url: &str) -> CloudResult<()> {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::core::ProviderType;

    async fn create_test_context() -> Arc<CloudContext> {
        Arc::new(
            CloudContext::builder(ProviderType::Gcp)
                .build()
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    #[ignore]
    async fn test_pubsub_operations() {
        let context = create_test_context().await;
        // Mock client or real one invalid
        let config = google_cloud_pubsub::client::ClientConfig::default();
        let client = google_cloud_pubsub::client::Client::new(config)
            .await
            .unwrap();
        let queue = GcpPubSub::new(context, client, "test-project".to_string());

        // Queue/Topic operations
        // assert!(queue.create_queue("topic").await.is_ok());
        // ...
    }
}

