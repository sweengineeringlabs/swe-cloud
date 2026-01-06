//! S3 HTTP Handlers

use super::xml;
use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    body::Body,
    extract::{Path, Query, State},
    http::{HeaderMap, Method, StatusCode, header},
    response::Response,
};
use std::{collections::HashMap, sync::Arc};
use tracing::{info, debug};

/// List all buckets (GET /)
pub async fn list_buckets(
    State(emulator): State<Arc<Emulator>>,
) -> Result<Response<Body>, EmulatorError> {
    info!("S3: ListBuckets");
    
    let buckets = emulator.storage.list_buckets()?;
    let xml_body = xml::list_buckets_xml(&buckets, &emulator.config.account_id);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml")
        .header("x-amz-request-id", uuid::Uuid::new_v4().to_string())
        .body(Body::from(xml_body))
        .unwrap())
}

/// Handle bucket-level operations (GET/PUT/DELETE/HEAD /:bucket)
pub async fn bucket_handler(
    State(emulator): State<Arc<Emulator>>,
    method: Method,
    Path(bucket): Path<String>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<Response<Body>, EmulatorError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    
    // Check for sub-resource operations
    if params.contains_key("versioning") {
        return handle_bucket_versioning(&emulator, &method, &bucket, &body, &request_id).await;
    }
    if params.contains_key("policy") {
        return handle_bucket_policy(&emulator, &method, &bucket, &body, &request_id).await;
    }
    if params.contains_key("location") {
        return handle_bucket_location(&emulator, &bucket, &request_id).await;
    }
    if params.contains_key("list-type") {
        // ListObjectsV2
        return handle_list_objects_v2(&emulator, &bucket, &params, &request_id).await;
    }
    
    info!("S3: {} /{}", method, bucket);
    debug!("Params: {:?}", params);
    
    match method {
        Method::PUT => {
            // CreateBucket
            emulator.storage.create_bucket(&bucket, &emulator.config.region)?;
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/xml")
                .header("Location", format!("/{}", bucket))
                .header("x-amz-request-id", &request_id)
                .body(Body::empty())
                .unwrap())
        }
        Method::DELETE => {
            // DeleteBucket
            emulator.storage.delete_bucket(&bucket)?;
            
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header("x-amz-request-id", &request_id)
                .body(Body::empty())
                .unwrap())
        }
        Method::GET => {
            // ListObjects (legacy) or ListObjectsV2
            let prefix = params.get("prefix").map(|s| s.as_str());
            let delimiter = params.get("delimiter").map(|s| s.as_str());
            let max_keys: u32 = params.get("max-keys")
                .and_then(|s| s.parse().ok())
                .unwrap_or(1000);
            let continuation_token = params.get("continuation-token").map(|s| s.as_str());
            
            let result = emulator.storage.list_objects(
                &bucket,
                prefix,
                delimiter,
                max_keys,
                continuation_token,
            )?;
            
            let xml_body = xml::list_objects_v2_xml(&result);
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/xml")
                .header("x-amz-request-id", &request_id)
                .body(Body::from(xml_body))
                .unwrap())
        }
        Method::HEAD => {
            // HeadBucket
            if emulator.storage.bucket_exists(&bucket)? {
                Ok(Response::builder()
                    .status(StatusCode::OK)
                    .header("x-amz-request-id", &request_id)
                    .header("x-amz-bucket-region", &emulator.config.region)
                    .body(Body::empty())
                    .unwrap())
            } else {
                Err(EmulatorError::NoSuchBucket(bucket))
            }
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::empty())
                .unwrap())
        }
    }
}

/// Handle bucket versioning operations
async fn handle_bucket_versioning(
    emulator: &Emulator,
    method: &Method,
    bucket: &str,
    body: &[u8],
    request_id: &str,
) -> Result<Response<Body>, EmulatorError> {
    info!("S3: {} /{}?versioning", method, bucket);
    
    match *method {
        Method::GET => {
            let status = emulator.storage.get_bucket_versioning(bucket)?;
            let xml_body = xml::get_bucket_versioning_xml(&status);
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/xml")
                .header("x-amz-request-id", request_id)
                .body(Body::from(xml_body))
                .unwrap())
        }
        Method::PUT => {
            // Parse versioning configuration from body
            let body_str = String::from_utf8_lossy(body);
            let status = if body_str.contains("<Status>Enabled</Status>") {
                "Enabled"
            } else if body_str.contains("<Status>Suspended</Status>") {
                "Suspended"
            } else {
                return Err(EmulatorError::MalformedXml("Invalid versioning configuration".to_string()));
            };
            
            emulator.storage.set_bucket_versioning(bucket, status)?;
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header("x-amz-request-id", request_id)
                .body(Body::empty())
                .unwrap())
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap())
    }
}

