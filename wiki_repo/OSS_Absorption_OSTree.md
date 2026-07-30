# OSS Absorption: OSTree

## Overview

OSTree is a content-addressed filesystem and update mechanism used by Fedora Silverblue, Endless OS, and GNOME OS for atomic, image-based OS deployments.

## Key Principles Absorbed

### Content-Addressed Object Store

- All filesystem objects (files, directories, commits) are identified by SHA-256 hash digests.
- Objects are immutable once written — no in-place mutation.
- Deduplication is automatic: identical content maps to the same hash.

### Atomic Deployments

- OS updates are deployed as new commit objects referencing a complete rootfs tree.
- The active deployment is switched atomically via a pointer swap (analogous to a symlink flip).
- Rollback is trivially achieved by reverting the pointer to the previous commit.

### SigmaOS Implementation

All of these principles have been absorbed into `sigpkg` (`userland/sigpkg/src/lib.rs`):

- `ContentHash` — 32-byte SHA-256 digest type (`#[repr(C)]`, `no_std` compatible).
- `CASObject` — Typed content-addressed object (Blob, Tree, Commit).
- `ContentAddressedStorage` — Fixed-capacity store with deduplication, staging, and atomic deploy/rollback.
- `TransactionManager` — Deterministic state machine for atomic package transactions.

## Displaced Technologies

| Technology | SigmaOS Replacement |
| --- | --- |
| `ostree` CLI | `sigpkg` transaction manager + CAS engine |
| `ostree admin deploy` | `ContentAddressedStorage::deploy()` |
| `ostree pull` | `ContentAddressedStorage::store()` |
| libostree (C library) | Zero-dependency `#[no_std]` Rust implementation |

## Status

**Fully Absorbed** — All core OSTree primitives are implemented as native Rust types in `sigpkg`.
