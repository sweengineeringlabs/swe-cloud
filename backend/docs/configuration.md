# Configuration

CloudKit supports configuration through environment variables, configuration files, and programmatic configuration.

## Environment Variables

### Generic CloudKit Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `CLOUD_ACCESS_KEY` | Access key ID | - |
| `CLOUD_SECRET_KEY` | Secret access key | - |
| `CLOUD_SESSION_TOKEN` | Session token (temporary credentials) | - |
| `CLOUD_REGION` | Default region | `us-east-1` |
| `CLOUD_ENDPOINT` | Custom endpoint URL | Provider default |
| `CLOUD_TIMEOUT` | Request timeout in seconds | `30` |
| `CLOUD_MAX_RETRIES` | Maximum retry attempts | `3` |

### AWS-Specific Variables

| Variable | Description |
|----------|-------------|
| `AWS_ACCESS_KEY_ID` | AWS access key |
| `AWS_SECRET_ACCESS_KEY` | AWS secret key |
| `AWS_SESSION_TOKEN` | AWS session token |
| `AWS_REGION` | AWS region |
| `AWS_PROFILE` | AWS profile name |
| `AWS_CONFIG_FILE` | Path to AWS config file |
| `AWS_SHARED_CREDENTIALS_FILE` | Path to credentials file |

### Azure-Specific Variables

| Variable | Description |
|----------|-------------|
| `AZURE_STORAGE_ACCOUNT` | Storage account name |
| `AZURE_STORAGE_KEY` | Storage account key |
| `AZURE_STORAGE_CONNECTION_STRING` | Connection string |
| `AZURE_TENANT_ID` | Azure AD tenant ID |
| `AZURE_CLIENT_ID` | Azure AD client ID |
| `AZURE_CLIENT_SECRET` | Azure AD client secret |

### GCP-Specific Variables

| Variable | Description |
|----------|-------------|
| `GOOGLE_APPLICATION_CREDENTIALS` | Path to service account JSON |
| `GCP_PROJECT_ID` | GCP project ID |
| `GOOGLE_CLOUD_PROJECT` | Alternative project ID variable |

### Oracle-Specific Variables

| Variable | Description |
|----------|-------------|
| `OCI_TENANCY_OCID` | Tenancy OCID |
| `OCI_USER_OCID` | User OCID |
| `OCI_FINGERPRINT` | API key fingerprint |
| `OCI_PRIVATE_KEY_PATH` | Path to private key |
| `OCI_REGION` | OCI region |

## Programmatic Configuration

### Using CloudConfig

```rust
use cloudkit::prelude::*;
use std::time::Duration;

let config = CloudConfig::builder()
    .region(Region::aws_us_east_1())
    .timeout(Duration::from_secs(60))
    .request_timeout(Duration::from_secs(120))
    .max_retries(5)
    .enable_tracing(true)
    .build()?;

let context = CloudKit::from_config(ProviderType::Aws, config)
    .build()
    .await?;
```

### Custom Endpoint

For local testing with LocalStack, MinIO, or Azurite:

```rust
let config = CloudConfig::builder()
    .region(Region::aws_us_east_1())
    .endpoint("http://localhost:4566")  // LocalStack
    .build()?;
```

### Credentials

```rust
use cloudkit::common::Credentials;

// From environment
let creds = Credentials::from_env()?;

// Static credentials
let creds = Credentials::new("access-key", "secret-key");

// With session token
let creds = Credentials::with_session_token(
    "access-key",
    "secret-key",
    "session-token",
);
```

## Retry Configuration

### Default Retry Policy

CloudKit uses exponential backoff by default:

- Initial delay: 100ms
- Maximum delay: 30s
- Maximum attempts: 3
- Multiplier: 2.0

### Custom Retry Policy

```rust
use cloudkit::spi::{ExponentialBackoff, FixedDelay, NoRetry};
use std::time::Duration;

// Exponential backoff
let retry = ExponentialBackoff::new(5)
    .with_initial_delay(Duration::from_millis(200))
    .with_max_delay(Duration::from_secs(60))
    .with_multiplier(1.5);

// Fixed delay
let retry = FixedDelay::new(Duration::from_secs(1), 3);

// No retry
let retry = NoRetry;
```

## Observability

### Tracing

Enable tracing in configuration:

```rust
let config = CloudConfig::builder()
    .enable_tracing(true)
    .build()?;
```

Initialize a tracing subscriber:

```rust
tracing_subscriber::fmt()
    .with_env_filter("cloudkit=debug")
    .init();
```

### Custom Metrics

```rust
use cloudkit::spi::{MetricsCollector, OperationMetrics};

struct PrometheusMetrics {
    // Prometheus client
}

#[async_trait]
impl MetricsCollector for PrometheusMetrics {
    async fn record(&self, metrics: OperationMetrics) {
        // Record to Prometheus
    }
    
    async fn increment_counter(&self, name: &str, value: u64, tags: &[(&str, &str)]) {
        // Increment counter
    }
    
    async fn record_gauge(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        // Record gauge
    }
    
    async fn record_histogram(&self, name: &str, value: f64, tags: &[(&str, &str)]) {
        // Record histogram
    }
}
```

## Configuration Files

### AWS Configuration

`~/.aws/config`:
```ini
[default]
region = us-east-1
output = json

[profile dev]
region = us-west-2

[profile prod]
region = eu-west-1
role_arn = arn:aws:iam::123456789012:role/prod-role
source_profile = default
```

`~/.aws/credentials`:
```ini
[default]
aws_access_key_id = AKIA...
aws_secret_access_key = ...

[dev]
aws_access_key_id = AKIA...
aws_secret_access_key = ...
```

### GCP Configuration

Service account JSON file:
```json
{
  "type": "service_account",
  "project_id": "my-project",
  "private_key_id": "key-id",
  "private_key": "-----BEGIN PRIVATE KEY-----\n...",
  "client_email": "sa@project.iam.gserviceaccount.com",
  "client_id": "123456789",
  "auth_uri": "https://accounts.google.com/o/oauth2/auth",
  "token_uri": "https://oauth2.googleapis.com/token"
}
```

### Oracle Configuration

`~/.oci/config`:
```ini
[DEFAULT]
user=ocid1.user.oc1..aaa...
fingerprint=12:34:56:78:90:ab:cd:ef:12:34:56:78:90:ab:cd:ef
tenancy=ocid1.tenancy.oc1..aaa...
region=us-ashburn-1
key_file=~/.oci/oci_api_key.pem
```

## Best Practices

1. **Never commit credentials** - Use environment variables or credential files
2. **Use IAM roles** - When running on cloud infrastructure
3. **Rotate credentials** - Regularly rotate access keys
4. **Least privilege** - Grant only necessary permissions
5. **Separate environments** - Use different credentials per environment
6. **Enable tracing** - For debugging in development
7. **Configure timeouts** - Based on your use case
8. **Handle retries** - Configure appropriate retry limits
