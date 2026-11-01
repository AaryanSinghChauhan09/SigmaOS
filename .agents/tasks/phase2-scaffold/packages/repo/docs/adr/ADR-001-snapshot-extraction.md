# ADR-001: SigmaOS-packages Snapshot Extraction Strategy

| Field       | Value                            |
|-------------|----------------------------------|
| Status      | Accepted                         |
| Date        | 2025-07-14                       |
| Deciders    | SigmaOS maintainers              |
| Supersedes  | —                                |

## Context

The SigmaOS monorepo (`src/package/`) contains 150+ Rust source files that
implement every aspect of package management: dependency resolution, multi-distro
package formats, signing and provenance, GUI app stores, update channels,
rollback snapshots, and more.  Maintaining all of this code inside a single
`sigmaos` crate makes independent versioning, CI, and downstream consumption
impractical.

The Phase 2 scaffold initiative extracts each major subsystem into its own
repository with a standalone `Cargo.toml`, its own CI pipeline, and its own
release cadence — following the Linux Mint / Omarchy multi-repository model.

## Decision

Extract a minimal, compilable snapshot of the **package manager core** into a
new `SigmaOS-packages` repository.  The snapshot contains:

- A single `src/lib.rs` that re-implements the four key public APIs in
  self-contained, `#![forbid(unsafe_code)]` Rust:
  - `DependencyGraph` (shared resolution logic — DRY)
  - `PackageResolver` (topological DFS with fail-fast cycle detection)
  - `PackageVerifier` (defensive integrity checks before any install)
  - `PackageInstaller` (orchestrates resolve → verify → install → rollback)
- Integration test files under `tests/` covering resolution, install workflow,
  and verification.
- A CI workflow under `.github/workflows/component-ci.yml`.

### What was intentionally excluded

The original `src/package/` contains many modules that depend on `crate::sigpkg`
and other SigmaOS-internal crates (`crate::sigpkg::SovereignUniversalPackageManagerInteropEngine`,
etc.), as well as a `manager.rs` that defines a local `Vec<T>` using FFI-backed
`alloc`/`free` primitives.  Bringing those cross-crate references into the
standalone crate would require either vendoring large portions of the monorepo
or creating stubs with no semantic value.

The snapshot therefore provides **stable, correct public APIs** using only
`std` and `thiserror`, deferring multi-distro gateway and FFI interop to future
extraction phases.

## Rationale

| Principle applied              | How                                                                 |
|-------------------------------|---------------------------------------------------------------------|
| DRY                           | `DependencyGraph` is the single resolution source used by both `Resolver` and `Installer`. |
| Fail-fast                     | `PackageResolver::dfs` returns `Err(Cycle)` immediately on first cycle detection. |
| Defensive programming         | `PackageVerifier::verify` runs **all** checks before any state change. |
| Stable APIs                   | Public structs/enums carry semver-stable field names; no `pub(crate)` leaks. |
| Rollback safety               | `PackageInstaller::install` snapshots state before any mutation. |
| `#![forbid(unsafe_code)]`     | Eliminates entire classes of memory-safety bugs in this component. |
| SOLID                         | Each struct has one responsibility; `Installer` delegates to `Resolver` and `Verifier`. |
| Memory safety                 | All collections use `std::collections`; no raw pointer arithmetic. |

## Consequences

- The `sigmaos-packages` crate compiles cleanly with `cargo check --lib` and
  all integration tests pass with `cargo test`.
- Future phases can add multi-distro adapters (APT, DNF, Pacman, etc.) as
  separate feature-gated modules without breaking the core API.
- The stable API contract means downstream tools (SigmaOS installer, GUI app
  store, update daemon) can depend on `sigmaos-packages` without worrying about
  breaking changes in minor versions.
