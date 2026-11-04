# SigmaOS Memory Management Policy

## Overview
SigmaOS manages main memory as a safe, observable, testable lifecycle subsystem across all allocation, paging, virtual memory, and swapping paths. Memory is managed without leaks, races, corruption, or undefined behavior.

## Core Allocator Architecture
- **Physical Allocator Class**: Hybrid buddy allocator with slab allocation for kernel object caches.
- **Frame Allocator Policy**: Bitmap and buddy frame allocators track 4 KiB physical page frames across lower and high memory regions, enforcing bounded allocation limits and typed frame exhaustion errors (`FrameAllocationError::Exhausted`).
- **Paging Policy**: 4-level page table indexing (`PML4` -> `PDPT` -> `PD` -> `PT`) on x86_64 architectures with canonical address validation and race-safe mapping update operations.
- **Virtual Memory Policy**: strict W^X (Write XOR Execute) memory region protection and clean kernel/userspace address space isolation (`USER_SPACE_MAX = 0x0000_7FFF_FFFF_FFFF`).
- **Swap Policy**: Priority-queued swap manager supporting bounded swap slots with atomic state transitions and observable slot allocations.
- **Reclaim Policy**: Active/inactive LRU page list scanning triggered when free frame reserves fall below high/low watermarks.
- **OOM Policy**: Deterministic score-based process selection and termination when free frames and swap capacity are fully exhausted, preventing kernel panics.
- **Memory Limits**: Bounded process-level virtual memory allocation limits (`RLIMIT_AS`, `RLIMIT_DATA`) enforced at VMA creation time.
- **NUMA Behavior**: Single-node architecture with ACPI SLIT awareness for multi-socket distance tracking.
- **Hotplug Behavior**: Memory dynamic hot-add/hot-remove reserved frame region tracking with atomic state lock operations.
