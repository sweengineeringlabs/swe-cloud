use axum::{
    routing::{get, post, put},
    Router,
};
use gcp_data_core::StorageEngine;
use std::sync::Arc;
use crate::handlers;

pub fn create_router(storage: Arc<StorageEngine>) -> Router {
    Router::new()
        // Health
        .route("/health", get(|| async { "OK" }))

        // GCS
        // Note: Real GCS is /storage/v1/b/{bucket}/o/{object}
        // Simplified route for now
        .route("/storage/:bucket", put(handlers::gcs::create_bucket))
        .route("/storage/:bucket/o", get(handlers::gcs::list_objects))
        .route("/storage/:bucket/o/:object", put(handlers::gcs::put_object).get(handlers::gcs::get_object))
        
        // Firestore
        // Real Firestore: /v1/projects/{project}/databases/{database}/documents/{collection}/{document}
        .route("/firestore/:project/databases/:database", put(handlers::firestore::create_database))
        .route("/firestore/:project/databases/:database/documents/:collection/:document", 
               put(handlers::firestore::create_document).get(handlers::firestore::get_document))
               
        // Compute (Instances)
        .route("/compute/:project/zones/:zone/instances/:instance", 
               put(handlers::compute::create_instance).get(handlers::compute::get_instance))
               
        // Pub/Sub
        .route("/pubsub/:project/topics/:topic", 
               put(handlers::pubsub::create_topic).get(handlers::pubsub::get_topic))
        .route("/pubsub/:project/subscriptions/:subscription", 
               put(handlers::pubsub::create_subscription).get(handlers::pubsub::get_subscription))
        
        .with_state(storage)
}
