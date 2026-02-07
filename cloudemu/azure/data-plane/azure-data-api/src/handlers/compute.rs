use axum::{
    extract::{Path, State},
    Json,
};
use azure_data_core::{StorageEngine, VirtualMachineMetadata};
use serde::Deserialize;
use std::sync::Arc;
use crate::error::ApiError;

#[derive(Deserialize)]
pub struct CreateVmRequest {
    pub location: String,
    pub vm_size: String,
    pub os_type: String,
    pub admin_username: String,
}

pub async fn create_vm(
    State(storage): State<Arc<StorageEngine>>,
    Path((resource_group, vm_name)): Path<(String, String)>,
    Json(req): Json<CreateVmRequest>,
) -> Result<Json<VirtualMachineMetadata>, ApiError> {
    let metadata = storage.create_virtual_machine(
        &vm_name, 
        &req.location, 
        &resource_group,
        &req.vm_size,
        &req.os_type,
        &req.admin_username
    )?;
    Ok(Json(metadata))
}

pub async fn get_vm(
    State(storage): State<Arc<StorageEngine>>,
    Path((_resource_group, vm_name)): Path<(String, String)>,
) -> Result<Json<VirtualMachineMetadata>, ApiError> {
    let metadata = storage.get_virtual_machine(&vm_name)?;
    Ok(Json(metadata))
}
