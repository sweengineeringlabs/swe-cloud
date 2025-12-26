# 06 - Configuration Design

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. Configuration Hierarchy

```
┌─────────────────────────────────────────────────────────────────┐
│                   Configuration Precedence                       │
│                     (Highest to Lowest)                          │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  1. Programmatic Configuration                          │   │
│   │     CloudConfig::builder().region(...).build()          │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │ Overrides                           │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  2. Environment Variables                                │   │
│   │     CLOUD_REGION, AWS_REGION, AZURE_REGION, etc.        │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │ Overrides                           │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  3. Configuration Files                                  │   │
│   │     ~/.aws/config, ~/.oci/config, etc.                  │   │
│   └────────────────────────┬────────────────────────────────┘   │
│                            │ Overrides                           │
│                            ▼                                     │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │  4. Default Values                                       │   │
│   │     Region: us-east-1, Timeout: 30s, etc.               │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. CloudConfig Structure

```rust
pub struct CloudConfig {
    /// Cloud region
    pub region: Region,
    
    /// Request timeout
    pub timeout: Duration,
    
    /// Connection timeout
    pub connect_timeout: Duration,
    
    /// Maximum retry attempts
    pub max_retries: u32,
    
    /// Custom endpoint URL (for testing)
    pub endpoint: Option<String>,
    
    /// Enable tracing
    pub tracing_enabled: bool,
    
    /// Provider-specific configuration
    pub provider_config: HashMap<String, String>,
}

impl CloudConfig {
    pub fn builder() -> CloudConfigBuilder {
        CloudConfigBuilder::default()
    }
}
```

### Builder Pattern

```
┌─────────────────────────────────────────────────────────────────┐
│                    CloudConfig Builder                           │
│                                                                  │
│   let config = CloudConfig::builder()                           │
│       .region(Region::aws_us_east_1())                          │
│       .timeout(Duration::from_secs(60))                         │
│       .connect_timeout(Duration::from_secs(10))                 │
│       .max_retries(5)                                           │
│       .endpoint("http://localhost:4566")  // LocalStack         │
│       .enable_tracing(true)                                     │
│       .build()?;                                                 │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. Environment Variables

### Generic CloudKit Variables

```
┌─────────────────────────────────────────────────────────────────┐
│              Generic Environment Variables                       │
│                                                                  │
│   Variable              │ Description         │ Default          │
│   ──────────────────────┼─────────────────────┼─────────────     │
│   CLOUD_ACCESS_KEY      │ Access key ID       │ -                │
│   CLOUD_SECRET_KEY      │ Secret access key   │ -                │
│   CLOUD_SESSION_TOKEN   │ Session token       │ -                │
│   CLOUD_REGION          │ Default region      │ us-east-1        │
│   CLOUD_ENDPOINT        │ Custom endpoint     │ Provider default │
│   CLOUD_TIMEOUT         │ Timeout (seconds)   │ 30               │
│   CLOUD_MAX_RETRIES     │ Max retry attempts  │ 3                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### AWS-Specific Variables

```
┌─────────────────────────────────────────────────────────────────┐
│                 AWS Environment Variables                        │
│                                                                  │
│   Variable                    │ Description                      │
│   ────────────────────────────┼───────────────────────────       │
│   AWS_ACCESS_KEY_ID           │ AWS access key                   │
│   AWS_SECRET_ACCESS_KEY       │ AWS secret key                   │
│   AWS_SESSION_TOKEN           │ AWS session token                │
│   AWS_REGION                  │ AWS region                       │
│   AWS_PROFILE                 │ Profile from ~/.aws/credentials  │
│   AWS_CONFIG_FILE             │ Path to config file              │
│   AWS_SHARED_CREDENTIALS_FILE │ Path to credentials file         │
│   AWS_ENDPOINT_URL            │ Custom endpoint (LocalStack)     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Azure-Specific Variables

```
┌─────────────────────────────────────────────────────────────────┐
│                Azure Environment Variables                       │
│                                                                  │
│   Variable                      │ Description                    │
│   ──────────────────────────────┼─────────────────────────       │
│   AZURE_STORAGE_ACCOUNT         │ Storage account name           │
│   AZURE_STORAGE_KEY             │ Storage account key            │
│   AZURE_STORAGE_CONNECTION_STRING│ Full connection string        │
│   AZURE_TENANT_ID               │ Azure AD tenant ID             │
│   AZURE_CLIENT_ID               │ Azure AD client ID             │
│   AZURE_CLIENT_SECRET           │ Azure AD client secret         │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### GCP-Specific Variables

```
┌─────────────────────────────────────────────────────────────────┐
│                GCP Environment Variables                         │
│                                                                  │
│   Variable                       │ Description                   │
│   ───────────────────────────────┼────────────────────────       │
│   GOOGLE_APPLICATION_CREDENTIALS │ Path to service account JSON  │
│   GCP_PROJECT_ID                 │ GCP project ID                │
│   GOOGLE_CLOUD_PROJECT           │ Alternative project ID var    │
│   CLOUDSDK_CORE_PROJECT          │ gcloud SDK project            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Oracle-Specific Variables

