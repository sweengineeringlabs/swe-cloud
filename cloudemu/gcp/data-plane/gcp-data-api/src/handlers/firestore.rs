use axum::{
    extract::{Path, State},
    Json,
};
use gcp_data_core::{StorageEngine, FirestoreDatabaseMetadata, FirestoreDocumentMetadata};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateDatabaseRequest {
    pub location_id: String,
    pub database_type: String,
}

#[derive(Deserialize)]
pub struct CreateDocumentRequest {
    pub fields: serde_json::Value,
}

pub async fn create_database(
    State(storage): State<Arc<StorageEngine>>,
    Path((project, database_id)): Path<(String, String)>,
    Json(req): Json<CreateDatabaseRequest>,
) -> Result<Json<FirestoreDatabaseMetadata>, ApiError> {
    storage.create_firestore_database(&database_id, &project, &req.location_id)?;
    let metadata = storage.get_firestore_database(&database_id)?;
    Ok(Json(metadata))
}

pub async fn create_document(
    State(storage): State<Arc<StorageEngine>>,
    Path((_project, database, collection, document_id)): Path<(String, String, String, String)>,
    Json(req): Json<CreateDocumentRequest>,
) -> Result<Json<FirestoreDocumentMetadata>, ApiError> {
    let metadata = storage.create_document(&database, &collection, &document_id, &req.fields)?;
    Ok(Json(metadata))
}

pub async fn get_document(
    State(storage): State<Arc<StorageEngine>>,
    Path((_project, database, collection, document_id)): Path<(String, String, String, String)>,
) -> Result<Json<FirestoreDocumentMetadata>, ApiError> {
    let metadata = storage.get_document(&database, &collection, &document_id)?;
    Ok(Json(metadata))
}
