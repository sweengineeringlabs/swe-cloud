use cloudemu_core::{Request, Response, CloudResult, CloudError};

/// Azure Blob Storage Service Handler
pub struct BlobService;

impl BlobService {
    pub fn new() -> Self {
        Self
    }

    pub async fn handle_request(&self, req: Request) -> CloudResult<Response> {
        // Parse path and query
        // Expected format: /<container>/<blob>
        // Query param `restype=container` indicates container operation
        
        let (path_only, query_str) = match req.path.split_once('?') {
            Some((p, q)) => (p, Some(q)),
            None => (req.path.as_str(), None),
        };

        // Remove leading slash
        let clean_path = path_only.trim_start_matches('/');
        let parts: Vec<&str> = clean_path.split('/').collect();

        // Root path: List Containers
        if clean_path.is_empty() {
            return self.list_containers().await;
        }

        let container_name = parts[0];
        
        // Check for container operation
        let is_container_op = query_str.map(|q| q.contains("restype=container")).unwrap_or(false);
        let is_comp_list = query_str.map(|q| q.contains("comp=list")).unwrap_or(false);

        if is_container_op {
            match req.method.as_str() {
                "PUT" => self.create_container(container_name).await,
                "DELETE" => self.delete_container(container_name).await,
                "GET" if is_comp_list => self.list_blobs(container_name).await,
                "GET" => self.get_container_properties(container_name).await,
                _ => Err(CloudError::Validation(format!("Unsupported container method: {}", req.method))),
            }
        } else {
            // Blob operations (no restype=container)
            // But verify we have a blob name
            if parts.len() < 2 {
                 // If path is just /container and no query, it might be GetContainerAttributes? 
                 // Actually Azure uses restype=container for that.
                 // If no restype, and method is GET /container, it's invalid URL usually?
                 // Or maybe ListBlobs requires comp=list? Yes.
                 return Err(CloudError::Validation("Missing blob name".into()));
            }
            
            let blob_name = parts[1..].join("/");
            
            match req.method.as_str() {
                "PUT" => self.put_blob(container_name, &blob_name, req.body).await,
                "GET" => self.get_blob(container_name, &blob_name).await,
                "DELETE" => self.delete_blob(container_name, &blob_name).await,
                _ => Err(CloudError::Validation(format!("Unsupported blob method: {}", req.method))),
            }
        }
    }

    // --- Container Operations ---

    async fn list_containers(&self) -> CloudResult<Response> {
        // Return empty enumeration for now
        let xml_body = r#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ServiceEndpoint="http://localhost:4567/">
  <Containers />
  <NextMarker />
</EnumerationResults>"#;
        
        Ok(Response::ok(xml_body).with_header("Content-Type", "application/xml"))
    }

    async fn create_container(&self, _name: &str) -> CloudResult<Response> {
        // TODO: Persist container
        Ok(Response::created("").with_header("x-ms-request-id", "uuid"))
    }

    async fn delete_container(&self, _name: &str) -> CloudResult<Response> {
         Ok(Response {
             status: 202,
             headers: std::collections::HashMap::new(),
             body: vec![],
         }.with_header("x-ms-request-id", "uuid"))
    }

    async fn get_container_properties(&self, _name: &str) -> CloudResult<Response> {
        Ok(Response::ok("").with_header("x-ms-meta-type", "container"))
    }
    
    // --- Blob Operations ---

    async fn list_blobs(&self, container: &str) -> CloudResult<Response> {
        let xml_body = format!(r#"<?xml version="1.0" encoding="utf-8"?>
<EnumerationResults ContainerName="{}">
  <Blobs />
  <NextMarker />
</EnumerationResults>"#, container);
        Ok(Response::ok(xml_body).with_header("Content-Type", "application/xml"))
    }

    async fn put_blob(&self, _container: &str, _blob: &str, _body: Vec<u8>) -> CloudResult<Response> {
        // TODO: Persist blob
        Ok(Response::created("").with_header("Content-MD5", "hash"))
    }

    async fn get_blob(&self, _container: &str, _blob: &str) -> CloudResult<Response> {
        // TODO: Retrieve blob
        Ok(Response::ok("blob data").with_header("Content-Type", "application/octet-stream"))
    }

    async fn delete_blob(&self, _container: &str, _blob: &str) -> CloudResult<Response> {
        Ok(Response {
             status: 202,
             headers: std::collections::HashMap::new(),
             body: vec![],
        })
    }

}
