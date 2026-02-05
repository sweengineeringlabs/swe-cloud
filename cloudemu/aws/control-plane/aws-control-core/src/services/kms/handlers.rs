use crate::Emulator;
use crate::error::EmulatorError;
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::{json, Value};
use std::sync::Arc;
use tracing::info;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use hmac::{Hmac, Mac};
use sha2::Sha256;

// Type alias
type HmacSha256 = Hmac<Sha256>;

pub async fn handle_request(
    State(emulator): State<Arc<Emulator>>,
    headers: HeaderMap,
    Json(body): Json<Value>,
) -> Response {
    let target = headers
        .get("x-amz-target")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("");
    
    info!("KMS: {}", target);
    let action = target.split('.').next_back().unwrap_or(target);

    let result = match action {
        "CreateKey" => create_key(&emulator, body).await,
        "ListKeys" => list_keys(&emulator).await,
        "DescribeKey" => describe_key(&emulator, body).await,
        "EnableKey" => enable_key(&emulator, body).await,
        "DisableKey" => disable_key(&emulator, body).await,
        "Encrypt" => encrypt(&emulator, body).await,
        "Decrypt" => decrypt(&emulator, body).await,
        "Sign" => sign(&emulator, body).await,
        "Verify" => verify(&emulator, body).await,
        "ScheduleKeyDeletion" => schedule_key_deletion(&emulator, body).await,
        "CancelKeyDeletion" => cancel_key_deletion(&emulator, body).await,
        _ => Err(EmulatorError::InvalidRequest(format!("Unknown or unsupported target: {}", target))),
    };

    match result {
        Ok(json_val) => Json::<Value>(json_val).into_response(),
        Err(e) => {
            let code = e.code();
            let msg = e.message();
            let status = e.status_code();
            let json_err = json!({
                "__type": code,
                "message": msg
            });
            (status, Json::<Value>(json_err)).into_response()
        }
    }
}

async fn create_key(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let desc = body["Description"].as_str();
    let usage = body["KeyUsage"].as_str().unwrap_or("ENCRYPT_DECRYPT");
    let tags = body["Tags"].to_string();

    let key = emulator.storage.create_key(
        desc, 
        usage, 
        Some(&tags),
        &emulator.config.account_id, 
        &emulator.config.region
    )?;

    Ok(json!({
        "KeyMetadata": {
            "KeyId": key.id,
            "Arn": key.arn,
            "Description": key.description,
            "KeyUsage": key.key_usage,
            "KeyState": key.key_state,
            "CreationDate": 1234567890.0,
            "Enabled": true,
            "AWSAccountId": emulator.config.account_id
        }
    }))
}

async fn list_keys(emulator: &Emulator) -> Result<Value, EmulatorError> {
    let keys = emulator.storage.list_keys()?;
    let key_list: Vec<Value> = keys.into_iter().map(|k| {
        json!({
            "KeyId": k.id,
            "KeyArn": k.arn
        })
    }).collect();

    Ok(json!({
        "Keys": key_list,
        "Truncated": false
    }))
}

async fn describe_key(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
    let key = emulator.storage.get_key(key_id)?;

    Ok(json!({
        "KeyMetadata": {
            "KeyId": key.id,
            "Arn": key.arn,
            "Description": key.description,
            "KeyUsage": key.key_usage,
            "KeyState": key.key_state,
            "CreationDate": 1234567890.0,
            "Enabled": key.key_state == "Enabled",
            "AWSAccountId": emulator.config.account_id
        }
    }))
}

async fn enable_key(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
     let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
     emulator.storage.enable_key(key_id)?;
     Ok(json!({}))
}

async fn disable_key(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
     let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
     emulator.storage.disable_key(key_id)?;
     Ok(json!({}))
}

async fn schedule_key_deletion(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
    // Stub: immediately "scheduled"
    let _key = emulator.storage.get_key(key_id)?;
    Ok(json!({
        "KeyId": key_id,
        "DeletionDate": 1234567890.0
    }))
}

async fn cancel_key_deletion(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
    let _key = emulator.storage.get_key(key_id)?;
    Ok(json!({
        "KeyId": key_id
    }))
}

