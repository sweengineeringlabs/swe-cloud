# {Module Name}

> **Scope**: High-level overview only. Implementation details belong in [Developer Guide](4-development/developer-guide.md).

## WHAT

{1-2 sentence description of what this module provides}

Key capabilities:
- **{Capability 1}** - Brief description
- **{Capability 2}** - Brief description
- **{Capability 3}** - Brief description

## WHY

| Problem | Solution |
|---------|----------|
| {Problem 1} | {How this module solves it} |
| {Problem 2} | {How this module solves it} |
| {Problem 3} | {How this module solves it} |

## HOW

### Architecture

```
┌─────────────┐
│   Facade    │  ← Entry point
├─────────────┤
│    Core     │  ← Business logic
├─────────────┤
│  API / SPI  │  ← Contracts
├─────────────┤
│   Common    │  ← Types, errors
└─────────────┘
```

### Quick Start

```rust
use {module_name}::prelude::*;

let service = {Service}::new(config);
let result = service.do_something(&input)?;
```

### Layer Responsibilities

| Layer | Purpose |
|-------|---------|
| `{module}-common` | Shared types, errors |
| `{module}-spi` | Provider/extension traits |
| `{module}-api` | Consumer traits |
| `{module}-core` | Implementation |
| `{module}` | Facade, prelude |

## Documentation

| Document | Description |
|----------|-------------|
| [Architecture](3-design/architecture.md) | System design, diagrams |
| [Integration](3-design/integration.md) | How to integrate |
| [Testing Strategy](5-testing/strategy.md) | Test approach |

---

**Status**: {Stable / Beta / Alpha}
