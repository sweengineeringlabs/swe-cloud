# Authentication SPI

The Authentication Service Provider Interface allows you to implement custom authentication mechanisms.

## AuthProvider Trait

```rust
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Get current credentials.
    async fn get_credentials(&self) -> CloudResult<Credentials>;

    /// Refresh credentials if expired.
    async fn refresh_credentials(&self) -> CloudResult<Credentials> {
        self.get_credentials().await
    }

    /// Check if credentials are valid.
    async fn is_valid(&self) -> bool {
        self.get_credentials().await.is_ok()
    }
}
```

## Built-in Providers

### EnvAuthProvider

Reads credentials from environment variables:

```rust
use cloudkit::spi::EnvAuthProvider;

let provider = EnvAuthProvider;
let creds = provider.get_credentials().await?;
```

Environment variables checked:
1. `CLOUD_ACCESS_KEY` / `AWS_ACCESS_KEY_ID`
2. `CLOUD_SECRET_KEY` / `AWS_SECRET_ACCESS_KEY`
3. `CLOUD_SESSION_TOKEN` / `AWS_SESSION_TOKEN` (optional)

### StaticAuthProvider

Uses fixed credentials:

```rust
use cloudkit::spi::StaticAuthProvider;
use cloudkit::common::Credentials;

let creds = Credentials::new("access-key", "secret-key");
let provider = StaticAuthProvider::new(creds);
```

## Custom Implementations

### HashiCorp Vault

```rust
use async_trait::async_trait;
use cloudkit::prelude::*;
use cloudkit::spi::AuthProvider;

struct VaultAuthProvider {
    vault_url: String,
    role: String,
    http_client: reqwest::Client,
}

impl VaultAuthProvider {
    pub fn new(vault_url: &str, role: &str) -> Self {
        Self {
            vault_url: vault_url.to_string(),
            role: role.to_string(),
            http_client: reqwest::Client::new(),
        }
    }
}

#[async_trait]
impl AuthProvider for VaultAuthProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        // Authenticate with Vault
        let response = self.http_client
            .get(&format!("{}/v1/aws/creds/{}", self.vault_url, self.role))
            .header("X-Vault-Token", std::env::var("VAULT_TOKEN").unwrap())
            .send()
            .await
            .map_err(|e| CloudError::Auth(AuthError::MissingCredentials(e.to_string())))?;

        if !response.status().is_success() {
            return Err(CloudError::Auth(AuthError::InvalidCredentials(
                "Vault returned error".to_string()
            )));
        }

        let body: serde_json::Value = response.json().await
            .map_err(|e| CloudError::Serialization(e.to_string()))?;

        let data = &body["data"];
        
        Ok(Credentials::new(
            data["access_key"].as_str().unwrap_or_default(),
            data["secret_key"].as_str().unwrap_or_default(),
        ))
    }

    async fn refresh_credentials(&self) -> CloudResult<Credentials> {
        // Vault credentials are leased, so we just get new ones
        self.get_credentials().await
    }
}
```

### AWS STS AssumeRole

```rust
struct StsAssumeRoleProvider {
    role_arn: String,
    session_name: String,
    base_provider: Box<dyn AuthProvider>,
}

#[async_trait]
impl AuthProvider for StsAssumeRoleProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        // Get base credentials
        let base_creds = self.base_provider.get_credentials().await?;
        
        // Call STS AssumeRole
        // ... (would use AWS SDK)
        
        // Return temporary credentials
        Ok(Credentials::with_session_token(
            "temporary-access-key",
            "temporary-secret-key",
            "session-token",
        ))
    }
}
```

### Caching Provider

```rust
use std::sync::RwLock;
use std::time::{Duration, Instant};

struct CachingAuthProvider {
    inner: Box<dyn AuthProvider>,
    cache: RwLock<Option<(Credentials, Instant)>>,
    ttl: Duration,
}

impl CachingAuthProvider {
    pub fn new(inner: Box<dyn AuthProvider>, ttl: Duration) -> Self {
        Self {
            inner,
            cache: RwLock::new(None),
            ttl,
        }
    }
}

#[async_trait]
impl AuthProvider for CachingAuthProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        // Check cache
        {
            let cache = self.cache.read().unwrap();
            if let Some((creds, timestamp)) = cache.as_ref() {
                if timestamp.elapsed() < self.ttl {
                    return Ok(creds.clone());
                }
            }
        }

        // Fetch new credentials
        let creds = self.inner.get_credentials().await?;
        
        // Update cache
        {
            let mut cache = self.cache.write().unwrap();
            *cache = Some((creds.clone(), Instant::now()));
        }

        Ok(creds)
    }
}
```

### Chain Provider

```rust
struct ChainAuthProvider {
    providers: Vec<Box<dyn AuthProvider>>,
}

impl ChainAuthProvider {
    pub fn new(providers: Vec<Box<dyn AuthProvider>>) -> Self {
        Self { providers }
    }
}

#[async_trait]
impl AuthProvider for ChainAuthProvider {
    async fn get_credentials(&self) -> CloudResult<Credentials> {
        for provider in &self.providers {
            match provider.get_credentials().await {
                Ok(creds) => return Ok(creds),
                Err(_) => continue,
            }
        }
        
        Err(CloudError::Auth(AuthError::MissingCredentials(
            "No provider returned valid credentials".to_string()
        )))
    }
}

// Usage
let chain = ChainAuthProvider::new(vec![
    Box::new(EnvAuthProvider),
    Box::new(VaultAuthProvider::new("https://vault.example.com", "cloud-reader")),
]);
```

## Using Custom Providers

```rust
use cloudkit_aws::AwsBuilder;

let vault_auth = VaultAuthProvider::new("https://vault.example.com", "aws-role");

// Build client with custom auth
// Note: This requires the builder to accept an AuthProvider
// which would be implemented in the provider crate
```

## Best Practices

1. **Cache credentials** - Avoid fetching on every request
2. **Handle expiration** - Check credential validity
3. **Retry on refresh** - Handle transient failures
4. **Log authentication** - But never log credentials
5. **Secure secrets** - Use Vault or similar for production
