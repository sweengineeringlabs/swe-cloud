# 07 - SPI Extensions

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. SPI Overview

```
┌─────────────────────────────────────────────────────────────────┐
│              Service Provider Interface (SPI)                    │
│                                                                  │
│   Purpose: Allow users to customize CloudKit behavior            │
│                                                                  │
│   ┌─────────────────────────────────────────────────────────┐   │
│   │                   Extension Points                       │   │
│   │                                                          │   │
│   │   ┌─────────────┐  ┌─────────────┐  ┌─────────────┐     │   │
│   │   │AuthProvider │  │RetryPolicy  │  │MetricsCol-  │     │   │
│   │   │             │  │             │  │lector       │     │   │
│   │   │ Custom auth │  │ Custom      │  │ Custom      │     │   │
│   │   │ mechanisms  │  │ retry       │  │ metrics     │     │   │
│   │   │             │  │ strategies  │  │ collection  │     │   │
│   │   └─────────────┘  └─────────────┘  └─────────────┘     │   │
│   │                                                          │   │
│   │   ┌─────────────┐                                        │   │
│   │   │Logger       │                                        │   │
│   │   │             │                                        │   │
│   │   │ Custom      │                                        │   │
│   │   │ logging     │                                        │   │
│   │   │             │                                        │   │
│   │   └─────────────┘                                        │   │
│   │                                                          │   │
│   └─────────────────────────────────────────────────────────┘   │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 2. AuthProvider Trait

### Purpose
Custom authentication mechanisms (Vault, AWS SSO, custom identity providers).

### Trait Definition

```rust
#[async_trait]
pub trait AuthProvider: Send + Sync {
    /// Get current credentials
    async fn get_credentials(&self) -> CloudResult<Credentials>;

    /// Refresh credentials if expired
    async fn refresh_credentials(&self) -> CloudResult<Credentials> {
        self.get_credentials().await
    }

    /// Check if credentials need refresh
    async fn is_valid(&self) -> bool {
        self.get_credentials().await.is_ok()
    }
}
```

### Built-in Implementations

```
┌─────────────────────────────────────────────────────────────────┐
│                  AuthProvider Implementations                    │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ EnvAuthProvider (Default)                              │     │
│   │                                                        │     │
│   │ Reads from environment variables:                      │     │
│   │ • CLOUD_ACCESS_KEY / AWS_ACCESS_KEY_ID                │     │
│   │ • CLOUD_SECRET_KEY / AWS_SECRET_ACCESS_KEY            │     │
│   │ • CLOUD_SESSION_TOKEN (optional)                       │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ StaticAuthProvider                                     │     │
│   │                                                        │     │
│   │ Fixed credentials:                                     │     │
│   │ let creds = Credentials::new("key", "secret");        │     │
│   │ let provider = StaticAuthProvider::new(creds);        │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Custom Implementation Example

```
┌─────────────────────────────────────────────────────────────────┐
│              Vault AuthProvider Example                          │
│                                                                  │
│   struct VaultAuthProvider {                                     │
│       vault_url: String,                                         │
│       role: String,                                              │
│       client: reqwest::Client,                                   │
│   }                                                              │
│                                                                  │
│   #[async_trait]                                                 │
│   impl AuthProvider for VaultAuthProvider {                     │
│       async fn get_credentials(&self) -> CloudResult<Creds> {   │
│           // 1. Get Vault token from environment                │
│           // 2. Call Vault API: /v1/aws/creds/{role}            │
│           // 3. Parse response for access_key, secret_key       │
│           // 4. Return Credentials                               │
│       }                                                          │
│   }                                                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 3. RetryPolicy Trait

### Purpose
Custom retry strategies (circuit breaker, adaptive backoff, etc.).

### Trait Definition

```rust
#[async_trait]
pub trait RetryPolicy: Send + Sync {
    /// Determine if operation should be retried
    fn should_retry(&self, error: &CloudError, attempt: u32) -> RetryDecision;

    /// Maximum number of attempts
    fn max_attempts(&self) -> u32;
}

pub enum RetryDecision {
    /// Retry after the specified delay
    Retry(Duration),
    
    /// Do not retry
    DoNotRetry,
}
```

### Built-in Implementations

```
┌─────────────────────────────────────────────────────────────────┐
│                 RetryPolicy Implementations                      │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ ExponentialBackoff (Default)                           │     │
│   │                                                        │     │
│   │ Delay: initial_delay * 2^attempt                      │     │
│   │                                                        │     │
│   │ Attempt 1: 100ms                                       │     │
│   │ Attempt 2: 200ms                                       │     │
│   │ Attempt 3: 400ms                                       │     │
│   │ Attempt 4: 800ms                                       │     │
│   │ (capped at max_delay)                                  │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ FixedDelay                                             │     │
│   │                                                        │     │
│   │ Same delay between each attempt                        │     │
│   │ FixedDelay::new(Duration::from_secs(1), 3)            │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ NoRetry                                                │     │
│   │                                                        │     │
│   │ Never retry, fail immediately                          │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Retry Flow

