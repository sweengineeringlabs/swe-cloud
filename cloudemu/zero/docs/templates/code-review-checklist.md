# Code Review Checklist Template

> Use this template when reviewing any Rustratify crate for quality and consistency.

## Usage

Copy this template to `<crate>/doc/5-testing/code-review.md` and fill in the sections.

---

# {CRATE_NAME} Code Review

> Review of builder pattern, macros, infrastructure, error handling, error propagation, logging, configuration, testing, documentation, security, and performance.

## 1. Builder Pattern

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Uses `TypedBuilder` for config structs | ⬜ | |
| Builder methods use `setter(into)` for String fields | ⬜ | |
| Optional fields use `setter(strip_option)` | ⬜ | |
| Sensible defaults provided | ⬜ | |
| Validation on `build()` if needed | ⬜ | See decision tree below |

### Decision Tree: Validation on Build

```
Does the struct have constraints beyond type safety?
├─ NO → N/A (mark as ✅)
└─ YES → What kind of constraints?
    ├─ Field relationships (e.g., min < max)
    │   └─ REQUIRED: Validate in build()
    ├─ Format validation (e.g., URL, email, phone)
    │   └─ REQUIRED: Validate in build()
    ├─ Range constraints (e.g., port 1-65535)
    │   └─ REQUIRED: Validate in build()
    ├─ Required combinations (e.g., if A then B required)
    │   └─ REQUIRED: Validate in build()
    └─ All fields are independent primitives
        └─ N/A (mark as ✅)
```

### Questions to Answer

- [ ] Are there config structs without TypedBuilder?
- [ ] Are there manual builder impls that could use derive macros?
- [ ] Is validation performed before returning built objects?

---

## 2. Macros

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Provider init macro (if applicable) | ⬜ | See decision tree below |
| Error mapping macro | ⬜ | `map_<crate>_error!` |
| Logging helper macros | ⬜ | |
| Uses macros from `test-utils` | ⬜ | See decision tree below |

### Decision Tree: Provider Init Macro

```
Does the crate implement a provider pattern?
├─ NO → N/A (mark as ✅)
└─ YES → How many providers?
    ├─ Single provider, no variants
    │   └─ OPTIONAL: Macro adds little value
    └─ Multiple providers OR configurable initialization
        └─ REQUIRED: Use `<crate>_provider_init!` macro
           └─ Reduces: connection setup, config loading, health checks
```

### Decision Tree: test-utils Macros

```
Does the crate have tests?
├─ NO → N/A (mark as ✅)
└─ YES → What kind of tests?
    ├─ Unit tests (pure functions, no I/O)
    │   └─ OPTIONAL: Standard #[test] sufficient
    ├─ Integration tests with external services
    │   └─ REQUIRED: Use test-utils for:
    │      ├─ test_provider! - provider scaffolding
    │      ├─ with_test_env! - environment setup
    │      └─ assert_retryable! - error behavior
    └─ Async tests
        └─ RECOMMENDED: Use test-utils async helpers
```

### Questions to Answer

- [ ] Is there repeated boilerplate that could be macro-ized?
- [ ] Are existing rustratify macros being reused?
- [ ] Estimated lines saved with macros: ___

---

## 3. Infrastructure (Rustboot) Integration

### ⚠️ The Infrastructure Rule

**Domain crates MUST NOT reintroduce infrastructure that exists in Rustboot.**

This is a core architectural principle. Infrastructure belongs in Rustboot; domain logic belongs in Rustratify modules.

| Concern | Use From Rustboot | DO NOT Reimplement |
|---------|-------------------|-------------------|
| Error utilities | `rustboot-error` | Custom `From` impls, context helpers |
| Retry/Backoff | `rustboot-resilience` | Custom retry loops |
| Circuit breakers | `rustboot-resilience` | Custom circuit breaker |
| Caching | `rustboot-cache` | Custom cache implementations |
| Validation | `rustboot-validation` | Custom validation logic |
| Health checks | `rustboot-health` | Custom health traits |
| Metrics | `rustboot-observability` | Custom metrics collection |
| Logging setup | `rustboot-observability` | Custom logging infrastructure |
| HTTP clients | `rustboot-http` | Custom HTTP wrappers |
| Config loading | `rustboot-config` | Custom config parsers |
| Secrets | `swe-secrets` | Custom secret loading |

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| **No infrastructure duplication** | ⬜ | See table above |
| `rustboot-error::RetryableError` implemented | ⬜ | See decision tree below |
| `swe-secrets` integration (if has credentials) | ⬜ | See decision tree below |
| `test-utils` macros used in tests | ⬜ | See Macros section |
| Retry infrastructure used where appropriate | ⬜ | See decision tree below |