```
┌─────────────────────────────────────────────────────────────────┐
│               Oracle Environment Variables                       │
│                                                                  │
│   Variable                │ Description                          │
│   ────────────────────────┼───────────────────────────────       │
│   OCI_TENANCY_OCID        │ Tenancy OCID                         │
│   OCI_USER_OCID           │ User OCID                            │
│   OCI_FINGERPRINT         │ API key fingerprint                  │
│   OCI_PRIVATE_KEY_PATH    │ Path to private key file             │
│   OCI_REGION              │ OCI region identifier                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. Credentials Management

```
┌─────────────────────────────────────────────────────────────────┐
│                   Credentials Structure                          │
│                                                                  │
│   pub struct Credentials {                                       │
│       /// Access key ID                                          │
│       pub access_key: String,                                    │
│                                                                  │
│       /// Secret access key                                      │
│       pub secret_key: String,                                    │
│                                                                  │
│       /// Optional session token (for temporary credentials)    │
│       pub session_token: Option<String>,                        │
│                                                                  │
│       /// Optional expiration time                               │
│       pub expires_at: Option<DateTime<Utc>>,                    │
│   }                                                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Credential Sources

```
┌─────────────────────────────────────────────────────────────────┐
│                   Credential Sources                             │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │                  Credential Chain                      │     │
│   │                                                        │     │
│   │   1. ┌──────────────────────────────────────────┐     │     │
│   │      │ Environment Variables                     │     │     │
│   │      │ AWS_ACCESS_KEY_ID, AWS_SECRET_ACCESS_KEY │     │     │
│   │      └──────────────────────────────────────────┘     │     │
│   │                         │                              │     │
│   │                         ▼ if not found                 │     │
│   │   2. ┌──────────────────────────────────────────┐     │     │
│   │      │ Shared Credentials File                   │     │     │
│   │      │ ~/.aws/credentials, ~/.oci/config        │     │     │
│   │      └──────────────────────────────────────────┘     │     │
│   │                         │                              │     │
│   │                         ▼ if not found                 │     │
│   │   3. ┌──────────────────────────────────────────┐     │     │
│   │      │ Instance Metadata (IAM Role)              │     │     │
│   │      │ EC2, ECS, Lambda, GCE, Azure VM          │     │     │
│   │      └──────────────────────────────────────────┘     │     │
│   │                         │                              │     │
│   │                         ▼ if not found                 │     │
│   │   4. ┌──────────────────────────────────────────┐     │     │
│   │      │ Custom AuthProvider (SPI)                 │     │     │
│   │      │ Vault, AWS SSO, Azure AD, etc.           │     │     │
│   │      └──────────────────────────────────────────┘     │     │
│   │                                                        │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. Region Configuration

```
┌─────────────────────────────────────────────────────────────────┐
│                   Region Definitions                             │
│                                                                  │
│   pub struct Region {                                            │
│       /// Region code (e.g., "us-east-1")                       │
│       pub code: String,                                          │
│                                                                  │
│       /// Display name                                           │
│       pub name: String,                                          │
│                                                                  │
│       /// Cloud provider                                         │
│       pub provider: ProviderType,                               │
│   }                                                              │
│                                                                  │
│   // Pre-defined regions                                         │
│   Region::aws_us_east_1()       // AWS US East (N. Virginia)    │
│   Region::aws_eu_west_1()       // AWS EU (Ireland)             │
│   Region::azure_east_us()       // Azure East US                │
│   Region::gcp_us_central1()     // GCP US Central               │
│   Region::oracle_af_johannesburg_1() // OCI Africa              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Local Development Configuration

### LocalStack (AWS)

```
┌─────────────────────────────────────────────────────────────────┐
│   # docker-compose.yml                                           │
│   services:                                                      │
│     localstack:                                                  │
│       image: localstack/localstack                              │
│       ports:                                                     │
│         - "4566:4566"                                           │
│       environment:                                               │
│         - SERVICES=s3,dynamodb,sqs,sns,lambda                   │
│                                                                  │
│   # Application configuration                                    │
│   let config = CloudConfig::builder()                           │
│       .endpoint("http://localhost:4566")                        │
│       .region(Region::aws_us_east_1())                          │
│       .build()?;                                                 │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Azurite (Azure)

```
┌─────────────────────────────────────────────────────────────────┐
│   # Run Azurite                                                  │
│   docker run -p 10000:10000 mcr.microsoft.com/azure-storage/azurite
│                                                                  │
│   # Connection string                                            │
│   AZURE_STORAGE_CONNECTION_STRING=\                             │
│     "DefaultEndpointsProtocol=http;\                            │
│      AccountName=devstoreaccount1;\                             │
│      AccountKey=...;\                                           │
│      BlobEndpoint=http://127.0.0.1:10000/devstoreaccount1"      │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 7. Related Documents

- [04-provider-integration.md](04-provider-integration.md) - Provider details
- [07-spi-extensions.md](07-spi-extensions.md) - Custom auth providers
