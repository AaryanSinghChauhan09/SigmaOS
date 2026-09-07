# AI Agent Abstract Data Type (ADT) Operation Management Architecture in SigmaOS

This document specifies Abstract Data Type (ADT) abstractions, lock-free queue and circular buffer implementations, ring buffers, arena allocators, priority queues, and memory safety directives for AI agents working on kernel data structures in SigmaOS (`src/klib/ring_buffer.rs`, `src/klib/queue.rs`, `src/klib/arena.rs`).

---

## 🏗️ 1. Sovereign ADT Architecture & Data Structure Spectrum

SigmaOS provides zero-dependency, `#![no_std]` compliant Abstract Data Types (ADTs) optimized for microsecond latency, zero allocation overhead, and lock-free thread safety in kernel and userspace environments:

```
+---------------------------------------------------------------------------------+
| SigmaOS Core ADT Spectrum (`src/klib/`)                                         |
+---------------------------------------------------------------------------------+
     |                       |                       |                      |
     v                       v                       v                      v
+------------------+  +-------------------+  +-------------------+  +-------------------+
| Ring Buffer ADT  |  | Atomic Queue ADT  |  | Arena Allocator   |  | Priority Queue    |
| Lock-free SPSC / |  | Double-ended /    |  | Bump allocation   |  | Min/Max Heap for  |
| MPMC ring buffer |  | lock-free queue   |  | with reset scope  |  | RT task scheduling|
+------------------+  +-------------------+  +-------------------+  +-------------------+
```

---

## ⚙️ 2. Key ADT Categories & Implementations

### 1. Lock-Free Single-Producer Single-Consumer (SPSC) Ring Buffer (`RingBuffer<T, N>`)
- **Operations**: `push()`, `pop()`, `is_empty()`, `is_full()`, `capacity()`.
- **Implementation Principles**: Uses atomic head and tail pointers with power-of-two mask operations (`index & (CAPACITY - 1)`) to eliminate costly modulo division in interrupt contexts.

### 2. Atomic Lock-Free Queue (`AtomicQueue<T, N>`)
- **Operations**: `enqueue()`, `dequeue()`, `peek()`, `len()`.
- **Implementation Principles**: Utilizes `AtomicUsize` head/tail counters with Compare-And-Swap (CAS) loops to ensure thread-safe concurrent access without acquiring spinlocks.

### 3. Fixed-Capacity Bump Arena Allocator (`ArenaAllocator<N>`)
- **Operations**: `alloc<T>()`, `alloc_slice<T>()`, `reset()`, `allocated_bytes()`.
- **Implementation Principles**: Sequential bump allocation within static backing buffers, providing O(1) allocation time and zero fragmentation for short-lived operational frames.

### 4. Binary Priority Queue / Heap (`PriorityQueue<T, P, N>`)
- **Operations**: `push(item, priority)`, `pop()`, `peek()`.
- **Implementation Principles**: Maintains heap invariants using max-heap or min-heap order for real-time task priority scheduling and network packet queueing.

---

## 🛡️ 3. Rules & Directives for AI Agents

1. **Zero-Dependency & `#![no_std]` Compliance**
   - All ADT modules must remain `#![no_std]` compatible and refrain from importing external third-party crates or requiring heap allocations unless using `alloc::vec::Vec` or `alloc::boxed::Box`.
2. **Lock-Free Concurrency & Memory Ordering**
   - Ensure atomic operations specify correct memory orderings (`Acquire`, `Release`, `AcqRel`, or `SeqCst`) when reading and updating head/tail indices in ring buffers and queues.
3. **Power-of-Two Ring Capacity Gating**
   - Always enforce or statically verify that static ring buffer capacities (`N`) are powers of two to enable bitwise mask indexing (`n & (N - 1)`).
4. **Boundary & Overflow Protection**
   - Validate array indices and capacity limits before performing unchecked memory reads or writes to prevent buffer overruns.

---

## ⚙️ 4. Verification & Testing Instructions

- **Compile and Run ADT Unit Tests:**
  `rustc --test --edition=2021 src/klib/ring_buffer.rs -o build/test_ring_buffer && ./build/test_ring_buffer`
- **Full SigmaOS Test Pipeline:**
  `./run_sigma_tests.sh`
