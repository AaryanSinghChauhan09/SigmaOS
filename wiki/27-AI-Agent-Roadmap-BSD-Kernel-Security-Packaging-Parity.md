# AI Agent Roadmap: BSD Kernel, Security & Packaging Parity in PR Format

This document outlines the AI agent development roadmap for complete feature, kernel security, resource management, and packaging parity between SigmaOS and FreeBSD, OpenBSD, NetBSD, and DragonFly BSD.

## Overview & Architecture

SigmaOS natively absorbs BSD innovations into zero-dependency `#![no_std]` Rust engines, preserving OpenBSD pledge/unveil sandboxing, FreeBSD RACCT/RCTL resource governors, Poudriere bulk jail builders, NetBSD Rump Kernel VFS servers, and DragonFly HAMMER2 transactional filesystems.

```
+-----------------------------------------------------------------------------------+
|                        BSD Parity Subsystems in SigmaOS                          |
+-----------------------------------------------------------------------------------+
| 1. FreeBSD RACCT / RCTL Governor  | `SovereignFreeBsdRacctGovernorEngine`         |
| 2. OpenBSD Pledge & Unveil Guard | `SovereignOpenBsdPledgeUnveilSandboxEngine`   |
| 3. NetBSD Rump Kernel VFS Server | `SovereignNetBsdRumpVfsEngine`                |
| 4. DragonFly HAMMER2 Snapshot    | `SovereignDragonFlyHammer2FsEngine`           |
| 5. FreeBSD Poudriere Jail Builder| `SovereignFreeBsdPoudriereJailBuilderEngine`  |
| 6. OpenBSD Signify PQC Verifier  | `OpenBsdSignifyPkgAddVerifier`                |
| 7. Universal Foreign PM Interop  | `MultiDistroUniversalPmInteropEngineV19`       |
+-----------------------------------------------------------------------------------+
```

---

## AI Agent Milestones & Objectives

### Milestone 1: FreeBSD RACCT/RCTL Resource Throttling
- **Target**: `SovereignFreeBsdRacctGovernorEngine` (`src/compatibility/bsd_missing_components.rs`)
- **Objectives**:
  - Track per-process CPU, memory, and vmemory usage against RCTL resource limits.
  - Dynamically throttle process execution when resource thresholds are breached.

### Milestone 2: OpenBSD Pledge Syscall & Unveil Path Sandboxing
- **Target**: `SovereignOpenBsdPledgeUnveilSandboxEngine`
- **Objectives**:
  - Restrict process syscall promises (`stdio`, `rpath`, `wpath`, `cpath`, `inet`, `exec`).
  - Restrict filesystem access paths via `unveil()` path permission masks (`r`, `w`, `c`, `x`).

### Milestone 3: FreeBSD Poudriere Bulk Jail Builder & Signify Signing
- **Target**: `SovereignFreeBsdPoudriereJailBuilderEngine` & `OpenBsdSignifyPkgAddVerifier`
- **Objectives**:
  - Execute clean chroot bulk package builds inside FreeBSD Poudriere jails.
  - Verify OpenBSD `pkg_add` Signify and PQC Dilithium5 binary signatures.

### Milestone 4: DragonFly HAMMER2 Transactional Snapshots & NetBSD Rump VFS
- **Target**: `SovereignDragonFlyHammer2FsEngine` & `SovereignNetBsdRumpVfsEngine`
- **Objectives**:
  - Create HAMMER2 PseudoFS (PFS) zero-copy transactional snapshots and cluster replication.
  - Run NetBSD Rump Kernel VFS hypercall servers in isolated userland micro-jails.

---

*Verified & Implemented in `src/compatibility/bsd_missing_components.rs` with 100% standalone unit test coverage.*
