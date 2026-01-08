# {{project_name}} - Board Sample

A full-stack board (게시판) application generated with [boots](https://github.com/1eedaegon/boots).

## Quick Start

### Prerequisites

- Rust 1.75+
- Node.js 20+
- Docker & Docker Compose

### Development Mode (Recommended)

```bash
# 1. Install dependencies
make setup

# 2. Start database (Docker)
make db-up

# 3. Create seed data
make seed

# 4. Run backend server (localhost:8080)
make run-be

# 5. Run frontend dev server (localhost:3000)
make run-fe
```

### Docker Compose (Production Mode)

```bash
docker compose up --build
```

## Test Accounts

| Role | Email | Password | Permissions |
|------|-------|----------|-------------|
| Admin | admin@example.com | admin123 | Edit/delete all posts and comments |
| Writer | writer@example.com | writer123 | Edit/delete own posts only |
| Reader | reader@example.com | reader123 | Edit/delete own comments only |

## Permission Matrix

| Resource | Action | Admin | Writer | Reader |
|----------|--------|-------|--------|--------|
| Post | View | ✓ | ✓ | ✓ |
| Post | Create | ✓ | ✓ | ✗ |
| Post | Edit | ✓ | Own only | ✗ |
| Post | Delete | ✓ | Own only | ✗ |
| Comment | View | ✓ | ✓ | ✓ |
| Comment | Create | ✓ | ✓ | ✓ |
| Comment | Edit | ✓ | ✗ | Own only |
| Comment | Delete | ✓ | ✗ | Own only |

## API Documentation

API docs available at:
- Swagger UI: http://localhost:8080/swagger-ui
- OpenAPI JSON: http://localhost:8080/api-docs/openapi.json

See [docs/api.md](docs/api.md) for detailed API documentation.

## Testing

```bash
# Unit tests
make test

# E2E tests
make e2e

# All tests
make test-all
```

## Project Structure

```
{{project_name}}/
├── crates/
│   ├── api/          # API handlers (Axum)
│   ├── core/         # Domain models & business logic
│   │   └── src/
│   │       └── board/  # Board module (Post, Comment, Permission)
│   ├── cli/          # CLI entry point
│   └── persistence/  # Database (SQLx)
├── frontend/         # React SPA
├── e2e/              # E2E tests (Playwright)
├── docs/             # Documentation
│   ├── api.md
│   ├── architecture.md
│   └── e2e-testing.md
└── docker-compose.yml
```

## Documentation

- [API Documentation](docs/api.md)
- [Architecture](docs/architecture.md)
- [E2E Testing Guide](docs/e2e-testing.md)

## License

MIT
