use axum::{
    extract::{Path, State},
    Json,
};
use gcp_data_core::{StorageEngine, PubSubTopicMetadata, PubSubSubscriptionMetadata};
use serde::Deserialize;
use std::sync::Arc;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateSubscriptionRequest {
    pub topic: String,
    pub push_endpoint: Option<String>,
}

pub async fn create_topic(
    State(storage): State<Arc<StorageEngine>>,
    Path((project, topic)): Path<(String, String)>,
) -> Result<Json<PubSubTopicMetadata>, ApiError> {
    let metadata = storage.create_pubsub_topic(&topic, &project)?;
    Ok(Json(metadata))
}

pub async fn get_topic(
    State(storage): State<Arc<StorageEngine>>,
    Path((_project, topic)): Path<(String, String)>,
) -> Result<Json<PubSubTopicMetadata>, ApiError> {
    let metadata = storage.get_pubsub_topic(&topic)?;
    Ok(Json(metadata))
}

pub async fn create_subscription(
    State(storage): State<Arc<StorageEngine>>,
    Path((project, subscription)): Path<(String, String)>,
    Json(req): Json<CreateSubscriptionRequest>,
) -> Result<Json<PubSubSubscriptionMetadata>, ApiError> {
    let metadata = storage.create_pubsub_subscription(
        &subscription,
        &req.topic,
        &project,
        req.push_endpoint.as_deref()
    )?;
    Ok(Json(metadata))
}

pub async fn get_subscription(
    State(storage): State<Arc<StorageEngine>>,
    Path((_project, subscription)): Path<(String, String)>,
) -> Result<Json<PubSubSubscriptionMetadata>, ApiError> {
    let metadata = storage.get_pubsub_subscription(&subscription)?;
    Ok(Json(metadata))
}
