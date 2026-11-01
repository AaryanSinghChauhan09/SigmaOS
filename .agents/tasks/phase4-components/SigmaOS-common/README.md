# SigmaOS-common

## Overview

SigmaOS-common is the shared common library used across all SigmaOS components. It provides foundational utility functions, data types, and abstractions that promote consistent behaviour and avoid code duplication throughout the SigmaOS ecosystem.

## Features

- Shared error types and result handling helpers
- Logging utilities with consistent formatting for SigmaOS components
- Configuration file parsing and serialisation helpers
- Cross-component string, path, and I/O utilities
- Version and build-info structures shared by all SigmaOS crates
- Feature-flagged optional integrations (e.g., D-Bus, GTK)

## Upstream References

- [mintcommon](https://github.com/linuxmint/mintcommon) — common library used across Linux Mint components, primary design reference

## Building

Requires Rust 1.75+ and Cargo.

```bash
# Build in release mode
cargo build --release

# Run tests
cargo test

# Generate documentation
cargo doc --open
```

## Contributing

1. Fork the repository and create a feature branch.
2. Follow standard Rust idioms; run `cargo clippy` and `cargo fmt` before submitting.
3. Add unit tests for any new public API.
4. Open a pull request against `main` with a clear description of the change.

All contributions are subject to the [MIT License](LICENSE).
