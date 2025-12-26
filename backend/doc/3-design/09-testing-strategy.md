# 09 - Testing Strategy

## Document Information

| Field | Value |
|-------|-------|
| **Version** | 1.0.0 |
| **Status** | Design Complete |
| **Last Updated** | 2025-12-26 |

---

## 1. Testing Pyramid

```
                         /\
                        /  \
                       / E2E\      Few, slow
                      /______\
                     /        \
                    /Integration\ Medium
                   /______________\
                  /                \
                 /   Unit Tests     \  Many, fast
                /____________________\
```

## 2. Mock Storage Pattern

```rust
pub struct MockStorage {
    buckets: Arc<Mutex<HashMap<String, HashMap<String, Vec<u8>>>>>
}

#[async_trait]
impl ObjectStorage for MockStorage {
    async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> CloudResult<()> {
        let mut buckets = self.buckets.lock().unwrap();
        buckets.entry(bucket.to_string()).or_default()
            .insert(key.to_string(), data.to_vec());
        Ok(())
    }
}
```

## 3. Test Commands

```bash
cargo test                           # All tests
cargo test -p cloudkit               # Specific crate
cargo test -- --ignored              # E2E tests
cargo tarpaulin --out Html           # Coverage
```

## 4. Local Emulators

- **LocalStack** (AWS): `docker run -p 4566:4566 localstack/localstack`
- **Azurite** (Azure): `docker run -p 10000:10000 mcr.microsoft.com/azure-storage/azurite`

## 5. Coverage Goals

| Component | Target |
|-----------|--------|
| Common | 90%+ |
| Core | 85%+ |
| Providers | 75%+ |
