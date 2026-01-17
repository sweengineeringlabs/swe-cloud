# FR-{###} Configuration

**FR:** [FR-{###}](../../../../1-requirements/FR-{###}-{name}.md)

## Overview

Configuration options for FR-{###}.

## Configuration Interface

```typescript
interface {Feature}Config {
  /** {description} */
  option1: Type;

  /** {description} */
  option2?: Type;
}
```

## Options

### `option1`

| Property | Value |
|----------|-------|
| Type | `{Type}` |
| Default | `{default}` |
| Required | {Yes/No} |

{Description of what this option does.}

**Example:**
```typescript
const config: {Feature}Config = {
  option1: value,
};
```

### `option2`

| Property | Value |
|----------|-------|
| Type | `{Type}` |
| Default | `{default}` |
| Required | No |

{Description}

## Settings UI

Configuration is available in Settings > {Section}:

| Setting | Config Field | UI Element |
|---------|--------------|------------|
| {Label} | `option1` | {Dropdown/Toggle/Input} |

## Programmatic Configuration

```typescript
import { use{Feature}Store } from '@/stores/{feature}Store';

// Update configuration
const { updateConfig } = use{Feature}Store();
updateConfig({
  option1: value,
});
```

## Persistence

Configuration is persisted via:
- **localStorage:** `swe-studio-{feature}`
- **Store:** Zustand persist middleware

## Environment Variables

For development/testing:

| Variable | Config Field | Description |
|----------|--------------|-------------|
| `VITE_{FEATURE}_OPTION1` | `option1` | {description} |

## See Also

- [Troubleshooting](troubleshooting.md)
- [Architecture](../3-design/architecture.md)
