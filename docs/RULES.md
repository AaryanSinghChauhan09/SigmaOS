# SigmaOS Master Architectural & Operational Rules

This document establishes the fundamental architectural, security, and contribution rules for SigmaOS, synthesized from leading Linux distributions (Arch, Debian, Fedora, NixOS) and BSD operating systems (OpenBSD, FreeBSD).

---

## 1. Bare-Metal Zero-Dependency Rule (Linux & BSD Kernel Parity)
- **Principle:** Core kernel modules, memory allocators, drivers, and fundamental userland tools must be written in bare-metal, zero-dependency Rust or C++ without relying on heavy external third-party libraries or standard dynamic C runtimes (`glibc`/`musl`).
- **Enforcement:** Crate root `#![no_std]` compliance; isolated `klib` collections.

---

## 2. Least-Privilege & System Sandboxing Rule (OpenBSD Parity)
- **Principle:** All system daemons, user utilities, and network-facing services must run under strict default-deny authorization.
- **Enforcement:** Mandatory application of OpenBSD-style `pledge()` system call promises and `unveil()` filesystem path restrictions on process initialization.

---

## 3. Canonical Documentation Rule (Arch & FreeBSD Parity)
- **Principle:** Documentation must follow the single-source-of-truth model. Every executable command or system daemon shipped in SigmaOS must maintain a corresponding man page under `docs/man/man1/` or `docs/man/man8/`.
- **Enforcement:** PR reviews require man page updates for userland utility additions or command flag modifications.

---

## 4. Deterministic Reproducible Build Rule (NixOS & Debian Parity)
- **Principle:** Package compilations and ISO releases must yield 100% bit-for-bit identical binary output across independent build nodes.
- **Enforcement:** `SOURCE_DATE_EPOCH` environment variable freezing, hermetic build sandboxing (`MakepkgSandbox`), and automated `diffoscope` binary equivalence audits.

---

## 5. Subsystem Maintainer Governance Rule (Linux Kernel Parity)
- **Principle:** Codebase modifications are reviewed and gated by designated subsystem maintainers according to `docs/MAINTAINERS.md`.
- **Enforcement:** All pull requests require Developer Certificate of Origin (DCO `Signed-off-by`) trailers and automated CI test suite approval (`./run_sigma_tests.sh`).
