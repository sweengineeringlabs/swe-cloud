use async_trait::async_trait;
use chrono::{DateTime, Utc};
use cloudkit_api::{
    CreateKeyOptions, DataKey, DecryptResult, EncryptResult, EncryptionContext,
    KeyManagement, KeyMetadata, SigningAlgorithm, KeySpec, KeyUsage, KeyState,
};
use cloudkit_spi::{CloudError, CloudResult, Metadata};
use cloudkit_spi::CloudContext;
use std::sync::Arc;

/// AWS KMS implementation.
pub struct AwsKms {
    _context: Arc<CloudContext>,
    client: aws_sdk_kms::Client,
}

impl AwsKms {
    /// Create a new KMS client.
    pub fn new(context: Arc<CloudContext>, sdk_config: aws_config::SdkConfig) -> Self {
        let client = aws_sdk_kms::Client::new(&sdk_config);
        Self { _context: context, client }
    }
}

#[async_trait]
impl KeyManagement for AwsKms {
    async fn create_key(&self, options: CreateKeyOptions) -> CloudResult<KeyMetadata> {
        let mut req = self.client.create_key();
        
        if let Some(desc) = options.description {
            req = req.description(desc);
        }
        
        req = req.set_key_usage(Some(match options.usage {
            KeyUsage::EncryptDecrypt => aws_sdk_kms::types::KeyUsageType::EncryptDecrypt,
            KeyUsage::SignVerify => aws_sdk_kms::types::KeyUsageType::SignVerify,
            KeyUsage::GenerateVerifyMac => aws_sdk_kms::types::KeyUsageType::GenerateVerifyMac,
            _ => aws_sdk_kms::types::KeyUsageType::EncryptDecrypt,
        }));
        
        req = req.set_key_spec(Some(match options.key_spec {
            KeySpec::SymmetricDefault => aws_sdk_kms::types::KeySpec::SymmetricDefault,
            KeySpec::Rsa2048 => aws_sdk_kms::types::KeySpec::Rsa2048,
            KeySpec::Rsa3072 => aws_sdk_kms::types::KeySpec::Rsa3072,
            KeySpec::Rsa4096 => aws_sdk_kms::types::KeySpec::Rsa4096,
            KeySpec::EccNistP256 => aws_sdk_kms::types::KeySpec::EccNistP256,
            KeySpec::EccNistP384 => aws_sdk_kms::types::KeySpec::EccNistP384,
            KeySpec::EccNistP521 => aws_sdk_kms::types::KeySpec::EccNistP521,
            KeySpec::EccSecgP256K1 => aws_sdk_kms::types::KeySpec::EccSecgP256K1,
            KeySpec::Hmac256 => aws_sdk_kms::types::KeySpec::Hmac256,
            KeySpec::Hmac384 => aws_sdk_kms::types::KeySpec::Hmac384,
            KeySpec::Hmac512 => aws_sdk_kms::types::KeySpec::Hmac512,
        }));
        
        if options.multi_region {
            req = req.multi_region(true);
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        let metadata = resp.key_metadata().unwrap();
        Ok(KeyMetadata {
            key_id: metadata.key_id().to_string(),
            description: metadata.description().map(|s| s.to_string()),
            enabled: metadata.enabled(),
            usage: options.usage,
            key_spec: options.key_spec,
            multi_region: metadata.multi_region().unwrap_or(false),
            arn: Some(metadata.arn().unwrap_or_default().to_string()),
            created_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(metadata.creation_date().map(|d| d.secs()).unwrap_or(0), 0).unwrap_or_default()),
            state: KeyState::Enabled,
            deletion_date: None,
            tags: Metadata::new(),
        })
    }

    async fn describe_key(&self, key_id: &str) -> CloudResult<KeyMetadata> {
        let resp = self.client.describe_key()
            .key_id(key_id)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let metadata = resp.key_metadata().unwrap();
        Ok(KeyMetadata {
            key_id: metadata.key_id().to_string(),
            description: metadata.description().map(|s| s.to_string()),
            enabled: metadata.enabled(),
            usage: match metadata.key_usage().unwrap_or(&aws_sdk_kms::types::KeyUsageType::EncryptDecrypt) {
                aws_sdk_kms::types::KeyUsageType::SignVerify => KeyUsage::SignVerify,
                _ => KeyUsage::EncryptDecrypt,
            },
            key_spec: KeySpec::SymmetricDefault, // Could map properly if needed
            multi_region: metadata.multi_region().unwrap_or(false),
            arn: Some(metadata.arn().unwrap_or_default().to_string()),
            created_at: Some(chrono::DateTime::<chrono::Utc>::from_timestamp(metadata.creation_date().map(|d| d.secs()).unwrap_or(0), 0).unwrap_or_default()),
            state: KeyState::Enabled,
            deletion_date: None,
            tags: Metadata::new(),
        })
    }

