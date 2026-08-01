# Custom Memory Allocator Specification (sigma-alloc)

## Overview

As part of the Low-Level Independence and Zero-Bloat Ecosystem principles, SigmaOS relies on a custom, highly-optimized memory allocator for both the kernel and core userland services (`sigma-alloc`). This eliminates the reliance on generic `libc` `malloc`/`free` or Rust's default system allocators, which carry overhead for backward compatibility.

## Design Goals

1. **O(1) Allocation/Deallocation for Small Objects**: Use a slab allocator pattern for frequently used objects (e.g., IPC messages, network packets, microVM descriptors).
2. **Deterministic Fragmentation Control**: We prioritize predictable performance over absolute memory density, favoring anti-fragmentation techniques crucial for long-running servers and AI workflows.
3. **Hardware Isolation**: Allocations for different microVMs must be strictly segregated at the page level. A compromised process should never be able to corrupt the metadata of another slab.
4. **No Standard Library Dependency**: Built purely in `no_std` Rust using atomic intrinsics.

## Implementation Details

### Slab Allocator

The kernel maintains a series of pre-allocated page pools for common sizes: 32B, 64B, 128B, 256B, 512B, 1KB, 2KB, 4KB.

- **Fast Path**: Lock-free pop from a per-CPU core cache.
- **Slow Path**: Spinlock-protected allocation from the global slab pool.

### Large Object Allocator (Buddy System)

For allocations > 4KB, a standard Buddy Allocator is used, merging adjacent free pages up to massive huge-page limits (2MB and 1GB) to optimize TLB usage.

## Integration

- `sigpkg`: Will use a specialized version of this allocator tailored for string hashing and transaction logs.
- `Zenith`: Will use the slab allocator for rapid UI event processing.
- `kernel`: Native `GlobalAlloc` implementation in Rust.

## Future Roadmap

- Implementation of a secure allocator variant with guard pages and randomized free lists for high-security applications (e.g., cryptographic keys).
