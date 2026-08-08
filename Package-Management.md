# Package Management

SigmaOS's native package manager **SigmaPkg** supports 15+ Linux package formats without any external tools.

## Supported Formats

| Format | Distribution | Adapter File |
|---|---|---|
| `.deb` | Debian / Ubuntu | `src/sigpkg/universal_adapter.rs` |
| `.rpm` | Fedora / RHEL / openSUSE | `src/sigpkg/rpm_compat.rs` |
| `.pkg.tar.zst` | Arch Linux | `src/sigpkg/arch_compat.rs` |
| `.apk` | Alpine Linux | `src/sigpkg/universal_adapter.rs` |
| `.snap` | Ubuntu Snap | `src/sigpkg/universal_adapter.rs` |
| Flatpak | Desktop apps | `src/sigpkg/universal_adapter.rs` |
| AppImage | Portable apps | `src/sigpkg/universal_adapter.rs` |
| Nix flake | NixOS | `src/sigpkg/universal_adapter.rs` |
| `ebuild` | Gentoo Portage | `src/sigpkg/universal_adapter.rs` |
| `.xbps` | Void Linux | `src/sigpkg/universal_adapter.rs` |
| `.txz` | Slackware | `src/sigpkg/universal_adapter.rs` |
| `.eopkg` | Solus | `src/sigpkg/universal_adapter.rs` |
| guix | GNU Guix | `src/sigpkg/universal_adapter.rs` |
| `SigmaPkg` | Native | `src/sigpkg/spec.rs` |

## Architecture

```
User Request
     ↓
Universal Package Adapter (detects format)
     ↓
Format-specific parser (deb/rpm/apk/…)
     ↓
Dependency Resolver (NativeDependencyResolver – Kahn's toposort)
     ↓
Post-Quantum Signature Verification (Dilithium)
     ↓
Transaction Manager (atomic install/rollback)
     ↓
Store (NixStyleStore – content-addressed)
     ↓
Hook Runner (pre/post install user-defined hooks)
```

## Key Files

- `src/sigpkg/spec.rs` — package spec (native SigmaPkg format)
- `src/sigpkg/universal_adapter.rs` — all 15 format adapters
- `src/sigpkg/transaction.rs` — atomic install/remove transactions
- `src/sigpkg/resolver.rs` — dependency resolution
- `src/sigpkg/zero_alloc_resolver.rs` — zero-allocation resolver for constrained environments
- `src/sigpkg/store.rs` / `src/package/store.rs` — package store
- `src/sigpkg/recipe.rs` — build recipe (like PKGBUILD / ebuild)
- `src/sigpkg/arch_compat.rs` — Arch-specific helpers
- `src/sigpkg/rpm_compat.rs` — RPM-specific helpers

## Dependency Resolution

Uses **Kahn's topological sort** (native, no std::collections) from `src/distro/linux_ideas.rs::NativeDependencyResolver`:

```rust
let mut r = NativeDependencyResolver::new();
r.add_package("libssl".into(), vec![]);
r.add_package("curl".into(), vec!["libssl".into()]);
let install_order = r.resolve_order().unwrap();
// → ["libssl", "curl"]
```

Cycle detection included — returns `Err("Circular dependency detected")` if cycles found.

## Security

Every package is verified before installation:
- **PQC signatures**: Dilithium (`src/crypto/pqc_dilithium.rs`)
- **GPG-compatible signatures**: via PKI (`src/security/pki.rs`)
- **Checksum verification**: SHA-256 of package contents
- **Capability check**: installer requires `INSTALL` capability token

## CPU Architecture Optimization

`CpuArchLevel` microarchitecture routing (inspired by Clear Linux):

| Level | Target | Optimization |
|---|---|---|
| `v1` | Any x86-64 | Baseline |
| `v2` | Haswell+ | SSE4.2, POPCNT |
| `v3` | Skylake+ | AVX2, BMI |
| `v4` | Zen4 / Sapphire Rapids | AVX-512 |
