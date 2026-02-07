//! # Identity API
//!
//! Cross-cloud identity and authentication operations.
//!
//! ## Implementations
//!
//! - **AWS**: Cognito User Pools
//! - **Azure**: Azure AD B2C
//! - **GCP**: Identity Platform / Firebase Auth

use async_trait::async_trait;
use cloudkit_spi::{CloudResult, Metadata};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// User account information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    /// User ID (sub).
    pub id: String,
    /// Username.
    pub username: String,
    /// Email address.
    pub email: Option<String>,
    /// Email verified.
    pub email_verified: bool,
    /// Phone number.
    pub phone_number: Option<String>,
    /// Phone verified.
    pub phone_verified: bool,
    /// Account enabled.
    pub enabled: bool,
    /// Account status.
    pub status: UserStatus,
    /// When created.
    pub created_at: Option<DateTime<Utc>>,
    /// When last modified.
    pub updated_at: Option<DateTime<Utc>>,
    /// Custom attributes.
    pub attributes: Metadata,
}

impl User {
    /// Create a new user.
    pub fn new(id: impl Into<String>, username: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            username: username.into(),
            email: None,
            email_verified: false,
            phone_number: None,
            phone_verified: false,
            enabled: true,
            status: UserStatus::Unconfirmed,
            created_at: None,
            updated_at: None,
            attributes: Metadata::new(),
        }
    }
}

/// User account status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum UserStatus {
    #[default]
    /// User is not yet confirmed.
    Unconfirmed,
    /// User is confirmed and can sign in.
    Confirmed,
    /// User is archived.
    Archived,
    /// User account is compromised.
    Compromised,
    /// Status is unknown.
    Unknown,
    /// Password reset is required.
    ResetRequired,
    /// User must change password upon next sign in.
    ForceChangePassword,
}

/// User group.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserGroup {
    /// Group name.
    pub name: String,
    /// Description.
    pub description: Option<String>,
    /// Role ARN (for AWS).
    pub role_arn: Option<String>,
    /// Precedence (lower = higher priority).
    pub precedence: Option<u32>,
    /// When created.
    pub created_at: Option<DateTime<Utc>>,
}

/// Authentication result.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResult {
    /// Access token.
    pub access_token: String,
    /// ID token.
    pub id_token: Option<String>,
    /// Refresh token.
    pub refresh_token: Option<String>,
    /// Token type (usually "Bearer").
    pub token_type: String,
    /// Expires in seconds.
    pub expires_in: u64,
}

/// Challenge from authentication flow.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthChallenge {
    /// Challenge name.
    pub challenge_name: ChallengeType,
    /// Session token for continuing the flow.
    pub session: String,
    /// Challenge parameters.
    pub parameters: Metadata,
}

/// Authentication challenge types.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChallengeType {
    /// SMS MFA challenge.
    SmsMfa,
    /// Software token (TOTP) MFA challenge.
    SoftwareTokenMfa,
    /// New password required challenge.
    NewPasswordRequired,
    /// MFA setup required.
    MfaSetup,
    /// Select MFA type challenge.
    SelectMfaType,
    /// Custom challenge.
    Custom(String),
}

/// Result of initiating auth - either tokens or a challenge.
#[derive(Debug, Clone)]
pub enum InitiateAuthResult {
    /// Authentication succeeded.
    Success(AuthResult),
    /// Challenge required.
    Challenge(AuthChallenge),
}

/// Options for creating a user.
#[derive(Debug, Clone, Default)]
pub struct CreateUserOptions {
    /// Temporary password.
    pub temporary_password: Option<String>,
    /// Send welcome email.
    pub send_email: bool,
    /// Force email verification.
    pub email_verified: bool,
    /// Custom attributes.
    pub attributes: Metadata,
}

/// Identity provider operations.
#[async_trait]
pub trait IdentityProvider: Send + Sync {
    // --- User Management ---

    /// Create a new user.
    async fn create_user(
        &self,
        username: &str,
        email: Option<&str>,
        options: CreateUserOptions,
    ) -> CloudResult<User>;

    /// Get user by username.
    async fn get_user(&self, username: &str) -> CloudResult<User>;

    /// Update user attributes.
    async fn update_user(&self, username: &str, attributes: Metadata) -> CloudResult<User>;

    /// Delete a user.
    async fn delete_user(&self, username: &str) -> CloudResult<()>;

    /// Enable a user.
    async fn enable_user(&self, username: &str) -> CloudResult<()>;

    /// Disable a user.
    async fn disable_user(&self, username: &str) -> CloudResult<()>;

    /// List users.
    async fn list_users(&self, limit: Option<u32>) -> CloudResult<Vec<User>>;

    /// Search users by attribute.
    async fn search_users(&self, filter: &str) -> CloudResult<Vec<User>>;

    // --- Authentication ---

    /// Initiate authentication with username/password.
    async fn initiate_auth(
        &self,
        username: &str,
        password: &str,
    ) -> CloudResult<InitiateAuthResult>;

    /// Respond to an auth challenge.
    async fn respond_to_challenge(
        &self,
        challenge_name: ChallengeType,
        session: &str,
        responses: Metadata,
    ) -> CloudResult<InitiateAuthResult>;

    /// Refresh tokens.
    async fn refresh_tokens(&self, refresh_token: &str) -> CloudResult<AuthResult>;

    /// Sign out (revoke tokens).
    async fn sign_out(&self, access_token: &str) -> CloudResult<()>;

    /// Initiate forgot password flow.
    async fn forgot_password(&self, username: &str) -> CloudResult<()>;

    /// Confirm forgot password with code.
    async fn confirm_forgot_password(
        &self,
        username: &str,
        code: &str,
        new_password: &str,
    ) -> CloudResult<()>;

    /// Change password (authenticated).
    async fn change_password(
        &self,
        access_token: &str,
        old_password: &str,
        new_password: &str,
    ) -> CloudResult<()>;

    // --- Groups ---

    /// Create a group.
    async fn create_group(&self, name: &str, description: Option<&str>) -> CloudResult<UserGroup>;

    /// Delete a group.
    async fn delete_group(&self, name: &str) -> CloudResult<()>;

    /// List groups.
    async fn list_groups(&self) -> CloudResult<Vec<UserGroup>>;

    /// Add user to group.
    async fn add_user_to_group(&self, username: &str, group_name: &str) -> CloudResult<()>;

    /// Remove user from group.
    async fn remove_user_from_group(&self, username: &str, group_name: &str) -> CloudResult<()>;

    /// List groups for a user.
    async fn list_user_groups(&self, username: &str) -> CloudResult<Vec<UserGroup>>;

    /// List users in a group.
    async fn list_users_in_group(&self, group_name: &str) -> CloudResult<Vec<User>>;
}

