# SigmaOS Strategic Vision & System Target

## Executive Overview
SigmaOS is a general-purpose, pure Rust operating system built around a clear strategic wedge: delivering a memory-safe kernel and userland with atomic, rollbackable updates, capability-based security by default, and continuous AI-agent maintainability under strict human governance.

---

## 1. System Target & Target Profile
SigmaOS explicitly avoids attempting unachievable universal parity with legacy operating systems like Linux or FreeBSD. Instead, SigmaOS targets:

1. **Memory-Safe Foundation:** Pure Rust kernel and userland primitives eliminating memory-safety vulnerability classes.
2. **Atomic & Rollbackable System Updates:** Generation-based, crash-consistent image and package updates (inspired by NixOS and Silverblue) with single-command instant rollback.
3. **Capability-Based Security by Default:** Granular principle of least privilege utilizing `pledge`/`unveil`, Capsicum, Landlock, W^X, and KARL execution models.
4. **AI-Agent Safe Operating Model:** Machine-readable system interfaces (`FEATURE_STATUS.toml`, `CAPABILITY_MATRIX.toml`), least-privilege scoping, and strict test gates.

---

## 2. Licensing Policy & Borrowing Rules
- **Permissive Core:** SigmaOS core kernel, system libraries, and userland are licensed under dual MIT/Apache-2.0 or BSD-3-Clause licenses.
- **Cleanroom Implementation:** Architectural patterns learned from GPL-licensed systems (Linux, GNOME) are analyzed for design principles and re-implemented in cleanroom Rust without copying GPL source code.
- **Permissive Reuse:** BSD, MIT, and Apache-licensed source code may be reused directly with required copyright attributions.

---

## 3. Core Wedge Strategy
- Finish the core wedge end-to-end (QEMU boot → bare-metal boot → atomic update → capability sandbox) before expanding surface area.
- All feature completion claims are strictly evidence-based, backed by verifiable CI execution logs.