/// Handle bucket policy operations
async fn handle_bucket_policy(
    emulator: &Emulator,
    method: &Method,
    bucket: &str,
    body: &[u8],
    request_id: &str,
) -> Result<Response<Body>, EmulatorError> {
    info!("S3: {} /{}?policy", method, bucket);
    
    match *method {
        Method::GET => {
            let policy = emulator.storage.get_bucket_policy(bucket)?
                .ok_or_else(|| EmulatorError::NoSuchBucketPolicy(bucket.to_string()))?;
            
            Ok(Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, "application/json")
                .header("x-amz-request-id", request_id)
                .body(Body::from(policy))
                .unwrap())
        }
        Method::PUT => {
            let policy = String::from_utf8_lossy(body);
            
            // Validate it's valid JSON
            serde_json::from_str::<serde_json::Value>(&policy)
                .map_err(|e| EmulatorError::MalformedPolicy(e.to_string()))?;
            
            emulator.storage.set_bucket_policy(bucket, &policy)?;
            
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header("x-amz-request-id", request_id)
                .body(Body::empty())
                .unwrap())
        }
        Method::DELETE => {
            emulator.storage.delete_bucket_policy(bucket)?;
            
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header("x-amz-request-id", request_id)
                .body(Body::empty())
                .unwrap())
        }
        _ => Ok(Response::builder()
            .status(StatusCode::METHOD_NOT_ALLOWED)
            .body(Body::empty())
            .unwrap())
    }
}

/// Handle GetBucketLocation
async fn handle_bucket_location(
    emulator: &Emulator,
    bucket: &str,
    request_id: &str,
) -> Result<Response<Body>, EmulatorError> {
    info!("S3: GET /{}?location", bucket);
    
    let bucket_meta = emulator.storage.get_bucket(bucket)?;
    let xml_body = xml::get_bucket_location_xml(&bucket_meta.region);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml")
        .header("x-amz-request-id", request_id)
        .body(Body::from(xml_body))
        .unwrap())
}

/// Handle ListObjectsV2
async fn handle_list_objects_v2(
    emulator: &Emulator,
    bucket: &str,
    params: &HashMap<String, String>,
    request_id: &str,
) -> Result<Response<Body>, EmulatorError> {
    info!("S3: GET /{}?list-type=2", bucket);
    
    let prefix = params.get("prefix").map(|s| s.as_str());
    let delimiter = params.get("delimiter").map(|s| s.as_str());
    let max_keys: u32 = params.get("max-keys")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1000);
    let continuation_token = params.get("continuation-token").map(|s| s.as_str());
    
    let result = emulator.storage.list_objects(
        bucket,
        prefix,
        delimiter,
        max_keys,
        continuation_token,
    )?;
    
    let xml_body = xml::list_objects_v2_xml(&result);
    
    Ok(Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml")
        .header("x-amz-request-id", request_id)
        .body(Body::from(xml_body))
        .unwrap())
}