async fn encrypt(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
    let plaintext_b64 = body["Plaintext"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Plaintext".into()))?;
    
    // Check key exists and is enabled
    let key = emulator.storage.get_key(key_id)?;
    if key.key_state != "Enabled" {
        return Err(EmulatorError::InvalidRequest(format!("Key {} is not enabled", key_id)));
    }

    // Decode plaintext from base64
    let plaintext = BASE64.decode(plaintext_b64)
        .map_err(|_| EmulatorError::InvalidArgument("Invalid Plaintext base64".into()))?;

    // Use AES-256-GCM encryption
    use aes_gcm::{
        aead::{Aead, AeadCore, KeyInit, OsRng},
        Aes256Gcm,
    };
    use sha2::{Sha256, Digest};
    
    // Derive a 256-bit key from the key_id (deterministic for this emulator)
    let mut hasher = Sha256::new();
    hasher.update(key_id.as_bytes());
    hasher.update(b"cloudemu-kms-key");
    let key_bytes = hasher.finalize();
    
    let cipher = Aes256Gcm::new_from_slice(&key_bytes)
        .map_err(|e| EmulatorError::Internal(format!("Failed to create cipher: {}", e)))?;
    
    // Generate random nonce (96 bits for GCM)
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    // Encrypt
    let ciphertext = cipher.encrypt(&nonce, plaintext.as_ref())
        .map_err(|e| EmulatorError::Internal(format!("Encryption failed: {}", e)))?;
    
    // Format: nonce (12 bytes) + ciphertext + auth tag (16 bytes, included in ciphertext)
    let mut encrypted_data = nonce.to_vec();
    encrypted_data.extend_from_slice(&ciphertext);
    
    let ciphertext_blob = BASE64.encode(&encrypted_data);

    Ok(json!({
        "CiphertextBlob": ciphertext_blob,
        "KeyId": key.arn,
        "EncryptionAlgorithm": "SYMMETRIC_DEFAULT"
    }))
}

async fn decrypt(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let ciphertext_blob = body["CiphertextBlob"].as_str()
        .ok_or_else(|| EmulatorError::InvalidArgument("Missing CiphertextBlob".into()))?;
    
    // Decode the ciphertext blob
    let encrypted_data = BASE64.decode(ciphertext_blob)
        .map_err(|_| EmulatorError::InvalidArgument("Invalid CiphertextBlob base64".into()))?;
    
    if encrypted_data.len() < 12 {
        return Err(EmulatorError::InvalidArgument("CiphertextBlob too short".into()));
    }
    
    // Extract nonce (first 12 bytes) and ciphertext (rest)
    let (nonce_bytes, ciphertext) = encrypted_data.split_at(12);
    
    use aes_gcm::{
        aead::{Aead, KeyInit},
        Aes256Gcm, Nonce,
    };
    use sha2::{Sha256, Digest};
    
    // Try to decrypt with all keys (in real KMS, key_id is embedded in ciphertext metadata)
    // For simplicity, we'll try all keys until one works
    let keys = emulator.storage.list_keys()?;
    
    for key in keys {
        if key.key_state != "Enabled" {
            continue;
        }
        
        // Derive the same key
        let mut hasher = Sha256::new();
        hasher.update(key.id.as_bytes());
        hasher.update(b"cloudemu-kms-key");
        let key_bytes = hasher.finalize();
        
        let cipher = Aes256Gcm::new_from_slice(&key_bytes)
            .map_err(|e| EmulatorError::Internal(format!("Failed to create cipher: {}", e)))?;
        
        let nonce = Nonce::from_slice(nonce_bytes);
        
        // Try to decrypt
        if let Ok(plaintext) = cipher.decrypt(nonce, ciphertext) {
            let plaintext_b64 = BASE64.encode(&plaintext);
            
            return Ok(json!({
                "KeyId": key.arn,
                "Plaintext": plaintext_b64,
                "EncryptionAlgorithm": "SYMMETRIC_DEFAULT"
            }));
        }
    }
    
    Err(EmulatorError::InvalidRequest("Failed to decrypt: no matching key found".into()))
}

async fn sign(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
    let message = body["Message"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Message".into()))?; // Base64 input
    
    let _key = emulator.storage.get_key(key_id)?;
    
    let mut mac = HmacSha256::new_from_slice(key_id.as_bytes()).expect("HMAC can take any key length");
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let signature = BASE64.encode(result.into_bytes());

    Ok(json!({
        "KeyId": key_id,
        "Signature": signature,
        "SigningAlgorithm": "HMAC_SHA_256"
    }))
}

async fn verify(emulator: &Emulator, body: Value) -> Result<Value, EmulatorError> {
    let key_id = body["KeyId"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing KeyId".into()))?;
    let message = body["Message"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Message".into()))?;
    let signature = body["Signature"].as_str().ok_or_else(|| EmulatorError::InvalidArgument("Missing Signature".into()))?;

    let _key = emulator.storage.get_key(key_id)?;

    let mut mac = HmacSha256::new_from_slice(key_id.as_bytes()).expect("HMAC can take any key length");
    mac.update(message.as_bytes());
    let result = mac.finalize();
    let computed = BASE64.encode(result.into_bytes());
    
    let valid = computed == signature;

    Ok(json!({
        "KeyId": key_id,
        "SignatureValid": valid,
        "SigningAlgorithm": "HMAC_SHA_256"
    }))
}
