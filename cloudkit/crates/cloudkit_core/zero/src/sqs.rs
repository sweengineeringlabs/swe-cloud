use cloudkit_api::{MessageQueue, Message, SendOptions, ReceiveOptions};
use cloudkit_spi::{CloudResult, CloudError, ResourceId};
use async_trait::async_trait;
use std::time::Duration;
use zero_sdk::ZeroClient;

pub struct ZeroQueue {
    client: ZeroClient,
}

impl ZeroQueue {
    pub fn new(client: ZeroClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl MessageQueue for ZeroQueue {
    async fn create_queue(&self, name: &str) -> CloudResult<String> {
        self.client.queue().create_queue(name).await
            .map_err(|e| CloudError::Internal(e.to_string()))
    }

    async fn delete_queue(&self, _queue_url: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn get_queue_url(&self, name: &str) -> CloudResult<String> {
        // In ZeroCloud, queue name is used directly in URL
        Ok(name.to_string())
    }

    async fn list_queues(&self, _prefix: Option<&str>) -> CloudResult<Vec<String>> {
        self.client.queue().list_queues().await
            .map_err(|e| CloudError::Internal(e.to_string()))
    }

    async fn send(&self, queue_url: &str, body: &str) -> CloudResult<ResourceId> {
        let msg_id = self.client.queue().send_message(queue_url, body).await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(ResourceId::new(msg_id))
    }

    async fn send_with_options(
        &self,
        queue_url: &str,
        body: &str,
        _options: SendOptions,
    ) -> CloudResult<ResourceId> {
        self.send(queue_url, body).await
    }

    async fn send_batch(
        &self,
        _queue_url: &str,
        _messages: &[&str],
    ) -> CloudResult<Vec<ResourceId>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn receive(
        &self,
        queue_url: &str,
        _options: ReceiveOptions,
    ) -> CloudResult<Vec<Message>> {
        let msg = self.client.queue().receive_message(queue_url).await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        
        if let Some(m) = msg {
            Ok(vec![Message {
                id: ResourceId::new(m.id),
                body: m.body,
                receipt_handle: Some(m.receipt_handle),
                attributes: std::collections::HashMap::new(),
                receive_count: 1,
                sent_at: chrono::Utc::now(),
                first_received_at: Some(chrono::Utc::now()),
            }])
        } else {
            Ok(vec![])
        }
    }

    async fn delete(&self, queue_url: &str, message: &Message) -> CloudResult<()> {
        if let Some(handle) = &message.receipt_handle {
            self.client.queue().delete_message(queue_url, handle).await
                .map_err(|e| CloudError::Internal(e.to_string()))?;
            Ok(())
        } else {
            Err(CloudError::Validation("receipt_handle is required for deletion".to_string()))
        }
    }

    async fn delete_batch(&self, _queue_url: &str, _messages: &[&Message]) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn change_visibility(
        &self,
        _queue_url: &str,
        _message: &Message,
        _timeout: Duration,
    ) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn get_queue_depth(&self, _queue_url: &str) -> CloudResult<u64> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn purge(&self, _queue_url: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }
}
