# FR-{###} Troubleshooting

**FR:** [FR-{###}](../../../../1-requirements/FR-{###}-{name}.md)

## Common Issues

### Issue 1: {Issue Title}

#### Symptoms
- {symptom 1}
- {symptom 2}

#### Cause
{Explanation of what causes this issue.}

#### Solution
```typescript
// Solution code or steps
```

#### Prevention
{How to prevent this issue in the future.}

---

### Issue 2: {Issue Title}

#### Symptoms
- {symptom 1}

#### Cause
{Explanation}

#### Solution
{Steps to resolve}

---

## Error Messages

### `{Error message}`

**Cause:** {explanation}

**Solution:** {how to fix}

---

## Debugging

### Enable Debug Mode

```typescript
// In browser console
localStorage.setItem('debug:{feature}', 'true');

// Then refresh
```

### View Store State

```typescript
// In browser console
import { use{Feature}Store } from '@/stores/{feature}Store';
console.log(use{Feature}Store.getState());
```

### React DevTools

1. Install React DevTools extension
2. Open DevTools > Components
3. Search for `{ComponentName}`
4. Inspect props and state

## Performance Issues

### Issue: {Performance Issue}

**Symptoms:**
- {symptom}

**Diagnosis:**
1. Open DevTools > Performance
2. Record interaction
3. Look for {indicator}

**Solution:**
{solution}

## Getting Help

1. Check this troubleshooting guide
2. Check browser console for errors
3. Review [Configuration](configuration.md)
4. Search [existing issues](https://github.com/{org}/{repo}/issues)

## See Also

- [Configuration](configuration.md)
- [Architecture](../3-design/architecture.md)
