//! ZeroCloud Control Plane Orchestrator

use zero_control_spi::{ZeroRequest, ZeroResponse, ZeroResult, ZeroService, ZeroError};
use zero_data_core::ZeroEngine;
use async_trait::async_trait;
use std::sync::Arc;
use serde_json::json;

pub mod services;

pub struct ZeroProvider {
    engine: Arc<ZeroEngine>,
    pub store: services::store::StoreService,
    pub db: services::db::DbService,
    pub func: services::func::FuncService,
    pub queue: services::queue::QueueService,
    pub iam: services::iam::IamService,
    pub lb: services::lb::LbService,
    pub eks: services::eks::EksService,
}

impl ZeroProvider {
    pub fn new(engine: Arc<ZeroEngine>) -> Self {
        let store = services::store::StoreService::new(engine.clone());
        let db = services::db::DbService::new(engine.clone());
        let func = services::func::FuncService::new(engine.clone());
        let queue = services::queue::QueueService::new(engine.clone());
        let iam = services::iam::IamService::new(engine.clone());
        let lb = services::lb::LbService::new(engine.clone());
        let eks = services::eks::EksService::new(engine.clone());
        Self { engine, store, db, func, queue, iam, lb, eks }
    }
}

#[async_trait]
impl ZeroService for ZeroProvider {
    async fn handle_request(&self, req: ZeroRequest) -> ZeroResult<ZeroResponse> {
        let parts: Vec<&str> = req.path.split('/').filter(|s| !s.is_empty()).collect();
        // Path format: ["v1", "service", ...]
        // Example: /v1/store/buckets -> ["v1", "store", "buckets"]

        if parts.is_empty() || parts[0] != "v1" {
             return Ok(ZeroResponse::json(json!({ "message": "ZeroCloud API v1" })));
        }

        match parts.get(1) {
            Some(&"nodes") | Some(&"stats") | Some(&"workloads") | Some(&"volumes") => {
                self.route_core(&parts[1..], &req).await
            },
            Some(&"networks") | Some(&"loadbalancers") => {
                self.route_net(&parts[1..], &req).await
            },
            Some(&"store") => self.route_store(&parts[2..], &req).await,
            Some(&"db") => self.route_db(&parts[2..], &req).await,
            Some(&"func") => self.route_func(&parts[2..], &req).await,
            Some(&"queue") => self.route_queue(&parts[2..], &req).await,
            Some(&"iam") => self.route_iam(&parts[2..], &req).await,
            Some(&"eks") => self.route_eks(&parts[2..], &req).await,
            _ => Err(ZeroError::NotFound(format!("Service not found: {:?}", parts.get(1)))),
        }
    }
}

