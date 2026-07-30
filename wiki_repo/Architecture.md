# SigmaOS Architecture

SigmaOS is designed entirely in `no_std` Rust, built from the ground up for absolute silicon sovereignty. It relies on a capability-based microkernel architecture, ensuring that every subsystem is deeply decoupled and aggressively sandboxed.

## Core Subsystems

### 1. Scheduler (`kernel/core/sched_cfs.rs`)
The kernel uses a native implementation of the Completely Fair Scheduler (CFS), utilizing an intrusive Red-Black tree (`RbNode`) for `O(log N)` time-complexity scheduling. It natively supports multi-core load balancing, symmetric multiprocessing (SMP), and priority weighting without relying on any external crates.

### 2. Memory Management (`kernel/mm/buddy_slab_vmm.rs`)
Memory is managed through a multi-tier allocator:
- **Buddy Allocator**: Tracks physical pages in a bitmap, coalescing adjacent free blocks.
- **Slab Allocator**: Carves pages into fixed-size object caches for kernel structs (`Task`, `Inode`, etc.).
- **Virtual Memory Manager (VMM)**: Manages 4-level x86-64 page tables (PML4, PDPT, PD, PT) with huge-page support.

### 3. IPC Ring Buffers (`kernel/ipc/ring_channel.rs`)
Inter-process communication operates via lock-free SPSC and MPSC ring buffers. These rings use pure atomic operations (`AtomicU32::compare_exchange_weak`) and cache-line padding to prevent false sharing. Messages are authenticated via 64-bit capability tokens.

### 4. Virtual File System (`kernel/fs/vfs.rs`)
The capability-based VFS isolates raw device drivers from userland. It maintains `Inode` and `Dentry` mappings, enforcing access control securely at the kernel boundary before handing operations to the block drivers.

### 5. Network Stack (`kernel/net/tcp_stack.rs`)
SigmaOS features a zero-allocation, zero-dependency TCP/IPv4 stack. It implements a complete TCP state machine (SYN-SENT, ESTABLISHED, TIME-WAIT, etc.) natively in Rust, reading from memory-mapped NIC rings.

## Userspace & Tooling

- **sigpkg**: The sovereign package manager utilizing ED25519 signatures, SBOM verifications, and content-addressed storage.
- **Native Suites**: Instead of heavy dependencies on Microsoft/Adobe, SigmaOS embeds lightweight native stubs (SigmaWriter, SigmaSheet, SigmaVector) rendered directly through the Sigma Compositor.
