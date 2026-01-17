# FR-{###} Architecture

**FR:** [FR-{###}](../../../../1-requirements/FR-{###}-{name}.md)

## Overview

{High-level description of the feature architecture.}

## Component Diagram

```
┌─────────────────────────────────────────────────────────────┐
│                        App.tsx                               │
└─────────────────────────────────────────────────────────────┘
                              │
              ┌───────────────┼───────────────┐
              ▼               ▼               ▼
┌─────────────────┐ ┌─────────────────┐ ┌─────────────────┐
│   Component A   │ │   Component B   │ │   Component C   │
└─────────────────┘ └─────────────────┘ └─────────────────┘
              │               │               │
              └───────────────┼───────────────┘
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                        Store                                 │
│  ┌─────────────────────────────────────────────────────┐    │
│  │                  Zustand Store                       │    │
│  │  state: { ... }                                      │    │
│  │  actions: { ... }                                    │    │
│  └─────────────────────────────────────────────────────┘    │
└─────────────────────────────────────────────────────────────┘
```

## Components

### {ComponentName}

**Location:** `src/components/{path}/{ComponentName}.tsx`

**Purpose:** {description}

**Props:**
```typescript
interface {ComponentName}Props {
  prop1: Type;
  prop2?: Type;
}
```

**State:**
- Uses `use{Store}Store` for {what}

### {ComponentName2}

**Location:** `src/components/{path}/{ComponentName2}.tsx`

**Purpose:** {description}

## State Management

### Store: `{storeName}Store`

**Location:** `src/stores/{storeName}Store.ts`

```typescript
interface {StoreName}State {
  // State
  field1: Type;
  field2: Type;

  // Actions
  action1: () => void;
  action2: (param: Type) => Promise<void>;
}
```

### Hooks

| Hook | Purpose |
|------|---------|
| `use{Feature}` | {description} |
| `use{Feature}Actions` | {description} |

## Data Flow

```
┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐
│   User   │───▶│    UI    │───▶│  Action  │───▶│  Store   │
│  Action  │    │Component │    │          │    │  Update  │
└──────────┘    └──────────┘    └──────────┘    └──────────┘
                                                      │
                                                      ▼
                                               ┌──────────┐
                                               │ Re-render│
                                               └──────────┘
```

## Design Decisions

See [ADR folder](adr/) for architecture decision records.

## Integration Points

| System | Integration | Notes |
|--------|-------------|-------|
| {component} | {how} | {notes} |

## See Also

- [API Reference](api-reference.md)
- [Workflow](workflow.md)
