use axum::{
    extract::{Path, State},
    Json,
};
use gcp_data_core::{StorageEngine, GcpInstanceMetadata};
use serde::Deserialize;
use std::sync::Arc;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateInstanceRequest {
    pub machine_type: String,
    pub image: String,
    pub network: String,
}

pub async fn create_instance(
    State(storage): State<Arc<StorageEngine>>,
    Path((project, zone, instance)): Path<(String, String, String)>,
    Json(req): Json<CreateInstanceRequest>,
) -> Result<Json<GcpInstanceMetadata>, ApiError> {
    let metadata = storage.create_gcp_instance(
        &instance,
        &project,
        &zone,
        &req.machine_type,
        &req.image,
        &req.network
    )?;
    Ok(Json(metadata))
}

pub async fn get_instance(
    State(storage): State<Arc<StorageEngine>>,
    Path((project, zone, instance)): Path<(String, String, String)>,
) -> Result<Json<GcpInstanceMetadata>, ApiError> {
    let metadata = storage.get_gcp_instance(&instance, &project, &zone)?;
    Ok(Json(metadata))
}
