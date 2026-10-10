# AI Agent Roadmap: BSD Kernel, Security, and Packaging Parity Plan (PR Format)

## Overview & Architecture Principles
This document defines the AI Agent Roadmap in Pull Request (PR) proposal format for achieving 100% native kernel, security, and packaging feature parity with **FreeBSD, OpenBSD, NetBSD, and DragonFly BSD** within SigmaOS.

Inspired by landmark BSD subsystem designs, SigmaOS implements zero-dependency `#![no_std]` Rust parity engines in `src/compatibility/bsd_missing_components.rs` and `src/compatibility/bsd.rs` that provide extreme reliability, fine-grained sandboxing, and high-performance kernel primitives.

---

## Roadmap Phases & Pull Request Matrix

### Phase 1: FreeBSD Resource Accounting & Control (`SovereignFreeBsdRacctGovernorEngine`)
- **PR Title:** `feat(bsd): Implement FreeBSD RACCT/RCTL process resource accounting & throttling governor`
- **Inspiration:** FreeBSD RACCT / RCTL framework.
- **SigmaOS Subsystem:** `src/compatibility/bsd_missing_components.rs` -> `SovereignFreeBsdRacctGovernorEngine`
- **Key Capabilities:**
  - Real-time tracking of CPU time, memory resident set size (RSS), max process limits, and virtual memory usage per process/jail.
  - Dynamic rule evaluation supporting `deny`, `log`, and `throttle` action policies.

### Phase 2: OpenBSD Syscall & Path Sandboxing (`SovereignOpenBsdPledgeUnveilSandboxEngine`)
- **PR Title:** `feat(bsd): Implement OpenBSD pledge(2) syscall promises & unveil(2) path access restriction`
- **Inspiration:** OpenBSD `pledge` and `unveil` security primitives.
- **SigmaOS Subsystem:** `src/compatibility/bsd_missing_components.rs` -> `SovereignOpenBsdPledgeUnveilSandboxEngine`
- **Key Capabilities:**
  - Syscall promise set restriction (`stdio`, `rpath`, `wpath`, `cpath`, `inet`, `unix`, `exec`, `proc`).
  - Path-level filesystem visibility restrictions with granular permission masks (`r`, `w`, `x`, `c`).

### Phase 3: DragonFly BSD HAMMER2 Transactional Storage (`SovereignDragonFlyHammer2FsEngine`)
- **PR Title:** `feat(bsd): Implement DragonFly BSD HAMMER2 transactional snapshots & PFS replication engine`
- **Inspiration:** DragonFly BSD HAMMER2 Copy-on-Write (CoW) filesystem.
- **SigmaOS Subsystem:** `src/compatibility/bsd_missing_components.rs` -> `SovereignDragonFlyHammer2FsEngine`
- **Key Capabilities:**
  - Sub-millisecond snapshot creation and Pseudo-Filesystem (PFS) master-replica state synchronization.
  - Extent-level deduplication and metadata transaction group (TXG) logging.

### Phase 4: NetBSD Rump Kernel Userland VFS Driver Hypercalls (`SovereignNetBsdRumpVfsEngine`)
- **PR Title:** `feat(bsd): Implement NetBSD Rump Kernel VFS userland micro-domain driver hypercall engine`
- **Inspiration:** NetBSD Rump Kernels (run any kernel driver in userland).
- **SigmaOS Subsystem:** `src/compatibility/bsd_missing_components.rs` -> `SovereignNetBsdRumpVfsEngine`
- **Key Capabilities:**
  - Running native VFS file system drivers in isolated userland micro-domains.
  - Hypercall multiplexing for seamless syscall translation and fault isolation.

### Phase 5: FreeBSD Poudriere Clean-Room Bulk Package Builder (`SovereignFreeBsdPoudriereJailBuilderEngine`)
- **PR Title:** `feat(bsd): Implement FreeBSD Poudriere parallel clean-room bulk package builder with Signify signing`
- **Inspiration:** FreeBSD `poudriere` bulk build system & OpenBSD `signify`.
- **SigmaOS Subsystem:** `src/compatibility/bsd_missing_components.rs` -> `SovereignFreeBsdPoudriereJailBuilderEngine`
- **Key Capabilities:**
  - Multi-jail parallel package building with topological dependency graph ordering.
  - Ed25519 Signify + Post-Quantum Cryptography (PQC) package index signing.

---

## Verification & Continuous Integration
All BSD kernel and security parity engines are verified via standalone Rust unit tests:
```bash
rustc --test --edition=2021 src/compatibility/bsd_missing_components.rs -o build/bsd_missing_test && ./build/bsd_missing_test
```
