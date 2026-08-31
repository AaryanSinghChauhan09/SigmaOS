# Zero-Dependency Architecture

SigmaOS aims to eliminate all external C runtime dependencies from the core kernel.

## Implemented Custom Types

| Component | Replaces | Location |
|-----------|----------|----------|
| `klib::Vec<T>` | `std::vec::Vec` | `src/klib/vec.rs` |
| `klib::HashMap<K,V>` | `std::collections::HashMap` | `src/klib/hashmap.rs` |
| `klib::BTreeMap<K,V>` | `std::collections::BTreeMap` | `src/klib/btreemap.rs` |
| `klib::String` | `std::string::String` | `src/klib/string.rs` |
| `klib::Arc<T>` | `std::sync::Arc` | `src/klib/arc.rs` |
| `BuddyAllocator` | `malloc`/`free` | `src/klib/buddy_allocator.rs` |
| `SlabAllocator` | `kmem_cache_*` | `src/klib/slab.rs` |
| `RingBuffer<T>` | N/A | `src/klib/ring_buffer.rs` |
| `CustomHasher` | `DefaultHasher` | `src/klib/hash.rs` |

## Progress

| Subsystem | Dependency-Free? | Notes |
|-----------|-----------------|-------|
| Memory Manager | ✅ Yes | Pure Rust, no libc |
| Scheduler | ✅ Yes | `alloc` only |
| IPC Bus | ✅ Yes | Custom collections |
| VFS | 🚧 Partial | Some `alloc` usage |
| Network Stack | 🚧 Partial | TCP/UDP pure Rust |
| Security (SELinux) | ❌ No | Requires kernel LSM hooks |
| Graphics | ❌ No | Mesa dependency |
| Audio | ❌ No | PipeWire dependency |

## Goals

- [ ] 100% zero-dependency core kernel
- [ ] Custom audio processing
- [ ] Custom graphics subsystem
- [ ] Pure Rust DNS resolver (done)
- [ ] Pure Rust TLS 1.3 (done)
- [ ] Pure Rust WireGuard (in progress)