### Decision Tree: Infrastructure Duplication Check

```
Is there code that handles a cross-cutting concern?
├─ NO → Continue
└─ YES → Does Rustboot have a crate for this?
    ├─ YES → VIOLATION: Remove and use Rustboot crate
    │   └─ Common violations:
    │      ├─ Custom retry loops → use rustboot-resilience
    │      ├─ Custom error mapping → use rustboot-error
    │      ├─ Custom cache structs → use rustboot-cache
    │      ├─ Custom validation → use rustboot-validation
    │      └─ Custom health checks → use rustboot-health
    └─ NO → Is it truly domain-specific?
        ├─ YES → Keep in domain crate
        └─ NO → Consider contributing to Rustboot
```

### Decision Tree: RetryableError Implementation

```
Does the crate define error types?
├─ NO → N/A (mark as ✅)
└─ YES → Can any errors be transient?
    ├─ NO (all errors are permanent/fatal)
    │   └─ OPTIONAL: Implement with is_retryable() → false
    └─ YES → Which errors are transient?
        ├─ Network errors (timeout, connection refused)
        │   └─ REQUIRED: is_retryable() → true
        ├─ Rate limiting (429, quota exceeded)
        │   └─ REQUIRED: is_retryable() + retry_after_ms()
        ├─ Resource exhaustion (pool full, memory)
        │   └─ REQUIRED: is_retryable() → true
        └─ Server errors (5xx, temporary unavailable)
            └─ REQUIRED: is_retryable() → true
```

### Decision Tree: swe-secrets Integration

```
Does the crate handle credentials/secrets?
├─ NO → N/A (mark as ✅)
└─ YES → What types of secrets?
    ├─ API keys/tokens
    │   └─ REQUIRED: swe-secrets integration
    ├─ Database passwords
    │   └─ REQUIRED: swe-secrets integration
    ├─ TLS certificates/private keys
    │   └─ REQUIRED: swe-secrets integration
    └─ Non-sensitive config only
        └─ N/A (mark as ✅)

    Implementation:
    ├─ Add `secrets` feature flag
    ├─ Create `load_<type>_from_secrets()` helper
    └─ Document secret key names
```

### Decision Tree: Retry Infrastructure

```
Does the crate make external calls?
├─ NO → N/A (mark as ✅)
└─ YES → What type of calls?
    ├─ HTTP/REST APIs
    │   └─ REQUIRED: Use retry with exponential backoff
    ├─ Database queries
    │   ├─ Read queries → RECOMMENDED: Retry on connection errors
    │   └─ Write queries → CAREFUL: Only retry if idempotent
    ├─ Message queue operations
    │   └─ REQUIRED: Retry with backoff
    ├─ File I/O
    │   └─ OPTIONAL: Usually not needed
    └─ In-memory operations
        └─ N/A (mark as ✅)
```

### Required Implementations

```rust
// If crate has retryable errors:
impl_retryable_error! {
    for {Error} =>
    is_retryable: {patterns},
    retry_after: { {pattern} => Some({ms}) }
}
```

---

## 4. Error Handling

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Uses `thiserror` | ⬜ | |
| Two-layer errors (SPI + API) if SEA pattern | ⬜ | |
| Error codes for observability | ⬜ | `.code() -> &'static str` |
| `#[from]` for error conversions | ⬜ | |
| `is_retryable()` helper | ⬜ | |
| `is_client_error()` helper | ⬜ | |

### Error Code Convention

```rust
impl {Error} {
    pub fn code(&self) -> &'static str {
        match self {
            Self::NotConnected(_) => "{PREFIX}_NOT_CONNECTED",
            Self::Timeout(_) => "{PREFIX}_TIMEOUT",
            // ...
        }
    }
}
```

---

## 5. Error Propagation

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Uses `?` operator consistently | ⬜ | |
| No silent error swallowing (`if let Ok(...)`) | ⬜ | |
| Errors logged before being ignored (if intentional) | ⬜ | |
| `From` impls for error conversion | ⬜ | |

