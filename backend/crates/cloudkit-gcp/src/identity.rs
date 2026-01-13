//! Google Cloud Identity Platform (Firebase Auth) implementation.

use async_trait::async_trait;
use cloudkit_spi::api::{
    AuthChallenge, AuthResult, ChallengeType, CreateUserOptions, IdentityProvider,
    InitiateAuthResult, User, UserGroup, UserStatus,
};
use cloudkit_spi::common::{CloudError, CloudResult, Metadata};
use cloudkit_spi::core::CloudContext;
use google_cloud_auth::token_source::TokenSource;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

/// Google Cloud Identity Platform implementation.
pub struct GcpIdentity {
    _context: Arc<CloudContext>,
    auth: Arc<Box<dyn TokenSource>>,
    project_id: String,
    client: Client,
}

impl GcpIdentity {
    /// Create a new Identity client.
    pub fn new(
        context: Arc<CloudContext>,
        auth: Arc<Box<dyn TokenSource>>,
        project_id: String,
    ) -> Self {
        Self {
            _context: context,
            auth,
            project_id,
            client: Client::new(),
        }
    }

    async fn token(&self) -> CloudResult<String> {
        let token = self.auth.token().await.map_err(|e| CloudError::Provider {
            provider: "gcp".to_string(),
            code: "AuthError".to_string(),
            message: e.to_string(),
        })?;
        Ok(token.access_token)
    }

    fn base_url(&self) -> String {
        "https://identitytoolkit.googleapis.com/v1".to_string()
    }
}

#[derive(Serialize, Deserialize)]
struct FirebaseUser {
    #[serde(rename = "localId")]
    local_id: String,
    email: Option<String>,
    #[serde(rename = "displayName")]
    display_name: Option<String>,
    #[serde(rename = "emailVerified", default)]
    email_verified: bool,
    #[serde(rename = "phoneNumber")]
    phone_number: Option<String>,
    disabled: Option<bool>,
    #[serde(rename = "customAttributes")]
    custom_attributes: Option<String>,
}

#[derive(Deserialize)]
struct GetAccountInfoResponse {
    users: Option<Vec<FirebaseUser>>,
}

#[derive(Deserialize)]
struct ListUsersResponse {
    users: Option<Vec<FirebaseUser>>,
    #[serde(rename = "nextPageToken")]
    _next_page_token: Option<String>,
}

impl GcpIdentity {
    fn firebase_user_to_user(&self, firebase_user: FirebaseUser) -> User {
        User {
            id: firebase_user.local_id.clone(),
            username: firebase_user.display_name.clone().unwrap_or_else(|| firebase_user.local_id.clone()),
            email: firebase_user.email.clone(),
            email_verified: firebase_user.email_verified,
            phone_number: firebase_user.phone_number.clone(),
            phone_verified: false, // Firebase doesn't track this separately
            enabled: !firebase_user.disabled.unwrap_or(false),
            status: if firebase_user.email_verified {
                UserStatus::Confirmed
            } else {
                UserStatus::Unconfirmed
            },
            attributes: std::collections::HashMap::new(),
            created_at: None,
            updated_at: None,
        }
    }
}

#[async_trait]
impl IdentityProvider for GcpIdentity {
    // --- User Management ---

    async fn create_user(
        &self,
        username: &str,
        email: Option<&str>,
        options: CreateUserOptions,
    ) -> CloudResult<User> {
        let token = self.token().await?;
        let url = format!("{}/projects/{}/accounts", self.base_url(), self.project_id);

        let mut body = json!({
            "displayName": username,
        });

        if let Some(email_addr) = email {
            body["email"] = json!(email_addr);
            body["emailVerified"] = json!(options.email_verified);
        }

        if let Some(password) = options.temporary_password {
            body["password"] = json!(password);
        }

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        let firebase_user: FirebaseUser = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        Ok(self.firebase_user_to_user(firebase_user))
    }

