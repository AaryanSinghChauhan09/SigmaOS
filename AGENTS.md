# SigmaOS AI Agent Memory Management & Codebase Directives

This document defines core directives, architecture rules, and memory management invariants for all AI agents (Jules, Sentinel, Palette, Bolt) operating on the SigmaOS codebase.

## 1. Zero-Dependency Bare-Metal Memory Architecture
- **No External Allocators**: All memory management routines must utilize internal `klib` and kernel allocators (`src/memory/pmm_vmm.rs`, `src/memory/manager.rs`, `src/klib/custom_allocator.rs`, `src/klib/buddy_allocator.rs`).
- **`#![no_std]` Compatibility**: Kernel core modules must maintain strict `#![no_std]` + `extern crate alloc` compatibility.

## 2. Memory Subsystem Invariants & Safeguards
- **Physical & Virtual Memory Management**: PMM/VMM operations in `src/memory/pmm_vmm.rs` must enforce 4KiB page alignment and 2MiB/1GiB huge page boundaries.
- **Guard Pages & Hardened Allocations**: Heap and stack allocations must use hardened guard page allocators (`src/memory/resource_allocator.rs`) and ASLR randomized malloc guards (`src/klib/custom_allocator.rs`).
- **Memory Descriptor List (MDL) Pinning**: I/O and DMA memory buffers must pin memory ranges before descriptor transfers to prevent page fault race conditions under high concurrency.
- **Volatile Scrubbing**: Memory deallocations containing sensitive cryptographic material or keys must perform explicit volatile memory wipes (`AmnesicRamWipe` / zeroization) before returning pages to the buddy allocator.

## 3. Multi-Architecture Paging & Interrupt Balancing
- **x86_64 / x86_32**: PML4/PML5 vs 2-level PAE page tables and x2APIC/PIC8259 IRQ routing (`src/hal/multi_arch.rs`).
- **ARM64 / ARM32**: TTBR0_EL1 4-level 48-bit/52-bit translation vs Armv7 2-level paging with GICv3/GICv2 IRQ controllers.
- **RISC-V 64 / 32**: Sv39/Sv48 3/4-level vs Sv32 2-level paging with PLIC/CLINT timers.

## 4. AI Agent Testing & Verification Directives
- **Proactive Unit Testing**: Every code change or newly introduced feature must be accompanied by unit tests.
- **Master Test Runner**: Run `./run_sigma_tests.sh` to verify 100% test pass rate across Rust, C++, and Python test suites.
- **Standalone Module Testing**: Fast-verify specific modules using `rustc --test --edition 2021 <filepath> -o build/test_bin && ./build/test_bin`.

## 5. AI Agent Performance & Efficiency Directives
- **Zero-Allocation Hot Paths**: Avoid dynamic heap allocations inside fast-path syscall and packet handlers.
- **ISA Auto-Vectorization**: Route memory copies and hashing through SIMD feature routing (`src/klib/isa.rs`).
- **Optimal Lookups**: Use O(1) or O(log N) lookup data structures to minimize CPU cache miss rates.

## 6. AI Agent Kernel Management Directives
- **Zero Ring 0 Panics**: All kernel routines must return `Result<T, &'static str>` or error codes.
- **Syscall Audit Logging**: All syscall entrypoints must log invocations to `SovereignSyscallAuditLogger` (`src/syscall/table.rs`).
- **Capability Sandboxing**: Process creation must inherit minimal capability tokens (`src/security/capability.rs`, `src/security/sigma_unveil.rs`).

## 7. AI Agent Filesystem Management Directives
- **Atomic File Writes**: Perform file updates through staged temporary buffers followed by atomic rename operations.
- **CoW Subvolume Snapshots**: Duplicate extent pointers during subvolume modifications (`src/filesystem/cow_snapshot.rs`, `src/filesystem/btrfs_inspired.rs`).
- **Unveil Path Restrictions**: Enforce OpenBSD `unveil` permissions (`r`, `w`, `c`, `x`) before filesystem operations (`src/security/sigma_unveil.rs`).

