# SigmaOS AI Agent Instructions (`AGENTS.md`)

Welcome, AI Agent! This file contains repository-specific directives, architectural rules, coding standards, and testing procedures for working on **SigmaOS**.

---

## 1. Primary Directives & Code Conventions

1. **Zero External Crates Requirement:** SigmaOS is a zero-dependency operating system written in Rust nightly (`#![no_std]` core with optional `std` features for test binaries). Do NOT add external dependencies to `Cargo.toml`.
2. **Testing Standards:**
   - Always verify changes locally before completing steps.
   - Run the complete test suite using `./run_sigma_tests.sh`. This executes:
     1. C++ native test runners (`test_runner`).
     2. Rust inspection test binaries in `build/` using `rustc --edition 2021 --test`.
     3. Python pytest suites (`tests/test_unit_core.py`, `tests/test_integration_system.py`, `tests/test_stress_fuzz_bench.py`).
   - For standalone rustc testing on modified files, use `./scripts/changed_files_rustc_tests.sh`.
3. **Commit Branch Convention:**
   - Branch names MUST start with a valid prefix: `feat/`, `fix/`, `docs/`, `style/`, `refactor/`, `perf/`, `test/`, `chore/`, `revert/`, `impl/`, `driver/`, `security/`, `kernel/`, `arch/`, `ci/`, `pkg/`, `ai/`, `ux/`, `sdk/`, `boot/`, `bolt/`, `palette/`, `sentinel/`, or `jules-`.

---

## 2. Universal Package Management Guidelines for AI Agents (`SigmaPkg`)

When modifying or interacting with the package management subsystem (`src/package/`, `src/sigpkg/`):

1. **Universal Package Formats:**
   SigmaOS natively supports Linux, BSD, and Unix package formats:
   - Linux: `.deb`, `.rpm`, `.pkg.tar.zst`, `.apk`, `.moss`, `.hpkg`, `.tcz`, `.ipk`, `.opkg`, `.xbps`, `.eopkg`
   - BSD/UNIX: `.txz` (FreeBSD), `.openbsd.tgz` (OpenBSD), `.pkgsrc` (NetBSD), `.p5p` / `.ips` (Solaris), `.nar` (Guix/Nix)
   - App Bundles: `.flatpak`, `.appimage`, `.snap`

2. **Core Structs & APIs:**
   - `UniversalPackageManager` (`src/sigpkg/universal_oop_system.rs`): Handles package database state, installation, and removal.
   - `UniversalPackageAdapter` (`src/sigpkg/universal_adapter.rs`): Maps format-specific metadata to `StandardPackage`.
   - `UniversalDependencyMapper` (`src/sigpkg/universal_adapter.rs`): Maps cross-distro package dependency names (e.g. `python3-dev` -> `python`).
   - `UniversalDryRunSimulator` (`src/sigpkg/universal_adapter.rs`): Simulates installs to verify filesystem conflicts and missing dependencies before committing changes.

3. **Safety & Sandboxing Rules:**
   - Scriptlets (`pre-install`, `post-install`, `triggers`) MUST execute inside `pledge`/`unveil` or `Landlock`/`AppArmor` sandboxes (`src/package/sandbox.rs`).
   - All package operations MUST support atomic CoW snapshot rollbacks (`src/package/updater.rs`).

---

## 3. GitHub Actions CI Directives

1. **Action References Pinning:**
   - All GitHub Action references in `.github/workflows/` MUST be pinned to 40-character commit SHAs or valid tags.
   - `actions/download-artifact` MUST be pinned to `cc203385981b70ca67e3a982f6e5f6e62f59a86e` to remediate GHSA security vulnerabilities.
2. **Rust Toolchain Step Syntax:**
   - When using `uses: dtolnay/rust-toolchain@v1` (or `@stable`), always supply a `with:` block containing `toolchain: stable` (or `nightly`) with correct 8-space step key indentation under `- uses:`.

---

## 4. Documentation & Wiki Synchronization

When updating documentation or roadmap files in `docs/` or `WIKI/`, always run `./scripts/sync_wiki.sh` to keep `wiki/` and `wiki_repo/` documentation mirrors in sync.

---
*Maintained by the SigmaOS Core Architecture Team.*
