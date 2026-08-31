# SigmaOS Zero-Stdlib Dependency Architecture Guide

## Overview

SigmaOS minimizes dependence on Rust's standard library (`std`) and pre-defined external libraries. This guide documents the philosophy, current status, and roadmap for achieving full zero-dependency operation.

## Why Zero-Stdlib?

| Concern | `std` Dependency | SigmaOS Custom |
|---------|-----------------|----------------|
| **Binary size** | Large (~1.5MB overhead) | Minimal (only what's needed) |
| **Security surface** | Large, includes network, I/O | Zero unnecessary surface |
| **Portability** | Requires OS primitives | Runs bare-metal |
| **Auditability** | Hard to audit all stdlib | Every line of code auditable |
| **Performance** | General-purpose allocator | Domain-optimized allocators |

## Architecture Layers

    ┌───────────────────────────────────────────┐
    │           Userspace Applications          │
    ├───────────────────────────────────────────┤
    │         SigmaOS System Libraries          │
    │  (sigpkg, sigma-sh, kabi) - zero ext deps │
    ├───────────────────────────────────────────┤
    │           Core Kernel (src/)              │
    │  Uses: #![no_std] + extern crate alloc   │
    ├───────────────────────────────────────────┤
    │            src/klib/ Layer                │
    │  Vec, BTreeMap, BuddyAllocator, Paging   │
    ├───────────────────────────────────────────┤
    │       Hardware Abstraction Layer          │
    │  (raw pointers, MMIO, assembly)           │
    └───────────────────────────────────────────┘

## The `src/klib/` Custom Library

| Module | Provides | Replaces |
|--------|----------|---------|
| `klib::vec` | `Vec<T>` with custom allocator | `std::vec::Vec` |
| `klib::hashmap` | `BTreeMap<K,V>` bucket-based | `std::collections::HashMap` |
| `klib::buddy_allocator` | Physical page frame allocator | System malloc |
| `klib::paging` | Virtual memory page table management | OS paging APIs |
| `klib::hash` | FNV-1a and SipHash implementations | `std::hash` |
| `klib::error` | `SigmaError`, `SecurityError` types | `std::error::Error` |

## Usage Rules

```rust
// ❌ DO NOT USE (pulls in std)
use std::collections::HashMap;
use std::vec::Vec;

// ✅ PREFERRED: Use alloc
extern crate alloc;
use alloc::vec::Vec;
use alloc::string::String;

// ✅ BEST: Use klib for OS-internal code
use crate::klib::vec::Vec;
use crate::klib::hashmap::BTreeMap;
```

## Migration Status

### ✅ Completed (no std dependency)

*   `src/security/pledge.rs` - Full no\_std with alloc
*   `src/security/unveil.rs` - Full no\_std with alloc
*   `src/memory/kswapd.rs` - Full no\_std with alloc
*   `src/klib/buddy_allocator.rs` - Core-only
*   `src/crypto/pqc_dilithium.rs` - alloc only

### 🚧 In Progress

*   `src/net/` - Some modules still use `std::net` types
*   `src/filesystem/` - VFS layer in transition

### ❌ TODO

*   `tools/` - CLI tools (may legitimately use std for host tooling)
*   Test harnesses (`#[cfg(test)]` blocks may use std)

## References

*   [Kernel Architecture](Kernel-Architecture)
*   [BSD Inspirations and Parity](BSD-Inspirations-and-Parity)
*   [Zero-Dependency Subsystems](Zero-Dependency-Subsystems)
