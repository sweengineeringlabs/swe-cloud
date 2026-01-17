# {Module Name} Architecture

**FR:** [FR-{###}](../../../../doc/1-requirements/FR-{###}-{name}.md)

## Overview

{High-level description of the module architecture.}

## Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                         Facade                               │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                     Prelude                          │    │
│  │  pub use {module}_api::*;                           │    │
│  │  pub use {module}_core::*;                          │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                          Core                                │
│  ┌─────────────────────────────────────────────────────┐    │
│  │              Implementation                          │    │
│  │  impl {Trait} for {Impl} { ... }                    │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┴───────────────┐
              ▼                               ▼
┌─────────────────────────┐     ┌─────────────────────────┐
│           API           │     │           SPI           │
│  ┌───────────────────┐  │     │  ┌───────────────────┐  │
│  │   Public Traits   │  │     │  │  Extension Traits │  │
│  │   trait {Name}    │  │     │  │  trait {Provider} │  │
│  └───────────────────┘  │     │  └───────────────────┘  │
└─────────────────────────┘     └─────────────────────────┘
              │                               │
              └───────────────┬───────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                         Common                               │
│  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────┐  │
│  │    Types    │  │   Errors    │  │     Constants       │  │
│  └─────────────┘  └─────────────┘  └─────────────────────┘  │
└─────────────────────────────────────────────────────────────┘
```

## Layer Responsibilities

### Common (`{module}-common`)

**Purpose:** Shared types with no business logic.

**Contains:**
- DTOs (Data Transfer Objects)
- Error types
- Constants
- Serialization traits

**Dependencies:** External only (serde, thiserror)

### SPI (`{module}-spi`)

**Purpose:** Extension points for plugins.

**Contains:**
- Provider traits
- Extension interfaces
- Plugin contracts

**Dependencies:** `{module}-common`

### API (`{module}-api`)

**Purpose:** Public interface for consumers.

**Contains:**
- Consumer traits
- Registry interfaces
- Public types

**Dependencies:** `{module}-common`, `{module}-spi`

### Core (`{module}-core`)

**Purpose:** Business logic implementation.

**Contains:**
- Trait implementations
- Internal logic
- Unit tests (inline)
- Integration tests (`tests/`)

**Dependencies:** `{module}-common`, `{module}-spi`, `{module}-api`

### Facade (`{module}`)

**Purpose:** Entry point and re-exports.

**Contains:**
- Prelude module
- Re-exports of public API
- Convenience constructors

**Dependencies:** All layers

## Data Flow

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│  Caller  │───▶│  Facade  │───▶│   Core   │───▶│   SPI    │
└──────────┘    └──────────┘    └──────────┘    └──────────┘
                                      │               │
                                      ▼               ▼
                                ┌──────────┐    ┌──────────┐
                                │   API    │    │ Provider │
                                └──────────┘    └──────────┘
```

## Design Decisions

See [ADR folder](adr/) for architecture decision records.

## Integration Points

| System | Integration | Notes |
|--------|-------------|-------|
| {system} | {how} | {notes} |

## See Also

- [Overview](../overview.md)
- [Integration Guide](integration.md)
