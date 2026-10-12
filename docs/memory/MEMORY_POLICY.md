# SigmaOS Memory Management Policy Specification

## 1. Subsystem Objective

> **Maintain SigmaOS memory management as a safe, observable, testable subsystem: allocate, map, protect, swap, limit, reclaim, and release memory without leaks, races, corruption, or undefined behavior.**

SigmaOS manages main memory as a **lifecycle subsystem** requiring recurring development, validation, hardening, and maintenance of physical allocation, paging, virtual memory, swapping, OOM behavior, and observability.

---

## 2. Authoritative Memory Lifecycle Contract

All memory management operations within SigmaOS must conform to the authoritative 11-stage lifecycle contract:

```text
boot -> physical allocator init -> paging init -> virtual memory init ->
allocation -> mapping -> protection -> swap -> reclaim -> free -> cleanup
```

### Stage Requirements & Behaviors

| Stage | Required Behavior | Failure / Guard Condition |
|---|---|---|
| **Boot** | Discover BIOS/EFI e820 memory map and exclude reserved regions (MMIO, ACPI, firmware). | Invalid map halts early boot; reserved ranges strictly non-allocatable. |
| **Physical Allocation** | Race-free frame allocation with bounded capacity tracking (`free_count`, `allocated_count`). | Frame exhaustion returns `FrameAllocationError::OutOfMemory`. |
| **Paging** | Initialize 4-level x86_64 page tables with canonical address checking and race-safe mapping update. | Non-canonical addresses fail with `PagingError::NonCanonicalAddress`. |
| **Virtual Memory** | Enforce strict isolation between user (`0x00000000_00010000`) and kernel space (`0xFFFF8000_0000_0000`). | User access to kernel region yields `MmError::PermissionDenied`. |
| **Protection** | Enforce permissions (Read, Write, Execute, User) and W^X (Write XOR Execute) policy on all VMAs. | W+X permissions yield `MmError::PermissionDenied`. |
| **Swap** | Swap-out/in operations execute failure-safely using bounded swap slots and priority queueing. | Swap slot exhaustion sets OOM flag and returns `SwapError::OutOfSwapSlots`. |
| **Reclaim** | Proactive LRU/MGLRU page aging and background compaction under high memory pressure. | Lost-wakeup prevention notifies wait queue via atomic flag (`reclaim_wakeup_notified`). |
| **Free** | Frames, page table entries, and metadata accounting released exactly once. | Double-free attempt yields `FrameAllocationError::DoubleFree`. |
| **Cleanup** | Complete deallocation of process address spaces during thread termination or reaping. | No orphan frames or leaked page table nodes remain after process death. |

---

## 3. Subsystem Policies

### 3.1 Allocator Architecture & Classes
- **Physical Allocator Class**: Hybrid model combining a Buddy Allocator for contiguous page frames (4KiB, 2MiB, 1GiB) and a Slab Allocator (`SigmaSlabAllocator`) for small kernel data structure caching.
- **Frame Allocator Policy**: Bounded capacity, lock-protected free list with atomic counter tracking.
- **Paging Policy**: Multi-level page table tree supporting CR3 context switching, TLB invalidation, and lost-update prevention.
- **Virtual Memory Policy**: Virtual Memory Area (VMA) tree enforcing strict user/kernel isolation, permission checks, and total byte/VMA count limits.
- **Swap Policy**: Priority-driven swap slot queueing (`SwapPriority::High`, `Normal`, `Low`), zram/zswap memory compression, and double-swap prevention.
- **Reclaim & OOM Policy**: OOM is a designed state rather than an accidental panic. Memory pressure triggers `kswapd` and cgroup-aware OOM victim selection without kernel panics.
- **Memory Limits**: Max VMA capacity and per-cgroup RSS limits are enforced at allocation time.
- **NUMA Behavior**: Parse ACPI SLIT tables for distance-weighted node frame selection.
- **Memory Hotplug**: Dynamic frame descriptor updates with lock-ordered insertion.

---

## 4. Periodic Task Calendar

| Frequency | Task | Output | Required PR |
|---|---|---|---|
| Every PR | Build, lint, unit, and integration tests | CI result | `feat`, `fix`, `test`, `docs`, or `chore` PR |
| Daily | CI/nightly triage | Root cause + fix | `fix(memory)` or `fix(ci)` PR |
| Weekly | Memory subsystem review | Gap list + status | `docs(memory)` PR |
| Weekly | Linux/BSD memory study | Study note | `docs(studies)` PR |
| Biweekly | Allocator audit | Leak/OOM findings | `fix(memory)` / `test(memory)` PR |
| Biweekly | Paging and protection audit | Mapping findings | `fix(paging)` PR |
| Monthly | Boot-to-shell memory validation | Boot log + smoke report | `test(memory)` PR |
| Monthly | Memory stress/fuzz triage | Crash reports + fixes | `fix(memory)` PR |
| Monthly | Swap rehearsal | Swap enforcement report | `test(swap)` PR |
| Quarterly | Memory architecture review | ADR + roadmap | `docs(adr)` PR |
| Quarterly | Security review | Threat-model update | `security(memory)` PR |
| Quarterly | Release readiness review | Release checklist | `docs(release)` PR |

---

## 5. Memory Health Metrics

| Metric | Target |
|---|---|
| Memory CI Pass Rate | 100% |
| Boot-to-Shell Pass Rate | 100% |
| Orphan Frames After Stress Test | 0 |
| Double-Free Incidents | 0 |
| Frame Reuse Races | 0 |
| Memory Corruption Incidents | 0 |
| Declared Limits with Enforcement Tests | 100% |
| Memory Claims Linked to Evidence | 100% |
| Security-Sensitive PRs Reviewed | 100% |
