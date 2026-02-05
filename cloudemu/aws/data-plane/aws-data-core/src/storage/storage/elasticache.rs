use super::StorageEngine;
use crate::error::Result;
use serde::{Deserialize, Serialize};
use rusqlite::params;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheCluster {
    pub cache_cluster_id: String,
    pub cache_node_type: String,
    pub engine: String,
    pub engine_version: String,
    pub cache_cluster_status: String,
    pub num_cache_nodes: i32,
    pub created_at: String,
}

impl StorageEngine {
    pub fn init_elasticache_tables(&self) -> Result<()> {
        let conn = self.get_connection()?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS aws_cache_clusters (
                cache_cluster_id TEXT PRIMARY KEY,
                cache_node_type TEXT NOT NULL,
                engine TEXT NOT NULL,
                engine_version TEXT NOT NULL,
                cache_cluster_status TEXT NOT NULL,
                num_cache_nodes INTEGER NOT NULL,
                created_at TEXT NOT NULL
            )",
            [],
        )?;

        Ok(())
    }

    pub fn create_cache_cluster(&self, id: &str, node_type: &str, engine: &str, num_nodes: i32) -> Result<CacheCluster> {
        let conn = self.get_connection()?;
        let now = chrono::Utc::now().to_rfc3339();
        let status = "available";
        let version = "6.x"; // Mock version

        conn.execute(
            "INSERT INTO aws_cache_clusters (cache_cluster_id, cache_node_type, engine, engine_version, cache_cluster_status, num_cache_nodes, created_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            params![id, node_type, engine, version, status, num_nodes, now],
        )?;

        Ok(CacheCluster {
            cache_cluster_id: id.to_string(),
            cache_node_type: node_type.to_string(),
            engine: engine.to_string(),
            engine_version: version.to_string(),
            cache_cluster_status: status.to_string(),
            num_cache_nodes: num_nodes,
            created_at: now,
        })
    }
    
    pub fn list_cache_clusters(&self) -> Result<Vec<CacheCluster>> {
        let conn = self.get_connection()?;
        let mut stmt = conn.prepare("SELECT cache_cluster_id, cache_node_type, engine, engine_version, cache_cluster_status, num_cache_nodes, created_at FROM aws_cache_clusters")?;
        
        let rows = stmt.query_map([], |row| {
             Ok(CacheCluster {
                cache_cluster_id: row.get(0)?,
                cache_node_type: row.get(1)?,
                engine: row.get(2)?,
                engine_version: row.get(3)?,
                cache_cluster_status: row.get(4)?,
                num_cache_nodes: row.get(5)?,
                created_at: row.get(6)?,
            })
        })?;
        
        let mut clusters = Vec::new();
        for c in rows {
            clusters.push(c?);
        }
        
        Ok(clusters)
    }
}
