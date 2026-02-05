use oracle_data_core::StorageEngine;
use oracle_control_spi::{Request, Response, CloudResult};
use serde_json::{json, Value};
use std::sync::Arc;

pub struct ObjectStorageService {
    storage: Arc<StorageEngine>,
}

impl ObjectStorageService {
    pub fn new(storage: Arc<StorageEngine>) -> Self {
        Self { storage }
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Path: /n/{namespaceName}/b/{bucketName}/o...
        
        let path = req.path.clone();
        
        if path.contains("/b/") && req.method == "PUT" {
             // Check if it is bucket creation or object put
             // Bucket: /n/{namespace}/b/{bucketName} (no /o/)
             // Object: /n/{namespace}/b/{bucketName}/o/{objectName}
             
             if path.contains("/o/") {
                 return self.put_object(&req);
             } else {
                 return self.create_bucket(&req);
             }
        }
        
        if path.contains("/b/") && req.method == "GET" {
            // Get Bucket or Get Object
             if path.contains("/o/") {
                 // Get Object
                 return Ok(Response::not_found("Get Object Not Implemented"));
             } else {
                 // Get Bucket
                 return self.get_bucket(&req);
             }
        }

        Ok(Response::not_found("Not Found"))
    }

    fn create_bucket(&self, req: &Request) -> CloudResult<Response> {
        // Extract namespace and bucket name
        // /n/{namespace}/b/{bucketName}
        let parts: Vec<&str> = req.path.split('/').collect();
        let n_idx = parts.iter().position(|&x| x == "n").unwrap();
        let namespace = parts[n_idx + 1];
        let b_idx = parts.iter().position(|&x| x == "b").unwrap();
        let bucket_name = parts[b_idx + 1];

        let body: Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
        let compartment_id = body["compartmentId"].as_str().unwrap_or("ocid1.compartment.oc1..test");
        let created_by = "user-1";

        let bucket = self.storage.create_bucket(namespace, bucket_name, compartment_id, created_by).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "name": bucket.name,
            "namespace": bucket.namespace,
            "compartmentId": bucket.compartment_id,
            "createdBy": bucket.created_by,
            "timeCreated": bucket.time_created,
            "etag": bucket.etag
        })))
    }

    fn get_bucket(&self, req: &Request) -> CloudResult<Response> {
        let parts: Vec<&str> = req.path.split('/').collect();
        let n_idx = parts.iter().position(|&x| x == "n").unwrap();
        let namespace = parts[n_idx + 1];
        let b_idx = parts.iter().position(|&x| x == "b").unwrap();
        let bucket_name = parts[b_idx + 1];

        let bucket = self.storage.get_bucket(namespace, bucket_name).map_err(|e| oracle_control_spi::Error::NotFound(e.to_string()))?;

        Ok(Response::json(json!({
            "name": bucket.name,
            "namespace": bucket.namespace,
            "compartmentId": bucket.compartment_id,
            "createdBy": bucket.created_by,
            "timeCreated": bucket.time_created,
            "etag": bucket.etag
        })))
    }

    fn put_object(&self, req: &Request) -> CloudResult<Response> {
        // /n/{namespace}/b/{bucket}/o/{objectName}
        let parts: Vec<&str> = req.path.split('/').collect();
        let n_idx = parts.iter().position(|&x| x == "n").unwrap();
        let namespace = parts[n_idx + 1];
        let b_idx = parts.iter().position(|&x| x == "b").unwrap();
        let bucket_name = parts[b_idx + 1];
        let o_idx = parts.iter().position(|&x| x == "o").unwrap();
        
        // Join the rest as object name (might contain slashes)
        let object_name = parts[o_idx + 1..].join("/");

        let object = self.storage.put_object(namespace, bucket_name, &object_name, &req.body, None).map_err(|e| oracle_control_spi::Error::Internal(e.to_string()))?;

        Ok(Response::json(json!({
            "name": object.name,
            "bucketName": object.bucket_name,
            "namespace": object.namespace,
            "size": object.size,
            "md5": object.md5,
            "timeCreated": object.time_created,
            "etag": object.etag
        })))
    }
}
