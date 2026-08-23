# SigmaOS: BSD Systems Absorption Plan

## Overview

SigmaOS incorporates battle-tested features from FreeBSD, OpenBSD, NetBSD, and DragonflyBSD into its Rust-native kernel and userspace. This document details the absorption strategy and implementation status.

---

## FreeBSD Integrations

### Capsicum Capability Framework
- **Status**: ✅ Implemented (`src/security/`)
- **Description**: Fine-grained capability-based sandboxing. Each process only gets capabilities it explicitly requires.
- **SigmaOS Extension**: Integrated with SELinux labels for dual-layer sandboxing.

### GEOM Storage Framework
- **Status**: ✅ Implemented (`src/fs/`)
- **Description**: Modular disk I/O framework supporting RAID, encryption, journaling layers.
- **SigmaOS Extension**: Btrfs + GEOM hybrid with dm-crypt underneath.

### Jails (Lightweight Containers)
- **Status**: ✅ Implemented (`src/container/`)
- **Description**: Lightweight OS-level virtualization predating Docker by a decade.
- **SigmaOS Extension**: Rust-native jail implementation with eBPF syscall filtering.

### ZFS
- **Status**: ✅ Implemented (`src/fs/`)
- **Description**: Copy-on-write filesystem with built-in RAID, snapshots, checksums.
- **SigmaOS Extension**: Available as alternative to Btrfs; auto-detected and configured on install.

### DTrace / ktrace
- **Status**: 🔄 Planned
- **Description**: Dynamic tracing infrastructure for kernel and userspace.
- **SigmaOS Extension**: eBPF replaces DTrace for most use cases; ktrace compat layer planned.

### pf Firewall
- **Status**: 🔄 Planned (`src/network/`)
- **Description**: OpenBSD's packet filter, also used in FreeBSD/macOS.
- **SigmaOS Extension**: NetworkBolt will support pf rule syntax as alternative to nftables.

---

## OpenBSD Integrations

### pledge() System Call
- **Status**: ✅ Implemented (`src/security/`)
- **Description**: Processes declare what syscall categories they will use; kernel enforces it.
- **SigmaOS Extension**: `sigma_pledge()` wraps both pledge and seccomp-bpf in one call.

### unveil() System Call
- **Status**: ✅ Implemented (`src/security/`)
- **Description**: Restricts filesystem access to explicitly unveiled paths.
- **SigmaOS Extension**: Combined with Linux namespaces for path whitelisting.

### W^X Memory Policy
- **Status**: ✅ Implemented (kernel memory subsystem)
- **Description**: Pages are either writable OR executable, never both (prevents shellcode injection).
- **SigmaOS Extension**: Enforced by default; overridable only with root + special capability.

### ASLR (OpenBSD-style strict)
- **Status**: ✅ Implemented (`src/klib/paging.rs`)
- **Description**: OpenBSD has the strongest ASLR implementation; SigmaOS ports it.
- **SigmaOS Extension**: Entropy pool from hardware RNG seeded into every mmap call.

### Secure Memory Allocator (pledge-aware)
- **Status**: ✅ Implemented
- **Description**: Memory allocator that respects pledge restrictions.
- **SigmaOS Extension**: Rust's ownership model provides compile-time guarantees on top.

### LibreSSL
- **Status**: 🔄 Planned
- **Description**: OpenBSD's fork of OpenSSL, security-hardened.
- **SigmaOS Extension**: Used alongside post-quantum extensions (Kyber, Dilithium).

---

## NetBSD Integrations

### pkgsrc (Package Source Collection)
- **Status**: 🔄 Planned
- **Description**: NetBSD's cross-platform package system (works on Linux, macOS, Solaris).
- **SigmaOS Extension**: sigma-pkg will support pkgsrc as a backend for exotic platform targets.

### RAIDframe
- **Status**: ✅ Implemented (via GEOM layer)
- **Description**: Software RAID implementation.
- **SigmaOS Extension**: Merged into the unified storage abstraction layer.

### Microkernel-Inspired Design
- **Status**: ✅ Partial
- **Description**: NetBSD's modular driver model.
- **SigmaOS Extension**: Driver subsystem is dynamically loadable kernel modules in Rust.

---

## DragonflyBSD Integrations

### HAMMER2 Filesystem Concepts
- **Status**: 🔄 Research
- **Description**: Multi-master clustered filesystem with deduplication.
- **SigmaOS Extension**: Concepts being evaluated for SigmaCloud distributed FS.

### LWKT (Lightweight Kernel Thread) Scheduler
- **Status**: ✅ Absorbed (concepts)
- **Description**: CPU-affine thread scheduling for extreme SMP scalability.
- **SigmaOS Extension**: SigmaOS BORE scheduler incorporates LWKT affinity hints.

---

## BSD-Specific Security Hardening Applied to SigmaOS

| Hardening | Source | SigmaOS Status |
|-----------|--------|----------------|
| Stack Smashing Protection | OpenBSD | ✅ Active (all builds) |
| RELRO + Full RELRO | OpenBSD | ✅ Active |
| Position Independent Executables | OpenBSD | ✅ Active |
| Retpoline (Spectre v2 mitigation) | OpenBSD | ✅ Active |
| Shadow Stack (CET) | FreeBSD 14 | ✅ Active on x86-64 |
| Fortify Source | Glibc/BSD | ✅ Active |
| SafeStack | LLVM/OpenBSD | 🔄 Planned |
| Control Flow Integrity | LLVM | ✅ Active |
| Memory Tag Extension (MTE) | FreeBSD ARM64 | 🔄 Planned |

