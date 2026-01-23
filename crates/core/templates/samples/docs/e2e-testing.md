# E2E Testing Guide

## Setup

### 1. Install Dependencies

```bash
cd e2e
npm install
npx playwright install chromium
```

### 2. Environment Variables

```bash
# e2e/.env
TEST_SECRET=your-test-secret
```

### 3. Enable Test Endpoints

Run backend in test mode:

```bash
TEST_MODE=true cargo run -p {{project_name}}-cli
```

Test mode enables:
- `POST /api/test/reset` - Reset database
- `POST /api/test/seed` - Seed test data

## Running Tests

### All Tests

```bash
# From project root
make e2e

# Or from e2e directory
cd e2e && npx playwright test
```

### Specific Tests

```bash
# Posts tests only
npx playwright test posts.spec.ts

# Specific test case
npx playwright test -g "Writer - can create a post"
```

### Debug Mode

```bash
# UI mode
npx playwright test --ui

# Headed mode
npx playwright test --headed

# Debug mode
npx playwright test --debug
```

## Test Report

View report after tests:

```bash
npx playwright show-report
```

Report location:
- HTML: `e2e/playwright-report/index.html`
- JSON: `e2e/test-results.json`

## Test Scenarios

### Permission Matrix

| Test | Admin | Writer | Reader |
|------|-------|--------|--------|
| View post list | ✓ | ✓ | ✓ |
| Create post | ✓ | ✓ | ✗ (403) |
| Edit own post | ✓ | ✓ | ✗ |
| Edit others post | ✓ | ✗ | ✗ |
| Create comment | ✓ | ✓ | ✓ |
| Edit own comment | ✓ | ✗ | ✓ |
| Edit others comment | ✓ | ✗ | ✗ |
| Upload file | ✓ | ✓ | ✓ |
| Delete others file | ✓ | ✗ | ✗ |

## CI/CD

```yaml
# .github/workflows/e2e.yml
name: E2E Tests

on:
  push:
    branches: [main]
  pull_request:

jobs:
  e2e:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_DB: board_test
          POSTGRES_USER: test
          POSTGRES_PASSWORD: test
        ports:
          - 5432:5432

    steps:
      - uses: actions/checkout@v4

      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: 20

      - name: Install Playwright
        working-directory: e2e
        run: |
          npm ci
          npx playwright install chromium --with-deps

      - name: Run E2E Tests
        working-directory: e2e
        run: npx playwright test

      - uses: actions/upload-artifact@v4
        if: failure()
        with:
          name: playwright-report
          path: e2e/playwright-report
```

## Troubleshooting

### Test Failures

1. **Check screenshots**: `e2e/test-results/`
2. **Check videos**: Failed test videos
3. **Check trace**: `npx playwright show-trace trace.zip`

### Common Issues

| Issue | Solution |
|-------|----------|
| Server timeout | Increase `webServer.timeout` |
| Auth state corrupted | Delete `.auth/` folder |
| DB state mismatch | Run `make db-reset` |
