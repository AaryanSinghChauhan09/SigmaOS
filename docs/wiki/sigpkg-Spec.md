# `sigpkg` Package Format Specification
**Version:** 0.1 (Draft) | **Status:** Active Development

`sigpkg` is SigmaOS’s deterministic, sovereign package manager. It is designed to ensure reproducible builds, cryptographic trust, and atomic upgrades.

## 1. Package Metadata (`Sigma.toml`)

Every package in the SigmaOS ecosystem is defined by a TOML manifest. We selected TOML for its strict typing and readability, matching the Rust `Cargo.toml` ecosystem.

```toml
[package]
name = "sigma-coreutils"
version = "0.2.1"
description = "Sovereign core utilities in Rust"
architecture = ["x86_64-unknown-none", "aarch64"]
license = "MIT"
maintainer = "packaging-lead@sigmaos.local"

[dependencies]
sigma-libc = ">= 0.1.0"
sigma-crypto = "0.2.x"

[build]
# Deterministic build command
recipe = "cargo build --release --target {architecture}"
hash = "sha256-abcdef1234567890..." # Expected hash of the output tarball
```

## 2. Package Format Structure

A compiled `.sigpkg` file is an uncompressed tarball (for reproducible hashing) containing:
1. `Sigma.toml`: The metadata file.
2. `signature.sig`: Ed25519 signature of the `data/` directory.
3. `data/`: The actual compiled binaries, libraries, and assets mirroring the target filesystem hierarchy (e.g., `data/bin/ls`).

## 3. Cryptographic Signing & Verification

All packages must be signed before installation. The target system maintains a trusted keyring (seeded during OS installation).
- **Algorithm:** Ed25519 for signatures, SHA-256 for integrity hashing.
- **Verification Flow:**
  1. `sigpkg` extracts the tarball into memory.
  2. Computes the SHA-256 hash of the `data/` directory contents.
  3. Verifies `signature.sig` against the hash using the trusted keyring.
  4. Aborts installation if verification fails.

## 4. Repository Layout (Registry)

In the MVP phase, packages are hosted on a standard Git repository or static HTTP server with a central `index.toml`.
```
registry/
├── index.toml
├── core/
│   ├── sigma-coreutils/
│   │   ├── 0.2.1.sigpkg
│   │   └── metadata.toml
├── desktop/
│   ├── zenith-compositor/
│   │   ├── 0.4.0.sigpkg
```

## 5. System Profiles (Meta-Packages)

`sigpkg` defines full environments using meta-packages called Profiles:
- **`sigma-core`**: Boots a shell with basic networking.
- **`sigma-desktop`**: Pulls `sigma-core` + `zenith-compositor` + GPU drivers.
- **`sigma-cloud`**: Minimal cloud image with `sigma-container` orchestrator.
