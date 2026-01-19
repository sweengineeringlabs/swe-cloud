# Test Report Feature Implementation Plan

## Overview
Implement a test report feature that displays test results in a nicely formatted table within the swe-cloud-ui application.

## Data Source
The test runner already outputs JSON to `test-results/results.json`:
```json
{
  "summary": {
    "total": 346,
    "passed": 346,
    "failed": 0,
    "skipped": 0,
    "timed_out": 0,
    "duration_ms": 1234
  },
  "tests": [
    {
      "name": "test_name",
      "file": "path/to/test.test.rsx",
      "category": "feature",
      "status": "passed",
      "duration_ms": 45,
      "error": null
    }
  ]
}
```

## Implementation Steps

### 1. Create Test Report Types (`src/features/testing/types/report.type.rsx`)
Define type structures for test results:
- `TestSummary` - total, passed, failed, skipped, timed_out, duration_ms
- `TestResult` - name, file, category, status, duration_ms, error
- `TestReport` - summary + tests array

### 2. Create Test Report Table Component (`src/features/testing/components/test-report-table.component.rsx`)
A table component following the existing RequestTable pattern:
- Summary header with pass/fail/skip counts
- Status badges (passed=green, failed=red, skipped=yellow)
- Category column (feature, e2e, integration)
- Duration column formatted nicely
- Expandable error details for failed tests
- Sortable columns

### 3. Create Test Report Page (`src/features/testing/pages/report.page.rsx`)
A page that:
- Loads test results from JSON file or API
- Displays the TestReportTable component
- Shows overall summary statistics
- Supports filtering by category and status

### 4. Add CSS Styles (`src/styles/main.css`)
Add styles for:
- `.test-report-table` - table styling
- `.test-status-badge` - status indicator badges
- `.test-category-badge` - category indicator
- `.test-summary-card` - summary statistics card

### 5. Register Feature and Route
- Add feature to features registry
- Add route for `/testing/report`

## Component Structure
```
src/features/testing/
├── components/
│   └── test-report-table.component.rsx
├── pages/
│   └── report.page.rsx
├── types/
│   └── report.type.rsx
└── tests/
    └── feature/
        └── test-report-table.test.rsx
```

## Visual Design
```
┌─────────────────────────────────────────────────────────────┐
│  Test Report                                                │
│  ┌──────────┬──────────┬──────────┬──────────┬───────────┐ │
│  │  Total   │  Passed  │  Failed  │ Skipped  │  Duration │ │
│  │   346    │   340    │    4     │    2     │   1.2s    │ │
│  └──────────┴──────────┴──────────┴──────────┴───────────┘ │
│                                                             │
│  Filter: [All ▼] [feature ▼] [Search...]                   │
│                                                             │
│  ┌─────────────────────────────────────────────────────────┐│
│  │ Status │ Test Name        │ Category │ File   │ Time   ││
│  ├────────┼──────────────────┼──────────┼────────┼────────┤│
│  │ ✓ PASS │ renders_correctly│ feature  │ btn... │  45ms  ││
│  │ ✗ FAIL │ handles_error    │ e2e      │ api... │ 120ms  ││
│  │ ○ SKIP │ pending_feature  │ feature  │ new... │   0ms  ││
│  └─────────────────────────────────────────────────────────┘│
└─────────────────────────────────────────────────────────────┘
```

## Files to Create/Modify
1. **NEW**: `src/features/testing/types/report.type.rsx`
2. **NEW**: `src/features/testing/components/test-report-table.component.rsx`
3. **NEW**: `src/features/testing/pages/report.page.rsx`
4. **MODIFY**: `src/styles/main.css` - add test report styles
5. **NEW**: `src/features/testing/tests/feature/test-report-table.test.rsx`
