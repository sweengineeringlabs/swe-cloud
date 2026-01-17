# {Module Name} Testing Strategy

**FR:** [FR-{###}](../../../../1-requirements/FR-{###}-{name}.md)

## Overview

Testing strategy for {module-name}, extending the base testing strategy.

## Test Layers

| Layer | Type | Mocking | Location |
|-------|------|---------|----------|
| Common | Unit | N/A | `{module}-common/src/` (inline) |
| SPI | Unit | N/A | `{module}-spi/src/` (inline) |
| API | Unit | N/A | `{module}-api/src/` (inline) |
| Core | Unit | Mock dependencies | `{module}-core/src/` (inline) |
| Core | Integration | Real components | `{module}-core/tests/` |
| Facade | E2E | Real + fixtures | `{module}/tests/` |

## Unit Tests

### Location
```
{module}-core/src/
├── implementation.rs
│   └── #[cfg(test)] mod tests { ... }  // Inline tests
```

### Mocking Strategy
- Mock all dependencies via traits
- Use `mockall` or similar for trait mocking
- No mocking of internal implementation details

### Example
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mockall::predicate::*;

    #[test]
    fn test_something() {
        let mut mock = MockDependency::new();
        mock.expect_method()
            .returning(|_| Ok(()));

        let sut = Implementation::new(mock);
        assert!(sut.do_something().is_ok());
    }
}
```

## Integration Tests

### Location
```
{module}-core/tests/
├── integration.rs
└── helpers/
    └── mod.rs
```

### Strategy
- Use real components, no mocks
- Test component interactions
- May use test fixtures for data

### Example
```rust
// {module}-core/tests/integration.rs
use {module_name}_core::*;

#[test]
fn test_integration() {
    let real_dep = RealDependency::new();
    let sut = Implementation::new(real_dep);

    assert!(sut.full_workflow().is_ok());
}
```

## E2E Tests

### Location
```
{module}/tests/
├── e2e.rs
└── fixtures/
    └── data.json
```

### Strategy
- Test full module from facade
- Use fixtures for test data
- May interact with external systems (sandboxed)

## Test Coverage

| Component | Target | Current |
|-----------|--------|---------|
| `{module}-common` | 80% | {X}% |
| `{module}-spi` | 80% | {X}% |
| `{module}-api` | 80% | {X}% |
| `{module}-core` | 90% | {X}% |

## Running Tests

```bash
# All tests
cargo test -p {module-name}

# Unit tests only
cargo test -p {module-name}-core --lib

# Integration tests only
cargo test -p {module-name}-core --test integration

# E2E tests
cargo test -p {module-name} --test e2e
```

## See Also

- [Architecture](../../3-design/architecture.md)
- [Overview](../../overview.md)
