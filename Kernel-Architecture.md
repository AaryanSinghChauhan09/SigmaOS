# SigmaOS Kernel Architecture

## Overview

SigmaOS features a **hybrid microkernel/monolithic kernel** design written in Rust, with the ability to run in both `no_std` (bare metal) and hosted environments. The kernel draws inspiration from Linux, FreeBSD, HelenOS, and seL4.

***

## Kernel Structure

    kernel/
    ├── core/
    │   ├── crypto/          # Disk encryption (LUKS-compatible, ChaCha20)
    │   ├── mm/              # Memory management
    │   └── sched/           # Core scheduler primitives
    ├── crypto/
    │   ├── chacha20.rs      # ChaCha20 stream cipher
    │   └── sigma_vault.rs   # Key vault
    ├── security/
    │   └── sigma_vault.rs   # Kernel security vault
    └── proc/                # Process management

    src/
    ├── kernel/
    │   ├── proc/            # Process + signal management
    │   │   ├── signals.rs   # POSIX signal handling
    │   │   └── wdk_lists.rs # Windows Driver Kit-style linked lists
    │   ├── sched/           # Advanced schedulers
    │   │   ├── sigma_mlfq.rs           # Multi-level feedback queue
    │   │   ├── sigma_transformer_sched.rs # AI-driven scheduling
    │   │   └── sigma_thermal_sched.rs  # Thermal-aware scheduling
    │   ├── vfs/             # Virtual filesystem
    │   └── crypto/          # Kernel crypto module
    └── klib/                # Kernel standard library (no_std)
        ├── hashmap.rs        # Custom HashMap without std
        ├── paging.rs         # Page table management
        └── ...

***

## Scheduler Design

### 1. Multi-Level Feedback Queue (MLFQ)

**Module:** `src/kernel/sched/sigma_mlfq.rs`

Classic MLFQ with adaptive quantum sizing:

*   8 priority levels (0 = highest)
*   Dynamic priority boost for interactive processes
*   Starvation prevention via priority aging
*   Inspired by: BSD ULE scheduler + Linux CFS

### 2. Transformer Scheduler (AI-Driven)

**Module:** `src/kernel/sched/sigma_transformer_sched.rs`

Novel attention-based scheduling:

*   Attention weights computed from process history
*   Learns scheduling patterns without external ML frameworks
*   No-std compatible: pure Rust, no heap allocation for inference
*   Based on: [SigmaOS Competitive Development Master Plan](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SIGMAOS_COMPETITIVE_DEVELOPMENT_MASTER_PLAN.md)

### 3. Thermal-Aware Scheduler

**Module:** `src/kernel/sched/sigma_thermal_sched.rs`

Balances CPU load with thermal constraints:

*   Throttles CPU-bound tasks when thermal threshold exceeded
*   Migrates tasks to efficiency cores (big.LITTLE awareness)
*   Works with ACPI thermal zones
*   Inspired by: Android EAS + Intel TurboBoost

***

## Memory Management

### Page Tables

**Module:** `src/klib/paging.rs`

*   4-level page table hierarchy (PML4 → PDPT → PD → PT)
*   ASLR implementation with cryptographic randomization
*   Large page support (2MB hugepages)
*   SMEP/SMAP enforcement via CPU flags

### Custom HashMap

**Module:** `src/klib/hashmap.rs`

Zero-dependency `HashMap<K, V>` for `no_std`:

*   Open-addressing with Robin Hood hashing
*   Dynamic resizing (load factor 0.75)
*   No `std::collections` dependency

### Lock-Free Ring Buffer

**Module:** `src/distro/linux_bsd_inspirations.rs` — `SovereignRingBuffer`

Linux kfifo-inspired SPSC ring buffer:

```rust
let mut ring: SovereignRingBuffer<Event, 256> = SovereignRingBuffer::new();
ring.push(event).ok();
let ev = ring.pop();
```

***

## Process Management

### Signal Handling

**Module:** `src/kernel/proc/signals.rs`

POSIX-compliant signal delivery:

*   Full signal mask support (SIGPROCMASK)
*   Custom signal handlers via `sigaction()`
*   SIGTERM → SIGKILL escalation tracking
*   Real-time signals (SIGRTMIN..SIGRTMAX)

### Process Lists

**Module:** `src/kernel/proc/wdk_lists.rs`

Windows Driver Kit-style doubly linked list intrusive data structure, adapted for SigmaOS kernel process tracking.

***

## Virtual Filesystem (VFS)

**Module:** `src/kernel/vfs/vfs.rs`

The VFS layer provides a uniform interface across all supported filesystems:

| Filesystem | Status | Notes |
|-----------|--------|-------|
| SigmaFS | ✅ Native | Custom B-tree based filesystem |
| ext4 | 🔧 Compat | Read/write compatibility layer |
| FAT32 | ✅ | Boot partition support |
| tmpfs | ✅ | RAM-backed volatile filesystem |
| procfs | 🔧 WIP | /proc virtual filesystem |
| FUSE | 🔧 WIP | Userspace filesystem driver |

***

## io\_uring High-Performance I/O

**Module:** `src/distro/linux_bsd_inspirations.rs` — `SovereignIoUringEngine`

Clean-room implementation of Linux io\_uring:

*   Submission Queue (SQ) ring buffer
*   Completion Queue (CQ) ring buffer
*   Async, zero-copy I/O submission
*   Batch operation support

```rust
let mut engine = SovereignIoUringEngine::new(256);
let sqe = IoUringSqe::new(IoUringOpcode::Read, 3, 0x1000, 512, 0);
engine.submit_sqe(sqe);
// ... kernel processes I/O ...
let cqe = engine.pop_cqe(); // completion event
```

***

## DRM/KMS Display Stack

**Module:** `src/distro/linux_bsd_inspirations.rs` — `DrmModeInfo`

Direct Rendering Manager / Kernel Mode Setting support:

*   Atomic modesetting with timing validation
*   HDMI/DP timing formula (CVT/GTF)
*   Multi-monitor configuration
*   VSync support

***

## Boot Process

    UEFI Firmware
        └── sigma_boot_efi.rs (UEFI bootloader)
            └── Kernel init (src/init/sigma_init.rs)
                ├── Memory map initialization
                ├── CPU feature detection (AVX/AES-NI)
                ├── ACPI table parsing
                ├── Interrupt/IDT setup
                ├── Scheduler initialization
                └── Userland init process (PID 1)

*See also:*

*   [ARCHITECTURE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/ARCHITECTURE.md)
*   [KERNEL\_PERFORMANCE\_PLAN.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/KERNEL_PERFORMANCE_PLAN.md)
*   [SYSCALL\_TABLE.md](https://github.com/AaryanSinghChauhan09/SigmaOS/blob/main/SYSCALL_TABLE.md)
