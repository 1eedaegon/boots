# Boots

Bootstrap a Rust template generator for building modular architectures (monorepo)

[![Crates.io](https://img.shields.io/crates/v/boots.svg)](https://crates.io/crates/boots)
[![Test](https://github.com/1eedaegon/boots/workflows/Test/badge.svg)](https://github.com/1eedaegon/boots/actions)
[![Build](https://github.com/1eedaegon/boots/workflows/Build/badge.svg)](https://github.com/1eedaegon/boots/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Installation

### Using Cargo

```bash
cargo install boots
```

### Using Pre-built Binaries

Download pre-built binaries for your platform from [Releases](https://github.com/1eedaegon/boots/releases):

#### Linux & macOS

```bash
# Linux x64
curl -LO https://github.com/1eedaegon/boots/releases/latest/download/boots-linux-x64.tar.gz
tar xzf boots-linux-x64.tar.gz
sudo mv boots cargo-boots /usr/local/bin/

# macOS ARM64 (Apple Silicon)
curl -LO https://github.com/1eedaegon/boots/releases/latest/download/boots-darwin-arm64.tar.gz
tar xzf boots-darwin-arm64.tar.gz
sudo mv boots cargo-boots /usr/local/bin/
```

#### Windows (PowerShell)

```powershell
Invoke-WebRequest -Uri "https://github.com/1eedaegon/boots/releases/latest/download/boots-windows-x64.zip" -OutFile "boots.zip"
Expand-Archive -Path boots.zip -DestinationPath .
Move-Item boots.exe,cargo-boots.exe "$env:USERPROFILE\.cargo\bin\"
```

## Usage

### Service Project (Full-stack)

Create a service with API, runtime server, CLI, and core modules:

```bash
# Basic service
boots service my-api

# With PostgreSQL support
boots service my-api --options postgres

# With PostgreSQL and gRPC
boots service my-api --options postgres,grpc

# With all options
boots service my-api --options postgres,grpc,http

# Using cargo subcommand
cargo boots service my-api --options postgres
```

### CLI Project

Create a CLI application with core and optional modules:

```bash
# Basic CLI
boots cli my-tool

# With HTTP client
boots cli my-tool --options client

# With client and persistence
boots cli my-tool --options client,persistence
```

### Library Project

Create a minimal library with examples:

```bash
boots lib my-crate
```

## Generated Project Structures

### Service Project

```
my-api/
├── crates/
│   ├── api/           # HTTP/gRPC handlers and routes
│   ├── cli/           # Command-line interface
│   ├── core/          # Business logic and domain types
│   └── runtime/       # Server startup and configuration
├── .github/
│   └── workflows/     # CI/CD (build, test, release)
├── Cargo.toml         # Workspace configuration
├── Dockerfile
├── Makefile
└── README.md
```

**Runtime Features:**
- Health endpoint: `GET /health` returns `{"healthy": true}`
- Metrics endpoint: `GET /metrics` returns Prometheus format

### CLI Project

```
my-tool/
├── crates/
│   ├── cli/           # Command-line interface
│   ├── client/        # HTTP client (with --options client)
│   └── core/          # Business logic
├── Cargo.toml
├── Dockerfile
├── Makefile
└── README.md
```

### Library Project

```
my-crate/
├── crates/
│   └── core/
│       ├── src/
│       └── examples/  # Example usage
├── Cargo.toml
├── Dockerfile
├── Makefile
└── README.md
```

## Options Reference

### Service Options

| Option | Description |
|--------|-------------|
| `postgres` | Add PostgreSQL support with sqlx and migrations |
| `sqlite` | Add SQLite support |
| `grpc` | Add gRPC support with tonic and proto directory |
| `http` | HTTP API (enabled by default) |

### CLI Options

| Option | Description |
|--------|-------------|
| `client` | Add HTTP client module with reqwest |
| `persistence` | Add local file-based persistence |

## Examples

### Create and Run a Service

```bash
boots service my-api --options postgres,grpc
cd my-api
cargo build --all
cargo run -p my-api-cli -- --port 8080
```

Test the endpoints:
```bash
curl http://localhost:8080/health
# {"healthy":true}

curl http://localhost:8080/metrics
# # HELP up Server is up
# up 1
```

### Create and Run a CLI Tool

```bash
boots cli my-tool --options client
cd my-tool
cargo run -p my-tool-cli -- --help
```

### Create a Library

```bash
boots lib my-crate
cd my-crate
cargo build
cargo run --example basic
```

## Development

```bash
# Build
cargo build --all

# Test
cargo test --all

# Lint
cargo clippy --all -- -D warnings

# Format
cargo fmt --all
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the [MIT License](LICENSE)