### Anti-Pattern to Find

```rust
// BAD: Silent error swallowing
if let Ok(result) = fallible_operation() {
    // error silently ignored
}

// GOOD: Log if ignoring
match fallible_operation() {
    Ok(result) => { /* use result */ },
    Err(e) => warn!(error = %e, "Operation failed, continuing"),
}
```

---

## 6. Logging

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Uses `tracing` crate | ⬜ | |
| `#[instrument]` on all public async methods | ⬜ | |
| Structured fields (not string interpolation) | ⬜ | |
| Consistent field naming | ⬜ | |
| Appropriate log levels | ⬜ | |

### Instrumentation Pattern

```rust
#[instrument(skip(self), fields(key_field = %value))]
async fn public_method(&self, ...) -> Result<...> {
    // ...
}
```

---

## 7. Configuration

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| `TypedBuilder` on all config structs | ⬜ | |
| `Serialize`/`Deserialize` derives | ⬜ | |
| Secrets redacted in `Debug` output | ⬜ | See decision tree below |
| `from_env()` method for env loading | ⬜ | |
| `Validatable` trait implemented | ⬜ | |
| Human-readable durations (`humantime_serde`) | ⬜ | See decision tree below |
| swe-secrets integration (optional feature) | ⬜ | See Infrastructure section |

### Decision Tree: Secret Redaction

```
Does the config struct contain sensitive fields?
├─ NO → N/A (mark as ✅), use derive(Debug)
└─ YES → Which fields are sensitive?
    ├─ Passwords, tokens, API keys
    │   └─ REQUIRED: Custom Debug impl, show "***REDACTED***"
    ├─ Connection strings with embedded credentials
    │   └─ REQUIRED: Parse and redact password portion
    ├─ Private keys, certificates
    │   └─ REQUIRED: Show "[PRIVATE KEY]" or similar
    └─ Account IDs, non-secret identifiers
        └─ SAFE: Can show in Debug output
```

### Decision Tree: humantime_serde

```
Does the config struct contain Duration fields?
├─ NO → N/A (mark as ✅)
└─ YES → Will config be user-edited (TOML/JSON/YAML)?
    ├─ NO (programmatic only)
    │   └─ OPTIONAL: Standard serde sufficient
    └─ YES (user-facing config files)
        └─ REQUIRED: Use humantime_serde
           └─ Allows: "30s", "5m", "1h" instead of milliseconds
```

### Secret Redaction Pattern

```rust
impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Config")
            .field("api_key", &"***REDACTED***")
            .field("other_field", &self.other_field)
            .finish()
    }
}
```

---

## 8. Testing

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Unit tests for core logic | ⬜ | |
| Integration tests for providers | ⬜ | See decision tree below |
| Mock implementations available | ⬜ | See decision tree below |
| Tests use `test-utils` where applicable | ⬜ | See Macros section |
| Doc tests for public API | ⬜ | |

### Decision Tree: Integration Tests

```
Does the crate interact with external systems?
├─ NO → N/A (mark as ✅), unit tests sufficient
└─ YES → What external systems?
    ├─ APIs (REST, gRPC)
    │   └─ REQUIRED: Integration tests with:
    │      ├─ Mock server (wiremock, mockito)
    │      └─ Optional: Real API tests (feature-gated)
    ├─ Databases
    │   └─ REQUIRED: Integration tests with:
    │      ├─ Testcontainers OR
    │      └─ In-memory DB (SQLite for SQL tests)
    ├─ Message queues
    │   └─ REQUIRED: Integration tests with testcontainers
    └─ File system
        └─ REQUIRED: Use tempdir for isolation
```

### Decision Tree: Mock Implementations

```
Does the crate define traits for external interactions?
├─ NO → N/A (mark as ✅)
└─ YES → How complex are the traits?
    ├─ Simple (1-3 methods, no state)
    │   └─ RECOMMENDED: Manual mock in tests module
    ├─ Complex (many methods, stateful)
    │   └─ REQUIRED: Dedicated mock module
    │      ├─ MockProvider struct
    │      ├─ Configurable responses
    │      └─ Call verification
    └─ Used by consumers
        └─ REQUIRED: Export mock in `testing` feature
           └─ Allows downstream crates to mock
```

