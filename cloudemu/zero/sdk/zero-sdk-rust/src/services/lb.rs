use crate::{ClientInner, ZeroSdkError, common::request};
use std::sync::Arc;
use serde_json::json;

pub struct LbClient {
    inner: Arc<ClientInner>,
}

impl LbClient {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create_load_balancer(&self, name: &str, lb_type: &str) -> Result<serde_json::Value, ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/network/loadbalancers",
            Some(json!({ "name": name, "type": lb_type })),
        ).await
    }

    pub async fn create_target_group(&self, name: &str, port: i32) -> Result<String, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/network/targetgroups",
            Some(json!({ "name": name, "port": port })),
        ).await?;
        
        resp["TargetGroupArn"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| ZeroSdkError::Internal("Missing TargetGroupArn".into()))
    }

    pub async fn register_targets(&self, group_arn: &str, id: &str, port: i32) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            &format!("/network/targetgroups/{}/targets", group_arn),
            Some(json!({ "id": id, "port": port })),
        ).await?;
        Ok(())
    }

    pub async fn create_listener(&self, lb_name: &str, port: i32, target_group_arn: &str) -> Result<String, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/network/listeners",
            Some(json!({ 
                "load_balancer_name": lb_name, 
                "port": port, 
                "target_group_arn": target_group_arn 
            })),
        ).await?;
        
        resp["ListenerArn"].as_str()
            .map(|s| s.to_string())
            .ok_or_else(|| ZeroSdkError::Internal("Missing ListenerArn".into()))
    }

    pub async fn list_load_balancers(&self) -> Result<Vec<serde_json::Value>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/network/loadbalancers",
            None,
        ).await?;
        Ok(resp["LoadBalancers"].as_array().cloned().unwrap_or_default())
    }
}
