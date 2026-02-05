use axum::{
    extract::{Path, State},
    Json,
};
use azure_data_core::{StorageEngine, EventGridTopicMetadata};
use serde::Deserialize;
use std::sync::Arc;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateTopicRequest {
    pub location: String,
}

pub async fn create_topic(
    State(storage): State<Arc<StorageEngine>>,
    Path((resource_group, topic_name)): Path<(String, String)>,
    Json(req): Json<CreateTopicRequest>,
) -> Result<Json<EventGridTopicMetadata>, ApiError> {
    let metadata = storage.create_eventgrid_topic(
        &topic_name, 
        &req.location, 
        &resource_group
    )?;
    Ok(Json(metadata))
}

pub async fn get_topic(
    State(storage): State<Arc<StorageEngine>>,
    Path((_resource_group, topic_name)): Path<(String, String)>,
) -> Result<Json<EventGridTopicMetadata>, ApiError> {
    let metadata = storage.get_eventgrid_topic(&topic_name)?;
    Ok(Json(metadata))
}
