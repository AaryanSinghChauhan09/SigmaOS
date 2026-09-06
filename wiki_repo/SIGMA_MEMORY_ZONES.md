# SigmaOS Zone-Based Memory Allocator

## Overview

`src/kernel/sigma_memory_zones.rs` implements a zone-based physical memory
allocator for SigmaOS, inspired by the Linux buddy allocator and BSD UMA
(Universal Memory Allocator / slab subsystem).

---

## Architecture

```
┌─────────────────────────────────────────────────┐
│                ZoneAllocator                     │
│  ┌─────────┐  ┌─────────┐  ┌───────────────┐   │
│  │  Zone   │  │  Zone   │  │     Zone      │   │
│  │   DMA   │  │ Normal  │  │   HighMem     │   │
│  │ Buddy   │  │ Buddy   │  │   Buddy       │   │
│  └─────────┘  └─────────┘  └───────────────┘   │
│                                                  │
│  ┌───────────────────────────────────────────┐  │
│  │           SlabCache (per object type)     │  │
│  └───────────────────────────────────────────┘  │
└─────────────────────────────────────────────────┘
```

---

## Memory Zones

| Zone | Typical Range (x86-64) | Purpose |
|------|------------------------|---------|
| `Dma` | 0 – 16 MiB | Legacy ISA DMA-able memory |
| `Normal` | 16 MiB – all RAM | Standard kernel allocations |
| `HighMem` | >896 MiB (32-bit only) | Non-permanently-mapped pages |
| `Device` | MMIO / CXL / PMEM | Device and persistent memory |

Allocation prefers `Normal` first, falls back to `HighMem`, then `Device`,
and only uses `Dma` as a last resort to preserve DMA-capable pages for drivers.

---

## Buddy Allocator

### Design

The buddy system groups pages into **order-n blocks** where block size = 2^n pages.

| Order | Pages | Size |
|-------|-------|------|
| 0 | 1 | 4 KiB |
| 1 | 2 | 8 KiB |
| 2 | 4 | 16 KiB |
| … | … | … |
| 11 | 2048 | 8 MiB |

Free blocks are stored in per-order `VecDeque<usize>` lists.

### Allocation (`alloc_pages(order)`)

1. Check watermark: refuse if `free_pages < watermarks.min + 2^order`.
2. Find the smallest available order ≥ requested order.
3. Pop a block from that order's free list.
4. **Split** the block down to the requested order, returning the upper half of
   each split to the appropriate free list.

### Deallocation (`free_pages(addr, order)`)

1. Add `2^order` to `free_pages`.
2. Compute the buddy address: `buddy = block_frame XOR (1 << order)`.
3. If the buddy is free, remove it and merge into a `(order+1)` block.
4. Repeat until no buddy is free or maximum order is reached.

### Complexity

| Operation | Time |
|-----------|------|
| alloc_pages(order) | O(MAX_ORDER) ≈ O(12) |
| free_pages(addr, order) | O(MAX_ORDER) ≈ O(12) |

---

## Zone Watermarks

Each zone has three watermarks (in pages):

| Watermark | Default fraction | Meaning |
|-----------|-----------------|---------|
| `min` | 1% of zone | Allocation fails below this |
| `low` | 2.5% of zone | Background (kswapd-equiv) reclaim triggered |
| `high` | 5% of zone | Reclaim stops here |

### Pressure Levels

```rust
pub enum PressureLevel {
    Normal,    // free > high   — no action needed
    Moderate,  // low < free ≤ high — proactive reclaim beneficial
    High,      // min < free ≤ low  — background reclaim must run
    Critical,  // free ≤ min   — allocations fail
}
```

---

## Slab Cache

The `SlabCache` provides O(1) fixed-size object allocation on top of pages
obtained from the buddy allocator.

```rust
pub struct SlabCache {
    pub name:             String,
    pub obj_size:         usize,      // bytes per object (aligned)
    pub alignment:        usize,      // must be power of two
    free_objects:         VecDeque<usize>,
    pub total_allocated:  usize,
    pub in_use:           usize,
}
```

### Lifecycle

```
BuddyAllocator::alloc_pages(order)
         │
         ▼
SlabCache::grow(slab_base, slab_size)  -- register pages as object pool
         │
         ▼
SlabCache::alloc()   → address
SlabCache::free(addr)
```

### Comparison with Linux SLUB

| Feature | Linux SLUB | SigmaOS SlabCache |
|---------|------------|-------------------|
| Per-CPU caches | Yes | Planned |
| Kmem debugging | KASAN/SLUB_DEBUG | Future |
| Poisoning | Yes | Future |
| Size classes | Dynamic | Fixed per cache |

---

## NUMA Awareness (Planned)

Future versions will expose per-NUMA-node zone sets.  The `ZoneAllocator`
API will accept an optional `Numa` hint to prefer allocations from the
caller's local node before falling back to remote nodes.

---

## API Summary

```rust
// Create zones.
let descs = [
    (MemoryZone::Dma,    0,               4_096),  // 16 MiB
    (MemoryZone::Normal, 16 * 1024 * 1024, 262_144), // 1 GiB
];
let mut za = ZoneAllocator::new(&descs);

// Allocate a contiguous 16-page (64 KiB) block.
let addr = za.alloc_pages(4 /* order 4 = 16 pages */).unwrap();

// Free it.
za.free_pages(MemoryZone::Normal, addr, 4);

// Register and use a slab cache.
let mut cache = SlabCache::new("task_struct", 512, 16);
cache.grow(za.alloc_pages(0).unwrap(), PAGE_SIZE);
za.register_cache(cache);
let obj = za.slab_alloc("task_struct").unwrap();
za.slab_free("task_struct", obj);

// Print stats.
println!("{}", za.stats());
```

---

## Source Location

`src/kernel/sigma_memory_zones.rs`
