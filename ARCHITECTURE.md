# SigmaOS Architecture

> A deep dive into the design principles and subsystem organization of SigmaOS.

---

## Design Philosophy

SigmaOS combines the best ideas from:

| Inspiration | Ideas Adopted |
|------------|---------------|
| **Linux** | Modular monolithic kernel, proc filesystem, cgroups, namespaces, eBPF-inspired tracing |
| **FreeBSD** | Jails (containerization), Capsicum capabilities, high-quality TCP/IP stack, ZFS concepts |
| **OpenBSD** | Pledge/Unveil syscall restrictions, memory hardening, minimal attack surface, cryptography-first |
| **Plan 9** | Everything-is-a-file, per-process namespaces, distributed resource sharing |
| **NetBSD** | High portability, hardware abstraction layer, clean driver model |

---

## System Architecture Diagram

```
┌─────────────────────────────────────────────────────────┐
│                    User Space                           │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐             │
│  │ SigmaShell│  │  sigpkg  │  │ User Apps │             │
│  └────┬─────┘  └────┬─────┘  └────┬─────┘             │
└───────┼─────────────┼─────────────┼─────────────────────┘
        │             │             │
┌───────▼─────────────▼─────────────▼─────────────────────┐
│                 System Call Layer (src/syscall/)          │
│              POSIX-compatible + SigmaOS extensions       │
└───────────────────────────┬──────────────────────────────┘
                            │
┌───────────────────────────▼──────────────────────────────┐
│                   Kernel Core (kernel/)                   │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │Scheduler │  │  Memory  │  │  VFS     │  │ IPC     │ │
│  │(MLFQ)    │  │  Manager │  │  Layer   │  │         │ │
│  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │
└───────────────────────────┬──────────────────────────────┘
                            │
┌───────────────────────────▼──────────────────────────────┐
│                    HAL (src/driver/)                      │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌─────────┐ │
│  │  Storage  │  │ Network  │  │  Input   │  │  GPU    │ │
│  │  Drivers  │  │ Drivers  │  │  Drivers │  │ Drivers │ │
│  └──────────┘  └──────────┘  └──────────┘  └─────────┘ │
└──────────────────────────────────────────────────────────┘
```

---

## Subsystem Reference

### Kernel Core (`kernel/`)

| File/Dir | Description |
|----------|-------------|
| `kernel/boot/` | Stage 1/2 bootloader, multiboot2, UEFI |
| `kernel/mm/` | Buddy allocator, slab allocator, paging |
| `kernel/sched/` | Multi-Level Feedback Queue (MLFQ) scheduler |
| `kernel/ipc/` | Pipes, message queues, shared memory |
| `kernel/sync/` | Spinlocks, mutexes, RW-locks (custom, no std) |

### Memory Management (`kernel/mm/`)

SigmaOS uses a **buddy allocator** for physical memory management, inspired by Linux's `mm/page_alloc.c`:

```
Order 0:  4KB pages
Order 1:  8KB blocks
Order 2:  16KB blocks
...
Order 11: 8MB blocks
```

The slab allocator sits on top for kernel object caching.

### Scheduler (`src/kernel/sched/`)

Implements a **Multi-Level Feedback Queue (MLFQ)** scheduler:
- 8 priority levels (0 = highest, 7 = lowest/idle)
- Round-robin within each level
- Priority boosting to prevent starvation (inspired by FreeBSD's ULE scheduler)
- Real-time FIFO support for RT tasks

### System Calls (`src/syscall/`)

POSIX-compatible + SigmaOS-specific extensions:

| Category | Examples |
|----------|---------|
| Process | `fork`, `exec`, `wait`, `clone` |
| Memory | `mmap`, `brk`, `mprotect` |
| File I/O | `open`, `read`, `write`, `close`, `ioctl` |
| Network | `socket`, `bind`, `connect`, `sendto` |
| Security | `pledge` (OpenBSD-inspired), `unveil`, `capset` |
| SigmaOS | `sigma_pkg_*`, `sigma_vm_*` |

### Security Subsystem (`src/security/`)

| Module | Description |
|--------|-------------|
| `aslr.rs` | Address Space Layout Randomization |
| `mac.rs` | Mandatory Access Control (LSM-style) |
| `capabilities.rs` | Linux-compatible capability sets |
| `pledge.rs` | OpenBSD-inspired pledge/unveil |
| `audit.rs` | Security event auditing |
| `crypto/` | AES, ChaCha20, SHA-3, BLAKE3 (custom impl) |
| `rng.rs` | Kernel CSPRNG (Fortuna-based) |

### Package Manager (`src/sigpkg/`)

SigmaPkg — inspired by pacman (Arch), pkg (FreeBSD), and apt (Debian):
- Binary package format: `.sigpkg`
- Dependency resolution via SAT solver
- AUR-compatible helper in `aur_helper.rs`
- Delta updates support

### Networking (`src/network/`)

Custom TCP/IP stack inspired by FreeBSD's network stack:
- IPv4/IPv6 dual-stack
- TCP, UDP, ICMP
- AF_UNIX sockets
- Network namespaces
- Traffic shaping / QoS

### Filesystem (`src/fs/`)

Virtual Filesystem Switch (VFS) with pluggable backends:
- SigmaFS (native, journaled)
- ext4 read/write compatibility
- FAT32/exFAT for removable media
- Plan 9's 9P protocol for distributed filesystems
- tmpfs, procfs, sysfs

---

## Custom Library (`klib/`)

SigmaOS minimizes external dependencies by implementing core data structures in-house:

| Module | Replaces | Notes |
|--------|---------|-------|
| `klib/vec.rs` | `std::vec::Vec` | `no_std` compatible |
| `klib/string.rs` | `std::string::String` | UTF-8, no heap requirement in early boot |
| `klib/alloc.rs` | `alloc::alloc` | Custom buddy-backed allocator |
| `klib/hash.rs` | `std::collections::HashMap` | Open-addressing hash table |
| `klib/types.rs` | Primitive types | `SigmaU64`, `SigmaU32`, `SigmaBool` |

---

## Platform Support

| Architecture | Status |
|-------------|--------|
| x86_64 | ✅ Primary |
| AArch64 | 🔧 In Progress |
| RISC-V 64 | 📋 Planned |
| MIPS | 📋 Planned |

---

## Build System

```
Makefile                 # Top-level build orchestration
  └─ cargo build         # Rust compilation
       └─ Cargo.toml     # Workspace dependencies
            └─ linker.ld # Kernel linker script
```

Key Make targets:
- `make build` — Debug build
- `make release` — Optimized build
- `make run` — Launch in QEMU
- `make test` — Run all tests
- `make iso` — Create bootable ISO image

---

## Linux/BSD Feature Parity

SigmaOS actively absorbs proven features from Linux and BSD:

- **cgroups v2** — Resource accounting/limiting (`src/kernel/cgroups.rs`)
- **eBPF-inspired tracing** — In-kernel programmable tracing
- **ZFS-inspired checksumming** — Data integrity in SigmaFS
- **Capsicum capabilities** — Fine-grained sandboxing (FreeBSD)
- **kqueue** — Efficient event notification (BSD)
- **io_uring-inspired** — Async I/O interface
- **Jails** — Container-like isolation (FreeBSD)
