# Σ perf/mm — Memory Manager Performance Optimisations

Houses the high-performance memory management implementation for SigmaOS,
including the **buddy allocator**, **slab cache**, and **NUMA-aware page
allocator**.

## Allocator Stack

```
sigma_malloc(size)
   └─ Slab Cache (< 512 bytes, O(1) fixed-size slabs)
         └─ Buddy Allocator (≥ 512 bytes, power-of-two zones)
               └─ NUMA Page Allocator (selects nearest memory node)
                     └─ Physical Frame Allocator (bitmap)
```

## Key Design Points

| Feature | Detail |
|---|---|
| **Allocation** | O(1) slab, O(log N) buddy |
| **NUMA** | Allocates from the NUMA node closest to the requesting CPU |
| **Fragmentation** | Buddy coalescing keeps external fragmentation < 5% |
| **Safety** | Guard pages + canary values detect buffer overflows |
| **No libc** | `#![no_std]` — zero dependency on glibc/musl |

## API Interface

```c
// Allocate `size` bytes (kernel heap)
void *sigma_alloc(size_t size);

// Free a kernel heap pointer
void sigma_free(void *ptr);

// Allocate physically contiguous pages (for DMA)
void *sigma_alloc_pages(size_t order);

// Map a physical range into kernel virtual address space
void *sigma_map_phys(phys_addr_t phys, size_t size);

// Initialise the memory manager (called from kernel_main)
void init_perf_mm(void);
```

## NUMA Topology

SigmaOS discovers NUMA node topology from the ACPI SRAT table and maintains a
per-node free-list:

```
Node 0 (CPU 0-7,  RAM 0–64 GB)
Node 1 (CPU 8-15, RAM 64–128 GB)
```

Allocations prefer the local node; spill to remote only when local is exhausted.

## Roadmap

- [ ] Buddy allocator implementation
- [ ] Slab cache (fixed-size object pools)
- [ ] NUMA-aware page allocator
- [ ] Guard page + canary overflow detection
- [ ] Memory pressure callbacks (OOM handler)
- [ ] Transparent huge pages (THP) support
- [ ] Kani formal proofs: no double-free, no use-after-free

## Related Modules

- [`modules/core/kernel/memory`](../../core/kernel/memory/) — Page-table management
- [`modules/perf/bench`](../bench/README.md) — Memory allocator benchmarks