/// Handle object-level operations (GET/PUT/DELETE/HEAD /:bucket/*key)
pub async fn object_handler(
    State(emulator): State<Arc<Emulator>>,
    method: Method,
    Path((bucket, key)): Path<(String, String)>,
    Query(params): Query<HashMap<String, String>>,
    headers: HeaderMap,
    body: axum::body::Bytes,
) -> Result<Response<Body>, EmulatorError> {
    let request_id = uuid::Uuid::new_v4().to_string();
    info!("S3: {} /{}/{}", method, bucket, key);
    
    let version_id = params.get("versionId").map(|s| s.as_str());
    
    match method {
        Method::PUT => {
            // Check for copy operation
            if let Some(copy_source) = headers.get("x-amz-copy-source") {
                return handle_copy_object(&emulator, &bucket, &key, copy_source, &request_id).await;
            }
            
            // Regular PUT
            let content_type = headers.get(header::CONTENT_TYPE)
                .and_then(|h| h.to_str().ok());
            
            // Extract user metadata
            let mut metadata = HashMap::new();
            for (name, value) in headers.iter() {
                if let Some(meta_key) = name.as_str().strip_prefix("x-amz-meta-") {
                    if let Ok(v) = value.to_str() {
                        metadata.insert(meta_key.to_string(), v.to_string());
                    }
                }
            }
            let metadata_json = if metadata.is_empty() { 
                None 
            } else { 
                Some(serde_json::to_string(&metadata)?)
            };
            
            let obj_meta = emulator.storage.put_object(
                &bucket, 
                &key, 
                &body, 
                content_type,
                metadata_json.as_deref(),
            )?;
            
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header("ETag", &obj_meta.etag)
                .header("x-amz-request-id", &request_id);
            
            if let Some(vid) = obj_meta.version_id {
                response = response.header("x-amz-version-id", vid);
            }
            
            Ok(response.body(Body::empty()).unwrap())
        }
        Method::GET => {
            let (obj_meta, data) = emulator.storage.get_object(&bucket, &key, version_id)?;
            
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, &obj_meta.content_type)
                .header(header::CONTENT_LENGTH, obj_meta.size)
                .header("ETag", &obj_meta.etag)
                .header("Last-Modified", &obj_meta.last_modified)
                .header("x-amz-request-id", &request_id);
            
            if let Some(vid) = obj_meta.version_id {
                response = response.header("x-amz-version-id", vid);
            }
            
            Ok(response.body(Body::from(data)).unwrap())
        }
        Method::HEAD => {
            let (obj_meta, _) = emulator.storage.get_object(&bucket, &key, version_id)?;
            
            let mut response = Response::builder()
                .status(StatusCode::OK)
                .header(header::CONTENT_TYPE, &obj_meta.content_type)
                .header(header::CONTENT_LENGTH, obj_meta.size)
                .header("ETag", &obj_meta.etag)
                .header("Last-Modified", &obj_meta.last_modified)
                .header("x-amz-request-id", &request_id);
            
            if let Some(vid) = obj_meta.version_id {
                response = response.header("x-amz-version-id", vid);
            }
            
            Ok(response.body(Body::empty()).unwrap())
        }
        Method::DELETE => {
            let delete_marker_version = emulator.storage.delete_object(&bucket, &key, version_id)?;
            
            let mut response = Response::builder()
                .status(StatusCode::NO_CONTENT)
                .header("x-amz-request-id", &request_id);
            
            if let Some(vid) = delete_marker_version {
                response = response
                    .header("x-amz-version-id", vid)
                    .header("x-amz-delete-marker", "true");
            }
            
            Ok(response.body(Body::empty()).unwrap())
        }
        _ => {
            Ok(Response::builder()
                .status(StatusCode::METHOD_NOT_ALLOWED)
                .body(Body::empty())
                .unwrap())
        }
    }
}

/// Handle CopyObject
async fn handle_copy_object(
    emulator: &Emulator,
    dest_bucket: &str,
    dest_key: &str,
    copy_source: &axum::http::HeaderValue,
    request_id: &str,
) -> Result<Response<Body>, EmulatorError> {
    let source = copy_source.to_str().unwrap_or("");
    let source = percent_encoding::percent_decode_str(source)
        .decode_utf8_lossy();
    let source = source.trim_start_matches('/');
    
    let (src_bucket, src_key) = source.split_once('/')
        .ok_or_else(|| EmulatorError::InvalidArgument("Invalid x-amz-copy-source".to_string()))?;
    
    info!("S3: CopyObject {}/{} -> {}/{}", src_bucket, src_key, dest_bucket, dest_key);
    
    // Get source object
    let (_, data) = emulator.storage.get_object(src_bucket, src_key, None)?;
    
    // Put to destination
    let obj_meta = emulator.storage.put_object(dest_bucket, dest_key, &data, None, None)?;
    
    let xml_body = xml::copy_object_xml(&obj_meta.etag, &obj_meta.last_modified);
    
    let mut response = Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/xml")
        .header("x-amz-request-id", request_id);
    
    if let Some(vid) = obj_meta.version_id {
        response = response.header("x-amz-version-id", vid);
    }
    
    Ok(response.body(Body::from(xml_body)).unwrap())
}
