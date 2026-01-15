use axum::{
    extract::{Path, State},
    Json,
};
use azure_data_core::{StorageEngine, CosmosDatabaseMetadata, CosmosContainerMetadata, CosmosItemMetadata};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateDatabaseRequest {
    pub throughput: Option<i32>,
}

#[derive(Deserialize)]
pub struct CreateContainerRequest {
    pub partition_key_path: String,
}

#[derive(Deserialize)]
pub struct CreateItemRequest {
    pub item: serde_json::Value,
    pub partition_key: String,
}

pub async fn create_database(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, database)): Path<(String, String)>,
) -> Result<Json<CosmosDatabaseMetadata>, ApiError> {
    storage.create_cosmos_database(&account, &database)?;
    let metadata = storage.get_cosmos_database(&account, &database)?;
    Ok(Json(metadata))
}

pub async fn create_container(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, database, container)): Path<(String, String, String)>,
    Json(req): Json<CreateContainerRequest>,
) -> Result<Json<CosmosContainerMetadata>, ApiError> {
    storage.create_cosmos_container(&account, &database, &container, &req.partition_key_path)?;
    let metadata = storage.get_cosmos_container(&account, &database, &container)?;
    Ok(Json(metadata))
}

pub async fn create_item(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, database, container)): Path<(String, String, String)>,
    Json(req): Json<CreateItemRequest>,
) -> Result<Json<CosmosItemMetadata>, ApiError> {
    // core expects just the JSON value which should contain the PK
    let metadata = storage.create_cosmos_item(&account, &database, &container, &req.item)?;
    Ok(Json(metadata))
}

pub async fn get_item(
    State(storage): State<Arc<StorageEngine>>,
    Path((account, database, container, item_id, partition_key)): Path<(String, String, String, String, String)>,
) -> Result<Json<CosmosItemMetadata>, ApiError> {
    let metadata = storage.get_cosmos_item(&account, &database, &container, &item_id, &partition_key)?;
    Ok(Json(metadata))
}
