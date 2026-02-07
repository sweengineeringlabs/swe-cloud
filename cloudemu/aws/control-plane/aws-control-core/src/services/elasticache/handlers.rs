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
        "CreateCacheCluster" => {
            let id = body["CacheClusterId"].as_str().unwrap_or("");
            let node_type = body["CacheNodeType"].as_str().unwrap_or("cache.t3.micro");
            let engine = body["Engine"].as_str().unwrap_or("redis");
            let num_nodes = body["NumCacheNodes"].as_i64().unwrap_or(1) as i32;

            match emulator.storage.create_cache_cluster(id, node_type, engine, num_nodes) {
                Ok(cluster) => {
                    let resp = json!({
                        "CacheCluster": {
                            "CacheClusterId": cluster.cache_cluster_id,
                            "CacheNodeType": cluster.cache_node_type,
                            "Engine": cluster.engine,
                            "EngineVersion": cluster.engine_version,
                            "CacheClusterStatus": cluster.cache_cluster_status,
                            "NumCacheNodes": cluster.num_cache_nodes,
                            "CacheClusterCreateTime": cluster.created_at
                        }
                    });
                    Json(resp).into_response()
                },
                Err(e) => ApiError(e).into_response()
            }
        },
        "DescribeCacheClusters" => {
            match emulator.storage.list_cache_clusters() {
                Ok(clusters) => {
                    let clusters_json: Vec<Value> = clusters.into_iter().map(|c| {
                        json!({
                            "CacheClusterId": c.cache_cluster_id,
                            "CacheNodeType": c.cache_node_type,
                            "Engine": c.engine,
                            "EngineVersion": c.engine_version,
                            "CacheClusterStatus": c.cache_cluster_status,
                            "NumCacheNodes": c.num_cache_nodes,
                            "CacheClusterCreateTime": c.created_at
                        })
                    }).collect();

                     let resp = json!({
                         "CacheClusters": clusters_json
                    });
                    Json(resp).into_response()
                },
                Err(e) => ApiError(e).into_response()
            }
        },
        _ => (axum::http::StatusCode::NOT_IMPLEMENTED, "Not Implemented").into_response(),
    }
}