impl ZeroProvider {
    async fn route_eks(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("POST", ["clusters"]) => self.eks.handle("CreateCluster", &req.body).await.map(ZeroResponse::json_bytes),
            ("GET", ["clusters", _name]) => self.eks.handle("DescribeCluster", &req.body).await.map(ZeroResponse::json_bytes),
            ("DELETE", ["clusters", _name]) => self.eks.handle("DeleteCluster", &req.body).await.map(ZeroResponse::json_bytes),
             // Simple matching for now - AWS URLs are more complex (e.g. /clusters/{name}/nodegroups/{name})
             // I will add a catch-all or basic pattern for now to satisfy simple calls
             _ => {
                 // Try to guess action based on URL structure or just handle generically
                 // For now, let's just log and fail if not matched
                 Err(ZeroError::NotFound(format!("EKS route not found: {:?}", parts)))
             }
        }
    }

    async fn route_core(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("GET", ["nodes"]) => {
                 let nodes = self.engine.list_nodes().map_err(|e| ZeroError::Internal(e.to_string()))?;
                 Ok(ZeroResponse::json(json!({ "nodes": nodes })))
            },
            ("GET", ["stats"]) => {
                let stats = self.engine.compute.get_stats().await?;
                Ok(ZeroResponse::json(json!(stats)))
            },
            ("GET", ["workloads"]) => {
                let workloads = self.engine.compute.list_workloads().await?;
                Ok(ZeroResponse::json(json!({ "workloads": workloads })))
            },
            ("POST", ["workloads"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;
                let image = body["image"].as_str().ok_or_else(|| ZeroError::Validation("Missing image".into()))?;
                let cpu = body["cpu"].as_f64().unwrap_or(1.0) as f32;
                let memory = body["memory_mb"].as_i64().unwrap_or(512) as i32;
                let status = self.engine.compute.create_workload(id, image, cpu, memory).await?;
                Ok(ZeroResponse::json(json!(status)))
            },
            ("DELETE", ["workloads"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;
                self.engine.compute.delete_workload(id).await?;
                Ok(ZeroResponse::json(json!({ "status": "Deleted", "id": id })))
            },
            ("GET", ["volumes"]) => {
                let volumes = self.engine.storage.list_volumes().await?;
                Ok(ZeroResponse::json(json!({ "volumes": volumes })))
            },
            ("POST", ["volumes"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;
                let size = body["size_gb"].as_i64().unwrap_or(10) as i32;
                let status = self.engine.storage.create_volume(id, size).await?;
                Ok(ZeroResponse::json(json!(status)))
            },
            _ => Err(ZeroError::NotFound("Core route not found".into()))
        }
    }

    async fn route_net(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("GET", ["networks"]) => {
                let networks = self.engine.network.list_networks().await?;
                Ok(ZeroResponse::json(json!({ "networks": networks })))
            },
            ("POST", ["networks"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing id".into()))?;
                let cidr = body["cidr"].as_str().unwrap_or("10.0.0.0/24");
                let status = self.engine.network.create_network(id, cidr).await?;
                Ok(ZeroResponse::json(json!(status)))
            },
            ("GET", ["loadbalancers"]) => {
                let lbs = self.lb.list_load_balancers().await?;
                Ok(ZeroResponse::json(json!({ "LoadBalancers": lbs })))
            },
            ("POST", ["loadbalancers"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let name = body["name"].as_str().ok_or_else(|| ZeroError::Validation("Missing name".into()))?;
                let lb_type = body["type"].as_str().unwrap_or("application");
                let status = self.lb.create_load_balancer(name, lb_type).await?;
                Ok(ZeroResponse::json(status))
            },
            ("POST", ["targetgroups"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let name = body["name"].as_str().ok_or_else(|| ZeroError::Validation("Missing name".into()))?;
                let port = body["port"].as_i64().unwrap_or(80) as i32;
                let protocol = body["protocol"].as_str().unwrap_or("HTTP");
                let arn = self.lb.create_target_group(name, port, protocol).await?;
                Ok(ZeroResponse::json(json!({ "TargetGroupArn": arn })))
            },
            ("POST", ["targetgroups", arn, "targets"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let target_id = body["id"].as_str().ok_or_else(|| ZeroError::Validation("Missing target id".into()))?;
                let port = body["port"].as_i64().unwrap_or(80) as i32;
                self.lb.register_targets(arn, target_id, port).await?;
                Ok(ZeroResponse::json(json!({ "status": "Registered" })))
            },
            ("POST", ["listeners"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let lb_name = body["load_balancer_name"].as_str().ok_or_else(|| ZeroError::Validation("Missing lb name".into()))?;
                let port = body["port"].as_i64().ok_or_else(|| ZeroError::Validation("Missing port".into()))? as i32;
                let protocol = body["protocol"].as_str().unwrap_or("HTTP");
                let tg_arn = body["target_group_arn"].as_str().ok_or_else(|| ZeroError::Validation("Missing target group arn".into()))?;
                let arn = self.lb.create_listener(lb_name, port, protocol, tg_arn).await?;
                Ok(ZeroResponse::json(json!({ "ListenerArn": arn })))
            },
            _ => Err(ZeroError::NotFound("Network route not found".into()))
        }
    }

    async fn route_store(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("GET", ["buckets"]) => {
                let buckets = self.store.list_buckets().await?;
                Ok(ZeroResponse::json(json!({ "buckets": buckets })))
            },
            ("POST", ["buckets"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let name = body["name"].as_str().ok_or_else(|| ZeroError::Validation("Missing name".into()))?;
                self.store.create_bucket(name).await?;
                Ok(ZeroResponse::json(json!({ "status": "Created", "name": name })))
            },
            _ => Err(ZeroError::NotFound("Store route not found".into()))
        }
    }

    async fn route_db(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("GET", ["tables"]) => {
                let tables = self.db.list_tables().await?;
                Ok(ZeroResponse::json(json!({ "tables": tables })))
            },
            ("POST", ["tables"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let name = body["name"].as_str().ok_or_else(|| ZeroError::Validation("Missing name".into()))?;
                let pk = body["pk"].as_str().unwrap_or("id");
                self.db.create_table(name, pk).await?;
                Ok(ZeroResponse::json(json!({ "status": "Created", "name": name })))
            },
            ("POST", ["tables", table_name, "items"]) => {
                 let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                 let pk_value = body["pk"].as_str().ok_or_else(|| ZeroError::Validation("Missing pk value".into()))?.to_string();
                 self.db.put_item(table_name, &pk_value, body).await?;
                 Ok(ZeroResponse::json(json!({ "status": "ItemPut", "table": table_name })))
            },
            _ => Err(ZeroError::NotFound("DB route not found".into()))
        }
    }

    async fn route_func(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("GET", ["functions"]) => {
                let funcs = self.func.list_functions().await?;
                Ok(ZeroResponse::json(json!({ "functions": funcs })))
            },
            ("POST", ["functions"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let name = body["name"].as_str().ok_or_else(|| ZeroError::Validation("Missing name".into()))?;
                let handler = body["handler"].as_str().unwrap_or("index.handler");
                let code = body["code"].as_str().unwrap_or(""); 
                self.func.create_function(name, handler, code).await?;
                Ok(ZeroResponse::json(json!({ "status": "Created", "name": name })))
            },
            ("POST", ["functions", name, "invocations"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).unwrap_or(json!({}));
                let result = self.func.invoke_function(name, body).await?;
                Ok(ZeroResponse::json(result))
            },
            _ => Err(ZeroError::NotFound("Func route not found".into()))
        }
    }

    async fn route_queue(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("GET", ["queues"]) => {
                let urls = self.queue.list_queues().await?;
                Ok(ZeroResponse::json(json!({ "QueueUrls": urls })))
            },
            ("POST", ["queues"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let name = body["name"].as_str().ok_or_else(|| ZeroError::Validation("Missing name".into()))?;
                let url = self.queue.create_queue(name).await?;
                Ok(ZeroResponse::json(json!({ "QueueUrl": url })))
            },
            ("POST", ["queues", name, "messages"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let msg_body = body["body"].as_str().ok_or_else(|| ZeroError::Validation("Missing body".into()))?;
                let id = self.queue.send_message(name, msg_body).await?;
                Ok(ZeroResponse::json(json!({ "MessageId": id })))
            },
            ("GET", ["queues", name, "messages"]) => {
                let msg = self.queue.receive_message(name).await?;
                Ok(ZeroResponse::json(json!({ "Messages": msg })))
            },
            ("DELETE", ["queues", name, "messages", receipt_handle]) => {
                self.queue.delete_message(name, receipt_handle).await?;
                Ok(ZeroResponse::json(json!({ "status": "Deleted" })))
            },
            _ => Err(ZeroError::NotFound("Queue route not found".into()))
        }
    }

    async fn route_iam(&self, parts: &[&str], req: &ZeroRequest) -> ZeroResult<ZeroResponse> {
        match (req.method.as_str(), parts) {
            ("GET", ["users"]) => {
                let users = self.iam.list_users().await?;
                Ok(ZeroResponse::json(json!({ "Users": users })))
            },
            ("POST", ["users"]) => {
                let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
                let username = body["username"].as_str().ok_or_else(|| ZeroError::Validation("Missing username".into()))?;
                self.iam.create_user(username).await?;
            Ok(ZeroResponse::json(json!({ "User": { "UserName": username } })))
        },
        ("POST", ["users", username, "policy"]) => {
             let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
             let policy_doc = body["PolicyDocument"].to_string(); 
             self.iam.attach_user_policy(username, &policy_doc).await?;
             Ok(ZeroResponse::json(json!({ "status": "Attached" })))
        },
        ("GET", ["roles"]) => {
             let roles = self.iam.list_roles().await?;
             Ok(ZeroResponse::json(json!({ "Roles": roles })))
        },
        ("POST", ["roles"]) => {
             let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
             let rolename = body["Rolename"].as_str().ok_or_else(|| ZeroError::Validation("Missing Rolename".into()))?;
             self.iam.create_role(rolename).await?;
             Ok(ZeroResponse::json(json!({ "Role": { "RoleName": rolename } })))
        },
        ("GET", ["groups"]) => {
             let groups = self.iam.list_groups().await?;
             Ok(ZeroResponse::json(json!({ "Groups": groups })))
        },
        ("POST", ["groups"]) => {
             let body: serde_json::Value = serde_json::from_slice(&req.body).map_err(|e| ZeroError::Validation(e.to_string()))?;
             let groupname = body["Groupname"].as_str().ok_or_else(|| ZeroError::Validation("Missing Groupname".into()))?;
             self.iam.create_group(groupname).await?;
             Ok(ZeroResponse::json(json!({ "Group": { "GroupName": groupname } })))
        },
         _ => Err(ZeroError::NotFound("IAM route not found".into()))
        }
    }
}
