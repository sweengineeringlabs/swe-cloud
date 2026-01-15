use axum::{
    extract::{Path, State},
    response::IntoResponse,
    Json,
    http::{StatusCode, HeaderMap, header},
};
use gcp_data_core::{StorageEngine, GcsBucketMetadata, GcsObjectMetadata};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateBucketRequest {
    pub location: String,
}

#[derive(Deserialize)]
pub struct PutObjectRequest {
    pub content: String, // Base64 or raw string?
    pub content_type: Option<String>,
}

#[derive(Serialize)]
pub struct ListObjectsResponse {
    pub objects: Vec<GcsObjectMetadata>,
}

pub async fn create_bucket(
    State(storage): State<Arc<StorageEngine>>,
    Path(bucket): Path<String>,
    Json(req): Json<CreateBucketRequest>,
) -> Result<Json<GcsBucketMetadata>, ApiError> {
    // create_gcs_bucket takes location but also project_id. Project ID should likely come from header or route.
    // For now assuming "default" project or fixing signature.
    // Core: create_gcs_bucket(name, project_id, location)
    // We'll use "default-project" for now or extract if we change route.
    let project_id = "default-project";
    storage.create_gcs_bucket(&bucket, project_id, &req.location)?;
    
    let metadata = storage.get_gcs_bucket(&bucket)?;
    Ok(Json(metadata))
}

pub async fn put_object(
    State(storage): State<Arc<StorageEngine>>,
    Path((bucket, object)): Path<(String, String)>,
    Json(payload): Json<PutObjectRequest>,
) -> Result<Json<GcsObjectMetadata>, ApiError> {
    let data = payload.content.into_bytes();
    let metadata = storage.insert_gcs_object(
        &bucket, 
        &object, 
        &data, 
        payload.content_type.as_deref()
    )?;
    Ok(Json(metadata))
}

pub async fn get_object(
    State(storage): State<Arc<StorageEngine>>,
    Path((bucket, object)): Path<(String, String)>,
) -> Result<impl IntoResponse, ApiError> {
    // get_gcs_object takes optional generation
    let (metadata, data) = storage.get_gcs_object(&bucket, &object, None)?;
    
    let mut headers = HeaderMap::new();
    if let Some(ct) = metadata.content_type {
        headers.insert(header::CONTENT_TYPE, ct.parse().unwrap_or("application/octet-stream".parse().unwrap()));
    }
    headers.insert(header::ETAG, metadata.etag.parse().unwrap_or("".parse().unwrap()));

    Ok((headers, data))
}

pub async fn list_objects(
    State(storage): State<Arc<StorageEngine>>,
    Path(bucket): Path<String>,
) -> Result<Json<ListObjectsResponse>, ApiError> {
    // list_gcs_objects takes bucket and optional prefix
    let objects = storage.list_gcs_objects(&bucket, None)?;
    Ok(Json(ListObjectsResponse { objects }))
}
