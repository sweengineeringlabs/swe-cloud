use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use std::sync::Arc;
use crate::Emulator;
use serde_json::{json, Value};
use crate::error::ApiError;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers.get("x-amz-target").and_then(|h| h.to_str().ok()).unwrap_or("");
    let action = target.split('.').last().unwrap_or("");

    match action {
        "CreateRepository" => {
            let name = body["repositoryName"].as_str().unwrap_or("");
            
            match emulator.storage.create_repository(name) {
                Ok(repo) => {
                    let resp = json!({
                        "repository": {
                            "repositoryArn": repo.repository_arn,
                            "registryId": repo.registry_id,
                            "repositoryName": repo.repository_name,
                            "repositoryUri": repo.repository_uri,
                            "createdAt": repo.created_at
                        }
                    });
                    Json(resp).into_response()
                },
                Err(e) => ApiError(e).into_response()
            }
        },
        "DescribeRepositories" => {
             match emulator.storage.list_repositories() {
                Ok(repos) => {
                    let repos_json: Vec<Value> = repos.into_iter().map(|r| {
                         json!({
                            "repositoryArn": r.repository_arn,
                            "registryId": r.registry_id,
                            "repositoryName": r.repository_name,
                            "repositoryUri": r.repository_uri,
                            "createdAt": r.created_at
                        })
                    }).collect();

                    let resp = json!({
                        "repositories": repos_json
                    });
                    Json(resp).into_response()
                },
                 Err(e) => ApiError(e).into_response()
             }
        },
        _ => (axum::http::StatusCode::NOT_IMPLEMENTED, "Not Implemented").into_response(),
    }
}
