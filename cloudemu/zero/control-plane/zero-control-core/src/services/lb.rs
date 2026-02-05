use zero_control_spi::{ZeroResult, ZeroError};
use zero_data_core::ZeroEngine;
use std::sync::Arc;
use serde_json::json;
use std::collections::HashMap;
use tokio::sync::Mutex;
use axum::{
    extract::Request,
    response::IntoResponse,
    routing::Router,
};
use reqwest::Client;

pub struct LbService {
    engine: Arc<ZeroEngine>,
    listeners: Arc<Mutex<HashMap<u16, tokio::task::JoinHandle<()>>>>,
    http_client: Client,
}

impl LbService {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        Self { 
            engine, 
            listeners: Arc::new(Mutex::new(HashMap::new())),
            http_client: Client::new(),
        }
    }

    pub async fn create_load_balancer(&self, name: &str, lb_type: &str) -> ZeroResult<serde_json::Value> {
        let conn = self.engine.db.lock();
        
        // Ensure tables exist
        conn.execute("CREATE TABLE IF NOT EXISTS load_balancers (
            name TEXT PRIMARY KEY,
            type TEXT,
            dns_name TEXT,
            status TEXT
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        conn.execute("CREATE TABLE IF NOT EXISTS listeners (
            id TEXT PRIMARY KEY,
            lb_name TEXT,
            port INTEGER,
            protocol TEXT,
            target_group_arn TEXT
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        conn.execute("CREATE TABLE IF NOT EXISTS target_groups (
            arn TEXT PRIMARY KEY,
            name TEXT,
            port INTEGER,
            protocol TEXT,
            health_check_path TEXT
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        conn.execute("CREATE TABLE IF NOT EXISTS targets (
            group_arn TEXT,
            target_id TEXT,
            port INTEGER,
            status TEXT,
            PRIMARY KEY(group_arn, target_id)
        )", []).map_err(|e| ZeroError::Internal(e.to_string()))?;

        let dns_name = format!("{}.lb.zero.local", name);
        let status = "active";
        
        let insert = "INSERT OR REPLACE INTO load_balancers (name, type, dns_name, status) VALUES (?1, ?2, ?3, ?4)";
        conn.execute(insert, zero_data_core::rusqlite::params![name, lb_type, dns_name, status])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
            
        Ok(json!({ 
            "LoadBalancerName": name, 
            "DNSName": dns_name, 
            "Status": { "Code": status },
            "Type": lb_type 
        }))
    }

    pub async fn create_target_group(&self, name: &str, port: i32, protocol: &str) -> ZeroResult<String> {
        let conn = self.engine.db.lock();
        let arn = format!("arn:zero:elasticloadbalancing:000000:targetgroup/{}/{}", name, uuid::Uuid::new_v4());
        
        let sql = "INSERT INTO target_groups (arn, name, port, protocol, health_check_path) VALUES (?1, ?2, ?3, ?4, ?5)";
        conn.execute(sql, zero_data_core::rusqlite::params![arn, name, port, protocol, "/health"])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        
        Ok(arn)
    }

    pub async fn register_targets(&self, group_arn: &str, target_id: &str, port: i32) -> ZeroResult<()> {
        let conn = self.engine.db.lock();
        let sql = "INSERT OR REPLACE INTO targets (group_arn, target_id, port, status) VALUES (?1, ?2, ?3, ?4)";
        conn.execute(sql, zero_data_core::rusqlite::params![group_arn, target_id, port, "healthy"])
            .map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(())
    }

    pub async fn create_listener(&self, lb_name: &str, port: i32, protocol: &str, target_group_arn: &str) -> ZeroResult<String> {
        let id = format!("arn:zero:elasticloadbalancing:000000:listener/{}/{}", lb_name, uuid::Uuid::new_v4());
        
        {
            let conn = self.engine.db.lock();
            let sql = "INSERT INTO listeners (id, lb_name, port, protocol, target_group_arn) VALUES (?1, ?2, ?3, ?4, ?5)";
            conn.execute(sql, zero_data_core::rusqlite::params![id, lb_name, port, protocol, target_group_arn])
                .map_err(|e| ZeroError::Internal(e.to_string()))?;
        }
        
        // Spawn the data plane for this listener
        self.spawn_listener_task(port as u16, target_group_arn.to_string()).await?;
        
        Ok(id)
    }

    async fn spawn_listener_task(&self, port: u16, target_group_arn: String) -> ZeroResult<()> {
        let mut listeners = self.listeners.lock().await;
        if listeners.contains_key(&port) {
            return Ok(());
        }

        let engine = self.engine.clone();
        let http_client = self.http_client.clone();
        let tg_arn = target_group_arn.clone();

        let handle = tokio::spawn(async move {
            let app = Router::new()
                .fallback(move |req: Request| {
                    let engine = engine.clone();
                    let http_client = http_client.clone();
                    let tg_arn = tg_arn.clone();
                    async move {
                        proxy_handler(engine, http_client, tg_arn, req).await
                    }
                });

            let addr = std::net::SocketAddr::from(([0, 0, 0, 0], port));
            let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
            println!("📡 ZeroLB listening on port {}", port);
            axum::serve(listener, app).await.unwrap();
        });

        listeners.insert(port, handle);
        Ok(())
    }

    pub async fn list_load_balancers(&self) -> ZeroResult<Vec<serde_json::Value>> {
        let conn = self.engine.db.lock();
        
        let table_exists: bool = conn.query_row(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='load_balancers'",
            [],
            |row| row.get(0),
        ).unwrap_or(false);

        if !table_exists { return Ok(vec![]); }

        let mut stmt = conn.prepare("SELECT name, type, dns_name, status FROM load_balancers").map_err(|e| ZeroError::Internal(e.to_string()))?;
        let lbs = stmt.query_map([], |row| {
             Ok(json!({ 
                 "LoadBalancerName": row.get::<_, String>(0)?,
                 "Type": row.get::<_, String>(1)?, 
                 "DNSName": row.get::<_, String>(2)?,
                 "Status": { "Code": row.get::<_, String>(3)? }
             }))
        }).map_err(|e| ZeroError::Internal(e.to_string()))?
            .collect::<Result<Vec<serde_json::Value>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
        Ok(lbs)
    }

    /// Sync active listeners from database
    pub async fn sync_data_plane(&self) -> ZeroResult<()> {
        let items = {
            let conn = self.engine.db.lock();
            let table_exists: bool = conn.query_row(
                "SELECT count(*) FROM sqlite_master WHERE type='table' AND name='listeners'",
                [],
                |row| row.get(0),
            ).unwrap_or(false);

            if !table_exists { return Ok(()); }

            let mut stmt = conn.prepare("SELECT port, target_group_arn FROM listeners").map_err(|e| ZeroError::Internal(e.to_string()))?;
            let items: Vec<(u16, String)> = stmt.query_map([], |row| {
                Ok((row.get::<_, i32>(0)? as u16, row.get::<_, String>(1)?))
            }).map_err(|e| ZeroError::Internal(e.to_string()))?
                .collect::<Result<Vec<(u16, String)>, _>>().map_err(|e| ZeroError::Internal(e.to_string()))?;
            items
        };

        for (port, tg_arn) in items {
            self.spawn_listener_task(port, tg_arn).await?;
        }
        Ok(())
    }
}

async fn proxy_handler(engine: Arc<ZeroEngine>, client: Client, tg_arn: String, req: Request) -> impl IntoResponse {
    // 1. Find healthy targets
    let targets = {
        let conn = engine.db.lock();
        let mut stmt = conn.prepare("SELECT target_id, port FROM targets WHERE group_arn = ?1 AND status = 'healthy'").unwrap();
        let list = stmt.query_map([tg_arn], |row| {
            Ok((row.get::<_, String>(0)?, row.get::<_, i32>(1)?))
        }).unwrap().collect::<Result<Vec<(String, i32)>, _>>().unwrap();
        list
    };

    if targets.is_empty() {
        return (axum::http::StatusCode::SERVICE_UNAVAILABLE, "No healthy targets").into_response();
    }

    // 2. Round-Robin select (Simple Random for now)
    let (target_host, target_port) = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..targets.len());
        targets[idx].clone()
    };

    // 3. Forward request
    let path = req.uri().path();
    let query = req.uri().query().map(|q| format!("?{}", q)).unwrap_or_default();
    let url = format!("http://{}:{}{}{}", target_host, target_port, path, query);

    let method = req.method().clone();
    let headers = req.headers().clone();
    
    // We need to convert axum Request to reqwest Request but for a simple proxy we can just build it
    let mut proxy_req = client.request(method, &url);
    for (name, value) in headers {
        if let Some(n) = name {
            proxy_req = proxy_req.header(n, value);
        }
    }

    // Body streaming is complex, for local emulator we just collect (small payloads)
    let body_bytes = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await.unwrap_or_default();
    proxy_req = proxy_req.body(body_bytes);

    match proxy_req.send().await {
        Ok(res) => {
            let status = res.status();
            let headers = res.headers().clone();
            let body = res.bytes().await.unwrap_or_default();
            
            let mut builder = axum::http::Response::builder().status(status.as_u16());
            for (name, value) in headers.iter() {
                builder = builder.header(name, value);
            }
            builder.body(axum::body::Body::from(body)).unwrap().into_response()
        }
        Err(e) => {
            (axum::http::StatusCode::BAD_GATEWAY, format!("Gateway Error: {}", e)).into_response()
        }
    }
}
