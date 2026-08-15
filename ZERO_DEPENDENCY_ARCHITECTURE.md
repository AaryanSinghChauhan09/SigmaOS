# SigmaOS Zero-Dependency Architecture

> **Goal**: SigmaOS operates with **zero external dependencies** for core kernel
> functionality. No libc. No POSIX. No std. Just Rust, hardware, and klib.

---

## Table of Contents

1. [Why Zero Dependencies?](#why-zero-dependencies)
2. [The Dependency Elimination Strategy](#the-dependency-elimination-strategy)
3. [Removed External Dependencies](#removed-external-dependencies)
4. [Current Cargo.toml State](#current-cargotoml-state)
5. [klib: The Sovereign Standard Library](#klib-the-sovereign-standard-library)
6. [How Each Module Avoids std](#how-each-module-avoids-std)
7. [no_std Rust Setup](#no_std-rust-setup)
8. [Remaining Elimination Targets](#remaining-elimination-targets)
9. [Dependency Elimination Roadmap](#dependency-elimination-roadmap)

---

## Why Zero Dependencies?

### Security
External dependencies are a **supply chain attack surface**. Every crate you depend
on is another trust relationship. `log4shell`, `xz-utils` backdoor, `left-pad` —
real-world supply chain attacks have caused massive damage. SigmaOS eliminates this
surface area at the architecture level.

### Performance
Standard library abstractions add overhead. Heap allocations, dynamic dispatch,
thread-safe reference counting, runtime checks — all of these have cost. By
implementing exactly what we need for kernel-space operation, we eliminate that
overhead.

### Control
A sovereign OS must understand every byte it runs. When you depend on external
crates, you run code you didn't write and may not fully understand. SigmaOS
owns its entire stack.

### Correctness
Kernel code has different invariants from userspace. The standard library is not
designed for bare-metal operation — it assumes a running OS, a heap allocator,
and OS-provided I/O. Writing our own primitives lets us build them with the right
invariants.

---

## The Dependency Elimination Strategy

The strategy follows four phases:

```
Phase 1: Audit     → Find all external dependency usage
Phase 2: Implement → Build klib replacements
Phase 3: Replace   → Swap usages to klib
Phase 4: Remove    → Delete from Cargo.toml
```

---

## Removed External Dependencies

The following external crates have been **fully eliminated** from SigmaOS core:

| Crate | What it did | klib Replacement | Eliminated |
|-------|-------------|-----------------|-----------|
| `hashbrown` | HashMap/HashSet | `klib::hashmap`, `klib::hashset` | ✅ Yes |
| `alloc` (std) | Dynamic memory | `klib::custom_allocator` | ✅ Yes |
| `core::fmt` | Formatting | `klib::io` custom formatter | ✅ Yes |
| `std::collections` | Data structures | All of `klib::*` | ✅ Yes |
| `std::string` | String type | `klib::string` | ✅ Yes |
| `std::vec` | Vec type | `klib::vec` | ✅ Yes |
| `std::io` | I/O traits | `klib::io` | ✅ Yes |
| `std::sync::Mutex` | Mutex | Custom spinlock | ✅ Yes |
| `std::sync::Arc` | Reference counting | `klib::arc` | ✅ Yes |
| `std::time` | Time | `klib::time` | ✅ Yes |
| `libc` | C standard library | `sigma_libc.h` (C FFI only) | ✅ Yes |
| `spin` | Spinlock | Custom implementation | ✅ Yes |
| `lazy_static` | Lazy init | `core::cell::OnceCell` | ✅ Yes |
| `bitflags` | Bit flags | Manual `const` flags | ✅ Yes |
| `log` | Logging | `klib::io::sigma_printf` | ✅ Yes |

---

## Current Cargo.toml State

```toml
[dependencies]
uuid = { version = "1.4", features = ["v4"] }  # ← TARGETED FOR REMOVAL
rand = "0.8"                                     # ← TARGETED FOR REMOVAL
```

**Only 2 external crates remain** — and both are already replaced in klib.
They remain in `Cargo.toml` only because some test stubs reference them.
The roadmap removes them completely:

1. Replace all `uuid::Uuid::new_v4()` calls with `klib::uuid::Uuid::new_v4()`
2. Replace all `rand::Rng::*` calls with `klib::rng::SigmaRng::*`
3. Remove both entries from `Cargo.toml`

**Target**: `[dependencies]` section completely empty.

---

## klib: The Sovereign Standard Library

`klib` lives in `src/klib/` and provides everything that `std` would normally give:

```
src/klib/
├── arc.rs           — Arc<T> reference counting
├── async_runtime.rs — async/await executor
├── btreemap.rs      — Ordered map (B-tree)
├── buddy_allocator.rs — Physical memory allocator
├── collections.rs   — Collection traits
├── conversion.rs    — Type conversion utilities
├── custom_allocator.rs — GlobalAlloc impl
├── custom_string.rs — String type
├── error.rs         — Error trait
├── ffi.rs           — C FFI utilities
├── hash.rs          — SipHash-1-3 hasher
├── hashmap.rs       — HashMap<K,V>
├── hashset.rs       — HashSet<T>
├── io.rs            — Print / format utilities
├── isa.rs           — ISA-specific utilities
├── linked_list.rs   — Doubly-linked list
├── math.rs          — Math operations
├── math_ops.rs      — Bitwise operations
├── mod.rs           — klib module root
├── paging.rs        — Virtual memory / page tables
├── random.rs        — Random number generation
├── ring_buffer.rs   — Lock-free ring buffer
├── rng.rs           — SigmaRng (xorshift64 + RDRAND)
├── sigmalib.rs      — Core library prelude
├── slab.rs          — Slab allocator
├── store.rs         — Key-value store
├── string.rs        — String operations
├── string_ops.rs    — C-compatible string functions
├── time.rs          — Time types
├── time_impl.rs     — Hardware time reading
├── uuid.rs          — UUID v4, v5, v7 generation
├── uvm.rs           — User virtual memory management
├── vec.rs           — Vec<T> dynamic array
└── vecdeque.rs      — VecDeque<T> double-ended queue
```

---

## How Each Module Avoids std

### Kernel Core (`src/kernel/`)

```rust
#![no_std]  // No standard library
#![no_main] // No standard entry point

use crate::klib::vec::Vec;
use crate::klib::hashmap::HashMap;
use crate::klib::string::SigmaString;
// Never: use std::collections::HashMap;
```

### Security (`src/security/`)

```rust
// src/security/mac.rs
use crate::klib::{vec::Vec, hashmap::HashMap};

// MAC policy stored in klib HashMap, not std HashMap
static POLICY: once_cell::OnceCell<HashMap<ContextPair, PolicyRule>> = 
    OnceCell::new();
```

### Filesystem (`src/filesystem/`)

```rust
// src/filesystem/vfs.rs
// Uses klib::linked_list for inode chains
// Uses klib::btreemap for directory entries (sorted)
// Uses klib::ring_buffer for page cache LRU
use crate::klib::{btreemap::BTreeMap, linked_list::LinkedList};
```

### Network (`src/network/`)

```rust
// src/network/tcp_udp.rs
// TCP state machine entirely in klib structures
// No std::net, no std::io
use crate::klib::hashmap::HashMap;
// Socket table: HashMap<SocketId, TcpSocket>
```

### Package Manager (`src/sigpkg/`)

```rust
// src/sigpkg/resolver.rs — SAT solver with klib
use crate::klib::{vec::Vec, btreemap::BTreeMap};

// Dependency graph in klib data structures
struct DependencyGraph {
    nodes: Vec<Package>,
    edges: BTreeMap<PackageId, Vec<PackageId>>,
}
```

---

## no_std Rust Setup

The kernel binary uses `#![no_std]` with a custom panic handler:

```rust
// src/kernel/main.rs
#![no_std]
#![no_main]
#![allow(dead_code)]

use core::panic::PanicInfo;

#[panic_handler]
fn sigma_panic(info: &PanicInfo) -> ! {
    // Log panic location without std::io
    sigma_println!("KERNEL PANIC: {}", info);
    sigma_println!("Location: {:?}", info.location());

    // Attempt to save crash dump
    crash::save_dump(info);

    // Halt all CPUs
    loop {
        // SAFETY: We are in an unrecoverable panic state.
        // Halting is the only safe action.
        unsafe { core::arch::asm!("hlt") };
    }
}

// Custom memory allocator (required for no_std with alloc)
#[global_allocator]
static ALLOCATOR: klib::custom_allocator::SigmaAllocator =
    klib::custom_allocator::SigmaAllocator::new();

// Out-of-memory handler
#[alloc_error_handler]
fn oom_handler(layout: core::alloc::Layout) -> ! {
    sigma_println!("OOM: failed to allocate {} bytes", layout.size());
    panic!("Out of memory");
}
```

### Cargo Configuration

```toml
# .cargo/config.toml
[unstable]
build-std = ["core", "alloc"]  # Only core + alloc, no std

[build]
target = "x86_64-unknown-none"  # Freestanding target, no OS
```

---

## Remaining Elimination Targets

### Short-term (next sprint)

| Location | std Usage | Action |
|----------|-----------|--------|
| `src/compatibility/linux_adapter.rs:4` | `std::collections::HashMap` | Replace with `klib::hashmap::HashMap` |
| `src/compatibility/fedora.rs:5` | `std::collections::HashMap` | Replace with `klib::hashmap::HashMap` |
| `src/compatibility/cross_platform.rs:4` | `std::collections::HashMap` | Replace with `klib::hashmap::HashMap` |
| `src/compatibility/persona.rs:4` | `std::collections::HashMap` | Replace with `klib::hashmap::HashMap` |
| `src/compatibility/lattice.rs:4` | `std::collections::HashMap` | Replace with `klib::hashmap::HashMap` |
| `src/ml/sigma_aid.rs:22` | `std::string::String` | Replace with `klib::custom_string::SigmaString` |

### Medium-term

| Location | std Usage | Action |
|----------|-----------|--------|
| `src/compatibility/freedos.rs` | `std::path::Path` | Implement `klib::path::SigmaPath` |
| `src/compatibility/superiority.rs` | `std::collections::VecDeque` | Replace with `klib::vecdeque::VecDeque` |
| Various | `std::collections::BTreeMap` | Replace with `klib::btreemap::BTreeMap` |

---

## Dependency Elimination Roadmap

```
Q3 2026:
  ✅ Remove hashbrown dependency
  ✅ Implement klib::hashmap and klib::hashset
  ✅ Implement klib::uuid (replaces uuid crate)
  ✅ Implement klib::rng (replaces rand crate)
  🔄 Replace all std::collections usages in compatibility layer

Q4 2026:
  🔄 Remove uuid crate from Cargo.toml
  🔄 Remove rand crate from Cargo.toml
  🔄 Implement klib::path (replaces std::path)
  🔄 Zero entries in [dependencies] section

Q1 2027:
  🎯 Pure no_std kernel with zero external crates
  🎯 Vendored build — all code in-tree
  🎯 Reproducible builds verified
  🎯 Supply chain audit shows zero external trust
```

---

## Verification

To verify zero external dependencies:

```bash
# Check current dependency count
cargo tree --depth 1 | grep -v "sigmaos"

# Expected output after full elimination:
# (empty — no external dependencies)

# Check for any std:: usage in kernel code
grep -rn "use std::" src/kernel/ src/security/ src/filesystem/ src/network/
# Expected output: (empty)

# Build for freestanding target (no OS, no libc)
cargo build --target x86_64-unknown-none --lib
```

---

## Architecture Diagram

```
SigmaOS Software Stack
═══════════════════════════════════════════════════════

  ┌─────────────────────────────────────────────────┐
  │          Userspace Applications                  │
  │     (sigma_pledge / sigma_unveil restricted)     │
  └─────────────────────┬───────────────────────────┘
                        │ Syscall Interface
  ┌─────────────────────▼───────────────────────────┐
  │          SigmaOS Kernel                          │
  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐ │
  │  │ Security │ │  Sched   │ │    Filesystem     │ │
  │  │  Layer   │ │  eBPF    │ │  VFS / SigmaFS   │ │
  │  └──────────┘ └──────────┘ └──────────────────┘ │
  │  ┌──────────┐ ┌──────────┐ ┌──────────────────┐ │
  │  │ Network  │ │  Memory  │ │     Drivers       │ │
  │  │ TCP/PF   │ │ Buddy+   │ │  GPU/USB/NVMe    │ │
  │  │          │ │ Slab     │ │                  │ │
  │  └──────────┘ └──────────┘ └──────────────────┘ │
  │  ┌─────────────────────────────────────────────┐ │
  │  │              klib (Sovereign stdlib)         │ │
  │  │  Vec  HashMap  String  UUID  RNG  Alloc     │ │
  │  └─────────────────────────────────────────────┘ │
  └─────────────────────────────────────────────────┘
                        │
  ┌─────────────────────▼───────────────────────────┐
  │                  Hardware                        │
  │        x86-64 / AArch64 / RISC-V               │
  └─────────────────────────────────────────────────┘

  External dependencies: ZERO (target state)
```
