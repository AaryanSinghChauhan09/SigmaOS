# SigmaOS Memory Policy Specification

`docs/memory/MEMORY_POLICY.md`

## Overview

SigmaOS treats memory management as an integrated, safe, observable, and testable subsystem. Memory is managed through an authoritative lifecycle that spans physical allocation, page table indexing, virtual address space mapping, protection, swapping, page reclamation, and OOM handling.

## Allocator Architecture

### Physical Allocator
- **Class:** Hybrid Buddy & Slab Allocator with fallback bitmap PMM.
- **Buddy Allocator:** Manages order-0 through order-10 power-of-two frame blocks for kernel and user allocations.
- **Slab Allocator:** Allocates fixed-size kernel data structures (e.g., PCBs, inodes, file descriptors) from dedicated caches to eliminate internal fragmentation and reduce allocation latency.
- **Bitmap PMM:** Used during early boot discovery before buddy tree initialization.

### Frame Allocator Policy
- **Concurrency:** All frame allocation and free operations are atomic and race-free, synchronized via spinlocks or lock-free atomic bitsets.
- **Exhaustion:** Frame exhaustion returns a typed `PhysicalMemoryError::Exhausted` rather than triggering a panic.
- **Reserved Memory Protection:** Low memory (first 1 MiB), ACPI tables, and kernel image regions are marked as reserved during boot discovery and protected against allocation or reuse.
- **Double-Free Protection:** Frame metadata tracks allocation state; double-free attempts return `PhysicalMemoryError::DoubleFree`.

### Paging & Virtual Memory Policy
- **Paging Scheme:** 4-level x86_64 paging (PML4, PDPT, Page Directory, Page Table) with optional 2 MiB / 1 GiB huge page support.
- **Address Space Isolation:** Strict boundary at `0x0000_8000_0000_0000` separating User Space (`0x0000_0000_0000_0000` to `0x0000_7FFF_FFFF_FFFF`) and Kernel Space (`0xFFFF_8000_0000_0000` to `0xFFFF_FFFF_FFFF_FFFF`).
- **Protection Semantics:** W^X (Write XOR Execute) security policy enforced across all VMAs and page tables. Mappings cannot be simultaneously writable and executable.
- **Address Translation:** Non-canonical virtual addresses trigger an immediate page fault or typed error.

### Swap & Reclaim Policy
- **Swap Strategy:** Bounded swap slots managed via priority queues (`SwapPriority::High`, `Normal`, `Low`).
- **Swap Operations:** Swap-out and swap-in operations are race-free with atomic slot allocation and double-swap protection.
- **Reclaim Engine:** `kswapd` periodically scans MGLRU (Multi-Generational Least Recently Used) page lists under memory pressure, flushing dirty pages to swap or page cache.
- **Lost-Wakeup Prevention:** Reclaim wait queues enforce atomic state checks and explicit notification flags (`reclaim_wakeup_notified`).

### OOM & Resource Limits
- **OOM Design:** Out-Of-Memory is a designed operational state, not an accidental kernel panic.
- **OOM Killer:** Evaluates process OOM scores based on VMA usage, process priority, and cgroup limits to terminate or throttle high-impact tasks while preserving system-critical services (PID 1, init, display driver).
- **Limits:** Per-process virtual memory and physical frame limits are strictly enforced via cgroups and resource limits (`rlimits`).

### NUMA & Hotplug Behavior
- **NUMA Support:** ACPI SLIT/SRAT tables parse node topologies; allocations favor local NUMA nodes with cross-node fallback policies.
- **Memory Hotplug:** Online/offline physical memory ranges update the active zone descriptors dynamically without disrupting existing mappings.
