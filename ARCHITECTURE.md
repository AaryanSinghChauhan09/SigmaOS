# SigmaOS Architecture

> A sovereign, post-quantum resilient, zero-dependency operating system written in Rust.

***

## Table of Contents

1.  [Design Philosophy](#design-philosophy)
2.  [High-Level Architecture](#high-level-architecture)
3.  [Directory Structure](#directory-structure)
4.  [Kernel Subsystems](#kernel-subsystems)
5.  [Memory Management](#memory-management)
6.  [Kernel Library (klib)](#kernel-library-klib)
7.  [Security Subsystem](#security-subsystem)
8.  [Package Manager (sigpkg)](#package-manager-sigpkg)
9.  [Distro Compatibility Layer](#distro-compatibility-layer)
10. [Desktop: Zenith Compositor](#desktop-zenith-compositor)
11. [Networking Stack](#networking-stack)
12. [Filesystem Architecture](#filesystem-architecture)
13. [Build System](#build-system)
14. [Key Design Decisions](#key-design-decisions)

***

## Design Philosophy

SigmaOS is built on four core principles:

1.  **Sovereignty** — Zero dependency on external proprietary software or closed-source toolchains.
2.  **Safety** — Memory safety enforced at compile time via Rust's borrow checker; `unsafe` blocks are explicitly audited.
3.  **Performance** — Bare-metal performance via custom allocators, zero-copy data paths, and compile-time optimisations.
4.  **Parity** — Full Linux/BSD compatibility layer to run existing software while superseding legacy design limitations.

***

## High-Level Architecture

    ┌─────────────────────────────────────────────────────────────────┐
    │                        USERSPACE SHARDS                         │
    │  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌───────────────┐  │
    │  │  SigmaShell  │  ZenithDE  │  SigmaWeb  │  │  Applications │  │
    │  └────┬─────┘  └────┬─────┘  └────┬─────┘  └───────┬───────┘  │
    ├───────┼─────────────┼─────────────┼─────────────────┼──────────┤
    │                    SYSCALL INTERFACE (POSIX + Sigma)             │
    ├─────────────────────────────────────────────────────────────────┤
    │                        KERNEL CORE                              │
    │  ┌────────────┐  ┌───────────┐  ┌────────────┐  ┌──────────┐  │
    │  │  Scheduler  │  │    VFS    │  │  Net Stack │  │   IPC    │  │
    │  └────────────┘  └───────────┘  └────────────┘  └──────────┘  │
    │  ┌────────────────────────────────────────────────────────────┐ │
    │  │               MEMORY MANAGEMENT                            │ │
    │  │   BuddyAllocator  │  SlabAllocator  │  Paging  │  NUMA    │ │
    │  └────────────────────────────────────────────────────────────┘ │
    ├─────────────────────────────────────────────────────────────────┤
    │                     KERNEL LIBRARY (klib)                       │
    │   HashMap · Vec · String · BTreeMap · HashSet · AsyncRuntime    │
    │   Merkle · JSON · TOML · Base64 · UUID · Path · Time · Net     │
    ├─────────────────────────────────────────────────────────────────┤
    │                      SECURITY SUBSYSTEM                         │
    │   pledge/unveil · Capsicum · SELinux · MAC · PQC · Jails       │
    ├─────────────────────────────────────────────────────────────────┤
    │                    HARDWARE ABSTRACTION (HAL)                   │
    │   x86_64 · aarch64 · riscv64 · PCIe · USB · NVMe · NIC        │
    └─────────────────────────────────────────────────────────────────┘

***

## Directory Structure

    SigmaOS/
    ├── src/                    # All Rust source code
    │   ├── kernel/             # Core kernel: scheduler, VFS, IPC, syscalls
    │   │   ├── memory/         # Memory management subsystem
    │   │   ├── sched/          # CPU scheduler (CFS-inspired + EDF)
    │   │   ├── vfs/            # Virtual filesystem layer
    │   │   ├── net/            # Network stack
    │   │   ├── syscall/        # Syscall dispatch table
    │   │   └── irq/            # Interrupt request handling
    │   ├── klib/               # Kernel library: custom data structures
    │   │   ├── vec.rs          # Custom Vec with bulk-copy optimisations
    │   │   ├── string.rs       # Custom String with trim allocations opt.
    │   │   ├── hashmap.rs      # Zero-std HashMap
    │   │   ├── buddy_allocator.rs  # Physical memory allocator
    │   │   ├── sigma_string_utils.rs  # No-alloc string utilities
    │   │   └── ...
    │   ├── security/           # Security subsystem
    │   │   ├── pledge.rs       # OpenBSD-inspired pledge()
    │   │   ├── capsicum.rs     # FreeBSD Capsicum capability model
    │   │   ├── selinux.rs      # SELinux MAC integration
    │   │   ├── jails.rs        # FreeBSD-style jails
    │   │   └── ...
    │   ├── distro/             # Linux/BSD distro parity modules
    │   │   ├── arch_inspirations.rs    # Arch: rolling release, AUR, PKGBUILD
    │   │   ├── nixos_inspirations.rs   # NixOS: declarative config, atomic upgrades
    │   │   ├── gentoo_inspirations.rs  # Gentoo: USE flags, Portage, ebuilds
    │   │   ├── linux_bsd_inspirations.rs
    │   │   └── ...
    │   ├── compatibility/      # Binary/ABI compatibility layers
    │   │   ├── arch_linux.rs
    │   │   ├── cachy_os.rs
    │   │   ├── fedora.rs
    │   │   └── ...
    │   ├── sigpkg/             # Package manager
    │   ├── filesystem/         # Filesystem drivers (ext4, btrfs, ZFS parity)
    │   ├── network/            # Network protocols
    │   ├── desktop/            # Zenith desktop compositor
    │   └── ...
    ├── kernel/                 # Low-level C kernel components
    ├── drivers/                # Hardware drivers
    ├── tools/                  # Build tools, debugger (kdb)
    ├── tests/                  # Test suites
    ├── wiki/                   # Local wiki markdown files
    └── scripts/                # Build and automation scripts

***

## Kernel Subsystems

### Scheduler

SigmaOS implements a hybrid scheduler combining:

*   **CFS (Completely Fair Scheduler)** — proportional CPU time distribution
*   **EDF (Earliest Deadline First)** — for real-time workloads
*   **NUMA-aware scheduling** — topology-aware task placement
*   **BoreSched integration** — CachyOS-inspired BORE (Burst-Oriented Response Enhancer)

### VFS (Virtual Filesystem)

A unified filesystem interface supporting:

*   ext4, FAT32, Btrfs, ZFS-parity (HAMMER2 B-tree)
*   OverlayFS for container layering
*   Plan 9-style resource namespaces
*   Union mounts (BSD-inspired)

### IPC (Inter-Process Communication)

*   **Sigma IPC** — capability-based message passing
*   Unix domain sockets (POSIX compatibility)
*   Shared memory with hardware-enforced isolation
*   io\_uring-style async I/O rings

***

## Memory Management

### BuddyAllocator (`src/klib/buddy_allocator.rs`)

Physical page allocator using the binary buddy system:

*   O(log n) allocation and deallocation
*   Coalescing of adjacent free blocks
*   Per-NUMA-node freelists
*   Configurable order range (4KB to 64MB pages)

### SlabAllocator (`src/kernel/slab_allocator.rs`)

Object cache allocator for kernel data structures:

*   Per-CPU magazines for lock-free hot-path allocation
*   Cache coloring to reduce false sharing
*   Emergency reserve pools

### Paging (`src/klib/paging.rs`)

4-level paging for x86\_64:

*   Transparent huge pages (2MB, 1GB)
*   W^X enforcement (Write XOR Execute)
*   KASLR (Kernel Address Space Layout Randomisation)
*   SMEP/SMAP enforcement

### Custom Vec (`src/klib/vec.rs`)

Optimised over `alloc::vec::Vec`:

*   `extend_from_slice` uses `copy_from_slice` for bulk copy avoiding element-by-element loops
*   Capacity growth strategy tuned for kernel allocation patterns

***

## Kernel Library (klib)

The `klib` module provides std-equivalent data structures that work in `no_std` + `alloc` environments:

| Module | Description |
|--------|-------------|
| `vec.rs` | Bulk-copy optimised growable array |
| `string.rs` | Custom String with trim allocation optimisation |
| `sigma_string_utils.rs` | Zero-alloc byte-slice string utilities |
| `hashmap.rs` | FNV-based open-addressing HashMap |
| `hashset.rs` | HashSet backed by custom HashMap |
| `btreemap.rs` | B-tree ordered map |
| `buddy_allocator.rs` | Physical memory allocator |
| `slab.rs` | Object cache allocator |
| `paging.rs` | Page table management |
| `async_runtime.rs` | No-std async executor |
| `merkle.rs` | Merkle tree for integrity verification |
| `json.rs` | Zero-copy JSON parser |
| `toml.rs` | TOML configuration parser |
| `uuid.rs` | RFC-4122 UUID generation |
| `base64.rs` | Base64 encode/decode |
| `rng.rs` | Cryptographically-seeded PRNG |

***

## Security Subsystem

### OpenBSD-Inspired: pledge/unveil

*   `src/security/pledge.rs` — restricts a process to a declared set of syscall classes
*   `src/security/sigma_unveil.rs` — masks filesystem paths not explicitly exposed
*   `src/security/openbsd_karl.rs` — KARL (Kernel Address Randomised Link) implementation

### FreeBSD-Inspired: Capsicum + Jails

*   `src/security/capsicum.rs` — capability-mode sandboxing, descriptor rights
*   `src/security/jails.rs` — virtualised OS instances with isolated namespaces
*   `src/security/cgroups.rs` — RACCT/RCTL-style resource controls

### Linux-Inspired: SELinux + MAC

*   `src/security/selinux.rs` — type-enforcement mandatory access control
*   `src/security/mac.rs` — TrustedBSD MAC framework integration
*   `src/security/lsm.rs` — Linux Security Module interface

### Post-Quantum Cryptography

*   `src/security/pqc_enclave.rs` — CRYSTALS-Kyber key encapsulation
*   `src/security/pqc_measurement.rs` — TPM 2.0-style measurement log
*   `src/security/crypto_utils.rs` — BLAKE3, SHA-3, ChaCha20-Poly1305

***

## Package Manager (sigpkg)

Located in `src/sigpkg/`, sigpkg is a universal multi-format package manager:

### Supported Formats

| Format | Distro | Status |
|--------|--------|--------|
| `.pkg.tar.zst` | Arch Linux | ✅ Full |
| `.deb` | Debian/Ubuntu | ✅ Full |
| `.rpm` | Fedora/RHEL | ✅ Full |
| `.apk` | Alpine Linux | ✅ Full |
| `ebuild` | Gentoo | ✅ Full |
| Nix expressions | NixOS | ✅ Full |
| FreeBSD ports | FreeBSD | ✅ Full |

### Key Components

*   **SAT Solver** — zero-allocation dependency resolution
*   **PKGBUILD parser** — build recipes from source
*   **Reproducible builds** — content-addressed store at `/sigma/store`
*   **Atomic transactions** — two-phase commit, instant rollback
*   **AUR bridge** — `src/sigpkg/arch_compat.rs`

***

## Distro Compatibility Layer

SigmaOS implements parity with major Linux/BSD distributions:

### Arch Linux Parity (`src/distro/arch_inspirations.rs`)

*   Rolling release channels (Edge, Stable, LTS)
*   `SigmaPkgBuild` — PKGBUILD recipe parser
*   `SigmaMakePkg` — source package builder with sandboxing
*   Signed package database (`.db.tar.gz` format)
*   Pacman-compatible dependency graph solver

### NixOS Parity (`src/distro/nixos_inspirations.rs`)

*   `SigmaNixConfig` — declarative system configuration
*   Content-addressed store (`StoreHash` + `/sigma/store/`)
*   `SigmaGeneration` — atomic upgrade/rollback via generations
*   Derivation model: pure function from inputs to outputs

### Gentoo Parity (`src/distro/gentoo_inspirations.rs`)

*   `UseFlag` — enable/disable features at compile time
*   `Ebuild` — package build specification
*   `PortageResolver` — USE-flag-aware dependency solver
*   Source compilation with custom CFLAGS

### Fedora/RHEL Parity

*   Cockpit web console (`src/remote/`)
*   PipeWire desktop audio
*   FreeIPA Kerberos authentication
*   Anitya upstream release monitoring
*   Fedora Messaging (bugzilla2fedmsg bridge)
*   Tahrir social badges system

### CachyOS Parity (`src/compatibility/cachy_os.rs`)

*   BORE CPU scheduler integration
*   LLVM PGO + BOLT optimisations
*   x86-64-v3 microarchitecture tuning
*   `CachyosKernelFeatureMatrix`

### OpenBSD/FreeBSD Hardening

*   W^X memory policies
*   Retguard return-address canaries
*   Jails with nested hierarchies
*   PF (packet filter) firewall

***

## Desktop: Zenith Compositor

Located in `src/desktop/` and `zenith_desktop/`:

*   **Direct-to-hardware framebuffer** — no Wayland/X11 dependency
*   **HiDPI fractional scaling** — Wayland-spec fractional-scale-v1
*   **Variable Refresh Rate (VRR)** — adaptive sync
*   **Sway-style tiling** — i3-compatible tiling window manager
*   **Gamescope-inspired direct scanout** — low-latency game rendering
*   **MATE Desktop parity** — `src/desktop/mate_betsy.rs`

***

## Networking Stack

*   TCP/IP, UDP, IPv6, ICMP (`src/network/`)
*   WireGuard-style VPN (`src/security/vpn.rs`)
*   DNS resolver with DNSSEC
*   Netfilter/nftables-parity packet filtering
*   io\_uring-based async network I/O

***

## Filesystem Architecture

| Filesystem | Type | Notes |
|-----------|------|-------|
| SigmaFS | Native | Custom B-tree FS, CoW, snapshots |
| HAMMER2 parity | BSD | DragonFly BSD-inspired PFS |
| ext4 | Linux compat | Read/write |
| Btrfs parity | Linux compat | Subvolumes, snapshots |
| ZFS parity | BSD compat | RAIDZ, datasets |
| OverlayFS | Container | Union mounts |
| Plan9 9P | Distributed | Network-transparent FS |

***

## Build System

SigmaOS supports multiple build paths:

```bash
# Rust kernel build
cargo build --release --features microkernel

# Full ISO build
bash scripts/build-iso.sh

# Cross-compile for aarch64
cmake -DCMAKE_TOOLCHAIN_FILE=toolchain-aarch64.cmake

# QEMU smoke test
python3 scripts/qemu_smoke_test.py
```

### Supported Target Architectures

| Architecture | Status |
|-------------|--------|
| x86\_64 | ✅ Primary |
| aarch64 | ✅ Supported |
| riscv64 | 🔧 Experimental |

***

## Key Design Decisions

### No std, Only alloc

The kernel and klib are compiled with `#![no_std]` + `extern crate alloc`. This eliminates the Rust standard library and forces all allocations through the kernel's own allocators.

### Capability-Based Security by Default

Every process starts in capability mode. Syscalls outside the declared capability set cause immediate termination — no exceptions, no overrides.

### Zero External Dependencies

`Cargo.toml` has zero `[dependencies]`. Everything from JSON parsing to cryptography is implemented within the repository in `src/klib/`.

### Rolling Release + Atomic Upgrades

Inspired by Arch Linux (rolling) and NixOS (atomic), SigmaOS delivers updates continuously while guaranteeing that any failed update can be rolled back to the previous generation in under 1 second.
