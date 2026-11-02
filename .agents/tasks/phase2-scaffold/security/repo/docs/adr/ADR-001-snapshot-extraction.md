# ADR-001: Snapshot Extraction of SigmaOS Security Subsystem

**Status:** Accepted  
**Date:** 2025-01-01  
**Deciders:** SigmaOS Core Team  
**Component:** SigmaOS-security  

---

## Context

The SigmaOS monorepo (`AaryanSinghChauhan09/SigmaOS`) hosts all OS subsystems
in a single Cargo crate.  As the codebase grows toward the 1 M-LOC mark, build
times, review surface, and component ownership become difficult to manage.

The security subsystem (`src/security/`) is particularly sensitive: it defines
LSM hooks, privilege separation, capability-based access control, and
cryptographic primitives.  Keeping it co-located with unrelated code increases
the blast radius of a mistake and makes independent auditing harder.

This ADR records the decision to extract the security subsystem into its own
repository (`SigmaOS-security`) following the multi-repo model pioneered by
Linux Mint and Omarchy Linux.

---

## Decision

Extract `src/security/` from the SigmaOS monorepo into an independent Cargo
library crate at `github.com/AaryanSinghChauhan09/SigmaOS-security`, following
the Phase 2 scaffold process.

### What is extracted

| Concern | Public surface |
|---------|---------------|
| LSM hook interface | `LsmHook` trait |
| Privilege separation | `PrivilegeSeparator` struct |
| Capability model | `Capability` enum, `CapabilitySet` struct |
| Cryptographic context | `CryptoContext` struct, `CryptoError` enum |
| Error type | `SecurityError` enum |
| VFS stub | `Inode`, `InodeAttr` (inline stubs; replaced by `sigmaos-kernel` in workspace) |

### Cross-crate dependency strategy

The upstream `lsm.rs` depends on `crate::filesystem::vfs::{FsError, Inode}`.
Rather than creating a hard dependency on `SigmaOS-kernel` at extraction time,
minimal in-crate stubs are provided.  When the workspace is assembled, the
workspace `Cargo.toml` can override the stub types via a feature flag or
path dependency.

---

## Consequences

### Positive

- **Least privilege in development:** contributors editing the security crate
  cannot accidentally touch kernel, networking, or UI code.
- **Independent CI:** the security crate has its own `cargo check`, `cargo test`,
  `cargo clippy`, and `cargo audit` pipeline that runs on every push.
- **Faster auditing:** security reviewers operate on a smaller, focused codebase.
- **Stable API surface:** `[lib]` exposes a versioned, semver-tracked API rather
  than internal module paths.
- **Gall's Law compliance:** the extracted crate works end-to-end from day one
  (`cargo check` passes, 39 tests pass).

### Negative / Trade-offs

- **Stub divergence risk:** the `Inode` / `InodeAttr` stubs must be kept in sync
  with the upstream types until workspace integration is complete.
- **Additional repository to maintain:** contributors need to be aware of the
  multi-repo structure when making cross-subsystem changes.

### Mitigations

- The stubs are clearly marked with `// stub` comments and documented in this ADR.
- A workspace `Cargo.toml` integration step is planned for Phase 3.
- The `EXTRACTION-LOG.md` in the parent directory records the exact files copied
  and any divergence from the upstream source.

---

## Alternatives Considered

| Alternative | Reason rejected |
|-------------|----------------|
| Keep everything in the monorepo | Does not scale; makes independent security audits impossible. |
| Extract as a Cargo workspace member (not a separate repo) | Defeats the goal of independent ownership and separate CI. |
| Depend directly on `sigmaos-kernel` for VFS types | Creates a circular build dependency before `sigmaos-kernel` is itself extracted. |

---

## References

- SigmaOS Phase 2 scaffold plan
- Linux Mint multi-repo model: `github.com/linuxmint`
- Omarchy Linux: `github.com/omacom`
- Linux `include/linux/lsm_hooks.h` — LSM hook interface reference
- `capabilities(7)` man page — POSIX capability model
