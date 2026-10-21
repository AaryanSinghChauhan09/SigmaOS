# GNU Info Pages Texinfo Navigation Tree

This document presents the consolidated GNU Info Texinfo documentation tree for SigmaOS, providing structured hierarchical navigation.

## Node Navigation Index

| Node Name | Title | Up Node | Next Node | Prev Node |
| :--- | :--- | :--- | :--- | :--- |
| **`Kernel Architecture`** | Kernel Architecture Details | `Top` | `Packaging System` | `Top` |
| **`Network Stack`** | Network Stack & eBPF Filters | `Top` | `Toolchain & C Runtime` | `Storage and Filesystems` |
| **`Packaging System`** | Packaging System Manual | `Top` | `Sandboxing Security` | `Kernel Architecture` |
| **`Sandboxing Security`** | Sandboxing Security Architecture | `Top` | `Storage and Filesystems` | `Packaging System` |
| **`Storage and Filesystems`** | Storage and Filesystems Handbook | `Top` | `Network Stack` | `Sandboxing Security` |
| **`Toolchain & C Runtime`** | Toolchain & C Runtime Architecture | `Top` | `Top` | `Network Stack` |
| **`Top`** | SigmaOS Core Utilities | `(dir)` | `Kernel Architecture` | `None` |

---

## Detailed Texinfo Nodes

# Info Node: Kernel Architecture Details

**Node Navigation:** Up: `[Top]` | Next: `[Packaging System]` | Prev: `[Top]`

Detailed documentation on SMP multi-core dispatch, eBPF JIT, and lockless memory queues.


---

# Info Node: Network Stack & eBPF Filters

**Node Navigation:** Up: `[Top]` | Next: `[Toolchain & C Runtime]` | Prev: `[Storage and Filesystems]`

Dual IPv4/IPv6 networking stack with eBPF socket filters, XDP packet acceleration, and WireGuard VPN tunnel routing.


---

# Info Node: Packaging System Manual

**Node Navigation:** Up: `[Top]` | Next: `[Sandboxing Security]` | Prev: `[Kernel Architecture]`

Comprehensive Texinfo documentation covering universal package ingestion across 100+ package formats and DPLL SAT dependency resolution.


---

# Info Node: Sandboxing Security Architecture

**Node Navigation:** Up: `[Top]` | Next: `[Storage and Filesystems]` | Prev: `[Packaging System]`

Multi-layered sandboxing governor synthesizing OpenBSD pledge/unveil, FreeBSD Capsicum, and Linux Landlock/Seccomp-BPF.


---

# Info Node: Storage and Filesystems Handbook

**Node Navigation:** Up: `[Top]` | Next: `[Network Stack]` | Prev: `[Sandboxing Security]`

High-reliability transactional storage management with OpenZFS pools, Btrfs subvolume Snapper snapshots, and DragonFly HAMMER2 PFS.


---

# Info Node: Toolchain & C Runtime Architecture

**Node Navigation:** Up: `[Top]` | Next: `[Top]` | Prev: `[Network Stack]`

Sovereign toolchain with LLVM/Clang integration, PQC signature validation, and musl/BSD libc compatibility layer.


---

# Info Node: SigmaOS Core Utilities

**Node Navigation:** Up: `[(dir)]` | Next: `[Kernel Architecture]` | Prev: `[None]`

Welcome to the GNU Info Texinfo manual tree for SigmaOS.

## MENU
* * Kernel Architecture: SMP, eBPF, Buddy Allocator
* * Packaging System: Universal package format and SAT solver
* * Sandboxing Security: Pledge, Unveil, Capsicum, Landlock


---
