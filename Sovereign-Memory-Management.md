# Sovereign Memory Management (S-MM)

The **Sovereign Memory Manager (S-MM)** is the high-performance paging and slab allocation engine designed for the **SigmaOS v15.0 "Horizon"** release. It replaces the legacy bump-allocator with an industrial-grade memory singularity.

## Architecture

S-MM operates on a dual-tier allocation strategy:

1.  **Paging Tier (HugePages)**:
    *   Industrial identity mapping for core kernel shards.
    *   2MB HugePage support for zero-latency memory access.
    *   Post-quantum attested page table protection.

2.  **Slab Tier (Industrial Caches)**:
    *   Power-of-two slab caches (16B to 2KB).
    *   Atomic free-list management for stable multitasking.
    *   Zero-fragmentation compaction via shard auditing.

## Implementation Details

The engine is encapsulated in the `SigmaOS::Kernel::Memory::SovereignMemoryManager` class, utilizing a singleton pattern for global silicon access.

### Core Primitives

- `mm_init()`: Initializes page tables and slab caches.
- `mm_malloc(size)`: Performs high-speed slab allocation or falls back to page allocation for large shards.
- `mm_free(ptr)`: Returns memory to the appropriate industrial cache.

## Security Features

- **NX (No-Execute)**: Data shards are hardware-enforced to prevent code injection.
- **ASLR Entropy**: High-entropy randomization for lattice node placement.
- **Shard Isolation**: Guaranteed boundary protection between Ring-0 modules.

---
*Stay Sovereign.*
