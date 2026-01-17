# {Module Name} Configuration

**FR:** [FR-{###}](../../../../1-requirements/FR-{###}-{name}.md)

## Overview

Configuration options for {module-name}.

## Configuration Structure

```rust
pub struct {Module}Config {
    /// {description}
    pub option_1: Type,

    /// {description}
    pub option_2: Type,
}

impl Default for {Module}Config {
    fn default() -> Self {
        Self {
            option_1: default_value,
            option_2: default_value,
        }
    }
}
```

## Options

### `option_1`

| Property | Value |
|----------|-------|
| Type | `{Type}` |
| Default | `{default}` |
| Required | {Yes/No} |
| Environment | `{MODULE}_OPTION_1` |

{Description of what this option does.}

**Example:**
```rust
config.option_1 = value;
```

### `option_2`

| Property | Value |
|----------|-------|
| Type | `{Type}` |
| Default | `{default}` |
| Required | {Yes/No} |
| Environment | `{MODULE}_OPTION_2` |

{Description of what this option does.}

## Environment Variables

| Variable | Config Field | Description |
|----------|--------------|-------------|
| `{MODULE}_OPTION_1` | `option_1` | {description} |
| `{MODULE}_OPTION_2` | `option_2` | {description} |

## Configuration File

```toml
# config.toml
[{module}]
option_1 = "value"
option_2 = 123
```

## Programmatic Configuration

```rust
use {module_name}::prelude::*;

let config = {Module}Config {
    option_1: value,
    option_2: value,
    ..Default::default()
};

let instance = {Module}::with_config(config);
```

## Validation

Configuration is validated at construction time:

```rust
impl {Module}Config {
    pub fn validate(&self) -> Result<(), ConfigError> {
        // Validation rules
    }
}
```

## See Also

- [Troubleshooting](troubleshooting.md)
- [Integration Guide](../3-design/integration.md)
