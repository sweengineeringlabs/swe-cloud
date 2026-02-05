use axum::{
    routing::{get, post, put},
    Router,
};
use azure_data_core::StorageEngine;
use std::sync::Arc;
use crate::handlers;

pub fn create_router(storage: Arc<StorageEngine>) -> Router {
    Router::new()
        // Health
        .route("/health", get(|| async { "OK" }))

        // Blob Storage
        .route("/blob/:account/:container", put(handlers::blob::create_container))
        .route("/blob/:account/:container/blobs", get(handlers::blob::list_blobs))
        .route("/blob/:account/:container/:blob", put(handlers::blob::put_blob).get(handlers::blob::get_blob))
        
        // Cosmos DB
        .route("/cosmos/:account/:database", put(handlers::cosmos::create_database))
        .route("/cosmos/:account/:database/:container", put(handlers::cosmos::create_container))
        .route("/cosmos/:account/:database/:container/items", post(handlers::cosmos::create_item))
        .route("/cosmos/:account/:database/:container/items/:item_id/:partition_key", get(handlers::cosmos::get_item))

        // Compute (VMs)
        .route("/compute/:resource_group/vms/:vm_name", 
               put(handlers::compute::create_vm).get(handlers::compute::get_vm))
               
        // Event Grid
        .route("/eventgrid/:resource_group/topics/:topic_name", 
               put(handlers::eventgrid::create_topic).get(handlers::eventgrid::get_topic))
        
        .with_state(storage)
}