---

## 9. Documentation

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Doc comments on all public items | ⬜ | `///` on pub fns, structs, enums |
| Module-level documentation | ⬜ | `//!` at top of each module |
| README.md with examples | ⬜ | |
| ADRs for key architectural decisions | ⬜ | `doc/3-design/adr/` |
| CHANGELOG maintained | ⬜ | |

### Documentation Pattern

```rust
//! # Module Name
//!
//! Brief description of what this module does.
//!
//! ## Example
//!
//! ```rust
//! use crate::module::Thing;
//! let thing = Thing::new();
//! ```

/// A thing that does something.
///
/// # Example
///
/// ```rust
/// let thing = Thing::builder().name("foo").build();
/// ```
pub struct Thing { ... }
```

---

## 10. Security

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Input validation at system boundaries | ⬜ | User input, external APIs |
| No secrets in logs or error messages | ⬜ | Custom Debug impls |
| TLS/encryption for network connections | ⬜ | See decision tree below |
| Dependency audit passes | ⬜ | `cargo audit` |
| No unsafe code (or justified/audited) | ⬜ | |
| SQL injection prevention | ⬜ | See decision tree below |
| Path traversal prevention | ⬜ | See decision tree below |

### Decision Trees

#### TLS/Encryption Applicability

```
Does crate make network connections?
├─ NO → N/A (mark as ✅)
└─ YES → What type of connection?
    ├─ HTTP/HTTPS APIs
    │   └─ REQUIRED: Use HTTPS only, reject HTTP
    ├─ Database connections
    │   ├─ Same host/localhost → Optional (document risk)
    │   └─ Remote/cross-network → REQUIRED: TLS
    ├─ Message queues (Redis, RabbitMQ, etc.)
    │   ├─ Same host/localhost → Optional (document risk)
    │   └─ Remote/cross-network → REQUIRED: TLS
    ├─ gRPC/WebSocket
    │   └─ REQUIRED: TLS for production
    └─ Custom TCP/UDP
        ├─ Transmits credentials/PII → REQUIRED: TLS
        └─ Internal metrics only → Optional (document risk)
```

#### SQL Injection Applicability

```
Does crate interact with SQL databases?
├─ NO → N/A (mark as ✅)
└─ YES → How are queries constructed?
    ├─ ORM only (Diesel, SeaORM, SQLx macros)
    │   └─ LOW RISK: Framework handles escaping
    │      └─ Still audit: any raw SQL or format!() in queries?
    ├─ Query builder with parameters
    │   └─ SAFE if: all user input via bind parameters
    │      └─ UNSAFE if: string concatenation/interpolation
    └─ Raw SQL strings
        ├─ Static queries only (no user input) → SAFE
        └─ User input in query → REQUIRED: Use parameterized queries
            └─ NEVER: format!("SELECT * FROM x WHERE id = {}", user_input)
```

#### Path Traversal Applicability

```
Does crate handle file paths from external input?
├─ NO → N/A (mark as ✅)
└─ YES → What is the input source?
    ├─ User-provided file names/paths
    │   └─ REQUIRED: Validate and sanitize
    │      ├─ Reject: "..", absolute paths, symlinks
    │      ├─ Use: canonicalize() + starts_with(allowed_root)
    │      └─ Consider: allowlist of valid characters
    ├─ API responses with file references
    │   └─ REQUIRED: Same validation as user input
    ├─ Config files (trusted source)
    │   └─ OPTIONAL: Validate if config could be user-modified
    └─ Internal/hardcoded paths
        └─ N/A (mark as ✅)
```

### Security Patterns

```rust
// Input validation at boundaries
pub fn process_input(input: &str) -> Result<Output, Error> {
    if input.len() > MAX_LENGTH {
        return Err(Error::InputTooLarge);
    }
    if !input.chars().all(|c| c.is_alphanumeric()) {
        return Err(Error::InvalidInput);
    }
    // Safe to process
    ...
}

// Never log secrets
error!(
    user = %request.user,
    // NOT: api_key = %request.api_key,
    "Authentication failed"
);

// Path traversal prevention
fn validate_path(user_path: &str, allowed_root: &Path) -> Result<PathBuf, Error> {
    let path = allowed_root.join(user_path);
    let canonical = path.canonicalize()?;
    if !canonical.starts_with(allowed_root) {
        return Err(Error::PathTraversal);
    }
    Ok(canonical)
}

