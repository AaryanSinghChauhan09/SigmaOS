# SigmaOS Memory Subsystem Policy

## Overview

SigmaOS manages main memory as a safe, observable, testable lifecycle subsystem across all execution layers. No allocation may be unbounded, and Out-Of-Memory (OOM) must be a designed, observable state rather than an accidental panic or crash.

## Authoritative Lifecycle State Machine

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

| Lifecycle Stage | Behavior & Requirements |
|---|---|
| **Boot** | Discover memory map, exclude reserved MMIO/ACPI regions from allocation pool. |
| **Physical Allocator Init** | Initialize 4KB physical frame descriptors with bounded total capacity and atomic state trackers. |
| **Paging Init** | Set up 4-level x86_64 page tables (PML4 tree) with canonical address enforcement. |
| **Virtual Memory Init** | Separate User space (`0x0000_0000_0000_1000` to `0x0000_7FFF_FFFF_FFFF`) and Kernel space (`0xFFFF_8000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF`). |
| **Allocation** | Allocate physical frames via race-free locks or atomic bit manipulation; return `FrameAllocationError::OutOfMemory` on exhaustion. |
| **Mapping** | Map virtual pages to physical frames with explicit page table entry flags (`present`, `writable`, `user_accessible`, `no_execute`). |
| **Protection** | Enforce access control and W^X (`Write XOR Execute`) security policy on every Virtual Memory Area (VMA). |
| **Swap** | Transparently swap cold anonymous pages out to secondary storage swap slots when memory pressure rises. |
| **Reclaim** | Invoke kernel reclaim loop (`kswapd` / page cache eviction) under high memory pressure prior to OOM invocation. |
| **Free** | Unmap virtual address ranges, return physical frames to free pool, and verify no double-free occurs. |
| **Cleanup** | Release address spaces and associated metadata descriptors on process termination or address space destruction. |

## Subsystem Policies

### Physical Allocator Class
- **Class**: Hybrid Buddy-Bitmap Allocator with Slab Caching for small kernel objects.
- **Frame Allocation Policy**: 4KB page frame allocation with thread-safe mutex/spinlock locks and double-free guard assertions.

### Paging Policy
- **Levels**: 4-level paging (PML4, PDPT, PD, PT).
- **Address Validation**: Strict canonical address checking (`bits 47..63` must equal sign bit of bit 47).
- **TLB Coherency**: Automatic INVLPG execution and multi-core TLB shootdown notifications on unmap/reprotection.

### Virtual Memory Policy
- **Isolation**: strict user-kernel isolation. Kernel pages carry no `User` flag.
- **Protection**: Enforcement of W^X (no mapping may be both writable and executable).

### Swap Policy
- **Slot Capacity**: Bounded swap space slots.
- **Queueing**: Priority-queued swap out/in requests with observable queue depth and latency tracking.

### Reclaim & OOM Policy
- **Reclaim**: Starvation-free reclaim loop triggering background page compaction and page cache flushing.
- **OOM**: Score-based OOM killer targeting the highest non-privileged consumer when swap and physical memory are completely exhausted.

### Memory Limits & Behavior
- **Queue Limits**: Kernel request queues and metadata pools have hard upper bounds.
- **NUMA Behavior**: SLIT/SRAT table parsing for NUMA node locality optimization.
- **Memory Hotplug**: Dynamic frame descriptor insertion and online frame bitmap expansion.
