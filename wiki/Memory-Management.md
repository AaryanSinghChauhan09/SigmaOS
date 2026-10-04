# Memory Management

SigmaOS implements a multi-tier memory management subsystem written entirely in Rust. It encompasses physical frame allocation, virtual memory areas (VMAs), demand paging, copy-on-write (CoW), slab allocation, page caching, and NUMA-aware placement — all without any unsafe C dependencies.

---

## Architecture Overview

```
 ┌────────────────────────────────────────────────────────┐
 │                   User Address Space                    │
 │  VMAs: [code][data][heap][mmap][stack]                  │
 └───────────────────┬────────────────────────────────────┘
                     │ #PF / mmap / brk
 ┌───────────────────▼────────────────────────────────────┐
 │              Virtual Memory Manager                     │
 │  VMA tree │ Page Fault Handler │ CoW │ ASLR            │
 └───────────────────┬────────────────────────────────────┘
                     │ alloc_page / free_page
 ┌───────────────────▼────────────────────────────────────┐
 │            Physical Memory Manager                      │
 │  Buddy Allocator │ Slab Allocator │ Page Cache         │
 │  NUMA Zones      │ Huge Pages     │ Memory Pressure    │
 └────────────────────────────────────────────────────────┘
```

---

## Subsystems

### 1. Buddy Allocator (`src/memory/buddy.rs`)
- Power-of-two block allocator (orders 0–11, i.e. 4KB–8MB)
- O(log n) alloc/free
- Anti-fragmentation coalescing
- Per-NUMA-zone free lists

### 2. Slab Allocator (`src/memory/slab_allocator.rs`)
- Object caching for frequently allocated kernel structs
- Three slab states: `Empty`, `Partial`, `Full`
- Per-CPU magazines for lock-free fast path
- Automatic slab reclaim under memory pressure

### 3. Page Cache (`src/memory/page_cache.rs`)
- LRU-based file page cache
- Dirty page writeback with configurable throttle
- Integrated with VFS for transparent file reads
- Eviction pressure feedback to buddy allocator

### 4. Page Fault Handler (`src/mm/page_fault.rs`)
- Handles x86 `#PF` exceptions
- **Demand paging**: allocate zeroed pages on first access
- **Copy-on-Write**: private writable fork of shared pages
- User-mode access to kernel addresses → SIGSEGV
- Kernel null-dereference → KernelBug (oops)

### 5. Virtual Memory Areas (`src/memory/`)
- Red-black tree of VMAs per process (`vm_area_struct` equivalent)
- Flags: `READ | WRITE | EXEC | SHARED | GROWSDOWN`
- Backing: anonymous or file-mapped
- `mmap`, `munmap`, `mprotect`, `mremap` system calls

### 6. NUMA Support
- Physical memory divided into NUMA nodes
- Allocation policy: `LOCAL`, `INTERLEAVE`, `BIND`
- Distance matrix for cross-node latency awareness

### 7. Huge Pages
- 2 MB transparent huge pages (THP)
- Explicit `mmap(MAP_HUGETLB)` for 1 GB pages
- Automatic promotion of hot anonymous regions

### 8. RCU (`src/kernel/rcu.rs`)
- Read-Copy-Update for lock-free concurrent structures
- `RcuPointer<T>` with `AtomicPtr`-based epoch tracking
- Grace-period reclamation without stopping the world

---

## Memory Pressure & OOM

| Pressure Level | Action |
|---------------|--------|
| Low | Background kswapd writeback |
| Medium | Aggressive reclaim, drop clean caches |
| High | OOM killer activates, score-based victim selection |
| Critical | Emergency compaction + SIGKILL highest-score process |

---

## Comparison vs Linux / Omarchy / Mint

| Feature | Linux | Omarchy | Mint | **SigmaOS** |
|---------|-------|---------|------|-------------|
| Buddy allocator | ✅ | ✅ | ✅ | ✅ Rust, no unsafe |
| Slab/SLUB | ✅ | ✅ | ✅ | ✅ Per-CPU magazines |
| THP | ✅ | ✅ | ✅ | ✅ |
| NUMA-aware | ✅ | ❌ | ✅ | ✅ |
| CoW page fault | ✅ | ✅ | ✅ | ✅ Rust safe |
| RCU | ✅ | ✅ | ✅ | ✅ AtomicPtr |
| Async writeback | ✅ | ✅ | ✅ | ✅ |

---

## Source Files

| File | Description |
|------|-------------|
| `src/memory/buddy.rs` | Buddy frame allocator |
| `src/memory/slab_allocator.rs` | Slab object cache |
| `src/memory/page_cache.rs` | File page cache + writeback |
| `src/mm/page_fault.rs` | #PF handler, demand paging, CoW |
| `src/kernel/rcu.rs` | Read-Copy-Update |
| `src/buddy.rs` | Top-level buddy re-export |
| `src/slab.rs` | Top-level slab re-export |

---

## AI Agent Maintenance Instructions

> **For AI agents maintaining this page:**
> - Source directories: `src/memory/`, `src/mm/`, `src/kernel/rcu.rs`
> - Update NUMA node count and huge page sizes when hardware support expands
> - Add new allocator tiers (e.g., jemalloc-style arenas) to the architecture diagram
> - Keep the comparison table current vs latest kernel releases