// SQL injection prevention (parameterized query)
sqlx::query("SELECT * FROM users WHERE id = $1")
    .bind(user_id)  // Safe: parameterized
    .fetch_one(&pool)
    .await?;
```

### Questions to Answer

- [ ] What sensitive data does this crate handle?
- [ ] Are there any network boundaries?
- [ ] What are the trust assumptions?

---

## 11. Performance

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Connection pooling for external services | ⬜ | See decision tree below |
| No blocking calls in async context | ⬜ | See decision tree below |
| Proper resource cleanup (`Drop` impls) | ⬜ | See decision tree below |
| Bounded channels/queues | ⬜ | See decision tree below |
| Efficient serialization | ⬜ | |
| Caching where appropriate | ⬜ | See decision tree below |

### Decision Tree: Connection Pooling

```
Does the crate create connections to external services?
├─ NO → N/A (mark as ✅)
└─ YES → How are connections used?
    ├─ Single long-lived connection
    │   └─ OPTIONAL: Pool not needed, but consider reconnection
    ├─ Connection per request
    │   └─ REQUIRED: Use connection pool
    │      ├─ Database: sqlx pool, deadpool, bb8
    │      ├─ HTTP: reqwest Client (built-in pooling)
    │      └─ Custom: deadpool or bb8
    └─ Batch operations
        └─ RECOMMENDED: Pool with appropriate size
```

### Decision Tree: Blocking in Async

```
Does the crate use async?
├─ NO → N/A (mark as ✅)
└─ YES → What operations are performed?
    ├─ Network I/O only
    │   └─ SAFE: Use async clients (reqwest, sqlx, etc.)
    ├─ File I/O
    │   ├─ Small files, infrequent → ACCEPTABLE: std::fs
    │   └─ Large files or frequent → REQUIRED: tokio::fs
    ├─ CPU-intensive computation
    │   └─ REQUIRED: spawn_blocking() or rayon
    ├─ Synchronous library calls
    │   └─ REQUIRED: spawn_blocking()
    └─ Sleep/delays
        └─ REQUIRED: tokio::time::sleep (NOT std::thread::sleep)
```

### Decision Tree: Resource Cleanup (Drop)

```
Does the crate manage external resources?
├─ NO → N/A (mark as ✅)
└─ YES → What resources?
    ├─ File handles
    │   └─ AUTOMATIC: File implements Drop
    ├─ Network connections
    │   └─ RECOMMENDED: Custom Drop for graceful shutdown
    ├─ Temporary files/directories
    │   └─ REQUIRED: Drop impl to cleanup
    ├─ Thread handles
    │   └─ REQUIRED: Join or abort in Drop
    ├─ External process handles
    │   └─ REQUIRED: Kill/wait in Drop
    └─ Memory-mapped files, shared memory
        └─ REQUIRED: Proper cleanup in Drop
```

### Decision Tree: Bounded Channels

```
Does the crate use async channels?
├─ NO → N/A (mark as ✅)
└─ YES → What's the producer/consumer relationship?
    ├─ Producer faster than consumer possible
    │   └─ REQUIRED: Bounded channel
    │      └─ Size: based on expected backpressure tolerance
    ├─ Consumer always faster (guaranteed)
    │   └─ OPTIONAL: Unbounded acceptable if proven
    ├─ Unknown/variable rates
    │   └─ REQUIRED: Bounded channel with backpressure
    └─ Broadcast (one-to-many)
        └─ REQUIRED: Bounded, lagging receivers will miss
```

### Decision Tree: Caching

```
Does the crate perform repeated expensive operations?
├─ NO → N/A (mark as ✅)
└─ YES → What type of operations?
    ├─ External API calls (same params, same result)
    │   └─ RECOMMENDED: Cache with TTL
    ├─ Database lookups (read-heavy, rarely changing)
    │   └─ RECOMMENDED: Cache with invalidation strategy
    ├─ Computation (deterministic, pure functions)
    │   └─ OPTIONAL: Memoization if profiling shows benefit
    ├─ Config/metadata loading
    │   └─ RECOMMENDED: Load once, cache for lifetime
    └─ User-specific data
        └─ CAREFUL: Consider memory limits, use LRU
