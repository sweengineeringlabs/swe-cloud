//! AWS Cognito User Pools implementation.

use async_trait::async_trait;
use cloudkit_api::{
    AuthChallenge, AuthResult, ChallengeType, CreateUserOptions, IdentityProvider,
    InitiateAuthResult, User, UserGroup, UserStatus,
};
use cloudkit_spi::{CloudError, CloudResult, Metadata};
use cloudkit_spi::CloudContext;
use std::sync::Arc;

/// AWS Cognito User Pools implementation.
pub struct AwsIdentity {
    _context: Arc<CloudContext>,
    client: aws_sdk_cognitoidentityprovider::Client,
}

impl AwsIdentity {
    /// Create a new Cognito Identity Provider client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_cognitoidentityprovider::Client::new(&sdk_config);
        Self { _context: context, client }
    }

    fn map_user(&self, user: &aws_sdk_cognitoidentityprovider::types::UserType) -> User {
        User {
            id: user.username().unwrap_or_default().to_string(),
            username: user.username().unwrap_or_default().to_string(),
            email: user.attributes().iter().find(|a| a.name() == "email").and_then(|a| a.value()).map(|s| s.to_string()),
            email_verified: user.attributes().iter().find(|a| a.name() == "email_verified").and_then(|a| a.value()) == Some("true"),
            phone_number: user.attributes().iter().find(|a| a.name() == "phone_number").and_then(|a| a.value()).map(|s| s.to_string()),
            phone_verified: user.attributes().iter().find(|a| a.name() == "phone_number_verified").and_then(|a| a.value()) == Some("true"),
            enabled: user.enabled(),
            status: self.map_user_status(user.user_status()),
            created_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(user.user_create_date().map(|d| d.secs()).unwrap_or(0), 0).unwrap_or_default()),
            updated_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(user.user_last_modified_date().map(|d| d.secs()).unwrap_or(0), 0).unwrap_or_default()),
            attributes: user.attributes().iter().map(|a| (a.name().to_string(), a.value().unwrap_or_default().to_string())).collect(),
        }
    }

    fn map_admin_user(&self, user: &aws_sdk_cognitoidentityprovider::operation::admin_get_user::AdminGetUserOutput) -> User {
        User {
            id: user.username().to_string(),
            username: user.username().to_string(),
            email: user.user_attributes().iter().find(|a| a.name() == "email").and_then(|a| a.value()).map(|s| s.to_string()),
            email_verified: user.user_attributes().iter().find(|a| a.name() == "email_verified").and_then(|a| a.value()) == Some("true"),
            phone_number: user.user_attributes().iter().find(|a| a.name() == "phone_number").and_then(|a| a.value()).map(|s| s.to_string()),
            phone_verified: user.user_attributes().iter().find(|a| a.name() == "phone_number_verified").and_then(|a| a.value()) == Some("true"),
            enabled: user.enabled(),
            status: self.map_user_status(user.user_status()),
            created_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(user.user_create_date().map(|d| d.secs()).unwrap_or(0), 0).unwrap_or_default()),
            updated_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(user.user_last_modified_date().map(|d| d.secs()).unwrap_or(0), 0).unwrap_or_default()),
            attributes: user.user_attributes().iter().map(|a| (a.name().to_string(), a.value().unwrap_or_default().to_string())).collect(),
        }
    }

    fn map_user_status(&self, status: Option<&aws_sdk_cognitoidentityprovider::types::UserStatusType>) -> UserStatus {
        match status {
            Some(aws_sdk_cognitoidentityprovider::types::UserStatusType::Confirmed) => UserStatus::Confirmed,
            Some(aws_sdk_cognitoidentityprovider::types::UserStatusType::Unconfirmed) => UserStatus::Unconfirmed,
            Some(aws_sdk_cognitoidentityprovider::types::UserStatusType::Archived) => UserStatus::Archived,
            Some(aws_sdk_cognitoidentityprovider::types::UserStatusType::Compromised) => UserStatus::Compromised,
            Some(aws_sdk_cognitoidentityprovider::types::UserStatusType::ResetRequired) => UserStatus::ResetRequired,
            Some(aws_sdk_cognitoidentityprovider::types::UserStatusType::ForceChangePassword) => UserStatus::ForceChangePassword,
            _ => UserStatus::Unknown,
        }
    }

    fn get_user_pool_id(&self) -> CloudResult<String> {
        self._context.config.parameters.get("aws.cognito.user_pool_id")
            .cloned()
            .or_else(|| std::env::var("AWS_COGNITO_USER_POOL_ID").ok())
            .ok_or_else(|| CloudError::Config("Missing Cognito User Pool ID. Set 'aws.cognito.user_pool_id' in config or 'AWS_COGNITO_USER_POOL_ID' env var.".into()))
    }

    fn get_client_id(&self) -> CloudResult<String> {
        self._context.config.parameters.get("aws.cognito.client_id")
            .cloned()
            .or_else(|| std::env::var("AWS_COGNITO_CLIENT_ID").ok())
            .ok_or_else(|| CloudError::Config("Missing Cognito Client ID. Set 'aws.cognito.client_id' in config or 'AWS_COGNITO_CLIENT_ID' env var.".into()))
    }
}