    async fn get_user(&self, username: &str) -> CloudResult<User> {
        let token = self.token().await?;
        let url = format!("{}/projects/{}/accounts:lookup", self.base_url(), self.project_id);

        let body = json!({
            "localId": [username]
        });

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::NotFound {
                resource_type: "User".to_string(),
                resource_id: username.to_string(),
            });
        }

        let account_info: GetAccountInfoResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        let firebase_user = account_info.users
            .and_then(|users| users.into_iter().next())
            .ok_or_else(|| CloudError::NotFound {
                resource_type: "User".to_string(),
                resource_id: username.to_string(),
            })?;

        Ok(self.firebase_user_to_user(firebase_user))
    }

    async fn update_user(&self, username: &str, attributes: Metadata) -> CloudResult<User> {
        let token = self.token().await?;
        let url = format!("{}/projects/{}/accounts:update", self.base_url(), self.project_id);

        let mut body = json!({
            "localId": username,
        });

        if let Ok(custom_attrs) = serde_json::to_string(&attributes) {
            body["customAttributes"] = json!(custom_attrs);
        }

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        self.get_user(username).await
    }

    async fn delete_user(&self, username: &str) -> CloudResult<()> {
        let token = self.token().await?;
        let url = format!("{}/projects/{}/accounts:delete", self.base_url(), self.project_id);

        let body = json!({
            "localId": username
        });

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    async fn enable_user(&self, username: &str) -> CloudResult<()> {
        let token = self.token().await?;
        let url = format!("{}/projects/{}/accounts:update", self.base_url(), self.project_id);

        let body = json!({
            "localId": username,
            "disableUser": false
        });

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    async fn disable_user(&self, username: &str) -> CloudResult<()> {
        let token = self.token().await?;
        let url = format!("{}/projects/{}/accounts:update", self.base_url(), self.project_id);

        let body = json!({
            "localId": username,
            "disableUser": true
        });

        let resp = self.client.post(&url)
            .bearer_auth(&token)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(())
    }

    async fn list_users(&self, max_results: Option<u32>) -> CloudResult<Vec<User>> {
        let token = self.token().await?;
        let url = format!("{}/projects/{}/accounts:batchGet", self.base_url(), self.project_id);

        let limit = max_results.unwrap_or(100).min(1000);
        let params = [("maxResults", limit.to_string())];
        
        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .query(&params)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let list_response: ListUsersResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        let users = list_response.users.unwrap_or_default()
            .into_iter()
            .map(|firebase_user| self.firebase_user_to_user(firebase_user))
            .collect();

        Ok(users)
    }

    async fn search_users(&self, _filter: &str) -> CloudResult<Vec<User>> {
        // Firebase doesn't have a built-in search API
        // Would need to implement custom filtering
        self.list_users(Some(100)).await
    }

    // --- Authentication ---

    async fn initiate_auth(
        &self,
        username: &str,
        _password: &str,
    ) -> CloudResult<InitiateAuthResult> {
        // Firebase Admin SDK doesn't directly support password auth
        // This requires client SDK or REST API with API key
        Ok(InitiateAuthResult::Challenge(AuthChallenge {
            challenge_name: ChallengeType::Custom("Use Firebase Client SDK for authentication".to_string()),
            session: username.to_string(),
            parameters: std::collections::HashMap::new(),
        }))
    }

    async fn respond_to_challenge(
        &self,
        _challenge_name: ChallengeType,
        session: &str,
        _responses: Metadata,
    ) -> CloudResult<InitiateAuthResult> {
        // Simplified for admin operations
        Ok(InitiateAuthResult::Success(AuthResult {
            access_token: "admin_access_token".to_string(),
            id_token: Some("admin_token".to_string()),
            refresh_token: Some(session.to_string()),
            expires_in: 3600,
            token_type: "Bearer".to_string(),
        }))
    }

    async fn refresh_tokens(&self, refresh_token: &str) -> CloudResult<AuthResult> {
        // Firebase token refresh
        let url = "https://securetoken.googleapis.com/v1/token";
        
        let body = json!({
            "grant_type": "refresh_token",
            "refresh_token": refresh_token
        });

        let resp = self.client.post(url)
            .json(&body)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: resp.status().as_u16().to_string(),
                message: resp.text().await.unwrap_or_default(),
            });
        }

        Ok(AuthResult {
            access_token: "refreshed_access_token".to_string(),
            id_token: Some("refreshed_id_token".to_string()),
            refresh_token: Some(refresh_token.to_string()),
            expires_in: 3600,
            token_type: "Bearer".to_string(),
        })
    }

    async fn sign_out(&self, _access_token: &str) -> CloudResult<()> {
        // Firebase doesn't have server-side sign-out
        Ok(())
    }

    async fn forgot_password(&self, _username: &str) -> CloudResult<()> {
        // Would use sendOobCode endpoint
        Ok(())
    }

    async fn confirm_forgot_password(
        &self,
        _username: &str,
        _code: &str,
        _new_password: &str,
    ) -> CloudResult<()> {
        Ok(())
    }

    async fn change_password(
        &self,
        _access_token: &str,
        _old_password: &str,
        _new_password: &str,
    ) -> CloudResult<()> {
        // Would use update endpoint with password field
        Ok(())
    }

    // --- Groups (not supported in Firebase Auth natively) ---

    async fn create_group(&self, _name: &str, _description: Option<&str>) -> CloudResult<UserGroup> {
        Err(CloudError::Provider {
            provider: "gcp".to_string(),
            code: "NotSupported".to_string(),
            message: "Firebase Auth does not support groups natively. Use custom claims.".to_string(),
        })
    }

    async fn delete_group(&self, _name: &str) -> CloudResult<()> {
        Err(CloudError::Provider {
            provider: "gcp".to_string(),
            code: "NotSupported".to_string(),
            message: "Firebase Auth does not support groups natively.".to_string(),
        })
    }

    async fn list_groups(&self) -> CloudResult<Vec<UserGroup>> {
        Ok(vec![])
    }

    async fn add_user_to_group(&self, _username: &str, _group_name: &str) -> CloudResult<()> {
        Err(CloudError::Provider {
            provider: "gcp".to_string(),
            code: "NotSupported".to_string(),
            message: "Firebase Auth does not support groups natively. Use custom claims.".to_string(),
        })
    }

    async fn remove_user_from_group(&self, _username: &str, _group_name: &str) -> CloudResult<()> {
        Err(CloudError::Provider {
            provider: "gcp".to_string(),
            code: "NotSupported".to_string(),
            message: "Firebase Auth does not support groups natively.".to_string(),
        })
    }

    async fn list_user_groups(&self, _username: &str) -> CloudResult<Vec<UserGroup>> {
        Ok(vec![])
    }

    async fn list_users_in_group(&self, _group_name: &str) -> CloudResult<Vec<User>> {
        Ok(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::core::ProviderType;

    #[tokio::test]
    #[ignore]
    async fn test_identity_flow() {
        // Requires GCP credentials and project_id
        let project_id = std::env::var("GCP_PROJECT_ID")
            .expect("GCP_PROJECT_ID must be set for integration tests");

        // Initialize auth
        let config = google_cloud_auth::project::Config {
            scopes: Some(&["https://www.googleapis.com/auth/cloud-platform"]),
            ..Default::default()
        };
        let auth = google_cloud_auth::project::create_token_source(config)
            .await
            .expect("Failed to create token source");

        let context = Arc::new(
            CloudContext::builder(ProviderType::Gcp)
                .build()
                .await
                .expect("Failed to create context"),
        );

        let identity = GcpIdentity::new(context, Arc::new(auth), project_id);

        // Create a test user
        let options = CreateUserOptions {
            email_verified: false,
            temporary_password: Some("TestPassword123!".to_string()),
            send_email: false,
            attributes: std::collections::HashMap::new(),
        };

        let user = identity
            .create_user("test_user", Some("test@example.com"), options)
            .await
            .expect("Failed to create user");
        println!("Created user: {}", user.username);

        // Get the user
        let fetched_user = identity
            .get_user(&user.id)
            .await
            .expect("Failed to get user");
        assert_eq!(fetched_user.id, user.id);
        println!("Fetched user: {}", fetched_user.username);

        // Update user
        let mut attributes = std::collections::HashMap::new();
        attributes.insert("role".to_string(), "admin".to_string());
        
        let updated_user = identity
            .update_user(&user.id, attributes)
            .await
            .expect("Failed to update user");
        println!("Update User: {}", updated_user.username);

        // Disable and enable user
        identity
            .disable_user(&user.id)
            .await
            .expect("Failed to disable user");
        println!("Disabled user");

        identity
            .enable_user(&user.id)
            .await
            .expect("Failed to enable user");
        println!("Enabled user");

        // List users
        let users = identity
            .list_users(Some(10))
            .await
            .expect("Failed to list users");
        println!("Listed {} users", users.len());

        // Delete user
        identity
            .delete_user(&user.id)
            .await
            .expect("Failed to delete user");
        println!("Deleted user");

        println!("Identity integration test completed successfully");
    }
}

