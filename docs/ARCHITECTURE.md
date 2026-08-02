# SigmaOS Kernel Architecture

## Overview

SigmaOS is a next-generation, Rust-first operating system designed for safety, sovereignty, and extensibility. It borrows the best ideas from Linux, BSD, NixOS, and other open-source ecosystems while introducing novel OS concepts like the Metakernel Orchestration Layer, the BORE-inspired SigmaScheduler, and a fully declarative configuration model.

This document describes the overall architecture of the SigmaOS kernel and its major subsystems.

---

## Table of Contents

1. [Design Principles](#design-principles)
2. [High-Level Architecture](#high-level-architecture)
3. [Kernel Subsystems](#kernel-subsystems)
4. [Memory Model](#memory-model)
5. [Process and Scheduling](#process-and-scheduling)
6. [File Systems](#file-systems)
7. [Device and Driver Framework](#device-and-driver-framework)
8. [Networking Stack](#networking-stack)
9. [Security Architecture](#security-architecture)
10. [IPC Mechanisms](#ipc-mechanisms)
11. [Boot Sequence](#boot-sequence)
12. [Modularity and Extension Points](#modularity-and-extension-points)
13. [Build System](#build-system)
14. [Source Tree Layout](#source-tree-layout)

---

## Design Principles

| Principle | Description |
|-----------|-------------|
| **Memory Safety** | Entire kernel written in Rust with `#![no_std]` at the crate root; unsafe blocks minimized and audited. |
| **Sovereignty** | Minimal dependency on third-party crates; custom klib replaces most stdlib primitives. |
| **Declarative Configuration** | System state described in `sigma.toml`; atomic upgrades guarantee consistency. |
| **Rolling Updates** | Package channels support continuous delivery with pre-upgrade snapshots. |
| **Capability-Based Security** | All access mediated by the capability system; no ambient authority. |
| **Minimal Footprint** | Inspired by Alpine Linux; base installation targets sub-512 MB disk, sub-64 MB RAM. |
| **Extensibility** | Driver, filesystem, and scheduler frameworks are trait-based; new implementations require no kernel patches. |

---

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                        User Applications                            │
├─────────────────────────────────────────────────────────────────────┤
│                       Userland (Sigma Shell, GUI, Tools)            │
├──────────────┬──────────────────────────────┬───────────────────────┤
│  SigmaPkg    │     Sigma Init (sigma_init)   │  Sigma Desktop        │
│  (Package    │     OpenRC-style runlevels    │  (Zenith WM)          │
│   Manager)   │                              │                       │
├──────────────┴──────────────────────────────┴───────────────────────┤
│                      System Call Interface                          │
│           (src/syscall/dispatcher.rs — 256 syscalls)                │
├──────────────┬──────────────┬─────────────────┬─────────────────────┤
│   Memory     │   Scheduler  │   VFS / Btrfs   │  Network Stack      │
│   Manager    │   (BORE)     │   (src/fs/)     │  (src/net/)         │
│   (src/mm/)  │  (src/sched) │                 │                     │
├──────────────┴──────────────┴─────────────────┴─────────────────────┤
│                   HAL — Hardware Abstraction Layer                   │
│                          (src/hal/, src/arch/)                      │
├─────────────────────────────────────────────────────────────────────┤
│           Driver Framework (src/driver/, src/drivers/)              │
├─────────────────────────────────────────────────────────────────────┤
│                     Physical Hardware / UEFI                        │
└─────────────────────────────────────────────────────────────────────┘
```

---

## Kernel Subsystems

### 1. Memory Manager (`src/mm/`, `src/memory/`, `src/klib/`)

The memory subsystem has three layers:

- **Physical Page Allocator** — buddy allocator (`src/klib/buddy_allocator.rs`), manages 4 KB pages.
- **Virtual Memory** — page table management (`src/klib/paging.rs`, `src/mm/virtual_memory.rs`), supports 4-level paging (x86-64).
- **Heap Allocator** — `SigmaBumpAllocator` (`src/klib/custom_allocator.rs`), bump-pointer with recycle bin.

```
Physical RAM
   │
   ▼
Buddy Allocator (4 KB granularity)
   │
   ▼
Virtual Memory Manager (Page Tables)
   │
   ▼
Slab / Bump Heap (SigmaBumpAllocator)
   │
   ▼
User/Kernel Heap (alloc::*)
```

### 2. Process Manager (`src/process/`)

- Process spawn via `src/process/spawn.rs`.
- Threads managed in `src/thread/management.rs`.
- Namespace isolation modeled after Linux `clone()` flags.
- OOM kill policy integrated with resource manager (`src/resource/manager.rs`).

### 3. Scheduler (`src/scheduler/`)

The **SigmaScheduler** is inspired by the BORE (Burst-Oriented Response Enhancer) patch applied to Linux's CFS:

- Tasks maintain a `burst_score` accumulated from their recent CPU burst history.
- The scheduler picks the task with the lowest `vruntime + burst_score`.
- Desktop profile: burst tolerance 8 ms, base slice 4 ms.
- Server profile: burst tolerance 2 ms, base slice 10 ms.

See `src/distro/improvements.rs` — `BoreScheduler` for the implementation.

### 4. Interrupt Subsystem (`src/interrupt/`)

- IDT management and IRQ routing via `src/interrupt/handler.rs`.
- APIC and IOAPIC support.
- IRQ affinity for multi-core balancing.

### 5. System Call Dispatcher (`src/syscall/`)

256-entry syscall table in `src/syscall/dispatcher.rs`. Categories:

| Range | Category |
|-------|----------|
| 0–63 | Process / Thread |
| 64–127 | File / VFS |
| 128–191 | Network |
| 192–223 | Security / Capabilities |
| 224–255 | Platform / HW |

---

## Memory Model

SigmaOS uses a split address space:

```
Virtual Address Space (x86-64, 48-bit canonical)
┌────────────────────────────────────────────────────────────────────┐
│ 0xFFFF_8000_0000_0000 – 0xFFFF_FFFF_FFFF_FFFF  ← Kernel space      │
│   • Direct physical map (first 128 GB of RAM)                      │
│   • Kernel text/data/stack                                         │
│   • vmalloc / ioremap region                                       │
├────────────────────────────────────────────────────────────────────┤
│ 0x0000_0000_0000_0000 – 0x0000_7FFF_FFFF_FFFF  ← User space        │
│   • Text, data, BSS, heap, stack, mmap                             │
└────────────────────────────────────────────────────────────────────┘
```

---

## Process and Scheduling

Each process has:

- **PID namespace** — nested namespaces for container isolation.
- **Capability set** — bitmask of kernel capabilities.
- **Address space** — unique page table root.
- **File descriptor table** — VFS-backed, per-process.
- **Signal queue** — Rust-safe signal delivery.

Scheduling policy per task:

| Policy | Use Case |
|--------|----------|
| `SCHED_BORE` | Default desktop/interactive tasks |
| `SCHED_FIFO` | Real-time, PREEMPT_RT tasks |
| `SCHED_IDLE` | Background jobs |
| `SCHED_BATCH` | Build farm, bulk processing |

---

## File Systems

SigmaOS supports multiple filesystems via the **VFS layer** (`src/fs/vfs.rs`):

| Filesystem | Status | Notes |
|-----------|--------|-------|
| BtrFS | Implemented | Default root FS; zstd compression |
| SigmaFS | Implemented | Custom CoW FS optimized for Rust |
| ext4 | Read-only | Compatibility |
| FAT32 | R/W | EFI System Partition |
| tmpfs | R/W | RAM-backed ephemeral storage |
| OverlayFS | Implemented | Container root layers |
| procfs | R/W | Kernel state exposure |

Btrfs subvolume layout (Garuda-inspired):

```
@ (root)
@home
@snapshots
@tmp (nodatacow)
@var/log (nodatacow)
```

---

## Device and Driver Framework

All drivers implement the `SigmaDriver` trait in `src/driver/framework.rs`:

```rust
pub trait SigmaDriver {
    fn name(&self) -> &str;
    fn probe(&mut self, device: &DeviceInfo) -> Result<(), DriverError>;
    fn init(&mut self) -> Result<(), DriverError>;
    fn read(&self, buf: &mut [u8], offset: u64) -> Result<usize, DriverError>;
    fn write(&mut self, buf: &[u8], offset: u64) -> Result<usize, DriverError>;
    fn ioctl(&mut self, cmd: u32, arg: usize) -> Result<usize, DriverError>;
    fn shutdown(&mut self);
}
```

Driver categories:

- **Storage**: NVMe, AHCI/SATA, SD/MMC
- **Network**: Ethernet, WiFi (802.11ax), Bluetooth
- **Input**: USB HID, PS/2, I2C touchscreens
- **GPU**: KMS/DRM compatible via `src/drivers/gpu/`
- **Audio**: Intel HDA, AC97 legacy
- **Embedded**: 200+ sensor/peripheral drivers in `src/embedded/`

---

## Networking Stack

The network stack (`src/net/`) is fully implemented in Rust:

```
Application (BSD sockets API)
        │
    Socket Layer (src/net/socket.rs)
        │
    TCP/IP Stack (src/net/tcpip_stack.rs)
        │
    IPv4 / IPv6 (src/net/ipv6.rs)
        │
    Routing (src/net/routing.rs)
        │
    Firewall (src/net/firewall.rs)
        │
    Driver (e.g., Intel e1000)
```

Privacy features (Tails-inspired):
- Tor client integration (`src/net/tor_client.rs`)
- Network isolation per session

---

## Security Architecture

See [SECURITY.md](SECURITY.md) for full details. Summary:

- Capability system (`src/security/capability.rs`)
- Vulnerability scanner (`src/security/vulnerability.rs`)
- TPM 2.0 integration (`src/tpm/module.rs`)
- Secure Enclave for key storage (`src/secure/enclave.rs`)
- Post-quantum cryptography (`src/crypto/postquantum.rs`)

---

## IPC Mechanisms

| Mechanism | File | Description |
|-----------|------|-------------|
| Message Passing | `src/ipc/message.rs` | Typed, zero-copy |
| Shared Memory | `src/ipc/mechanism.rs` | Capability-gated |
| Pipes | `src/ipc/ipc.rs` | POSIX-like |
| D-Bus compat | `src/integration/api.rs` | IPC bridge |

---

## Boot Sequence

```
UEFI Firmware
    │
    ▼
SigmaOS UEFI Loader (src/sigma-boot/uefi.rs)
    │  — Loads kernel ELF
    │  — Sets up page tables
    │  — Enables SSE/AVX
    ▼
Kernel Entry (_start in src/boot/)
    │  — Initializes BSS
    │  — Sets up GDT/IDT
    │  — Initializes memory subsystem
    │  — Starts SigmaScheduler
    ▼
SigmaInit (src/init/sigma_init.rs)
    │  — Reads sigma.toml
    │  — Starts services in dependency order
    │  — Mounts filesystems
    ▼
SigmaShell / Desktop
```

---

## Modularity and Extension Points

SigmaOS is designed for extensibility:

1. **Kernel modules** — loadable `.skm` files (Rust dylibs)
2. **Driver plugins** — auto-discovered via `sigma-modprobe`
3. **Filesystem plugins** — register via VFS trait
4. **Scheduler plugins** — implement `SchedulerPolicy` trait
5. **Package format plugins** — SPAC, deb, RPM, Pacman all supported

---

## Build System

```
cargo build --release    # Main kernel and library
make iso                 # Build bootable ISO
make qemu                # Test in QEMU
```

Key build targets:

| Target | Description |
|--------|-------------|
| `x86_64-sigma-none` | Bare-metal x86-64 |
| `aarch64-sigma-none` | ARM64 (Raspberry Pi 4, Apple M-series) |
| `riscv64-sigma-none` | RISC-V 64-bit |

---

## Source Tree Layout

```
src/
├── kernel/          # Core kernel logic (linux_absorb, IRQ, etc.)
├── mm/              # Virtual memory manager
├── memory/          # Heap, SLUB-style allocator
├── klib/            # Custom library (Vec, HashMap, String, allocator)
├── scheduler/       # Process scheduler (BORE-inspired)
├── syscall/         # System call dispatcher
├── fs/              # VFS layer
├── filesystem/      # Btrfs, SigmaFS, ext4 shims
├── driver/          # Driver framework
├── drivers/         # Concrete drivers
├── embedded/        # 200+ embedded sensor drivers
├── net/             # TCP/IP stack
├── network/         # High-level networking
├── security/        # Capability, vulnerability, sandbox
├── crypto/          # AES, RSA, PQC, hashing
├── ipc/             # IPC mechanisms
├── init/            # System initializer
├── boot/            # UEFI boot stubs
├── distro/          # Distro-specific innovations
├── sigpkg/          # SigmaPkg package manager engine
├── package/         # Package abstraction layer
├── container/       # OCI container runtime
└── ...
```