```

### Performance Patterns

```rust
// BAD: Blocking in async
async fn bad_read() -> String {
    std::fs::read_to_string("file.txt").unwrap() // Blocks!
}

// GOOD: Use async or spawn_blocking
async fn good_read() -> String {
    tokio::fs::read_to_string("file.txt").await.unwrap()
}

// GOOD: Connection pooling
pub struct Service {
    pool: Pool<PostgresConnectionManager>,
}

// GOOD: Bounded channels
let (tx, rx) = tokio::sync::mpsc::channel(100); // Bounded!
// NOT: tokio::sync::mpsc::unbounded_channel()

// GOOD: Resource cleanup
impl Drop for Connection {
    fn drop(&mut self) {
        // Cleanup resources
    }
}
```

### Questions to Answer

- [ ] What are the expected throughput requirements?
- [ ] Are there any hot paths that need optimization?
- [ ] What resources need explicit cleanup?

---

## 12. Health Checks (rustboot-health)

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Implements `HealthCheck` trait | ⬜ | See decision tree below |
| Liveness probe available | ⬜ | See decision tree below |
| Readiness probe available | ⬜ | See decision tree below |
| Critical vs non-critical checks defined | ⬜ | |
| Health metadata included | ⬜ | Version, dependencies |

### Decision Tree: HealthCheck Implementation

```
Does the crate represent a service or provider?
├─ NO → N/A (mark as ✅)
└─ YES → What does it connect to?
    ├─ Nothing (pure computation)
    │   └─ OPTIONAL: Simple liveness check
    ├─ Database
    │   └─ REQUIRED: HealthCheck with connection test
    │      └─ Use: SELECT 1 or ping
    ├─ External API
    │   └─ REQUIRED: HealthCheck with endpoint test
    │      └─ Use: HEAD request or health endpoint
    ├─ Message queue
    │   └─ REQUIRED: HealthCheck with connection test
    └─ Multiple dependencies
        └─ REQUIRED: Composite check with:
           ├─ Individual checks per dependency
           ├─ Critical flag per check
           └─ Aggregate status
```

### Decision Tree: Liveness vs Readiness

```
What type of probe is needed?
├─ Liveness (is the app alive?)
│   └─ Check: Can the process respond?
│      ├─ Simple: return true
│      ├─ Better: verify main loop running
│      └─ Fail: restart the container/process
├─ Readiness (can it serve traffic?)
│   └─ Check: Are all dependencies available?
│      ├─ Database connected?
│      ├─ Cache warmed?
│      ├─ Config loaded?
│      └─ Fail: remove from load balancer
└─ Both needed?
    └─ Implement HealthCheck trait
       └─ Use for both via HealthAggregator
```

### Health Check Pattern

```rust
use rustboot_health::{HealthCheck, CheckResult, HealthStatus};

struct DatabaseHealthCheck {
    pool: Pool,
}

#[async_trait]
impl HealthCheck for DatabaseHealthCheck {
    fn name(&self) -> &str {
        "database"
    }

    async fn check(&self) -> CheckResult {
        match self.pool.ping().await {
            Ok(_) => CheckResult::healthy("database"),
            Err(e) => CheckResult::unhealthy("database", e.to_string()),
        }
    }

    fn is_critical(&self) -> bool {
        true  // App can't function without DB
    }
}
```

---

## 13. Resilience Patterns (rustboot-resilience)

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Circuit breaker for external calls | ⬜ | See decision tree below |
| Retry with backoff | ⬜ | See decision tree below |
| Timeout on all external calls | ⬜ | See decision tree below |
| Bulkhead isolation | ⬜ | See decision tree below |
| Fallback strategies defined | ⬜ | |

### Decision Tree: Circuit Breaker

```
Does the crate call external services?
├─ NO → N/A (mark as ✅)
└─ YES → What happens if the service is down?
    ├─ App fails completely
    │   └─ REQUIRED: Circuit breaker
    │      └─ Fail fast when service is known down
    ├─ Degraded operation possible
    │   └─ REQUIRED: Circuit breaker + fallback
    │      └─ Return cached/default when open
    └─ Retries sufficient
        └─ OPTIONAL: Consider for high-volume calls
           └─ Prevents cascade failures
