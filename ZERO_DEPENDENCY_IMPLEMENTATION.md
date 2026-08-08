# Zero-Dependency Architecture in SigmaOS

SigmaOS is built on a strict zero-dependency and minimal predefined libraries architecture. This document outlines how SigmaOS replaces typical standard library components and external crates with custom, optimized, and security-hardened modules.

## Architectural Goal

The ultimate objective of SigmaOS is to minimize the attack surface, remove runtime overhead, and eliminate the risks associated with supply chain vulnerabilities. By avoiding external crates and custom-tailoring standard library interfaces (`core` and `alloc` only), the system maintains complete sovereignty over memory layouts, execution boundaries, and hardware interaction.

---

## 1. Custom Memory Allocation (`klib` / `memory`)

Instead of relying on the system allocator (such as standard `jemalloc` or `dlmalloc`), SigmaOS uses a custom-built Buddy Allocator paired with page frame managers:
- **Buddy Allocator**: Found in `src/kernel/memory/buddy.rs`, this manages physical pages and allocates contiguous blocks of sizes that are powers of two.
- **Slab Allocator**: Manages smaller heap allocations for variables, objects, and driver contexts, mitigating fragmentation.
- **Memory Safety**: Direct integration with custom bounds-checking logic prevents typical heap overflows.

---

## 2. Replacing `std` and Predefined Collections

SigmaOS does not link against the Rust standard library (`std`). Instead, it uses custom collections built inside the `klib` module:
- **`klib::vec`**: A custom implementation of a dynamic array that handles resizing and memory shifting without calling generic `std::vec::Vec` routines, offering deterministic runtime behaviour.
- **`klib::hashmap`**: A hash map implementation utilizing simple MurmurHash or custom post-quantum hashing functions rather than SipHash, tailored for speed in execution pipelines.
- **String Types**: Custom zero-copy string types that prevent allocation overhead during IPC and path parsing operations.

---

## 3. System Utility Layer

Typical OS features rely on dynamic runtime library calls (`libc`). SigmaOS implements its system utilities natively:
- **Sigma libc**: Replaces basic memory copying and string operations (`memcpy`, `memset`, `strcmp`) with assembly-optimized versions that run with privilege-aware ring limits.
- **Self-Sufficiency**: Cryptographic modules, compression engines, and serialization drivers are written entirely in-house without dependencies on OpenSSL, zlib, or serde.