```
┌─────────────────────────────────────────────────────────────────┐
│                       Retry Flow                                 │
│                                                                  │
│   Request                                                        │
│      │                                                           │
│      ▼                                                           │
│   Execute ────► Success? ────► Yes ────► Return Result          │
│      ▲              │                                            │
│      │              ▼ No                                         │
│      │          Retryable?                                       │
│      │              │                                            │
│      │         ┌────┴────┐                                       │
│      │         ▼         ▼                                       │
│      │        Yes        No ────► Return Error                  │
│      │         │                                                 │
│      │         ▼                                                 │
│      │    Max Attempts?                                          │
│      │         │                                                 │
│      │    ┌────┴────┐                                            │
│      │    ▼         ▼                                            │
│      │   No        Yes ────► Return Error                       │
│      │    │                                                      │
│      │    ▼                                                      │
│      │   Get Delay                                               │
│      │    │                                                      │
│      │    ▼                                                      │
│      └── Sleep(delay)                                            │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 4. MetricsCollector Trait

### Purpose
Integration with observability platforms (Prometheus, DataDog, etc.).

### Trait Definition

```rust
#[async_trait]
pub trait MetricsCollector: Send + Sync {
    /// Record operation metrics
    async fn record(&self, metrics: OperationMetrics);

    /// Increment a counter
    async fn increment_counter(&self, name: &str, value: u64, tags: &[(&str, &str)]);

    /// Record a gauge value
    async fn record_gauge(&self, name: &str, value: f64, tags: &[(&str, &str)]);

    /// Record a histogram value
    async fn record_histogram(&self, name: &str, value: f64, tags: &[(&str, &str)]);
}

pub struct OperationMetrics {
    pub provider: String,
    pub service: String,
    pub operation: String,
    pub duration: Duration,
    pub outcome: OperationOutcome,
    pub retry_count: u32,
}
```

### Built-in Implementations

```
┌─────────────────────────────────────────────────────────────────┐
│               MetricsCollector Implementations                   │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ NoopMetrics (Default)                                  │     │
│   │                                                        │     │
│   │ Does nothing, zero overhead                            │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ LoggingMetrics                                         │     │
│   │                                                        │     │
│   │ Logs metrics via tracing                               │     │
│   │ INFO [aws] s3.put_object completed in 150ms (success)  │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### Custom Implementation Example

```
┌─────────────────────────────────────────────────────────────────┐
│           Prometheus MetricsCollector Example                    │
│                                                                  │
│   struct PrometheusMetrics {                                     │
│       operation_duration: Histogram,                             │
│       operation_count: Counter,                                  │
│       retry_count: Counter,                                      │
│   }                                                              │
│                                                                  │
│   #[async_trait]                                                 │
│   impl MetricsCollector for PrometheusMetrics {                 │
│       async fn record(&self, m: OperationMetrics) {             │
│           self.operation_duration                                │
│               .with_label_values(&[&m.provider, &m.service])    │
│               .observe(m.duration.as_secs_f64());               │
│                                                                  │
│           self.operation_count                                   │
│               .with_label_values(&[&m.outcome.as_str()])        │
│               .inc();                                            │
│       }                                                          │
│   }                                                              │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 5. Logger Trait

### Purpose
Custom logging integration.

### Trait Definition

```rust
#[async_trait]
pub trait Logger: Send + Sync {
    /// Log a message
    fn log(&self, entry: LogEntry);

    /// Log at specific level
    fn trace(&self, message: &str) { self.log(LogEntry::trace(message)); }
    fn debug(&self, message: &str) { self.log(LogEntry::debug(message)); }
    fn info(&self, message: &str)  { self.log(LogEntry::info(message)); }
    fn warn(&self, message: &str)  { self.log(LogEntry::warn(message)); }
    fn error(&self, message: &str) { self.log(LogEntry::error(message)); }
}

pub struct LogEntry {
    pub level: LogLevel,
    pub message: String,
    pub fields: HashMap<String, String>,
    pub timestamp: DateTime<Utc>,
}
```

### Built-in Implementations

```
┌─────────────────────────────────────────────────────────────────┐
│                   Logger Implementations                         │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ TracingLogger (Default)                                │     │
│   │                                                        │     │
│   │ Uses tracing crate macros                              │     │
│   │ tracing::info!("{}", entry.message);                  │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
│   ┌───────────────────────────────────────────────────────┐     │
│   │ NoopLogger                                             │     │
│   │                                                        │     │
│   │ Discards all logs                                      │     │
│   └───────────────────────────────────────────────────────┘     │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 6. Using SPIs Together

```
┌─────────────────────────────────────────────────────────────────┐
│                   Complete SPI Integration                       │
│                                                                  │
│   let context = CloudContext::builder(ProviderType::Aws)        │
│       .config(config)                                            │
│       .auth_provider(Box::new(VaultAuthProvider::new(...)))    │
│       .retry_policy(Box::new(CircuitBreaker::new(...)))        │
│       .metrics_collector(Box::new(PrometheusMetrics::new()))   │
│       .logger(Box::new(StructuredLogger::new()))               │
│       .build()                                                   │
│       .await?;                                                   │
│                                                                  │
│   // All operations now use custom implementations:              │
│   // - Auth from Vault                                           │
│   // - Circuit breaker retry                                     │
│   // - Prometheus metrics                                        │
│   // - Structured logging                                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## 7. Related Documents

- [02-architecture.md](02-architecture.md) - SPI layer in architecture
- [05-error-handling.md](05-error-handling.md) - Retry on errors
- [06-configuration.md](06-configuration.md) - AuthProvider config