#[async_trait]
impl IdentityProvider for AwsIdentity {
    async fn create_user(
        &self,
        username: &str,
        email: Option<&str>,
        options: CreateUserOptions,
    ) -> CloudResult<User> {
        let mut req = self.client.admin_create_user()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username);
            
        if let Some(e) = email {
            req = req.user_attributes(aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
                .name("email")
                .value(e)
                .build()
                .unwrap());
        }
        
        if let Some(pw) = options.temporary_password {
            req = req.temporary_password(pw);
        }
        
        // Map other options if needed
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        let user = resp.user().unwrap();
        
        Ok(self.map_user(user))
    }

    async fn get_user(&self, username: &str) -> CloudResult<User> {
        let resp = self.client.admin_get_user()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(self.map_admin_user(&resp))
    }

    async fn update_user(&self, username: &str, attributes: Metadata) -> CloudResult<User> {
        let mut aws_attrs = Vec::new();
        for (k, v) in attributes {
            aws_attrs.push(aws_sdk_cognitoidentityprovider::types::AttributeType::builder()
                .name(k)
                .value(v)
                .build()
                .unwrap());
        }
        
        self.client.admin_update_user_attributes()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .set_user_attributes(Some(aws_attrs))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        self.get_user(username).await
    }

    async fn delete_user(&self, username: &str) -> CloudResult<()> {
        self.client.admin_delete_user()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn enable_user(&self, username: &str) -> CloudResult<()> {
        self.client.admin_enable_user()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn disable_user(&self, username: &str) -> CloudResult<()> {
        self.client.admin_disable_user()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_users(&self, limit: Option<u32>) -> CloudResult<Vec<User>> {
        let mut req = self.client.list_users()
            .user_pool_id(self.get_user_pool_id()?);
        if let Some(l) = limit {
            req = req.limit(l as i32);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(resp.users().iter().map(|u| self.map_user(u)).collect())
    }

    async fn search_users(&self, filter: &str) -> CloudResult<Vec<User>> {
        let resp = self.client.list_users()
            .user_pool_id(self.get_user_pool_id()?)
            .filter(filter)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.users().iter().map(|u| self.map_user(u)).collect())
    }

    async fn initiate_auth(
        &self,
        username: &str,
        password: &str,
    ) -> CloudResult<InitiateAuthResult> {
        let resp = self.client.admin_initiate_auth()
            .user_pool_id(self.get_user_pool_id()?)
            .client_id(self.get_client_id()?)
            .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::AdminNoSrpAuth)
            .auth_parameters("USERNAME", username)
            .auth_parameters("PASSWORD", password)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        if let Some(challenge_name) = resp.challenge_name() {
             Ok(InitiateAuthResult::Challenge(AuthChallenge {
                challenge_name: ChallengeType::Custom(format!("{:?}", challenge_name)),
                session: resp.session().unwrap_or_default().to_string(),
                parameters: resp.challenge_parameters().cloned().unwrap_or_default(),
            }))
        } else {
            let auth_result = resp.authentication_result().unwrap();
            Ok(InitiateAuthResult::Success(AuthResult {
                access_token: auth_result.access_token().unwrap_or_default().to_string(),
                id_token: auth_result.id_token().map(|s| s.to_string()),
                refresh_token: auth_result.refresh_token().map(|s| s.to_string()),
                token_type: "Bearer".to_string(),
                expires_in: auth_result.expires_in() as u64,
            }))
        }
    }

    async fn respond_to_challenge(
        &self,
        _challenge_name: ChallengeType,
        session: &str,
        responses: Metadata,
    ) -> CloudResult<InitiateAuthResult> {
        let resp = self.client.admin_respond_to_auth_challenge()
            .user_pool_id(self.get_user_pool_id()?)
            .client_id(self.get_client_id()?)
            .challenge_name(aws_sdk_cognitoidentityprovider::types::ChallengeNameType::CustomChallenge) // Mapping needed
            .session(session)
            .set_challenge_responses(Some(responses))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        if let Some(challenge_name) = resp.challenge_name() {
             Ok(InitiateAuthResult::Challenge(AuthChallenge {
                challenge_name: ChallengeType::Custom(format!("{:?}", challenge_name)),
                session: resp.session().unwrap_or_default().to_string(),
                parameters: resp.challenge_parameters().cloned().unwrap_or_default(),
            }))
        } else {
            let auth_result = resp.authentication_result().unwrap();
            Ok(InitiateAuthResult::Success(AuthResult {
                access_token: auth_result.access_token().unwrap_or_default().to_string(),
                id_token: auth_result.id_token().map(|s| s.to_string()),
                refresh_token: auth_result.refresh_token().map(|s| s.to_string()),
                token_type: "Bearer".to_string(),
                expires_in: auth_result.expires_in() as u64,
            }))
        }
    }

    async fn refresh_tokens(&self, refresh_token: &str) -> CloudResult<AuthResult> {
        let resp = self.client.admin_initiate_auth()
            .user_pool_id(self.get_user_pool_id()?)
            .client_id(self.get_client_id()?)
            .auth_flow(aws_sdk_cognitoidentityprovider::types::AuthFlowType::AdminNoSrpAuth)
            .auth_parameters("REFRESH_TOKEN", refresh_token)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let auth_result = resp.authentication_result().unwrap();
        Ok(AuthResult {
            access_token: auth_result.access_token().unwrap_or_default().to_string(),
            id_token: auth_result.id_token().map(|s| s.to_string()),
            refresh_token: None,
            token_type: "Bearer".to_string(),
            expires_in: auth_result.expires_in() as u64,
        })
    }

    async fn sign_out(&self, _access_token: &str) -> CloudResult<()> {
        Ok(())
    }

    async fn forgot_password(&self, username: &str) -> CloudResult<()> {
        self.client.forgot_password()
            .client_id(self.get_client_id()?)
            .username(username)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn confirm_forgot_password(
        &self,
        username: &str,
        code: &str,
        new_password: &str,
    ) -> CloudResult<()> {
        self.client.confirm_forgot_password()
            .client_id(self.get_client_id()?)
            .username(username)
            .confirmation_code(code)
            .password(new_password)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn change_password(
        &self,
        access_token: &str,
        old_password: &str,
        new_password: &str,
    ) -> CloudResult<()> {
        self.client.change_password()
            .access_token(access_token)
            .previous_password(old_password)
            .proposed_password(new_password)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn create_group(&self, name: &str, description: Option<&str>) -> CloudResult<UserGroup> {
        let mut req = self.client.create_group()
            .user_pool_id(self.get_user_pool_id()?)
            .group_name(name);
            
        if let Some(desc) = description {
            req = req.description(desc);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        let g = resp.group().unwrap();
        
        Ok(UserGroup {
            name: g.group_name().unwrap_or_default().to_string(),
            description: g.description().map(|s| s.to_string()),
            role_arn: g.role_arn().map(|s| s.to_string()),
            precedence: g.precedence().map(|p| p as u32),
            created_at: g.creation_date().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()),
        })
    }

    async fn delete_group(&self, name: &str) -> CloudResult<()> {
        self.client.delete_group()
            .user_pool_id(self.get_user_pool_id()?)
            .group_name(name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_groups(&self) -> CloudResult<Vec<UserGroup>> {
        let resp = self.client.list_groups()
            .user_pool_id(self.get_user_pool_id()?)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.groups().iter().map(|g| {
            UserGroup {
                name: g.group_name().unwrap_or_default().to_string(),
                description: g.description().map(|s| s.to_string()),
                role_arn: g.role_arn().map(|s| s.to_string()),
                precedence: g.precedence().map(|p| p as u32),
                created_at: g.creation_date().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()),
            }
        }).collect())
    }

    async fn add_user_to_group(&self, username: &str, group_name: &str) -> CloudResult<()> {
        self.client.admin_add_user_to_group()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .group_name(group_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn remove_user_from_group(&self, username: &str, group_name: &str) -> CloudResult<()> {
        self.client.admin_remove_user_from_group()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .group_name(group_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_user_groups(&self, username: &str) -> CloudResult<Vec<UserGroup>> {
        let resp = self.client.admin_list_groups_for_user()
            .user_pool_id(self.get_user_pool_id()?)
            .username(username)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.groups().iter().map(|g| {
            UserGroup {
                name: g.group_name().unwrap_or_default().to_string(),
                description: g.description().map(|s| s.to_string()),
                role_arn: g.role_arn().map(|s| s.to_string()),
                precedence: g.precedence().map(|p| p as u32),
                created_at: g.creation_date().map(|d| chrono::DateTime::<chrono::Utc>::from_timestamp(d.secs(), 0).unwrap_or_default()),
            }
        }).collect())
    }

    async fn list_users_in_group(&self, group_name: &str) -> CloudResult<Vec<User>> {
        let resp = self.client.list_users_in_group()
            .user_pool_id(self.get_user_pool_id()?)
            .group_name(group_name)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(resp.users().iter().map(|u| self.map_user(u)).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit_spi::ProviderType;

    async fn create_test_context() -> Arc<CloudContext> {
        Arc::new(
            CloudContext::builder(ProviderType::Aws)
                .build()
                .await
                .unwrap(),
        )
    }

    #[tokio::test]
    async fn test_cognito_new() {
        let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let context = create_test_context().await;
        let _cognito = AwsIdentity::new(context, sdk_config);
    }
}