## 8. AI Agent Block Device Drivers Management Directives
- **Physical Memory DMA Alignment**: Command list buffers and scatter-gather lists must enforce physical memory page alignment (`src/driver/ahci_sata_controller.rs`).
- **Driver Shard Sandboxing**: Driver shards must execute inside isolated containers with I/O byte quotas (`src/drivers/sovereign_driver_lifecycle.rs`).
- **Doorbell & Submission Queues**: Validate sector ranges and PRP page boundaries before ringing controller doorbells.

## 9. AI Agent Bottom Half Kernel Threads Directives
- **Top-Half/Bottom-Half Split**: Keep top-half hard IRQ handlers under 1 microsecond (`src/interrupt/handler.rs`).
- **Non-Blocking Softirqs**: Softirq vectors (`src/kernel/irq/softirq.rs`) must never sleep or wait on locks.
- **kworker Thread Deferral**: Defer process-context work to system workqueues (`src/kernel/irq/workqueue.rs`).

## 10. AI Agent Main Memory Management Directives
- **Physical Memory Zoning**: Enforce physical memory zone constraints (`ZONE_DMA`, `ZONE_DMA32`, `ZONE_NORMAL`, `ZONE_HIGHMEM`) in `src/memory/zone.rs`.
- **Watermark Reclamation**: Trigger asynchronous `kswapd` page reclamation when free pages hit `Watermark::Low` (`src/memory/kswapd.rs`).
- **Kernel Heap Guard Alignment**: Kernel heap expansion must maintain 4KiB page boundary alignment and ASLR guard page protection (`src/memory/heap.rs`).

## 11. AI Agent Cache Size Management Directives
- **Bounded Slab Caches**: Specify maximum capacity quotas per slab object type in `src/klib/slab.rs` and `src/memory/resource_allocator.rs`.
- **Package Cache Pruning**: Registry proxy caches must perform bulk `copy_from_slice` memory transfers (`src/package/cache.rs`) and enforce `paccache` version pruning.
- **CPU Cache Line Alignment**: Align spinlocks and ring buffer head/tail pointers to 64-byte boundaries (`#[repr(align(64))]`).

## 12. AI Agent Cloud Carrier Operation Management Directives
- **Sub-Second CARP Failover**: VRRP/CARP state transitions (`src/network/distro_net.rs`) must migrate Virtual IP addresses within < 50ms.
- **OpenStack Cinder Enforcers**: Enforce AES/PQC volume encryption masks and tenant volume quotas (`src/open_source_os_gap_closure.rs`).
- **5G/6G Cellular Slicing**: Mobile carrier engine slices (`src/unimplemented_features.rs`) must maintain cryptographic tenant isolation.

## 13. AI Agent Cache Operation Management Directives
- **Explicit CPU Cache Writebacks**: Issue `clwb`/`clflushopt` instructions followed by `sfence` barriers on persistent memory modifications.
- **TLB Shootdown Synchronization**: Issue SMP IPI TLB shootdowns (`src/memory/tlb_associative.rs`) prior to returning physical frames.
- **JIT Instruction Cache Invalidation**: Execute instruction cache invalidation (`isb`) after dynamic code generation.

For detailed specifications, see `docs/AGENTS_MEMORY_MANAGEMENT.md`, `docs/AGENTS_TESTING_GUIDELINES.md`, `docs/AGENTS_EFFICIENCY_GUIDELINES.md`, `docs/AGENTS_KERNEL_MANAGEMENT.md`, `docs/AGENTS_FILESYSTEM_MANAGEMENT.md`, `docs/AGENTS_BLOCK_DEVICE_DRIVERS_MANAGEMENT.md`, `docs/AGENTS_BOTTOM_HALF_THREADS.md`, `docs/AGENTS_MAIN_MEMORY_MANAGEMENT.md`, `docs/AGENTS_CACHE_SIZE_MANAGEMENT.md`, `docs/AGENTS_CLOUD_CARRIER_OPERATION.md`, `docs/AGENTS_CACHE_OPERATION_MANAGEMENT.md`, and `docs/memory-management.md`.