```

### Decision Tree: Timeout Configuration

```
Does the operation involve I/O?
├─ NO → N/A (mark as ✅)
└─ YES → What type?
    ├─ HTTP request
    │   └─ REQUIRED: Connect + read timeout
    │      └─ Typical: 5s connect, 30s read
    ├─ Database query
    │   └─ REQUIRED: Statement timeout
    │      └─ Typical: 30s for queries, 5s for simple ops
    ├─ Message queue
    │   └─ REQUIRED: Publish/consume timeout
    ├─ File I/O
    │   └─ OPTIONAL: Usually fast enough
    └─ gRPC
        └─ REQUIRED: Per-call deadline
```

### Decision Tree: Bulkhead Isolation

```
Does the crate handle multiple resource types?
├─ NO → N/A (mark as ✅)
└─ YES → Can one resource exhaust shared capacity?
    ├─ NO → N/A (mark as ✅)
    └─ YES → What resources?
        ├─ Thread pools
        │   └─ REQUIRED: Separate pools per concern
        ├─ Connection pools
        │   └─ REQUIRED: Separate pools per service
        ├─ Memory/buffers
        │   └─ REQUIRED: Bounded allocations
        └─ Rate limits
            └─ REQUIRED: Per-tenant limits
```

### Resilience Pattern

```rust
use rustboot_resilience::{
    CircuitBreaker, CircuitBreakerConfig,
    RetryPolicy, ExponentialBackoff,
};

// Circuit breaker configuration
let circuit = CircuitBreaker::new(CircuitBreakerConfig {
    failure_threshold: 5,
    success_threshold: 2,
    timeout: Duration::from_secs(30),
});

// Retry with exponential backoff
let backoff = ExponentialBackoff::new(
    Duration::from_millis(100),  // initial
    Duration::from_secs(10),      // max
    2.0,                          // multiplier
);
let retry = RetryPolicy::new(3).with_backoff(backoff);

// Combined usage
async fn call_api() -> Result<Response, Error> {
    circuit.call(|| {
        retry.execute(|| {
            with_timeout(Duration::from_secs(5), api_call())
        })
    }).await
}
```

---

## 14. Metrics & Observability (rustboot-observability)

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Key operations have counters | ⬜ | See decision tree below |
| Latency histograms for I/O | ⬜ | |
| Error rate tracking | ⬜ | |
| Business metrics exposed | ⬜ | |
| Trace context propagated | ⬜ | See decision tree below |

### Decision Tree: Metrics Implementation

```
What type of crate is this?
├─ Library (no runtime)
│   └─ OPTIONAL: Allow passing Metrics trait
├─ Service/Provider
│   └─ REQUIRED: Instrument with metrics
│      ├─ Counters: requests, errors, retries
│      ├─ Gauges: connections, queue depth
│      └─ Histograms: latency, payload size
└─ CLI tool
    └─ OPTIONAL: Usually not needed
```

### Decision Tree: Trace Context

```
Does the crate participate in distributed calls?
├─ NO → N/A (mark as ✅)
└─ YES → Role in the call chain?
    ├─ Entry point (receives requests)
    │   └─ REQUIRED: Extract trace context from headers
    ├─ Middle (calls other services)
    │   └─ REQUIRED: Propagate trace context
    │      └─ Inject into outgoing requests
    └─ Leaf (no outgoing calls)
        └─ RECOMMENDED: Log with trace ID
```

### Metrics Pattern

```rust
use rustboot_observability::{Counter, Gauge, Metrics};

struct ServiceMetrics<M: Metrics> {
    requests: Counter,
    errors: Counter,
    active_connections: Gauge,
}

impl<M: Metrics> ServiceMetrics<M> {
    fn new(metrics: &M) -> Self {
        Self {
            requests: metrics.counter("service_requests_total"),
            errors: metrics.counter("service_errors_total"),
            active_connections: metrics.gauge("service_connections_active"),
        }
    }
}

