# AGENTS.md — AI Agent Guidelines & Version Handling for SigmaOS

This document provides instructions, rules, and procedures for AI agents working in the SigmaOS repository, specifically regarding **Version Handling**, **Release Channels**, **Multi-Distro Packaging Parity**, and **Core Subsystem Changes**.

---

## 1. Core Principles & Philosophy

* **Zero External Dependencies:** SigmaOS kernel and core userland maintain a 100% self-sufficient `#![no_std]` Rust architecture. Do NOT introduce third-party external crates to `Cargo.toml`.
* **Semantic Versioning (SemVer 2.0.0):** All core components follow `MAJOR.MINOR.PATCH` versioning scheme.
* **Always Verify Code Changes:** Run `./run_sigma_tests.sh` to ensure all 13 test execution steps (unit, integration, python verification, multi-distro adapters) pass cleanly after making modifications.

---

## 2. Versioning Standards & Rules for AI Agents

When modifying, releasing, or updating versions in SigmaOS:

### 2.1 Core Repository & Cargo Version
* Core package version is declared in `Cargo.toml` (`version = "0.1.0"`).
* **MAJOR (x.0.0):** Incompatible API/ABI or kernel architecture changes (e.g., breaking KABI stability).
* **MINOR (0.x.0):** New backward-compatible kernel subsystems, drivers, or distro parity features.
* **PATCH (0.0.x):** Backward-compatible bug fixes, performance optimizations, or security patches.

### 2.2 Release Channel Configurations
SigmaOS maintains two distinct release streams:
1. **Sigma Stable (`sigma-stable.toml`):** Long-term support (LTS) releases, enterprise stability, strict KABI guarantees.
2. **Sigma Rolling (`sigma-rolling.toml`):** Bleeding-edge features, continuous integration, fast-path package updates.

AI agents updating release configs MUST maintain alignment across `sigma.toml.example`, `sigma-stable.toml`, and `sigma-rolling.toml`.

### 2.3 Universal Package Version Translation (`sigpkg`)
`sigpkg` translates multi-distro version formats into canonical SigmaOS package versions (`crate::sigpkg::Version`):
* **Debian / Ubuntu (`.deb`):** Epoch + Version + Revision (`[epoch:]upstream_version[-debian_revision]`).
* **Fedora / RHEL (`.rpm`):** Epoch + Version + Release (`[epoch:]version-release.dist`).
* **Arch Linux (`.pkg.tar.zst`):** Version + Package Release (`pkgver-pkgrel`).
* **Alpine Linux (`.apk`):** Upstream Version + Package Revision (`version-r<pkgrel>`).
* **Gentoo Linux (`.ebuild`):** Version + Subslot + Revision (`version-r<rev>`).
* **Haiku OS (`.hpkg`):** Version + Package Revision (`version-revision`).

When parsing or creating packages across distros, AI agents MUST preserve version epoch and revision fields to ensure correct dependency resolution order.

---

## 3. Kernel ABI (KABI) Versioning & Stability

* **Syscall Table (`src/kernel/syscall/table.rs`):** Syscall numbers MUST remain stable across minor versions. Extensions are added above index 500.
* **System Call Table Auditing:** Use `AntiRootkitGuard` and SSDT auditing when adding or modifying system call dispatch handlers.
* **Kernel Symbol Exports:** Public exported functions in `kabi/` must maintain symbol name and binary compatibility.

---

## 4. Checklist for AI Agents Incrementing Version

1. **Update `Cargo.toml`** if creating a new release tag.
2. **Sync Release Profiles:** Update `sigma-stable.toml` and `sigma-rolling.toml` version metadata.
3. **Update `CHANGELOG.md`:** Document breaking changes, added features, fixes, and security patches under the new version header.
4. **Execute Verification Pipeline:** Run `./run_sigma_tests.sh` and ensure all test suites pass.
5. **Commit Message Format:** Follow Conventional Commits:
   `feat(pkg): update version to x.y.z` or `fix(kernel): maintain KABI compatibility for syscall table`.

---

## 5. Detailed Documentation

For exhaustive technical reference on multi-distro version comparison logic, package manager epoch handling, and AI agent automation steps, see:
* [`docs/AGENTS_VERSION_HANDLING.md`](docs/AGENTS_VERSION_HANDLING.md)
* [`docs/RELEASE_CADENCE.md`](docs/RELEASE_CADENCE.md)
* [`docs/package-manager.md`](docs/package-manager.md)
