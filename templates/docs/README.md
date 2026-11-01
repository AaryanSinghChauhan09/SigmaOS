# COMPONENT_NAME

> SigmaOS component — *one-line description here*

---

## Overview

<!-- Describe what this component does, why it exists, and what problem it solves.
     Keep it to 2–4 sentences. Avoid marketing language. -->

`COMPONENT_NAME` is a focused, single-responsibility SigmaOS component that
<!-- TODO: describe purpose -->.

**Status:** 🚧 In Development / ✅ Stable / 🔒 Frozen

---

## Architecture

<!-- Summarise the internal design at a high level. Link to ARCHITECTURE.md for depth. -->

See [ARCHITECTURE.md](ARCHITECTURE.md) for full detail.

Key design decisions:
- <!-- decision 1 -->
- <!-- decision 2 -->

---

## Dependencies

### Runtime

| Crate / Library | Version | Purpose |
|-----------------|---------|---------|
| `serde`         | 1.x     | Serialisation |
| `thiserror`     | 1.x     | Ergonomic error types |
| `log`           | 0.4.x   | Logging facade |
| `env_logger`    | 0.11.x  | Log initialisation |
| `clap`          | 4.x     | CLI argument parsing |

### Build

| Tool      | Version | Purpose |
|-----------|---------|---------|
| Rust      | ≥ 1.75  | Primary language |
| `cargo`   | bundled | Build, test, package |
| `make`    | any     | Developer workflow shortcuts |

### Optional

| Crate    | Feature flag | Purpose |
|----------|-------------|---------|
| `tokio`  | `async`     | Async runtime |
| `metrics`| `metrics`   | Instrumentation |

---

## Building

### Prerequisites

```sh
# Install Rust toolchain
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify
rustc --version   # >= 1.75
cargo --version
```

### Debug build

```sh
make build
# or
cargo build
```

### Release build

```sh
make build-release
# or
cargo build --release
```

### Cross-compilation

```sh
# Example: ARM64 musl target
rustup target add aarch64-unknown-linux-musl
make build TARGET=aarch64-unknown-linux-musl
```

---

## Testing

```sh
# Run all tests
make test

# Run with verbose output
cargo test -- --nocapture

# Run a specific test
cargo test test_name

# Run tests with coverage (requires cargo-tarpaulin)
make coverage
```

Test categories:
- **Unit tests** — co-located with source in `src/`
- **Integration tests** — in `tests/`
- **Property-based tests** — using `proptest`, in `tests/`

All tests must satisfy **FIRST** (Fast, Isolated, Repeatable, Self-validating, Timely).

---

## Configuration

Configuration is loaded from a JSON file (via `--config <FILE>`) or falls back
to compiled-in defaults.

### Default configuration

```json
{
  "name": "COMPONENT_NAME",
  "verbose": false
}
```

### Environment variables

| Variable    | Effect |
|-------------|--------|
| `RUST_LOG`  | Override log level (e.g., `RUST_LOG=debug`) |

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

Quick rules:
1. No `.md` or `.txt` files on the `main` branch — docs go here on `docs`.
2. All PRs must pass `make lint test sigma-check`.
3. Follow [Conventional Commits](https://www.conventionalcommits.org/).
4. Open an issue before a large change.

---

## License

MIT — see [LICENSE](../main/LICENSE) (on the `main` branch).

Copyright © YEAR SigmaOS Contributors.

---

## Upstream Inspirations

SigmaOS stands on the shoulders of giants. The following projects and works
have informed its design:

| Project / Work | Relevance |
|----------------|-----------|
| Linux Kernel   | Modular kernel design, driver model, scheduling |
| seL4           | Formal verification, minimal trusted computing base |
| Redox OS       | Rust-native OS design |
| Plan 9         | Uniform resource model, composable tools |
| Fuchsia / Zircon | Capability-based security, component isolation |
| MINIX 3        | Microkernel architecture, fault isolation |
| Rust std       | Memory safety idioms, error handling patterns |
| POSIX          | Interface stability, portability |
| The Zen of Python | Code clarity principles |
| "The Power of Ten" (JPL) | Safety-critical coding rules |
| Worse Is Better (Gabriel) | Pragmatism over purity |
| Gall's Law     | Evolutionary system design |