// Usage in handler
async fn handle_request(&self) -> Result<Response, Error> {
    self.metrics.requests.increment();
    self.metrics.active_connections.increment();

    let result = self.do_work().await;

    self.metrics.active_connections.decrement();
    if result.is_err() {
        self.metrics.errors.increment();
    }
    result
}
```

---

## 15. SEA Architecture Compliance (Rustratify)

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Correct layer structure | ⬜ | See decision tree below |
| No upward dependencies | ⬜ | Lower layers don't depend on higher |
| Provider trait in SPI layer | ⬜ | |
| Consumer types in API layer | ⬜ | |
| Facade re-exports correctly | ⬜ | |

### Decision Tree: SEA Layer Placement

```
What type of code is being added?
├─ Core types (IDs, enums, constants)
│   └─ L1: Models layer (*-models or *-spi/models.rs)
├─ Provider/backend traits
│   └─ L2: SPI layer (*-spi)
│      └─ Trait + ProviderError + ProviderConfig
├─ Consumer-facing types and traits
│   └─ L3: API layer (*-api)
│      └─ DTOs, ServiceError, ServiceTrait
├─ Implementation logic
│   └─ L4: Core layer (*-core)
│      └─ DefaultService, provider implementations
└─ Public API and factory functions
    └─ L5: Facade layer (swe-*)
       └─ Re-exports, create_*() functions
```

### SEA Dependency Rules

```
Layer dependencies (→ = "may depend on"):

L5 Facade → L4 Core → L3 API → L2 SPI → L1 Models
                ↓         ↓        ↓
              L3 API   L2 SPI   L1 Models

NEVER:
- L1 → L2/L3/L4/L5
- L2 → L3/L4/L5
- L3 → L4/L5
- L4 → L5
```

### SEA File Structure

```
swe-{name}/
├── {name}-spi/          # L2: Provider interface
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models.rs    # L1: Core types (can be separate crate)
│   │   ├── errors.rs    # ProviderError
│   │   ├── config.rs    # ProviderConfig
│   │   └── traits.rs    # Provider trait
│   └── Cargo.toml
├── {name}-api/          # L3: Consumer interface
│   ├── src/
│   │   ├── lib.rs
│   │   ├── models.rs    # Consumer DTOs
│   │   ├── errors.rs    # ServiceError (wraps ProviderError)
│   │   └── traits.rs    # Service trait
│   └── Cargo.toml
├── {name}-core/         # L4: Implementation
│   ├── src/
│   │   ├── lib.rs
│   │   ├── service.rs   # DefaultService
│   │   └── providers/   # Provider implementations
│   └── Cargo.toml
└── swe-{name}/          # L5: Facade
    ├── src/
    │   └── lib.rs       # Re-exports + factories
    └── Cargo.toml
```

---

## 16. Feature Flags

### Checklist

| Item | Status | Notes |
|------|--------|-------|
| Optional dependencies behind features | ⬜ | See decision tree below |
| Feature documentation in Cargo.toml | ⬜ | |
| No feature conflicts | ⬜ | |
| Default features sensible | ⬜ | |
| Feature-gated tests | ⬜ | |

### Decision Tree: Feature Flag Usage

```
Should this dependency be optional?
├─ Core functionality
│   └─ NO: Required dependency
├─ Provider implementation
│   └─ YES: Feature per provider
│      └─ e.g., "twilio", "asterisk"
├─ Integration with external crate
│   └─ YES: Feature for integration
│      └─ e.g., "serde", "tokio"
├─ Development/testing only
│   └─ Use dev-dependencies instead
└─ Large dependency tree
    └─ YES: Keep optional to reduce compile time
```

### Feature Naming Convention

```toml
[features]
default = ["runtime-tokio"]

# Runtime features
runtime-tokio = ["tokio"]
runtime-async-std = ["async-std"]

# Provider features
provider-twilio = ["reqwest"]
provider-asterisk = ["tokio-tungstenite"]

# Integration features
serde = ["dep:serde", "dep:serde_json"]
secrets = ["swe-secrets"]

# Testing features (not in default)
testing = []  # Exports mocks for downstream
```

---

## Summary

### Priority Actions

| Priority | Action | Effort | Status |
|----------|--------|--------|--------|
| High | | | ⬜ |
| High | | | ⬜ |
| Medium | | | ⬜ |
| Medium | | | ⬜ |
| Low | | | ⬜ |

### Lines of Code Impact

| Category | Current | After Fixes | Savings |
|----------|---------|-------------|---------|
| Boilerplate | | | |
| Config structs | | | |
| Error handling | | | |
| **Total** | | | |

---

## Reviewer Notes

*Add any additional observations or recommendations here.*
