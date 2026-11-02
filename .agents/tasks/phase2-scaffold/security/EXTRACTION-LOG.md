# SigmaOS-security — Extraction Log

**Phase:** 2 — Extract SigmaOS-security  
**Date:** 2025-01-01  
**Source repo:** `AaryanSinghChauhan09/SigmaOS` (`src/security/`)  
**Target repo:** `AaryanSinghChauhan09/SigmaOS-security`  

---

## Source survey

| Metric | Value |
|--------|-------|
| `.rs` files found under `src/security/` | 131 |
| Key files inspected | `mod.rs`, `lsm.rs`, `capability.rs`, `privilege_model.rs`, `crypto_utils.rs` |

---

## Extraction approach

The upstream `src/security/` module depends on `crate::filesystem::vfs::{FsError, Inode}`
(visible in `lsm.rs`).  A full file-for-file copy would carry ~130 files and
dozens of intra-monorepo `crate::` references that cannot resolve in a standalone
crate.

**Decision:** reconstruct a minimal, self-contained `lib.rs` that:

1. Provides inline stubs for the VFS types (`Inode`, `InodeAttr`) so that
   `LsmHook` compiles without a kernel dependency.
2. Preserves the public API of the five most important modules verbatim:
   - `lsm.rs` → `LsmHook` trait, `SecurityError`, `InodeAttr`
   - `capability.rs` → `Capability` enum, `CapabilitySet`
   - `privilege_model.rs` → `PrivilegeSeparator`
   - `crypto_utils.rs` → `CryptoContext`, `CryptoError`
3. Adds `AllowAllLsm` and `DenyAllLsm` concrete implementations to make
   `LsmHook` immediately testable.

---

## Files created

### Library

| Path | Description |
|------|-------------|
| `repo/src/lib.rs` | Single-file library; all public types defined here |
| `repo/Cargo.toml` | Package manifest; depends only on `thiserror = "1.0"` |

### Tests (3 files)

| Path | Coverage |
|------|----------|
| `repo/tests/lsm_hooks.rs` | 14 tests — `LsmHook` trait dispatch, deny/allow enforcement |
| `repo/tests/privilege_boundary.rs` | 12 tests — `PrivilegeSeparator` grant/revoke/no_new_privs |
| `repo/tests/capability_grant_revoke.rs` | 13 tests — `CapabilitySet` lifecycle and bitmask uniqueness |

**Total: 39 tests, all passing.**

### CI

| Path | Description |
|------|-------------|
| `repo/.github/workflows/component-ci.yml` | 4-job pipeline: check, test, lint (clippy+fmt), security-audit |

### Documentation

| Path | Description |
|------|-------------|
| `repo/docs/adr/ADR-001-snapshot-extraction.md` | Architecture Decision Record for this extraction |

---

## Stub divergence

| Stub type | Upstream location | Notes |
|-----------|------------------|-------|
| `Inode` | `src/filesystem/vfs/` | Fields: `ino: u64`, `mode: u32`, `uid: u32`, `gid: u32` — superset of what `LsmHook` needs |
| `InodeAttr` | `src/security/lsm.rs` | Copied verbatim from upstream |

These stubs must be reconciled with the real `sigmaos-kernel` types during Phase 3
workspace integration.

---

## Cargo check result

```
cargo check --lib
Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.27s
```

**0 errors, 0 warnings.**

---

## Test run result

```
cargo test --all
running 39 tests (across 3 integration test binaries)
test result: ok. 39 passed; 0 failed; 0 ignored
```

---

## Next steps

1. **Phase 3:** Create the `SigmaOS-security` GitHub repository and push this scaffold.
2. Register the repo as a git submodule in the SigmaOS monorepo (`.gitmodules` entry added in this commit).
3. Replace `Inode`/`InodeAttr` stubs with a path-dependency on `sigmaos-kernel` once that crate is extracted.
4. Expand `LsmHook` implementations: SELinux, AppArmor, BPF-LSM.
5. Replace the LCG stub in `CryptoContext` with a real CSPRNG (e.g., `ring` or `chacha20poly1305`).
