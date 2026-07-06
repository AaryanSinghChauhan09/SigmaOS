# Core System Architecture

> SigmaOS v15.0 "Zenith" — Kernel Internals Reference

## Overview

SigmaOS is a sovereign, AI-native, freestanding microkernel OS built entirely in Rust (`#![no_std]`). It targets 8 deployment profiles: desktop, server, edge, embedded, HPC, cloud, IoT, and mobile. The kernel is self-contained and has zero runtime dependency on `glibc`, `musl`, or any external C runtime.

---

## Boot Sequence

```
UEFI Firmware
  └─ sigma_efi_entry.c   (GOP setup, memory map, ACPI parse, ELF load)
       └─ kernel_main()
            ├─ sigma_irq.rs     (APIC/PIC/GIC init)
            ├─ buddy_allocator  (physical page allocator)
            ├─ sigma_vmm.rs     (4-level page tables, ASLR, W^X)
            ├─ sigma_ubc.rs     (Unified Buffer Cache)
            ├─ sigma_vfs.rs     (VFS mount table)
            ├─ sigma_tmpfs.rs   (early RAM fs at /tmp)
            ├─ sigma_sched.rs   (MLFQ→CFS→EDF scheduler)
            └─ sigma_init.rs    (PID 1, parallel service startup)
```

---

## Memory Management

| Component | File | Description |
|---|---|---|
| Buddy Allocator | `kernel/mm/buddy_allocator.rs` | Physical page allocator, 2^n order blocks |
| VMM / Paging | `kernel/mm/sigma_vmm.rs` | x86-64 4-level paging, ASLR, W^X enforcement |
| Unified Buffer Cache | `klib/sigma_ubc.rs` | LRU page cache for VFS I/O |

### Address Space Layout

```
0xFFFF_FFFF_FFFF_FFFF ┐
  Kernel code/data     │  Kernel space (top 128TiB)
  MMIO mappings        │
  Kernel stacks        │
0xFFFF_8000_0000_0000 ┘
0x0000_7FFF_FFFF_FFFF ┐
  Userland stack       │  User space (bottom 128TiB)
  Userland mmap        │  ASLR randomizes base addresses
  Userland heap        │
  Userland code        │
0x0000_0040_0000_0000 ┘
```

---

## Scheduler

The SigmaOS scheduler uses a three-tier composite model:

| Tier | Algorithm | Target Workload |
|---|---|---|
| 0 (highest) | EDF | Hard real-time tasks (audio, video deadlines) |
| 1 | MLFQ (8 levels) | Interactive tasks, shell, GUI |
| 2 | CFS | Background, batch, server workloads |

- **Per-CPU runqueues** with work-stealing for load balancing
- **Priority boost** every 100 ticks (anti-starvation)
- **Context switch** saves full `CpuContext` (RIP, RSP, CR3, general-purpose registers)

Source: [`kernel/sched/sigma_sched.rs`](../kernel/sched/sigma_sched.rs)

---

## Virtual File System

The VFS layer (`kernel/vfs/sigma_vfs.rs`) provides:

- Mount table with up to 16 simultaneous mounts
- Unified `open / read / write / close / stat / seek` interface
- Backends: `tmpfs`, `SigmaFS`, `Btrfs`

### Filesystem Support

| Filesystem | File | Features |
|---|---|---|
| TmpFS | `sigma_tmpfs.rs` | RAM-backed, early boot |
| SigmaFS | `fs/sigmafs/sigma_mkfs.rs` | Native journaled extent FS |
| Btrfs | `fs/btrfs/sigma_btrfs.rs` | CoW, snapshots, rollback |

---

## IRQ Subsystem

`kernel/core/sigma_irq.rs` handles:
- **APIC** (Local and I/O APIC) for x86-64
- **PIC** (legacy 8259A) with cascade mode
- **GIC v2** (`arch/arm64/sigma_gic.rs`) for ARM64

---

## Syscall Interface

SigmaOS implements 30 core syscalls dispatched via `kernel/core/sigma_syscall_dispatch.rs`.  
Security gates enforced: **Pledge** (whitelist) + **Seccomp-BPF** style filtering.

| # | Name | Description |
|---|---|---|
| 0 | `read` | Read from file descriptor |
| 1 | `write` | Write to file descriptor |
| 2 | `open` | Open file |
| 3 | `close` | Close file descriptor |
| 4 | `fork` | Create child process |
| 5 | `exec` | Execute binary |
| 6 | `exit` | Terminate process |
| 7 | `mmap` | Map memory |
| 8 | `munmap` | Unmap memory |
| 9 | `mprotect` | Change page protections |
| … | … | … |

---

## Dependency Philosophy

SigmaOS deliberately avoids third-party runtime dependencies:

- `klib/sigma_libc.rs` — Rust-native C ABI shim (`strlen`, `memcpy`, `memset`, `malloc`, `free`)
- `klib/sigma_busybox.rs` — Single multi-tool binary replacing coreutils (`ls`, `cat`, `cp`, `rm`)
- All kernel code is `#![no_std]` with only `core` and `alloc` permitted
