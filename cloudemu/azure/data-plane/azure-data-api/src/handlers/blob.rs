use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    http::{StatusCode, HeaderMap, header},
};
use azure_data_core::{StorageEngine, BlobContainerMetadata, BlobMetadata};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error::ApiError;

// Types for requests
#[derive(Deserialize)]
pub struct CreateContainerRequest {
    pub public_access: Option<String>,
}

#[derive(Deserialize)]
pub struct PutBlobRequest {
    pub content: String, // Base64 or raw string? For simplicity string
    pub content_type: Option<String>,
}

#[derive(Serialize)]
pub struct ListBlobsResponse {
    pub blobs: Vec<BlobMetadata>,
}

pub async fn create_container(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, container)): Path<(String, String)>,
) -> Result<Json<BlobContainerMetadata>, ApiError> {
    let metadata = storage.create_container(&account, &container)?;
    // Core returns (), but we want metadata. Wait, core create_container returns ().
    // We should fetch it after create, or just return empty for now if core doesn't return it.
    // Core's create_container returns Result<()>.
    // So we need to call get_container.
    let metadata = storage.get_container(&account, &container)?;
    Ok(Json(metadata))
}

pub async fn put_blob(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, container, blob)): Path<(String, String, String)>,
    Json(payload): Json<PutBlobRequest>,
) -> Result<Json<BlobMetadata>, ApiError> {
    let data = payload.content.into_bytes();
    let metadata = storage.put_blob(
        &account, 
        &container, 
        &blob, 
        &data, 
        payload.content_type.as_deref()
    )?;
    Ok(Json(metadata))
}

pub async fn get_blob(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, container, blob)): Path<(String, String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    let (metadata, data) = storage.get_blob(&account, &container, &blob)?;
    
    let mut headers = HeaderMap::new();
    if let Some(ct) = metadata.content_type {
        headers.insert(header::CONTENT_TYPE, ct.parse().unwrap_or("application/octet-stream".parse().unwrap()));
    }
    headers.insert(header::ETAG, metadata.etag.parse().unwrap_or("".parse().unwrap()));

    Ok((headers, data))
}

pub async fn list_blobs(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, container)): Path<(String, String)>,
) -> Result<Json<ListBlobsResponse>, ApiError> {
    let blobs = storage.list_blobs(&account, &container)?;
    Ok(Json(ListBlobsResponse { blobs }))
}
