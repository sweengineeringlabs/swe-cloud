//! GCP Firestore service traits.

use async_trait::async_trait;
use gcp_control_spi::CloudResult;
use serde_json::Value;

/// GCP Firestore service trait.
#[async_trait]
pub trait FirestoreService: Send + Sync {
    /// Create a document.
    async fn create_document(&self, collection: &str, document_id: &str, data: Value) -> CloudResult<Value>;

    /// Get a document.
    async fn get_document(&self, collection: &str, document_id: &str) -> CloudResult<Option<Value>>;

    /// Update a document.
    async fn update_document(&self, collection: &str, document_id: &str, data: Value) -> CloudResult<Value>;

    /// Delete a document.
    async fn delete_document(&self, collection: &str, document_id: &str) -> CloudResult<()>;

    /// Query documents.
    async fn query_documents(&self, collection: &str, query: Value) -> CloudResult<Vec<Value>>;
}
