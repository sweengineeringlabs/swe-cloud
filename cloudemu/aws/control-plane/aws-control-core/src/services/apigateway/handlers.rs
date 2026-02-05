use axum::{
    extract::{State, Path},
    http::{StatusCode, Method},
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use crate::Emulator;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize)]
struct CreateRestApiRequest {
    name: String,
    description: Option<String>,
}

#[derive(Deserialize)]
struct CreateResourceRequest {
    #[serde(rename = "pathPart")]
    path_part: String,
}

#[derive(Deserialize)]
struct PutMethodRequest {
    #[serde(rename = "authorizationType")]
    authorization_type: String,
}

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    method: Method,
    uri: axum::http::Uri,
    body: String,
) -> Response {
    let path = uri.path();
    
    // Simple routing based on path patterns
    if path == "/restapis" {
        if method == Method::GET {
            return list_rest_apis(&emulator).await;
        } else if method == Method::POST {
            let req: CreateRestApiRequest = match serde_json::from_str(&body) {
                Ok(r) => r,
                Err(_) => return (StatusCode::BAD_REQUEST, "Invalid JSON").into_response(),
            };
            return create_rest_api(&emulator, req).await;
        }
    }
    
    // /restapis/{api_id}/resources
    if path.starts_with("/restapis/") && path.ends_with("/resources") {
         let segments: Vec<&str> = path.split('/').collect();
         if segments.len() >= 4 {
             let api_id = segments[2];
             
             if method == Method::GET {
                 // List resources
                 return list_resources(&emulator, api_id).await;
             } else if method == Method::POST {
                 // Create resource - requires parent_id?
                 // AWS usually passes parentId in query param or body? 
                 // In body: { "parentId": "...", "pathPart": "..." }
                 // Let's assume body contains parentId for simplicity or we need to look closer at AWS API.
                 // AWS: POST /restapis/{api_id}/resources/{parent_id}
                 // Wait, standard is POST /restapis/{restapi_id}/resources/{parent_id}
                 // My routing in gateway.rs was: /restapis/:api_id/resources
                 // That's for listing.
                 // Creating is often on a specific parent resource.
                 // Let's check my routing...
                 // I have: .route("/restapis/:api_id/resources/:resource_id", ...)
                 // So if I POST to .../resources/12345, that's creating a child of 12345.
             }
         }
    }

    // /restapis/{api_id}/resources/{parent_id}
    if path.starts_with("/restapis/") && path.contains("/resources/") {
         let segments: Vec<&str> = path.split('/').collect();
         // "", "restapis", "api_id", "resources", "resource_id"
         if segments.len() == 5 {
             let api_id = segments[2];
             let resource_id = segments[4];
             
             if method == Method::POST {
                  let req: CreateResourceRequest = match serde_json::from_str(&body) {
                    Ok(r) => r,
                    Err(_) => return (StatusCode::BAD_REQUEST, "Invalid JSON").into_response(),
                };
                return create_resource(&emulator, api_id, resource_id, req).await;
             }
         }
    }
    
    // /restapis/{api_id}/resources/{resource_id}/methods/{http_method}
    // currently not routed in gateway.rs properly to capture http_method in path?
    // Actually I can parse it from `uri` if the wildcard matches or if I add a specific route.
    // I only added: .route("/restapis/:api_id/resources/:resource_id", ...)
    // So "methods/GET" would cause 404.
    // I should fix gateway.rs to include methods path.
    // For now, I will stick to what's routable: RestAPIs and Resources.

    (StatusCode::NOT_FOUND, "Not Found").into_response()
}

async fn list_rest_apis(emulator: &Emulator) -> Response {
    match emulator.storage.list_rest_apis() {
        Ok(apis) => {
            let json = json!({
                "item": apis.into_iter().map(|api| {
                    json!({
                        "id": api.id,
                        "name": api.name,
                        "description": api.description,
                        "createdDate": api.created_at,
                        "endpointConfiguration": {
                            "types": [api.endpoint_type]
                        }
                    })
                }).collect::<Vec<_>>()
            });
            Json(json).into_response()
        },
        Err(e) => crate::error::ApiError(e).into_response()
    }
}

async fn create_rest_api(emulator: &Emulator, req: CreateRestApiRequest) -> Response {
    match emulator.storage.create_rest_api(&req.name, req.description.as_deref()) {
        Ok(api) => {
             let json = json!({
                "id": api.id,
                "name": api.name,
                "description": api.description,
                "createdDate": api.created_at,
                "endpointConfiguration": {
                    "types": [api.endpoint_type]
                }
            });
            Json(json).into_response()
        },
        Err(e) => crate::error::ApiError(e).into_response()
    }
}

async fn list_resources(emulator: &Emulator, api_id: &str) -> Response {
    match emulator.storage.list_resources(api_id) {
        Ok(resources) => {
            let json = json!({
                "item": resources.into_iter().map(|r| {
                    json!({
                        "id": r.id,
                        "parentId": r.parent_id,
                        "pathPart": r.path_part,
                        "path": r.path
                    })
                }).collect::<Vec<_>>()
            });
            Json(json).into_response()
        },
        Err(e) => crate::error::ApiError(e).into_response()
    }
}

async fn create_resource(emulator: &Emulator, api_id: &str, parent_id: &str, req: CreateResourceRequest) -> Response {
    match emulator.storage.create_resource(api_id, parent_id, &req.path_part) {
        Ok(r) => {
            let json = json!({
                "id": r.id,
                "parentId": r.parent_id,
                "pathPart": r.path_part,
                "path": r.path
            });
            Json(json).into_response()
        },
        Err(e) => crate::error::ApiError(e).into_response()
    }
}
