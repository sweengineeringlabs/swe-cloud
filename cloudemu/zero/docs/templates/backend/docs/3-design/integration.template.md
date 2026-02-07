# {Module Name} Integration Guide

**FR:** [FR-{###}](../../../../1-requirements/FR-{###}-{name}.md)

## Overview

How to integrate {module-name} with other systems.

## Quick Start

### Add Dependency

```toml
# Cargo.toml
[dependencies]
{module-name} = { path = "../{module-name}/{module-name}" }
```

### Basic Usage

```rust
use {module_name}::prelude::*;

fn main() {
    // Initialize
    let instance = {Module}::new(config);

    // Use
    let result = instance.do_something()?;
}
```

## Integration Patterns

### Pattern 1: {Pattern Name}

{Description}

```rust
// Example code
```

### Pattern 2: {Pattern Name}

{Description}

```rust
// Example code
```

## Configuration

| Option | Type | Default | Description |
|--------|------|---------|-------------|
| `{option}` | `{type}` | `{default}` | {description} |

## Error Handling

```rust
use {module_name}::error::{ModuleError, Result};

fn handle_errors() -> Result<()> {
    match some_operation() {
        Ok(value) => { /* success */ },
        Err(ModuleError::{Variant}) => { /* handle */ },
    }
}
```

## Integration with Other Modules

### {Other Module}

```rust
// How to use with {other module}
```

## Testing Integration

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        // Test code
    }
}
```

## Troubleshooting

| Issue | Cause | Solution |
|-------|-------|----------|
| {issue} | {cause} | {solution} |

## See Also

- [Architecture](architecture.md)
- [Overview](../overview.md)
