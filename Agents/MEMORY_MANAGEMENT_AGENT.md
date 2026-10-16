# Memory Management & Paging Component Agent

## Role
`SigmaMemMgr` is the physical and virtual memory management subsystem for SigmaOS — governing physical frame allocation, 4-level/5-level paging, virtual address spaces, SLAB/SLUB/UMA object caching, and page fault resolution.

## Linux & BSD Inspiration
| Concept | Linux | FreeBSD | SigmaOS |
|---|---|---|---|
| Page allocator | Buddy allocator | Buddy allocator | SigmaBuddy (power-of-2 free lists) |
| Slab allocator | SLUB/SLAB | UMA (Universal Memory Allocator) | SigmaUMA (cache-aligned object pools) |
| Virtual memory | `vmalloc` / `vmap` | `vm_map` | SigmaVMM (demand paging & CoW) |
| Huge pages | THP / HugeTLB (2MB/1GB) | Superpages | SigmaHugePages |
| OOM killer | `mm/oom_kill.c` | arc_reclaim | SigmaOOM (badness scoring) |
| Memory zones | ZONE_DMA / NORMAL / HIGHMEM | VM zones | SigmaZones |
| Reclaim / Swap | MGLRU / zram | compressed swap | SigmaZRAM (zstd compressed swap) |

## Current SigmaOS Status
Buddy frame allocator, 4-level x86_64 paging, and kernel heap allocator implemented in `src/mm/` and `src/memory/`. Copy-on-Write (CoW) fork semantics and memory cgroups quota enforcement are active enhancement targets.

## Responsibilities & Architecture
- Physical frame allocation via power-of-2 buddy allocator free-lists.
- Slab/slub object caching with guard pages to prevent use-after-free and buffer overflows.
- Virtual memory address space lifecycle per process (demand-paged `mmap`, `munmap`, `mprotect`).
- Copy-on-Write (CoW) page fault handling for rapid `fork()`.
- NUMA node-aware physical page striping and allocation.
- Transparent Huge Page (THP) promotion and splitting.
- Low-memory OOM killer selecting victim processes based on badness scoring.
- Page cache integration with Zstandard-compressed swap devices (zram).

## Source Files
- `src/mm/buddy.rs`
- `src/mm/paging.rs`
- `src/mm/slab.rs`
- `src/mm/vmm.rs`
- `src/memory/`

## Testing Strategy
```bash
rustc --test src/memory/page_alloc.rs --edition 2021 -o /tmp/test_mem && /tmp/test_mem --test-threads=1
```
- Multi-core TLB shootdown stress testing.
- Memory exhaustion OOM recovery testing under synthetic load.
- Guard-page violation detection in slab caches.

## Future Development
1. **DAMON** — Data Access MONitor for dynamic memory tiering and hot/cold page migration.
2. **CXL / NVRAM tiering** — Tiered memory orchestration across Compute Express Link pools.
3. **MGLRU** — Multi-generational LRU page eviction algorithm.
4. **Kernel Samepage Merging (KSM)** — Deduplication of identical physical pages across containers.
