# SigmaOS Dependency Reduction Guide

> **Goal:** Zero external crate dependencies at runtime. Every function needed
> by the kernel and userland is implemented in `src/klib/`.

---

## Table of Contents

1. [Why Reduce Dependencies?](#why-reduce-dependencies)
2. [The klib Philosophy](#the-klib-philosophy)
3. [What klib Replaces](#what-klib-replaces)
4. [Module Reference](#module-reference)
5. [How to Port Code to klib](#how-to-port-code-to-klib)
6. [Common std → klib Migration Patterns](#common-std--klib-migration-patterns)
7. [Build Verification](#build-verification)
8. [Adding New klib Modules](#adding-new-klib-modules)
9. [Forbidden Imports](#forbidden-imports)
10. [Current Status](#current-status)

---

## Why Reduce Dependencies?

### Security Surface
Every external crate is a potential supply-chain attack vector. The `solarwinds`
and `xz-utils` incidents demonstrated that even widely-trusted packages can be
compromised. SigmaOS's threat model requires that **every line of code running in
ring 0 be authored and audited by the SigmaOS project**.

### Minimal Binary
External crates often pull in large dependency trees. Eliminating them reduces
the final kernel image size, improves cache locality, and shortens boot time.

### `no_std` Kernel Requirement
The SigmaOS kernel runs before the OS is initialised. There is no runtime,
no allocator, no filesystem. Rust's `std` library requires all of these.
`klib` provides the same functionality without any of those preconditions.

### Auditability
A 3 000-line custom hash map is easier to audit than a 15-crate dependency chain
tracing back through `hashbrown → ahash → foldhash → ...`.

### Reproducibility
External crates change. Pinning crate versions locks checksums but not the entire
transitive closure. `klib` changes only when SigmaOS developers change it.

---

## The klib Philosophy

```
┌─────────────────────────────────────────────────────────┐
│                     SigmaOS kernel                      │
│                                                         │
│  src/kernel/  src/security/  src/network/  ...          │
│        │              │              │                  │
│        └──────────────┴──────────────┘                  │
│                       │                                 │
│               src/klib/ (no external deps)              │
│                       │                                 │
│           core (Rust built-in, no OS needed)            │
└─────────────────────────────────────────────────────────┘
```

The dependency graph is strictly:
```
kernel code  →  klib  →  core (no_std)  →  nothing
```

No arrow points outside SigmaOS. No crate from crates.io is used at runtime.

---

## What klib Replaces

### Standard Library Replacements

| `std` / external crate | klib replacement | File |
|------------------------|-----------------|------|
| `std::vec::Vec` | `klib::Vec<T>` | `src/klib/vec.rs` |
| `std::collections::HashMap` | `klib::HashMap<K,V>` | `src/klib/hashmap.rs` |
| `std::collections::HashSet` | `klib::HashSet<T>` | `src/klib/hashset.rs` |
| `std::collections::BTreeMap` | `klib::BTreeMap<K,V>` | `src/klib/btreemap.rs` |
| `std::collections::VecDeque` | `klib::VecDeque<T>` | `src/klib/vecdeque.rs` |
| `std::string::String` | `klib::SigmaString` | `src/klib/custom_string.rs` |
| `std::alloc::System` | `klib::SigmaAllocator` | `src/klib/custom_allocator.rs` |
| `uuid` crate | `klib::Uuid` | `src/klib/uuid.rs` |
| `libc` (memset/memcpy) | `klib::string::*` | `src/klib/string.rs` |
| `tokio` / `async-std` | `klib::AsyncRuntime` | `src/klib/async_runtime.rs` |
| `base64` crate | `klib::base64_encode/decode` | `src/klib/conversion.rs` |
| `num` math crate | `klib::math::*` | `src/klib/math.rs` |
| `time` / `chrono` | `klib::Timestamp`, `klib::Date` | `src/klib/time.rs` |
| `sha2` / `sha3` | `klib::hash::*` | `src/klib/hash.rs` |
| `x86_64` ISA crate | `klib::isa::*` | `src/klib/isa.rs` |
| Paging structures | `klib::paging::*` | `src/klib/paging.rs` |
| Buddy allocator crates | `klib::BuddyAllocator` | `src/klib/buddy_allocator.rs` |
| Virtual memory maps | `klib::UserVmMap` | `src/klib/uvm.rs` |
| Key-value stores | `klib::Store` | `src/klib/store.rs` |

---

## Module Reference

### `src/klib/custom_allocator.rs`

Implements `core::alloc::GlobalAlloc` with two allocation strategies:
1. **Bump allocator** – O(1) alloc, O(1) dealloc (dealloc is a no-op until
   reset). Used during early boot before the heap is set up.
2. **Free-list allocator** – standard next-fit free list. Used after boot.

```rust
use klib::custom_allocator::SigmaAllocator;

#[global_allocator]
static ALLOCATOR: SigmaAllocator = SigmaAllocator::new();
```

### `src/klib/custom_string.rs`

`SigmaString` is a heap-backed UTF-8 string backed by `SigmaVec<u8>`.

```rust
use klib::SigmaString;

let mut s = SigmaString::from("hello");
s.push_str(", world");
assert_eq!(s.as_str(), "hello, world");
```

Key traits implemented: `Display`, `PartialEq`, `PartialOrd`, `Clone`, `From<&str>`,
`From<&[u8]>`, `core::fmt::Write` (for `write!` macro support).

### `src/klib/vec.rs`

`Vec<T>` with the same API as `std::vec::Vec<T>` but backed by `SigmaAllocator`.

```rust
use klib::Vec;

let mut v: Vec<u32> = Vec::new();
v.push(1);
v.push(2);
assert_eq!(v.len(), 2);
```

### `src/klib/hashmap.rs`

Open-addressing hash map using Robin Hood hashing.

```rust
use klib::HashMap;

let mut map: HashMap<u64, SigmaString> = HashMap::new();
map.insert(42, SigmaString::from("answer"));
```

### `src/klib/async_runtime.rs`

Minimal single-threaded executor for `async/await` in the kernel.

```rust
use klib::async_runtime::block_on;

block_on(async {
    let result = async_read_sector(0).await;
});
```

No Tokio, no async-std. The executor polls futures using a work-stealing queue.

### `src/klib/isa.rs`

x86-64 ISA utilities.

```rust
use klib::isa::{cpuid, rdmsr, wrmsr, CpuFeatures};

let features = CpuFeatures::detect();
if features.avx512f {
    // use AVX-512 path
}
```

### `src/klib/paging.rs`

Page-table structures for x86-64 (4-level and 5-level paging).

```rust
use klib::paging::{PageTable, PageTableEntry, PageFlags};

let mut pt = PageTable::new();
pt.map(virt_addr, phys_addr, PageFlags::PRESENT | PageFlags::WRITABLE);
```

### `src/klib/uvm.rs`

Userspace virtual memory maps (VMA tracking, mmap/munmap operations).

### `src/klib/store.rs`

Key-value store abstraction used by the package manager and config system.

### `src/klib/buddy_allocator.rs`

Power-of-two buddy allocator for large (>4 KB) kernel allocations.

```rust
use klib::BuddyAllocator;

static KERNEL_HEAP: BuddyAllocator = BuddyAllocator::new();
KERNEL_HEAP.init(heap_start, heap_size);
let ptr = KERNEL_HEAP.alloc(4096 * 16); // alloc 16 pages
```

---

## How to Port Code to klib

### Step 1: Identify std Usage

```bash
grep -rn 'use std::' src/kernel/ src/security/ src/network/
grep -rn 'extern crate' src/
grep -rn '^use ' src/klib/  # check klib itself
```

### Step 2: Replace Imports

```rust
// Before
use std::vec::Vec;
use std::collections::HashMap;
use std::string::String;

// After
use crate::klib::Vec;
use crate::klib::HashMap;
use crate::klib::SigmaString as String;
```

### Step 3: Replace Allocations

```rust
// Before
let mut buf = String::new();
buf.push_str("hello");

// After
let mut buf = SigmaString::new();
buf.push_str("hello");
```

### Step 4: Replace Formatting

```rust
// Before
let s = format!("value = {}", x);

// After
use klib::custom_string::sigma_format;
let s = sigma_format!("value = {}", x);
```

### Step 5: Replace I/O

```rust
// Before (std)
println!("debug: {}", msg);

// After (klib)
klib::debug_print!("debug: {}", msg);  // writes to serial port
```

---

## Common std → klib Migration Patterns

### Pattern 1: Vec operations

```rust
// std
let v: Vec<u8> = vec![1, 2, 3];
let sum: u32 = v.iter().map(|x| *x as u32).sum();

// klib (iterators work the same way – klib Vec implements Iterator)
let v: klib::Vec<u8> = klib::vec![1, 2, 3];
let mut sum: u32 = 0;
for x in v.iter() { sum += *x as u32; }
```

### Pattern 2: String formatting

```rust
// std
eprintln!("Error: {}", e);

// klib
use klib::serial_println;
serial_println!("Error: {}", e);
```

### Pattern 3: Sorting

```rust
// std
v.sort();

// klib
klib::sort::insertion_sort(&mut v); // for small arrays
klib::sort::merge_sort(&mut v);     // for large arrays
```

### Pattern 4: Error handling

```rust
// std uses Box<dyn Error> which requires alloc
fn read() -> Result<u32, Box<dyn std::error::Error>> { ... }

// klib uses typed errors
fn read() -> Result<u32, klib::error::KlibError> { ... }
```

---

## Build Verification

After porting code to klib, verify no std leakage:

```bash
# Check for std usage in kernel crate
cargo check --no-default-features --features kernel 2>&1 | grep 'std'

# Check the final kernel binary has no libc symbols
readelf -s target/x86_64-unknown-none/release/sigma_kernel \
  | grep -i 'glibc\|musl\|libc'
# should return nothing

# Check binary size
ls -lh target/x86_64-unknown-none/release/sigma_kernel
```

The CI pipeline runs `scripts/smoke-test.sh` which includes:
```bash
# Verify no external symbols
nm target/*/release/sigma_kernel | grep 'U ' | grep -v '__' && exit 1 || echo "OK"
```

---

## Adding New klib Modules

When you need functionality that isn't in klib yet:

1. Create `src/klib/<module_name>.rs`
2. Add `#![no_std]` at the top (or `#[cfg(not(feature="std"))]` guards)
3. Add `pub mod <module_name>;` to `src/klib/mod.rs`
4. Add a `pub use <module_name>::*;` re-export if appropriate
5. Add unit tests using `#[cfg(test)]` modules
6. Document with `///` doc comments
7. Submit PR with benchmark data showing it is ≥ the std equivalent

### Module Template

```rust
//! klib::<module_name> – description
//!
//! No external dependencies. No std.

#![cfg_attr(not(test), no_std)]
#![allow(dead_code)]

// your code here

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        // ...
    }
}
```

---

## Forbidden Imports

The following are **never** allowed in kernel code:

```rust
// FORBIDDEN in src/kernel/, src/security/, src/klib/
use std::*;
use libc::*;
use tokio::*;
use async_std::*;
use serde::*;           // use klib serialisation instead
use log::*;             // use klib::serial_println! instead
use anyhow::*;          // use klib::error::KlibError instead
use thiserror::*;       // implement Error manually
use rand::*;            // use klib::random instead (CSPRNG)
```

The CI workflow (`.github/workflows/sigma_ci.yml`) runs:
```yaml
- name: Check forbidden imports
  run: |
    grep -rn 'use std' src/kernel src/klib src/security \
      && echo "FAIL: std usage found in kernel code" && exit 1
    echo "PASS"
```

---

## Current Status

| Module | Status | std-free? | Tested? |
|--------|--------|-----------|---------|
| `custom_allocator` | ✅ Complete | ✅ Yes | ✅ Yes |
| `custom_string`    | ✅ Complete | ✅ Yes | ✅ Yes |
| `vec`              | ✅ Complete | ✅ Yes | ✅ Yes |
| `hashmap`          | ✅ Complete | ✅ Yes | ✅ Yes |
| `hashset`          | ✅ Complete | ✅ Yes | ✅ Yes |
| `btreemap`         | ✅ Complete | ✅ Yes | ✅ Yes |
| `vecdeque`         | ✅ Complete | ✅ Yes | ✅ Yes |
| `paging`           | ✅ Complete | ✅ Yes | ✅ Yes |
| `buddy_allocator`  | ✅ Complete | ✅ Yes | ✅ Yes |
| `async_runtime`    | ✅ Complete | ✅ Yes | ⚠️ Partial |
| `isa`              | ✅ Complete | ✅ Yes | ✅ Yes |
| `uvm`              | ✅ Complete | ✅ Yes | ⚠️ Partial |
| `store`            | ✅ Complete | ✅ Yes | ✅ Yes |
| `math`             | ✅ Complete | ✅ Yes | ✅ Yes |
| `hash`             | ✅ Complete | ✅ Yes | ✅ Yes |
| `time`             | ✅ Complete | ✅ Yes | ✅ Yes |
| `uuid`             | ✅ Complete | ✅ Yes | ✅ Yes |
| `conversion`       | ✅ Complete | ✅ Yes | ✅ Yes |
| `error`            | ✅ Complete | ✅ Yes | ✅ Yes |

**Remaining std usage in non-kernel code:** 3 places (all in test harness, not shipped).

---

*Last updated: 2026-08-04*
