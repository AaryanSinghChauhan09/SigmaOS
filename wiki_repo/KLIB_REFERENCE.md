# SigmaOS klib — Sovereign Kernel Library Reference

> **Philosophy**: SigmaOS has zero external libc/std dependencies. `klib` is the
> drop-in replacement for Rust's standard library, implemented entirely in bare-metal
> Rust for `no_std` freestanding environments.

---

## Table of Contents

1. [Overview](#overview)
2. [Allocators](#allocators)
   - [Buddy Allocator](#buddy-allocator)
   - [Slab Allocator](#slab-allocator)
   - [Custom Allocator](#custom-allocator)
3. [Collections](#collections)
   - [Vec](#vec)
   - [HashMap / HashSet](#hashmap--hashset)
   - [BTreeMap](#btreemap)
   - [LinkedList](#linkedlist)
   - [VecDeque](#vecdeque)
   - [RingBuffer](#ringbuffer)
4. [String Handling](#string-handling)
5. [Hashing & RNG](#hashing--rng)
6. [Paging & Virtual Memory](#paging--virtual-memory)
7. [Math & Bit Operations](#math--bit-operations)
8. [UUID Generation](#uuid-generation)
9. [Time & Timers](#time--timers)
10. [Async Runtime](#async-runtime)
11. [FFI & ISA Utilities](#ffi--isa-utilities)
12. [Dependency Reduction Status](#dependency-reduction-status)

---

## Overview

| Module | File | std Equivalent | Status |
|--------|------|----------------|--------|
| `buddy_allocator` | `src/klib/buddy_allocator.rs` | `GlobalAlloc` | ✅ Complete |
| `slab` | `src/klib/slab.rs` | `SlabAllocator` | ✅ Complete |
| `custom_allocator` | `src/klib/custom_allocator.rs` | `std::alloc` | ✅ Complete |
| `vec` | `src/klib/vec.rs` | `std::vec::Vec` | ✅ Complete |
| `hashmap` | `src/klib/hashmap.rs` | `std::collections::HashMap` | ✅ Complete |
| `hashset` | `src/klib/hashset.rs` | `std::collections::HashSet` | ✅ Complete |
| `btreemap` | `src/klib/btreemap.rs` | `std::collections::BTreeMap` | ✅ Complete |
| `linked_list` | `src/klib/linked_list.rs` | `std::collections::LinkedList` | ✅ Complete |
| `vecdeque` | `src/klib/vecdeque.rs` | `std::collections::VecDeque` | ✅ Complete |
| `ring_buffer` | `src/klib/ring_buffer.rs` | N/A (kernel-specific) | ✅ Complete |
| `custom_string` | `src/klib/custom_string.rs` | `std::string::String` | ✅ Complete |
| `string` | `src/klib/string.rs` | `str` / `String` | ✅ Complete |
| `string_ops` | `src/klib/string_ops.rs` | `str` methods | ✅ Complete |
| `hash` | `src/klib/hash.rs` | `std::hash` | ✅ Complete |
| `rng` / `random` | `src/klib/rng.rs` | `rand` crate | ✅ Complete |
| `math` / `math_ops` | `src/klib/math.rs` | `std::num` | ✅ Complete |
| `uuid` | `src/klib/uuid.rs` | `uuid` crate | ✅ Complete |
| `time` | `src/klib/time.rs` | `std::time` | ✅ Complete |
| `async_runtime` | `src/klib/async_runtime.rs` | `tokio` / `async-std` | ✅ Complete |
| `paging` | `src/klib/paging.rs` | N/A | ✅ Complete |
| `uvm` | `src/klib/uvm.rs` | N/A | ✅ Complete |
| `ffi` | `src/klib/ffi.rs` | `std::ffi` | ✅ Complete |
| `error` | `src/klib/error.rs` | `std::error` | ✅ Complete |

---

## Allocators

### Buddy Allocator

**File**: `src/klib/buddy_allocator.rs`
**Inspired by**: Linux kernel `mm/page_alloc.c`, FreeBSD `vm_page.c`

The buddy allocator is the primary physical memory allocator. It manages memory in
power-of-two blocks and uses the "buddy system" to coalesce free blocks efficiently.

```rust
// Usage
use crate::klib::buddy_allocator::BuddyAllocator;

let mut allocator = BuddyAllocator::new(HEAP_START, HEAP_SIZE);

// Allocate 4096 bytes (one page)
let ptr = allocator.alloc(4096);

// Free the allocation
allocator.free(ptr, 4096);
```

**API**:

| Method | Parameters | Returns | Description |
|--------|-----------|---------|-------------|
| `new(start, size)` | `usize, usize` | `BuddyAllocator` | Initialize allocator over a memory region |
| `alloc(size)` | `usize` | `*mut u8` | Allocate `size` bytes (rounded to next power-of-two) |
| `free(ptr, size)` | `*mut u8, usize` | `()` | Release allocation back to the pool |
| `available()` | - | `usize` | Returns available bytes |
| `used()` | - | `usize` | Returns used bytes |

**Complexity**: O(log n) alloc/free, O(1) coalescing

---

### Slab Allocator

**File**: `src/klib/slab.rs`
**Inspired by**: Linux SLAB/SLUB allocator, Solaris slab allocator (Jeff Bonwick, 1994)

Slab allocators cache frequently allocated objects to reduce fragmentation and
allocation latency. Each slab holds fixed-size objects of a specific type.

```rust
use crate::klib::slab::SlabAllocator;

// Create a slab for 64-byte objects
let mut slab = SlabAllocator::new(64, 128); // 128 objects per slab

let obj = slab.alloc(); // O(1) allocation
slab.free(obj);         // O(1) deallocation
```

---

### Custom Allocator

**File**: `src/klib/custom_allocator.rs`
The `SigmaAllocator` implements `GlobalAlloc` and composes the buddy + slab
allocators for the full memory subsystem.

```rust
#[global_allocator]
static ALLOCATOR: SigmaAllocator = SigmaAllocator::new();
```

---

## Collections

### Vec

**File**: `src/klib/vec.rs`
**Replaces**: `std::vec::Vec`

A growable heap-allocated array using the klib buddy allocator.

```rust
use crate::klib::vec::Vec;

let mut v: Vec<u32> = Vec::new();
v.push(42);
v.push(100);

assert_eq!(v.len(), 2);
assert_eq!(v[0], 42);

// Iteration
for item in &v {
    // process item
}

// Pop
let last = v.pop(); // Some(100)
```

**API Summary**:

| Method | Description |
|--------|-------------|
| `new()` | Create empty Vec |
| `with_capacity(n)` | Pre-allocate for n elements |
| `push(val)` | Append element |
| `pop()` | Remove and return last element |
| `len()` | Number of elements |
| `is_empty()` | True if len == 0 |
| `clear()` | Remove all elements |
| `extend(iter)` | Append from iterator |
| `retain(pred)` | Keep elements matching predicate |
| `dedup()` | Remove consecutive duplicates |
| `sort()` | In-place sort |
| `contains(val)` | Linear search |

---

### HashMap / HashSet

**File**: `src/klib/hashmap.rs`, `src/klib/hashset.rs`
**Replaces**: `std::collections::HashMap`, `std::collections::HashSet`
**Hash function**: SipHash-1-3 (custom implementation, no dependency on `std::hash`)

```rust
use crate::klib::hashmap::HashMap;

let mut map: HashMap<u64, &str> = HashMap::new();
map.insert(1, "kernel");
map.insert(2, "userspace");

if let Some(val) = map.get(&1) {
    // val == "kernel"
}

map.remove(&2);

for (key, value) in &map {
    // iterate
}
```

---

### BTreeMap

**File**: `src/klib/btreemap.rs`
**Replaces**: `std::collections::BTreeMap`

Ordered map using a B-tree. Preferred for ordered key traversal.

```rust
use crate::klib::btreemap::BTreeMap;

let mut map: BTreeMap<u32, &str> = BTreeMap::new();
map.insert(10, "ten");
map.insert(5, "five");

// Iterates in sorted key order: 5, 10
for (k, v) in &map {
    // process
}
```

---

### RingBuffer

**File**: `src/klib/ring_buffer.rs`
**Purpose**: Lock-free producer-consumer buffer for kernel I/O and interrupt handlers.

```rust
use crate::klib::ring_buffer::RingBuffer;

let mut rb: RingBuffer<u8, 512> = RingBuffer::new();
rb.push(b'A');
let byte = rb.pop(); // Some(b'A')
```

---

## String Handling

**Files**: `src/klib/string.rs`, `src/klib/custom_string.rs`, `src/klib/string_ops.rs`
**Replaces**: `std::string::String`, `&str` methods

```rust
use crate::klib::string::SigmaString;

let mut s = SigmaString::from("Hello");
s.push_str(", World!");
let len = s.len(); // 13

// String comparison
let eq = s.as_str() == "Hello, World!";

// Format (no std::fmt, uses custom formatter)
let formatted = sigma_format!("Value: {}", 42);
```

**String Operations** (`string_ops.rs`):

| Function | Description |
|----------|-------------|
| `sigma_strlen(s)` | Length of null-terminated C string |
| `sigma_strcmp(a, b)` | C-style string comparison |
| `sigma_strcpy(dst, src)` | Copy string |
| `sigma_strcat(dst, src)` | Concatenate strings |
| `sigma_strstr(hay, needle)` | Substring search |
| `sigma_atoi(s)` | Parse integer from string |

---

## Hashing & RNG

### Custom Hashing (`src/klib/hash.rs`)

SigmaOS implements SipHash-1-3 without any external dependency:

```rust
use crate::klib::hash::SigmaHasher;

let mut hasher = SigmaHasher::new_with_keys(0xDEAD_BEEF, 0xCAFE_BABE);
hasher.write(b"data");
let hash = hasher.finish(); // u64
```

### RNG (`src/klib/rng.rs`, `src/klib/random.rs`)

**Replaces**: `rand` crate
**Algorithm**: xorshift64 + hardware RDRAND instruction fallback

```rust
use crate::klib::rng::SigmaRng;

let mut rng = SigmaRng::new_seeded(0xDEAD_BEEF_CAFE_BABE);
let val: u64 = rng.next_u64();
let byte: u8 = rng.next_u8();
```

---

## UUID Generation

**File**: `src/klib/uuid.rs`
**Replaces**: `uuid` crate
**Method**: UUID v4 (random), UUID v7 (time-ordered), UUID v5 (name-based SHA-1)

```rust
use crate::klib::uuid::{Uuid, UuidVersion};

let uuid_v4 = Uuid::new_v4();           // Random
let uuid_v7 = Uuid::new_v7();           // Time-ordered (better for DBs)
let uuid_v5 = Uuid::new_v5(b"name");   // Name-based

let s = uuid_v4.to_string(); // "550e8400-e29b-41d4-a716-446655440000"
```

---

## Paging & Virtual Memory

**Files**: `src/klib/paging.rs`, `src/klib/uvm.rs`

```rust
use crate::klib::paging::{PageTable, PageFlags};

let mut pt = PageTable::new();

// Map a physical page to virtual address
pt.map(
    VirtAddr(0xFFFF_8000_0000_0000),
    PhysAddr(0x0010_0000),
    PageFlags::PRESENT | PageFlags::WRITABLE | PageFlags::NO_EXECUTE,
);

// Unmap
pt.unmap(VirtAddr(0xFFFF_8000_0000_0000));
```

---

## Math & Bit Operations

**Files**: `src/klib/math.rs`, `src/klib/math_ops.rs`

```rust
use crate::klib::math::{pow2_ceil, log2_floor, clz, ctz};

let n = pow2_ceil(100);  // 128
let l = log2_floor(128); // 7
let z = clz(1u64);       // 63
```

---

## Dependency Reduction Status

| Crate Previously Used | klib Replacement | Status |
|-----------------------|-----------------|--------|
| `rand` (RNG) | `klib::rng` | ✅ Replaced |
| `uuid` | `klib::uuid` | ✅ Replaced |
| `hashbrown` | `klib::hashmap` | ✅ Replaced |
| `std::collections` | `klib::*` | ✅ Replaced |
| `std::string` | `klib::string` | ✅ Replaced |
| `std::alloc` | `klib::custom_allocator` | ✅ Replaced |
| `std::fmt` | `klib::io` | ✅ Replaced |
| `libc` | `sigma_libc.h` (C FFI) | ✅ Replaced |
| `tokio` | `klib::async_runtime` | 🔄 In Progress |

> **Current Cargo.toml dependency status**: `uuid` and `rand` crates are listed in
> `Cargo.toml` but are **not used** in core kernel code — only in test stubs.
> The roadmap is to remove these entirely once `klib::uuid` and `klib::rng` are
> fully integrated across all modules.
