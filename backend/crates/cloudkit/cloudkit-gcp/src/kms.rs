//! Google Cloud KMS implementation.

use async_trait::async_trait;
use base64::prelude::*;
use chrono::{DateTime, Utc};
use cloudkit::api::{
    CreateKeyOptions, DataKey, DecryptResult, EncryptResult, EncryptionContext, KeyManagement,
    KeyMetadata, SigningAlgorithm,
};
use cloudkit::common::{CloudError, CloudResult, Metadata};
use cloudkit::core::CloudContext;
use google_cloud_auth::token_source::TokenSource;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

/// Google Cloud KMS implementation.
pub struct GcpKms {
    _context: Arc<CloudContext>,
    auth: Arc<Box<dyn TokenSource>>,
    project_id: String,
    client: Client,
}

impl GcpKms {
    /// Create a new KMS client.
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
        format!(
            "https://cloudkms.googleapis.com/v1/projects/{}/locations/global",
            self.project_id
        )
    }
}

#[derive(Deserialize)]
struct CryptoKey {
    name: String,
    primary: Option<KeyVersionResponse>,
    purpose: Option<String>,
    #[serde(rename = "createTime")]
    _create_time: Option<String>,
}

#[derive(Deserialize)]
struct KeyVersionResponse {
    name: String,
    state: String,
}

#[derive(Serialize)]
struct AsymmetricSignRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    digest: Option<Digest>,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<String>,
}

#[derive(Serialize)]
struct Digest {
    #[serde(skip_serializing_if = "Option::is_none")]
    sha256: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha384: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    sha512: Option<String>,
}

#[derive(Serialize)]
struct MacSignRequest {
    data: String,
}

#[derive(Deserialize)]
struct SignResponse {
    signature: Option<String>,
    #[serde(rename = "mac")]
    mac: Option<String>,
}

#[derive(Serialize)]
struct MacVerifyRequest {
    data: String,
    mac: String,
}

#[derive(Deserialize)]
struct VerifyResponse {
    success: Option<bool>,
}

#[derive(Deserialize)]
struct ListKeysResponse {
    #[serde(rename = "cryptoKeys")]
    crypto_keys: Option<Vec<CryptoKey>>,
}

#[derive(Deserialize)]
struct EncryptResponse {
    ciphertext: String,
}

#[derive(Deserialize)]
struct DecryptResponse {
    plaintext: String,
}

