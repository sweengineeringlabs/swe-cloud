use crate::{ClientInner, ZeroSdkError, common::request};
use std::sync::Arc;
use serde_json::json;

pub struct IamClient {
    inner: Arc<ClientInner>,
}

impl IamClient {
    pub(crate) fn new(inner: Arc<ClientInner>) -> Self {
        Self { inner }
    }

    pub async fn create_user(&self, username: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/iam/users",
            Some(json!({ "username": username })),
        ).await?;
        Ok(())
    }

    pub async fn attach_user_policy(&self, username: &str, policy: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            &format!("/iam/users/{}/policy", username),
            Some(json!({ "PolicyDocument": policy })),
        ).await?;
        Ok(())
    }

    pub async fn create_role(&self, rolename: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/iam/roles",
            Some(json!({ "Rolename": rolename })),
        ).await?;
        Ok(())
    }

    pub async fn create_group(&self, groupname: &str) -> Result<(), ZeroSdkError> {
        request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::POST,
            "/iam/groups",
            Some(json!({ "Groupname": groupname })),
        ).await?;
        Ok(())
    }

    pub async fn list_users(&self) -> Result<Vec<serde_json::Value>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/iam/users",
            None,
        ).await?;
        Ok(resp["Users"].as_array().cloned().unwrap_or_default())
    }

    pub async fn list_roles(&self) -> Result<Vec<serde_json::Value>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/iam/roles",
            None,
        ).await?;
        Ok(resp["Roles"].as_array().cloned().unwrap_or_default())
    }

    pub async fn list_groups(&self) -> Result<Vec<serde_json::Value>, ZeroSdkError> {
        let resp = request::<serde_json::Value>(
            &self.inner,
            reqwest::Method::GET,
            "/iam/groups",
            None,
        ).await?;
        Ok(resp["Groups"].as_array().cloned().unwrap_or_default())
    }
}
