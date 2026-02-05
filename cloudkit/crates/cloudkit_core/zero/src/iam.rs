use cloudkit_api::{IdentityProvider, User, UserGroup, InitiateAuthResult, AuthResult, ChallengeType, CreateUserOptions, UserStatus};
use cloudkit_spi::{CloudResult, CloudError, Metadata};
use async_trait::async_trait;
use zero_sdk::ZeroClient;

pub struct ZeroId {
    client: ZeroClient,
}

impl ZeroId {
    pub fn new(client: ZeroClient) -> Self {
        Self { client }
    }
}

#[async_trait]
impl IdentityProvider for ZeroId {
    async fn create_user(
        &self,
        username: &str,
        _email: Option<&str>,
        _options: CreateUserOptions,
    ) -> CloudResult<User> {
        self.client.iam().create_user(username).await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(User::new(username, username))
    }

    async fn get_user(&self, username: &str) -> CloudResult<User> {
        let users = self.list_users(None).await?;
        users.into_iter().find(|u| u.username == username)
            .ok_or_else(|| CloudError::NotFound {
                resource_type: "User".to_string(),
                resource_id: username.to_string(),
            })
    }

    async fn update_user(&self, _username: &str, _attributes: Metadata) -> CloudResult<User> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn delete_user(&self, _username: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn enable_user(&self, _username: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn disable_user(&self, _username: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn list_users(&self, _limit: Option<u32>) -> CloudResult<Vec<User>> {
        let resp = self.client.iam().list_users().await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        
        Ok(resp.into_iter().map(|u| {
            let username = u["UserName"].as_str().unwrap_or_default();
            let mut user = User::new(username, username);
            user.status = UserStatus::Confirmed;
            user
        }).collect())
    }

    async fn search_users(&self, _filter: &str) -> CloudResult<Vec<User>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn initiate_auth(
        &self,
        _username: &str,
        _password: &str,
    ) -> CloudResult<InitiateAuthResult> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn respond_to_challenge(
        &self,
        _challenge_name: ChallengeType,
        _session: &str,
        _responses: Metadata,
    ) -> CloudResult<InitiateAuthResult> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn refresh_tokens(&self, _refresh_token: &str) -> CloudResult<AuthResult> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn sign_out(&self, _access_token: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn forgot_password(&self, _username: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn confirm_forgot_password(
        &self,
        _username: &str,
        _code: &str,
        _new_password: &str,
    ) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn change_password(
        &self,
        _access_token: &str,
        _old_password: &str,
        _new_password: &str,
    ) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn create_group(&self, name: &str, _description: Option<&str>) -> CloudResult<UserGroup> {
        self.client.iam().create_group(name).await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        Ok(UserGroup {
            name: name.to_string(),
            description: None,
            role_arn: None,
            precedence: None,
            created_at: Some(chrono::Utc::now()),
        })
    }

    async fn delete_group(&self, _name: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn list_groups(&self) -> CloudResult<Vec<UserGroup>> {
        let resp = self.client.iam().list_groups().await
            .map_err(|e| CloudError::Internal(e.to_string()))?;
        
        Ok(resp.into_iter().map(|g| UserGroup {
            name: g["GroupName"].as_str().unwrap_or_default().to_string(),
            description: None,
            role_arn: None,
            precedence: None,
            created_at: None,
        }).collect())
    }

    async fn add_user_to_group(&self, _username: &str, _group_name: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn remove_user_from_group(&self, _username: &str, _group_name: &str) -> CloudResult<()> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn list_user_groups(&self, _username: &str) -> CloudResult<Vec<UserGroup>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }

    async fn list_users_in_group(&self, _group_name: &str) -> CloudResult<Vec<User>> {
        Err(CloudError::ServiceError("Not implemented".to_string()))
    }
}
