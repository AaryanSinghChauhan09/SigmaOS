# Contributing to SigmaOS

Thank you for your interest in contributing to **SigmaOS** — a next-generation operating system written in Rust, drawing inspiration from Linux, BSD, and Plan 9 philosophies.

---

## Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Branch Strategy](#branch-strategy)
- [Coding Standards](#coding-standards)
- [Submitting Changes](#submitting-changes)
- [Testing](#testing)
- [Security](#security)
- [Architecture Overview](#architecture-overview)

---

## Getting Started

### Prerequisites

| Tool | Minimum Version | Purpose |
|------|----------------|---------|
| Rust (nightly) | 1.80+ | Core build toolchain |
| cargo | latest | Package management |
| make | 4.x | Build orchestration |
| git | 2.x | Version control |
| QEMU | 8.x | OS emulation for testing |

### Setup

```bash
git clone https://github.com/AaryanSinghChauhan09/SigmaOS.git
cd SigmaOS
rustup toolchain install nightly && rustup default nightly
rustup component add rust-src llvm-tools-preview
make build
make run
```

---

## Development Workflow

1. Branch from `main`: `git checkout -b feature/my-feature main`
2. Make atomic, well-described commits
3. Add tests for new functionality
4. Open a Pull Request against `main`

---

## Branch Strategy

SigmaOS uses a **single-trunk (`main`)** strategy:
- All development merged into `main` via PRs
- Feature branches deleted after merging
- No long-lived divergent branches

---

## Coding Standards

- **rustfmt**: `cargo fmt --all`
- **clippy**: `cargo clippy --all-targets`
- No `unwrap()` in library code — use `?` for error propagation
- `#![no_std]` in kernel modules unless impossible
- Prefer custom `klib/` implementations over external crates

### Custom Types (use in kernel code)

```rust
use crate::klib::types::{SigmaU64, SigmaU32, SigmaBool, SigmaU8};
```

### Dependencies Policy

Minimize external dependencies. Implement in-house using `klib/` where feasible:
- `klib/vec.rs` — custom `Vec<T>`
- `klib/string.rs` — custom `String`
- `klib/alloc.rs` — custom allocator

### Security Rules

- **Never hard-code cryptographic values** (keys, IVs, passwords)
- All crypto operations go through `src/security/`

---

## Testing

```bash
cargo test                          # All unit tests
cargo test --test <name>            # Specific test
cargo test -- --nocapture           # With output
make test-integration               # Integration tests
make test-qemu                      # Hardware-level via QEMU
```

---

## Security

Do not open public issues for security vulnerabilities. See [SECURITY.md](SECURITY.md).

---

## Architecture Overview

See [ARCHITECTURE.md](ARCHITECTURE.md) for details.

| Subsystem | Location | Description |
|-----------|----------|-------------|
| Kernel Core | `kernel/` | Boot, scheduling, memory |
| System Calls | `src/syscall/` | POSIX-compatible syscall layer |
| Memory | `kernel/mm/` | Buddy allocator, paging |
| Drivers | `src/driver/` | Hardware abstraction |
| Security | `src/security/` | Capabilities, MAC, crypto |
| Networking | `src/network/` | TCP/IP stack |
| File System | `src/fs/` | VFS layer |
| Package Manager | `src/sigpkg/` | SigmaPkg package manager |

---
*Happy hacking! — The SigmaOS Team*
