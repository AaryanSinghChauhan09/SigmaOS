# Dependency Reduction Progress — SigmaOS

This page tracks the ongoing effort to eliminate predefined library dependencies
and replace them with SigmaOS-native implementations, inspired by BSD and Linux kernel philosophy.

## Philosophy

> "Write everything you need, need everything you write."
> — SigmaOS Core Development Principle

Reducing external dependencies improves:

*   **Security** — smaller attack surface
*   **Portability** — runs on bare metal with no host OS
*   **Control** — full understanding of every subsystem
*   **Learning** — deep OS engineering knowledge

***

## Custom Implementations Completed

### Memory Management

| Module | Replaces | Location |
|--------|----------|----------|
| `klib/vec.rs` — Custom `Vec<T>` | `std::vec::Vec` | `src/klib/vec.rs` |
| `klib/buddy_allocator.rs` — Buddy allocator | `std::alloc` | `src/klib/buddy_allocator.rs` |
| `klib/paging.rs` — 4-level page tables | OS paging libs | `src/klib/paging.rs` |
| `kernel/memory.rs` — Frame allocator | `std::alloc` | `src/kernel/memory.rs` |

### String & I/O

| Module | Replaces | Location |
|--------|----------|----------|
| `sigma_libc.h` — Custom C header | system libc | `sigma_libc.h` |
| `klib/` string primitives | `std::str`, `std::string` | `src/klib/` |

### Synchronization

| Module | Replaces | Location |
|--------|----------|----------|
| Atomic operations via `core::sync::atomic` | `std::sync` | across `klib/` |
| Custom spinlocks (planned) | `std::sync::Mutex` | TBD |

***

## In Progress

| Component | Current State | Target |
|-----------|---------------|--------|
| Custom hashmap | Using `alloc::collections::BTreeMap` | `klib/map.rs` |
| Custom string type | Using `alloc::string::String` in some files | `klib/str.rs` |
| Custom I/O primitives | Partial (VESA, UART drivers) | Complete driver framework |
| Custom formatting | Using `core::fmt` | Custom fmt macros |

***

## Planned Replacements

### Phase 2 — Custom Collections

    klib/map.rs       → replaces BTreeMap/HashMap
    klib/set.rs       → replaces BTreeSet/HashSet  
    klib/linked_list.rs → replaces LinkedList
    klib/ringbuf.rs   → replaces VecDeque

### Phase 3 — Custom Primitives

    klib/fmt.rs       → custom print!/format! macros
    klib/str.rs       → custom &str + owned string  
    klib/slice.rs     → slice utility functions

### Phase 4 — No-std Kernel Core

    All kernel/ files → pure #![no_std] + #![no_main]
    No alloc crate   → custom allocator only
    No core re-use   → architecture-specific core

***

## Linux / BSD Inspirations

| Concept | Source OS | SigmaOS Implementation |
|---------|-----------|----------------------|
| Buddy allocator | Linux `mm/page_alloc.c` | `klib/buddy_allocator.rs` |
| EEVDF scheduler | Linux 6.6+ | `kernel/scheduler.rs` |
| Page table walk | x86\_64 Linux | `klib/paging.rs` |
| Pledge/Unveil | OpenBSD | `security/capability.rs` |
| Jails | FreeBSD | `security/` (planned) |
| Kqueue | FreeBSD/macOS | `net/` (planned) |
| POSIX-compatible VFS | Linux/BSD | `filesystem/` |
| DRM/KMS GPU abstraction | Linux DRM | `drivers/gpu.rs` |
| TDR (Timeout Detection Recovery) | Windows/Linux | `drivers/gpu.rs` |

***

## Metrics

| Metric | Before | Current | Target |
|--------|--------|---------|--------|
| External crate deps | ~15 | ~8 | 0 |
| std usage in kernel | Heavy | Partial | None |
| Custom allocators | 0 | 1 | 3 |
| Custom collections | 0 | 1 (Vec) | 6 |

***

## Related Pages

*   [Zero Dependency Architecture](Zero-Dependency-Architecture)
*   [STD Elimination Plan](STD-Elimination-Plan)
*   [Kernel Architecture](Architecture-Overview)
