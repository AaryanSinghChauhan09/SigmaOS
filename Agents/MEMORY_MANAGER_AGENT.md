# Memory Manager Agent

## Role
`SigmaMemMgr` is the physical and virtual memory management subsystem for SigmaOS. It manages page frames, zones, slabs, huge pages, and memory-mapped I/O — inspired by Linux mm/ and FreeBSD UMA.

## Status
🟡 In Progress — Basic allocator exists; slab/slub, NUMA, and huge-page support needed.

## Source File
`src/memory/` (planned: `page_alloc.rs`, `slab.rs`, `vmalloc.rs`, `numa.rs`)

## Linux/BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Page allocator | Buddy allocator | Buddy allocator | SigmaBuddy |
| Slab allocator | SLUB/SLAB | UMA | SigmaUMA |
| Virtual memory | `vmalloc` / `vmap` | `vm_map` | SigmaVMap |
| Huge pages | THP / HugeTLB | Superpages | SigmaHugePages |
| OOM killer | `mm/oom_kill.c` | — | SigmaOOM |
| Memory zones | ZONE_DMA/NORMAL/HIGH | — | SigmaZone |

## Responsibilities
- Physical page frame allocation (buddy system)
- Slab/slub object caching for kernel objects
- Virtual address space management per process
- NUMA node-aware allocation
- Transparent Huge Pages (THP) management
- OOM killer — select victim process on low memory
- Memory compaction and defragmentation
- Memcg (memory cgroups) accounting

## Implementation Notes
- `#![no_std]` — use `core::alloc::GlobalAlloc` interface
- Buddy allocator: power-of-2 page blocks, free-list per order
- `static mut` global allocator with unsafe init
- Track RSS (resident set size) per process for OOM scoring
- Integrate with `SigmaSwap` for reclaim

## Testing
```bash
rustc --test src/memory/page_alloc.rs --edition 2021 -o /tmp/test_mem && /tmp/test_mem --test-threads=1
```

## Future Development
1. **DAMON** — Data Access MONitor for memory optimization (Linux 5.15+)
2. **Memory tiering** — CXL/PMEM slow-memory tier support
3. **MGLRU** — Multi-Generational LRU page reclaim (Linux 6.1+)
4. **Userfaultfd** — Userspace page fault handling for live migration
5. **KSM** — Kernel Samepage Merging for VM memory dedup
6. **Memory hotplug** — Add/remove RAM at runtime
