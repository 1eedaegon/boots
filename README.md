# Boots

Rust template generator for building modular architectures

[![Crates.io](https://img.shields.io/crates/v/boots.svg)](https://crates.io/crates/boots)
[![Test](https://github.com/1eedaegon/boots/workflows/Test/badge.svg)](https://github.com/1eedaegon/boots/actions)
[![Build](https://github.com/1eedaegon/boots/workflows/Build/badge.svg)](https://github.com/1eedaegon/boots/actions)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

## Installation

### From crates.io

```bash
cargo install boots
```

### From GitHub Releases

Download pre-built binaries for your platform from [Releases](https://github.com/1eedaegon/boots/releases):

- Linux (x86_64)
- macOS (Apple Silicon)
- Windows (x86_64)

## Usage

### Generate a new project

```bash
# Interactive mode (prompts for project name)
boots generate

# With project name
boots generate sample-project

# Using cargo subcommand
cargo boots generate sample-project
```

### Add components to existing project

```bash
# Add GitHub Actions workflow
boots add gh:test      # Test workflow
boots add gh:build     # Build workflow
boots add gh:semver    # Release workflow

# Add performance benchmarks
boots add test:perf
```

## Generated Project Structure

```
sample-project/
├── .github/
│   └── workflows/      # CI/CD configurations
├── crates/
│   ├── core/          # Core library
│   └── cli/           # CLI application
├── Cargo.toml         # Workspace configuration
└── README.md
```

## Examples

### Create a new CLI tool

```bash
boots generate my-cli-tool
cd my-cli-tool
cargo run --bin my-cli-tool
```

### Create a library with CLI

```bash
boots generate my-library
cd my-library

# Work on the library
cargo build -p my-library-core

# Work on the CLI
cargo run -p my-library-cli
```


## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with [cargo-generate](https://github.com/cargo-generate/cargo-generate)
- Inspired by other project structures
