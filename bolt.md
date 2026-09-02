# Bolt ⚡ Agent Journal - Performance & Optimization Learnings

## 2026-03-31 - 64-bit Atomic Pointer Alignment and Zero-Copy Strategy

**Learning:** Atomic integer transmutes in Rust require matching bit widths between the backing `AtomicUsize` or `AtomicU32` and the target `repr(usize)` or `repr(u32)` enum. Mismatched transmutes fail at compile time on 64-bit target architectures (`E0512`).
**Action:** Always represent atomic state machine enums with explicit `#[repr(usize)]` or `AtomicU32` backing stores to ensure zero-cost, type-safe atomic swaps and zero-copy memory access across high-frequency package resolution loops.

## 2026-03-31 - SLUB/UMA Slab Object Caching in Kernel Allocator

**Learning:** High-frequency memory allocation in `src/kernel/memory/resource_allocator.rs` benefits significantly from slab object pools (`SlabObjectCacheAllocator`), reducing memory fragmentation and allocation latency for uniform kernel objects.
**Action:** Reuse pre-allocated slab caches for kernel process descriptors and network buffers rather than general buddy allocation.
