# 🏛️ SigmaOS System Architecture

SigmaOS implements a **Hybrid Microkernel/Monolithic Architecture** combining Object-Oriented Programming (OOP) design principles for source code organization with Procedural Programming principles for zero-overhead runtime execution.

---

## 1. Core Architectural Pillars

### 1.1 Hybrid Kernel Design (`docs/HYBRID_ARCHITECTURE.md`)
* **OOP Storage Layer:** Classes, structs, traits, and strategy patterns used for clean modularity, abstraction, and maintainability in source code.
* **Procedural Execution Layer:** Zero-cost function pointer dispatch tables (`ProceduralDriverDispatchTable` in `src/driver/framework.rs`), direct register manipulation, and zero-allocation memory routines.
* **Capability-Gated Security:** Every process operates under a fine-grained bitmask capability model (`src/security/capability.rs` & `src/security/mod.rs`), preventing unauthorized hardware or system calls.

### 1.2 Virtual Memory & Physical Memory Management (`PMM/VMM`)
* **Buddy Allocator & GlueBuddy:** Order-based physical page allocation (`src/kernel/memory/sigma_buddy.rs`) with Linux-inspired `MigrateType` (`Unmovable`, `Reclaimable`, `Movable`, `Cma`, `HighAtomic`), Contiguous Memory Allocator (`CmaBuddyReservationGlue`), and FreeBSD VM `PageQueueType` (`Active`, `Inactive`, `Wired`, `Free`).
* **Slab Allocator:** Zero-fragmentation slab caches (`src/klib/slab.rs`) for small kernel objects (`ProcessControlBlock`, `FileDescriptor`, `InodeStruct`).
* **ASLR & Guard Pages:** Dynamic Kernel Address Space Layout Randomization (`SovereignKaslrEngine`) and thread stack guard page flags (`has_guard_page` in `src/arch/cpu_sys.rs`).

### 1.3 Asynchronous I/O Engine (`io_uring`)
* **Ring Buffer Submission/Completion:** Asynchronous non-blocking file, network, and device I/O (`src/kernel/io_uring.rs`) with `SubmissionQueueEntry` (SQE) and `CompletionQueueEntry` (CQE) support for high-throughput zero-copy operations.

### 1.4 Virtual Filesystem (`VFS`) & Storage
* **Unified VFS Layer:** POSIX file operations, inode reference counting (`link_count`), hard link restrictions (`FsError::IsDirectory`), mount table management (`src/filesystem/vfs.rs`), and ext4/ntfs security label checks.
* **HAMMER2 & CoW Snapshots:** Pseudo-filesystem namespace snapshotting and BLAKE3 block-level deduplication (`src/filesystem/sigma_fs.rs`).

---

## 2. Kernel Process & IPC Subsystem

* **Round-Robin Scheduling:** Multi-priority round-robin process scheduler (`src/kernel/roundrobin.rs`) with CPU core affinity, voluntary yields, and interactive score boosting.
* **Zero-Copy IPC & ALPC:** Advanced Local Procedure Call (`src/ipc/alpc.rs`) and zero-copy ring buffer pipe splicing (`src/ipc/pipes.rs`) enabling microsecond inter-process communication.
* **Process activity management:** Register snapshots (`ProcRegisterSnapshot`), foreground focus tracking, and activity state transitions (`src/process/activity_manager.rs`).