    async fn list_keys(&self) -> CloudResult<Vec<KeyMetadata>> {
        let resp = self.client.list_keys()
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let mut results = Vec::new();
        for key in resp.keys() {
            if let Some(id) = key.key_id() {
                if let Ok(meta) = self.describe_key(id).await {
                    results.push(meta);
                }
            }
        }
        Ok(results)
    }

    async fn enable_key(&self, key_id: &str) -> CloudResult<()> {
        self.client.enable_key()
            .key_id(key_id)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn disable_key(&self, key_id: &str) -> CloudResult<()> {
        self.client.disable_key()
            .key_id(key_id)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn schedule_key_deletion(
        &self,
        key_id: &str,
        pending_window_days: u32,
    ) -> CloudResult<DateTime<Utc>> {
        let resp = self.client.schedule_key_deletion()
            .key_id(key_id)
            .pending_window_in_days(pending_window_days as i32)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        Ok(chrono::DateTime::<chrono::Utc>::from_timestamp(resp.deletion_date().map(|d| d.secs()).unwrap_or(0), 0).unwrap_or_default())
    }

    async fn cancel_key_deletion(&self, key_id: &str) -> CloudResult<()> {
        self.client.cancel_key_deletion()
            .key_id(key_id)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn update_key_description(&self, key_id: &str, description: &str) -> CloudResult<()> {
        self.client.update_key_description()
            .key_id(key_id)
            .description(description)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn encrypt(
        &self,
        key_id: &str,
        plaintext: &[u8],
        context: Option<EncryptionContext>,
    ) -> CloudResult<EncryptResult> {
        let mut req = self.client.encrypt()
            .key_id(key_id)
            .plaintext(aws_sdk_kms::primitives::Blob::new(plaintext));
            
        if let Some(ctx) = context {
            for (k, v) in ctx {
                req = req.encryption_context(k, v);
            }
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(EncryptResult {
            ciphertext: resp.ciphertext_blob().unwrap().clone().into_inner(),
            key_id: resp.key_id().unwrap_or_default().to_string(),
            algorithm: resp.encryption_algorithm().map(|a| a.as_str().to_string()),
        })
    }

    async fn decrypt(
        &self,
        ciphertext: &[u8],
        context: Option<EncryptionContext>,
    ) -> CloudResult<DecryptResult> {
        let mut req = self.client.decrypt()
            .ciphertext_blob(aws_sdk_kms::primitives::Blob::new(ciphertext));
            
        if let Some(ctx) = context {
            for (k, v) in ctx {
                req = req.encryption_context(k, v);
            }
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(DecryptResult {
            plaintext: resp.plaintext().unwrap().clone().into_inner(),
            key_id: resp.key_id().unwrap_or_default().to_string(),
        })
    }

    async fn re_encrypt(
        &self,
        ciphertext: &[u8],
        dest_key_id: &str,
        source_context: Option<EncryptionContext>,
        dest_context: Option<EncryptionContext>,
    ) -> CloudResult<EncryptResult> {
        let mut req = self.client.re_encrypt()
            .ciphertext_blob(aws_sdk_kms::primitives::Blob::new(ciphertext))
            .destination_key_id(dest_key_id);
            
        if let Some(ctx) = source_context {
            for (k, v) in ctx {
                req = req.source_encryption_context(k, v);
            }
        }
        
        if let Some(ctx) = dest_context {
            for (k, v) in ctx {
                req = req.destination_encryption_context(k, v);
            }
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(EncryptResult {
            ciphertext: resp.ciphertext_blob().unwrap().clone().into_inner(),
            key_id: resp.key_id().unwrap_or_default().to_string(),
            algorithm: resp.source_encryption_algorithm().map(|a| a.as_str().to_string()),
        })
    }

    async fn generate_data_key(
        &self,
        key_id: &str,
        context: Option<EncryptionContext>,
    ) -> CloudResult<DataKey> {
        let mut req = self.client.generate_data_key()
            .key_id(key_id)
            .key_spec(aws_sdk_kms::types::DataKeySpec::Aes256);
            
        if let Some(ctx) = context {
            for (k, v) in ctx {
                req = req.encryption_context(k, v);
            }
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(DataKey {
            plaintext: resp.plaintext().unwrap().clone().into_inner(),
            ciphertext: resp.ciphertext_blob().unwrap().clone().into_inner(),
            key_id: resp.key_id().unwrap_or_default().to_string(),
        })
    }

    async fn generate_data_key_without_plaintext(
        &self,
        key_id: &str,
        context: Option<EncryptionContext>,
    ) -> CloudResult<Vec<u8>> {
        let mut req = self.client.generate_data_key_without_plaintext()
            .key_id(key_id)
            .key_spec(aws_sdk_kms::types::DataKeySpec::Aes256);
            
        if let Some(ctx) = context {
            for (k, v) in ctx {
                req = req.encryption_context(k, v);
            }
        }
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(resp.ciphertext_blob().unwrap().clone().into_inner())
    }

    async fn sign(
        &self,
        key_id: &str,
        message: &[u8],
        algorithm: SigningAlgorithm,
    ) -> CloudResult<Vec<u8>> {
        let mut req = self.client.sign()
            .key_id(key_id)
            .message(aws_sdk_kms::primitives::Blob::new(message));
            
        req = req.set_signing_algorithm(Some(match algorithm {
            SigningAlgorithm::RsassaPssSha256 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPssSha256,
            SigningAlgorithm::RsassaPssSha384 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPssSha384,
            SigningAlgorithm::RsassaPssSha512 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPssSha512,
            SigningAlgorithm::RsassaPkcs1V15Sha256 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPkcs1V15Sha256,
            SigningAlgorithm::RsassaPkcs1V15Sha384 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPkcs1V15Sha384,
            SigningAlgorithm::RsassaPkcs1V15Sha512 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPkcs1V15Sha512,
            SigningAlgorithm::EcdsaSha256 => aws_sdk_kms::types::SigningAlgorithmSpec::EcdsaSha256,
            SigningAlgorithm::EcdsaSha384 => aws_sdk_kms::types::SigningAlgorithmSpec::EcdsaSha384,
            SigningAlgorithm::EcdsaSha512 => aws_sdk_kms::types::SigningAlgorithmSpec::EcdsaSha512,
        }));
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(resp.signature().unwrap().clone().into_inner())
    }

    async fn verify(
        &self,
        key_id: &str,
        message: &[u8],
        signature: &[u8],
        algorithm: SigningAlgorithm,
    ) -> CloudResult<bool> {
        let mut req = self.client.verify()
            .key_id(key_id)
            .message(aws_sdk_kms::primitives::Blob::new(message))
            .signature(aws_sdk_kms::primitives::Blob::new(signature));
            
        req = req.set_signing_algorithm(Some(match algorithm {
            SigningAlgorithm::RsassaPssSha256 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPssSha256,
            SigningAlgorithm::RsassaPssSha384 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPssSha384,
            SigningAlgorithm::RsassaPssSha512 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPssSha512,
            SigningAlgorithm::RsassaPkcs1V15Sha256 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPkcs1V15Sha256,
            SigningAlgorithm::RsassaPkcs1V15Sha384 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPkcs1V15Sha384,
            SigningAlgorithm::RsassaPkcs1V15Sha512 => aws_sdk_kms::types::SigningAlgorithmSpec::RsassaPkcs1V15Sha512,
            SigningAlgorithm::EcdsaSha256 => aws_sdk_kms::types::SigningAlgorithmSpec::EcdsaSha256,
            SigningAlgorithm::EcdsaSha384 => aws_sdk_kms::types::SigningAlgorithmSpec::EcdsaSha384,
            SigningAlgorithm::EcdsaSha512 => aws_sdk_kms::types::SigningAlgorithmSpec::EcdsaSha512,
        }));
        
        let resp = req.send().await.map_err(|e| CloudError::ServiceError(e.to_string()))?;
        
        Ok(resp.signature_valid())
    }

    async fn tag_key(&self, key_id: &str, tags: Metadata) -> CloudResult<()> {
        let aws_tags: Vec<aws_sdk_kms::types::Tag> = tags.into_iter()
            .map(|(k, v)| aws_sdk_kms::types::Tag::builder().tag_key(k).tag_value(v).build().unwrap()) // Added unwrap
            .collect();
            
        self.client.tag_resource()
            .key_id(key_id)
            .set_tags(Some(aws_tags))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn untag_key(&self, key_id: &str, tag_keys: &[&str]) -> CloudResult<()> {
        let keys: Vec<String> = tag_keys.iter().map(|s| s.to_string()).collect();
        
        self.client.untag_resource()
            .key_id(key_id)
            .set_tag_keys(Some(keys))
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
        Ok(())
    }

    async fn list_key_tags(&self, key_id: &str) -> CloudResult<Metadata> {
        let resp = self.client.list_resource_tags()
            .key_id(key_id)
            .send()
            .await
            .map_err(|e| CloudError::ServiceError(e.to_string()))?;
            
        let mut tags = Metadata::new();
        for tag in resp.tags() {
            tags.insert(tag.tag_key().to_string(), tag.tag_value().to_string());
        }
        Ok(tags)
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
    async fn test_kms_new() {
        let sdk_config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
        let context = create_test_context().await;
        let _kms = AwsKms::new(context, sdk_config);
    }
}

