//! S3 Service - High-level operations

use data_plane::storage::StorageEngine;

/// S3 Service
pub struct S3Service {
    storage: StorageEngine,
}

impl S3Service {
    /// Create a new S3 service
    pub fn new(storage: StorageEngine) -> Self {
        Self { storage }
    }
    
    /// Get the storage engine
    pub fn storage(&self) -> &StorageEngine {
        &self.storage
    }
}