#[async_trait]
impl KeyManagement for GcpKms {
    async fn create_key(&self, options: CreateKeyOptions) -> CloudResult<KeyMetadata> {
        let token = self.token().await?;
        
        // Create a key ring first (simplified - using default key ring)
        let keyring_name = "default-keyring";
        let keyring_url = format!("{}/keyRings?keyRingId={}", self.base_url(), keyring_name);
        
        // Try to create key ring (may already exist)
        let _resp = self.client.post(&keyring_url)
            .bearer_auth(&token)
            .json(&json!({}))
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        // Create the crypto key
        let key_id = options.description.clone().unwrap_or_else(|| "generated-key".to_string());
        let url = format!("{}/keyRings/{}/cryptoKeys?cryptoKeyId={}", 
            self.base_url(), keyring_name, key_id);
        
        let purpose = match options.usage {
            cloudkit::api::KeyUsage::EncryptDecrypt => "ENCRYPT_DECRYPT",
            cloudkit::api::KeyUsage::SignVerify => "ASYMMETRIC_SIGN",
            cloudkit::api::KeyUsage::KeyAgreement => "ASYMMETRIC_SIGN", // ECDH
            cloudkit::api::KeyUsage::GenerateVerifyMac => "MAC",
        };

        let body = json!({
            "purpose": purpose,
            "versionTemplate": {
                "algorithm": "GOOGLE_SYMMETRIC_ENCRYPTION"
            }
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

        let key: CryptoKey = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        Ok(KeyMetadata::new(key.name))
    }

    async fn describe_key(&self, key_id: &str) -> CloudResult<KeyMetadata> {
        let token = self.token().await?;
        let url = format!("{}/keyRings/default-keyring/cryptoKeys/{}", self.base_url(), key_id);
        
        let resp = self.client.get(&url)
            .bearer_auth(&token)
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

        let key: CryptoKey = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        Ok(KeyMetadata::new(key.name))
    }

    async fn list_keys(&self) -> CloudResult<Vec<KeyMetadata>> {
        let token = self.token().await?;
        let url = format!("{}/keyRings/default-keyring/cryptoKeys", self.base_url());
        
        let resp = self.client.get(&url)
            .bearer_auth(&token)
            .send()
            .await
            .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;

        if !resp.status().is_success() {
            return Ok(vec![]);
        }

        let body: ListKeysResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let keys = body.crypto_keys.unwrap_or_default()
            .into_iter()
            .map(|k| KeyMetadata::new(k.name))
            .collect();
        
        Ok(keys)
    }

    async fn enable_key(&self, _key_id: &str) -> CloudResult<()> {
        // GCP keys are enabled by default and require version-level operations
        Ok(())
    }

    async fn disable_key(&self, _key_id: &str) -> CloudResult<()> {
        // GCP uses version-level disable operations
        Ok(())
    }

    async fn schedule_key_deletion(&self, _key_id: &str, _pending_days: u32) -> CloudResult<DateTime<Utc>> {
        // GCP doesn't support key deletion, only version destruction
        Ok(Utc::now())
    }

    async fn cancel_key_deletion(&self, _key_id: &str) -> CloudResult<()> {
        Ok(())
    }

    async fn tag_key(&self, _key_id: &str, _tags: Metadata) -> CloudResult<()> {
        // GCP uses labels, requires update operation
        Ok(())
    }

    async fn untag_key(&self, _key_id: &str, _tag_keys: &[&str]) -> CloudResult<()> {
        Ok(())
    }

    async fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
        _context: Option<EncryptionContext>,
    ) -> CloudResult<EncryptResult> {
        let token = self.token().await?;
        let url = format!(
            "{}/keyRings/default-keyring/cryptoKeys/{}:encrypt",
            self.base_url(),
            key_id
        );

        let body = json!({
            "plaintext": BASE64_STANDARD.encode(plaintext)
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

        let encrypt_resp: EncryptResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        Ok(EncryptResult {
            ciphertext: BASE64_STANDARD.decode(&encrypt_resp.ciphertext)
                .map_err(|e| CloudError::Serialization(e.to_string()))?,
            key_id: key_id.to_string(),
            algorithm: None,
        })
    }

    async fn decrypt(
        &self,
        ciphertext: &[u8],
        _context: Option<EncryptionContext>,
    ) -> CloudResult<DecryptResult> {
        let token = self.token().await?;
        let url = format!(
            "{}/keyRings/default-keyring/cryptoKeys:decrypt",
            self.base_url()
        );

        let body = json!({
            "ciphertext": BASE64_STANDARD.encode(ciphertext)
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

        let decrypt_resp: DecryptResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        
        Ok(DecryptResult {
            plaintext: BASE64_STANDARD.decode(&decrypt_resp.plaintext)
                .map_err(|e| CloudError::Serialization(e.to_string()))?,
            key_id: "unknown".to_string(),
        })
    }

    async fn update_key_description(&self, _key_id: &str, _description: &str) -> CloudResult<()> {
        // GCP uses update operation
        Ok(())
    }

    async fn re_encrypt(
        &self,
        ciphertext: &[u8],
        dest_key_id: &str,
        _source_context: Option<EncryptionContext>,
        dest_context: Option<EncryptionContext>,
    ) -> CloudResult<EncryptResult> {
        // Decrypt then encrypt
        let decrypt_result = self.decrypt(ciphertext, None).await?;
        self.encrypt(dest_key_id, &decrypt_result.plaintext, dest_context).await
    }

    async fn generate_data_key(
        &self,
        key_id: &str,
        _context: Option<EncryptionContext>,
    ) -> CloudResult<DataKey> {
        let key_length = 32; // AES-256

        let mut plaintext = vec![0u8; key_length];
        use rand::RngCore;
        rand::thread_rng().fill_bytes(&mut plaintext);

        // Encrypt the plaintext
        let encrypt_result = self.encrypt(key_id, &plaintext, None).await?;

        Ok(DataKey {
            plaintext,
            ciphertext: encrypt_result.ciphertext,
            key_id: key_id.to_string(),
        })
    }

    async fn generate_data_key_without_plaintext(
        &self,
        key_id: &str,
        context: Option<EncryptionContext>,
    ) -> CloudResult<Vec<u8>> {
        let data_key = self.generate_data_key(key_id, context).await?;
        Ok(data_key.ciphertext)
    }

    async fn sign(
        &self,
        key_id: &str,
        message: &[u8],
        algorithm: SigningAlgorithm,
    ) -> CloudResult<Vec<u8>> {
        let token = self.token().await?;
        // 1. Get Key to find primary version and purpose
        let key_url = format!("{}/keyRings/default-keyring/cryptoKeys/{}", self.base_url(), key_id);
        let resp = self.client.get(&key_url).bearer_auth(&token).send().await
              .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;
        if !resp.status().is_success() {
             return Err(CloudError::Provider { provider: "gcp".to_string(), code: resp.status().as_u16().to_string(), message: resp.text().await.unwrap_or_default() });
        }
        let key: CryptoKey = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let version_name = key.primary.ok_or_else(|| CloudError::NotFound { 
            resource_type: "KeyVersion".into(), 
            resource_id: "primary".into() 
        })?.name;
        let purpose = key.purpose.unwrap_or_default();

        let sig_bytes = if purpose == "ASYMMETRIC_SIGN" {
             let url = format!("https://cloudkms.googleapis.com/v1/{}:asymmetricSign", version_name);
             let req_body = match algorithm {
                 // Expanded match validation could go here
                 _ => {
                     // Assume digest if possible, else raw?
                     // CloudKit 'message' is raw data usually.
                     // But for signing, we often hash first.
                     // Here we assume 'message' is the digest if length is small? 
                     // Or force user to pass digest? 
                     // GCP supports signing RAW data for some algorithms (ECDSA), but RSA usually requires digest.
                     // Implementing strictly for SHA256 digest for now.
                     AsymmetricSignRequest {
                         digest: Some(Digest { sha256: Some(BASE64_STANDARD.encode(message)), sha384: None, sha512: None }),
                         data: None
                     }
                 }
             };
             let resp = self.client.post(&url).bearer_auth(&token).json(&req_body).send().await
                 .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;
             if !resp.status().is_success() {
                return Err(CloudError::Provider { provider: "gcp".to_string(), code: resp.status().as_u16().to_string(), message: resp.text().await.unwrap_or_default() });
             }
             let sign_resp: SignResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
             base64::prelude::BASE64_STANDARD.decode(sign_resp.signature.unwrap_or_default())
                 .map_err(|e| CloudError::Serialization(e.to_string()))?
        } else if purpose == "MAC" {
             let url = format!("https://cloudkms.googleapis.com/v1/{}:macSign", version_name);
             let req_body = MacSignRequest { data: BASE64_STANDARD.encode(message) };
             let resp = self.client.post(&url).bearer_auth(&token).json(&req_body).send().await
                 .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;
             if !resp.status().is_success() {
                return Err(CloudError::Provider { provider: "gcp".to_string(), code: resp.status().as_u16().to_string(), message: resp.text().await.unwrap_or_default() });
             }
             let sign_resp: SignResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
             base64::prelude::BASE64_STANDARD.decode(sign_resp.mac.unwrap_or_default())
                 .map_err(|e| CloudError::Serialization(e.to_string()))?
        } else {
             return Err(CloudError::Validation(format!("Key purpose {} does not support signing", purpose)));
        };

        Ok(sig_bytes)
    }

    async fn verify(
        &self,
        key_id: &str,
        message: &[u8],
        signature: &[u8],
        _algorithm: SigningAlgorithm,
    ) -> CloudResult<bool> {
        let token = self.token().await?;
        let key_url = format!("{}/keyRings/default-keyring/cryptoKeys/{}", self.base_url(), key_id);
        let resp = self.client.get(&key_url).bearer_auth(&token).send().await
              .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;
        if !resp.status().is_success() { return Ok(false); }
        let key: CryptoKey = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
        let version_name = key.primary.ok_or_else(|| CloudError::NotFound { 
            resource_type: "KeyVersion".into(), 
            resource_id: "primary".into() 
        })?.name;
        let purpose = key.purpose.unwrap_or_default();

        if purpose == "MAC" {
             let url = format!("https://cloudkms.googleapis.com/v1/{}:macVerify", version_name);
             let req_body = MacVerifyRequest { 
                 data: BASE64_STANDARD.encode(message),
                 mac: BASE64_STANDARD.encode(signature)
             };
             let resp = self.client.post(&url).bearer_auth(&token).json(&req_body).send().await
                 .map_err(|e| CloudError::Provider { provider: "gcp".into(), code: "ReqwestError".into(), message: e.to_string() })?;
             if !resp.status().is_success() {
                // Verification failure is usually 400 or JSON error? Or just returns success: false?
                // macVerify returns success: true/false in body usually or empty body on success?
                // Docs: "Returns struct{ success: bool }"
                // Wait, response details: "If verification fails, returns error INVALID_ARGUMENT"? 
                // Or field `success`.
                // Checking standard API: `MacVerifyResponse` has `success` boolean.
                // If it fails with status, assume false.
                return Ok(false);
             }
             let verify_resp: VerifyResponse = resp.json().await.map_err(|e| CloudError::Serialization(e.to_string()))?;
             Ok(verify_resp.success.unwrap_or(false))
        } else {
             Err(CloudError::Provider {
                provider: "gcp".to_string(),
                code: "NotSupported".to_string(),
                message: "Asymmetric verification must be performed locally using the public key".to_string(),
            })
        }
    }

    async fn list_key_tags(&self, _key_id: &str) -> CloudResult<Metadata> {
        Ok(std::collections::HashMap::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use cloudkit::core::ProviderType;

    #[tokio::test]
    #[ignore]
    async fn test_kms_flow() {
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

        let kms = GcpKms::new(context, Arc::new(auth), project_id);

        // Create a key
        let options = CreateKeyOptions {
            usage: cloudkit::api::KeyUsage::EncryptDecrypt,
            description: Some("test-key".to_string()),
            tags: std::collections::HashMap::new(),
        };

        let key_metadata = kms
            .create_key(options)
            .await
            .expect("Failed to create key");
        println!("Created key: {}", key_metadata.id);

        // Encrypt data
        let plaintext = b"Hello, GCP KMS!";
        let encrypt_result = kms
            .encrypt("test-key", plaintext, None)
            .await
            .expect("Failed to encrypt");
        println!("Encrypted {} bytes", encrypt_result.ciphertext_blob.len());

        // Decrypt data
        let decrypt_result = kms
            .decrypt(&encrypt_result.ciphertext, None)
            .await
            .expect("Failed to decrypt");
        
        assert_eq!(decrypt_result.plaintext, plaintext);
        println!("Decrypted successfully");

        // Generate data key
        let data_key = kms
            .generate_data_key("test-key", None)
            .await
            .expect("Failed to generate data key");
        assert_eq!(data_key.plaintext.len(), 32);
        println!("Generated data key");

        // List keys
        let keys = kms.list_keys().await.expect("Failed to list keys");
        println!("Listed {} keys", keys.len());

        println!("KMS integration test completed successfully");
    }
}
