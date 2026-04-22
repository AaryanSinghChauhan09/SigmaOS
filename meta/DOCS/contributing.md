# Contributing to SigmaOS

Welcome to the Sovereign Lattice. This document explains how to set up your
environment, write shards, and submit changes.

## Quick Start

```bash
# One-command install
curl -fsSL https://raw.githubusercontent.com/AaryanSinghChauhan09/SigmaOS/main/install.sh | bash

# Or clone manually
git clone --recurse-submodules https://github.com/AaryanSinghChauhan09/SigmaOS
cd SigmaOS

# Run the setup wizard
python sigmactl.py wizard

# Start Zenith dashboard
node server.js        # GUI at http://localhost:8080

# Build everything
sigmactl build        # or: cargo build --workspace && make bin
```

## Devcontainer (Recommended)

Open in VS Code with the Remote Containers extension — everything is
pre-configured in `.devcontainer/devcontainer.json`.

## Writing a New Shard

```bash
# Scaffold a new shard via CLI
sigmactl shard add <name>

# Or manually:
# 1. Create shards/<name>/src/lib.rs  (Rust shard)
# 2. Or kernel/suites/SXX_<Name>/    (C11 shard)
# 3. Add to Cargo.toml workspace members (Rust)
# 4. Include sigma_utils.h in C shards
```

### Rust Shard Rules
- Use `#![no_std]` unless the shard genuinely needs `std`
- Expose a `C FFI` surface with `#[no_mangle] pub extern "C"`
- Include unit tests (`#[cfg(test)] mod tests`)
- No external crate dependencies without approval

### C Shard Rules
- `#include "sigma_utils.h"` for logging, config, and IPC
- C11 standard, `-ffreestanding` compatible
- No `malloc`/`free` — use the Rust memory manager FFI

## Pull Request Checklist

- [ ] New Rust shards have unit tests (`cargo test --workspace`)
- [ ] C shards compile with `make bin` (no warnings)
- [ ] `sigmactl status` exits 0
- [ ] Integration tests pass: `pytest tools/dev/integration_tests/ -v`
- [ ] `cargo clippy --workspace -- -D warnings` clean
- [ ] CHANGELOG.md updated (or CI auto-generates it on merge)

## Commit Message Format

```
<type>(<scope>): <short description>

Types: feat | fix | chore | docs | refactor | test | perf
Scope: kernel | shard | gui | cli | ci | docs | config

Examples:
  feat(shard): add analytics suite with event aggregation
  fix(kernel): resolve IDT alignment fault on x86_64
  chore(ci): enable cargo clippy gate
```

## Security

See [SECURITY.md](../SECURITY.md) for the zero-trust policy and
responsible disclosure process.
