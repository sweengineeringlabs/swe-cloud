//! # Encryption API
//!
//! Cross-cloud key management and encryption operations.
//!
//! ## Implementations
//!
//! - **AWS**: KMS
//! - **Azure**: Key Vault Keys
//! - **GCP**: Cloud KMS

use async_trait::async_trait;
use crate::common::{CloudResult, Metadata};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Encryption key metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyMetadata {
    /// Key ID.
    pub key_id: String,
    /// Key ARN or full resource path.
    pub arn: Option<String>,
    /// Description.
    pub description: Option<String>,
    /// Key state.
    pub state: KeyState,
    /// Key usage.
    pub usage: KeyUsage,
    /// Key spec (algorithm).
    pub key_spec: KeySpec,
    /// When created.
    pub created_at: Option<DateTime<Utc>>,
    /// When key is scheduled for deletion (if pending deletion).
    pub deletion_date: Option<DateTime<Utc>>,
    /// Is key enabled.
    pub enabled: bool,
    /// Is key a multi-region key.
    pub multi_region: bool,
    /// Tags.
    pub tags: Metadata,
}

impl KeyMetadata {
    /// Create new key metadata.
    pub fn new(key_id: impl Into<String>) -> Self {
        Self {
            key_id: key_id.into(),
            arn: None,
            description: None,
            state: KeyState::Enabled,
            usage: KeyUsage::EncryptDecrypt,
            key_spec: KeySpec::SymmetricDefault,
            created_at: None,
            deletion_date: None,
            enabled: true,
            multi_region: false,
            tags: Metadata::new(),
        }
    }
}

/// Key state.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyState {
    Creating,
    #[default]
    Enabled,
    Disabled,
    PendingDeletion,
    PendingImport,
    Unavailable,
}

/// Key usage type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeyUsage {
    /// Symmetric encryption/decryption.
    #[default]
    EncryptDecrypt,
    /// Asymmetric signing/verification.
    SignVerify,
    /// Key agreement (ECDH).
    KeyAgreement,
    /// Generate and verify HMACs.
    GenerateVerifyMac,
}

/// Key specification (algorithm).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum KeySpec {
    /// AES-256-GCM (default for symmetric).
    #[default]
    SymmetricDefault,
    /// RSA 2048.
    Rsa2048,
    /// RSA 3072.
    Rsa3072,
    /// RSA 4096.
    Rsa4096,
    /// Elliptic curve P-256.
    EccNistP256,
    /// Elliptic curve P-384.
    EccNistP384,
    /// Elliptic curve P-521.
    EccNistP521,
    /// HMAC-256.
    Hmac256,
    /// HMAC-384.
    Hmac384,
    /// HMAC-512.
    Hmac512,
}

/// Options for creating a key.
#[derive(Debug, Clone, Default)]
pub struct CreateKeyOptions {
    /// Description.
    pub description: Option<String>,
    /// Key usage.
    pub usage: KeyUsage,
    /// Key spec.
    pub key_spec: KeySpec,
    /// Is multi-region.
    pub multi_region: bool,
    /// Tags.
    pub tags: Metadata,
}

/// Encryption context (AAD).
pub type EncryptionContext = Metadata;

/// Result of encryption.
#[derive(Debug, Clone)]
pub struct EncryptResult {
    /// Ciphertext blob.
    pub ciphertext: Vec<u8>,
    /// Key ID used for encryption.
    pub key_id: String,
    /// Encryption algorithm used.
    pub algorithm: Option<String>,
}

/// Result of decryption.
#[derive(Debug, Clone)]
pub struct DecryptResult {
    /// Plaintext data.
    pub plaintext: Vec<u8>,
    /// Key ID used for decryption.
    pub key_id: String,
}

/// Data key for envelope encryption.
#[derive(Debug, Clone)]
pub struct DataKey {
    /// Plaintext key (use immediately, don't store).
    pub plaintext: Vec<u8>,
    /// Encrypted key (safe to store).
    pub ciphertext: Vec<u8>,
    /// Key ID of the CMK used to generate this key.
    pub key_id: String,
}

/// Signing algorithm.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SigningAlgorithm {
    #[default]
    RsassaPssSha256,
    RsassaPssSha384,
    RsassaPssSha512,
    RsassaPkcs1V15Sha256,
    RsassaPkcs1V15Sha384,
    RsassaPkcs1V15Sha512,
    EcdsaSha256,
    EcdsaSha384,
    EcdsaSha512,
}

/// Key management operations.
#[async_trait]
pub trait KeyManagement: Send + Sync {
    // --- Key Lifecycle ---

    /// Create a new customer master key.
    async fn create_key(&self, options: CreateKeyOptions) -> CloudResult<KeyMetadata>;

    /// Get key metadata.
    async fn describe_key(&self, key_id: &str) -> CloudResult<KeyMetadata>;

    /// List all keys.
    async fn list_keys(&self) -> CloudResult<Vec<KeyMetadata>>;

    /// Enable a key.
    async fn enable_key(&self, key_id: &str) -> CloudResult<()>;

    /// Disable a key.
    async fn disable_key(&self, key_id: &str) -> CloudResult<()>;

    /// Schedule key deletion.
    async fn schedule_key_deletion(
        &self,
        key_id: &str,
        pending_window_days: u32,
    ) -> CloudResult<DateTime<Utc>>;

    /// Cancel key deletion.
    async fn cancel_key_deletion(&self, key_id: &str) -> CloudResult<()>;

    /// Update key description.
    async fn update_key_description(&self, key_id: &str, description: &str) -> CloudResult<()>;

    // --- Encryption/Decryption ---

    /// Encrypt data.
    async fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
        context: Option<EncryptionContext>,
    ) -> CloudResult<EncryptResult>;

    /// Decrypt data.
    async fn decrypt(
        &self,
        ciphertext: &[u8],
        context: Option<EncryptionContext>,
    ) -> CloudResult<DecryptResult>;

    /// Re-encrypt data under a new key.
    async fn re_encrypt(
        &self,
        ciphertext: &[u8],
        dest_key_id: &str,
        source_context: Option<EncryptionContext>,
        dest_context: Option<EncryptionContext>,
    ) -> CloudResult<EncryptResult>;

    // --- Data Keys (Envelope Encryption) ---

    /// Generate a data key for client-side encryption.
    async fn generate_data_key(
        &self,
        key_id: &str,
        context: Option<EncryptionContext>,
    ) -> CloudResult<DataKey>;

    /// Generate a data key without plaintext (for re-encryption scenarios).
    async fn generate_data_key_without_plaintext(
        &self,
        key_id: &str,
        context: Option<EncryptionContext>,
    ) -> CloudResult<Vec<u8>>;

    // --- Digital Signatures ---

    /// Sign a message digest.
    async fn sign(
        &self,
        key_id: &str,
        message: &[u8],
        algorithm: SigningAlgorithm,
    ) -> CloudResult<Vec<u8>>;

    /// Verify a signature.
    async fn verify(
        &self,
        key_id: &str,
        message: &[u8],
        signature: &[u8],
        algorithm: SigningAlgorithm,
    ) -> CloudResult<bool>;

    // --- Tagging ---

    /// Tag a key.
    async fn tag_key(&self, key_id: &str, tags: Metadata) -> CloudResult<()>;

    /// Remove tags from a key.
    async fn untag_key(&self, key_id: &str, tag_keys: &[&str]) -> CloudResult<()>;

    /// List tags for a key.
    async fn list_key_tags(&self, key_id: &str) -> CloudResult<Metadata>;
}
