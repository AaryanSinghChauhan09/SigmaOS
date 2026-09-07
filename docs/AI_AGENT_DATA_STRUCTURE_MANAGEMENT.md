# AI Agent Guidelines for SigmaOS Data Structure Management

This document provides guidelines, architectural specifications, and verification protocols for AI agents developing, inspecting, or extending **SigmaOS `#![no_std]` Zero-Dependency Core Data Structures**.

---

## 1. System Architecture & Data Structure Overview

SigmaOS implements a zero-dependency kernel library (`klib`) in `src/klib/` and `src/kernel/unix_primitives.rs` providing kernel-safe, allocation-efficient data structures under `#![no_std]`:

| Data Structure | Primary Source File | Core Type / Struct | Operational Complexity & Characteristics |
| :--- | :--- | :--- | :--- |
| **Growable Vector** | `src/klib/vec.rs` | `Vec<T>` | Dynamic array with exponential reallocation ($1.5 \times$ or $2 \times$). |
| **B-Tree Map** | `src/klib/btreemap.rs`, `src/klib/hashmap.rs` | `BTreeMap<K, V>` | $O(\log N)$ sorted key-value store suitable for kernel range queries. |
| **Open-Addressing Hash Map** | `src/klib/hashmap.rs` | `HashMap<K, V>` | $O(1)$ average lookup via FNV-1a or DJB2 hashing. |
| **Hash Set** | `src/klib/hashset.rs` | `HashSet<T>` | Set abstraction built on open-addressing hash table. |
| **Doubly-Linked List** | `src/klib/linked_list.rs` | `LinkedList<T>`, `ListNode<T>` | Intrusive or heap-backed $O(1)$ push/pop at both head and tail. |
| **Singular Stack List** | `src/klib/linked_list.rs` | `SList<T>` | $O(1)$ LIFO singly-linked node stack for kernel free-lists. |
| **Static Ring Buffer** | `src/klib/ringbuf.rs` | `RingBuf<T, N>` | Fixed-capacity $O(1)$ FIFO queue with const generic parameter `N`. |
| **Atomic MPSC Ring Buffer** | `src/klib/ringbuf.rs` | `MpscRingBuf<T, N>` | Multi-Producer Single-Consumer lock-free ring buffer. |
| **Intrusive Red-Black Tree** | `src/kernel/unix_primitives.rs` | `RbTree<K, V>`, `RbNode<K, V>` | Arena-backed self-balancing RB-Tree for CFS task scheduling and timers. |
| **Sparse Key-Value Map** | `src/kernel/unix_primitives.rs` | `SparseMap<T>` | XArray / Radix-tree inspired sparse array keyed by `u64`. |

---

## 2. Data Structure Mechanics & Usage Patterns

AI agents modifying or utilizing kernel data structures must follow these core patterns:

### 1. Fixed-Capacity Const Generic Ring Buffers (`RingBuf`)
Used in I/O queues and interrupt handlers where heap allocations are forbidden:

```rust
use sigma::klib::ringbuf::RingBuf;

// Initialize 64-element fixed-capacity ring buffer
let mut buf = RingBuf::<u8, 64>::new();
buf.push(0xAA).unwrap();
let item = buf.pop();
assert_eq!(item, Some(0xAA));
```

### 2. Doubly-Linked Lists & Singly-Linked Stacks (`LinkedList`, `SList`)
Singly-linked stack lists (`SList`) are preferred for lock-free node pools and allocator freelists:

```rust
use sigma::klib::linked_list::{LinkedList, SList};

let mut list = LinkedList::new();
list.push_back(100);
list.push_front(50);

let mut stack = SList::new();
stack.push(1);
let top = stack.pop();
assert_eq!(top, Some(1));
```

### 3. Arena Red-Black Trees (`RbTree`)
Self-balancing RB-Trees keyed by `K: Ord` for vruntime ordering in schedulers:

```rust
use sigma::kernel::unix_primitives::RbTree;

let mut tree = RbTree::new();
tree.insert(50u64, "task_a");
tree.insert(20u64, "task_b"); // Leftmost / minimum key

let (min_key, min_val) = tree.min().unwrap();
assert_eq!(*min_key, 20);
```

---

## 3. Testing & Verification Protocol for AI Agents

When making changes to `klib` data structures, AI agents must execute the following validation steps:

### 1. Standalone Module Test Execution
Run standalone rustc test suites for `klib` collections and primitives:

```bash
rustc --test --edition=2021 src/klib/vec.rs -o build/test_vec && ./build/test_vec
rustc --test --edition=2021 src/klib/linked_list.rs -o build/test_ll && ./build/test_ll
rustc --test --edition=2021 src/klib/ringbuf.rs -o build/test_rb && ./build/test_rb
```

### 2. Full System Integration & Inspection Suite
Run the master test script to validate all C++ test runners, inspection test binaries, Python test suites, and core `klib` data structure consumers:

```bash
./run_sigma_tests.sh
```

---

## 4. Coding Standards & Memory Directives

- **`#![no_std]` Purity:** Code in `src/klib/` must never depend on `std`. Use `core::` or `alloc::` exclusively.
- **Panic Safety & Bounds Checks:** Array indexing and ring buffer wrap-around calculations must prevent integer overflow (`wrapping_add`).
- **Verification Rule:** Always confirm file creation/edits with `read_file` before completing steps.
