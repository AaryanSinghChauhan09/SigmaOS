# AI Agent Bounded Buffer Monitor Management in SigmaOS

## Overview
SigmaOS incorporates a lock-free, zero-allocation Bounded Buffer & Ring Buffer Subsystem managed by autonomous AI Agents (**Bolt** ⚡, **Sentinel** 🛡️, **Palette** 🎨). This document defines operational directives, memory ordering semantics, concurrency rules, and monitoring interfaces for AI agents supervising bounded queues and circular buffers.

AI agents interact directly with `src/klib/ring_buffer.rs` (`RingBuffer<T, CAP>`, `HeapRingBuffer<T>`) and `src/process/sovereign_process_engine.rs` (Zero-Copy IPC Ring Channels).

---

## 1. Bounded Buffer Architecture & Concurrency Control

### 1.1 Single-Producer Single-Consumer (SPSC) Lock-Free Ring Buffer
Implemented in `src/klib/ring_buffer.rs` (`RingBuffer<T, CAP>`):
* **Power-of-Two Capacity Requirement**: Buffer capacity (`CAP`) must be a power of two to replace expensive integer division (`%`) with bitwise masking (`& (CAP - 1)`).
* **Atomic Memory Ordering**:
  - **Write Index (`write`)**: Loaded with `Ordering::Relaxed`, stored with `Ordering::Release`.
  - **Read Index (`read`)**: Loaded with `Ordering::Relaxed`, stored with `Ordering::Release`.
  - Length check (`len()`) uses `Ordering::Acquire` loads on both atomic indices.

### 1.2 Heap-Allocated Variable Bounded Buffer (`HeapRingBuffer<T>`)
Implemented in `src/klib/ring_buffer.rs`. Dynamically allocates circular buffer memory using `std::alloc::alloc` for large I/O payloads while maintaining power-of-two bitmask indexing.

### 1.3 Zero-Copy IPC Bounded Channel
Implemented in `src/process/sovereign_process_engine.rs`. Enforces process communication bounds (`capacity_bytes`) to prevent IPC memory exhaustion.

---

## 2. AI Agent Operational Directives & Monitoring Rules

### 2.1 Buffer Overflow & Eviction Handling
1. **Push Rejection on Full Buffer**:
   When `len() == CAP`, `RingBuffer::push(item)` returns `Err(item)`. AI agents must inspect overflow counters and handle push rejections cleanly without dropping data.
2. **Atomic Index Wrapping Safety**:
   Atomic read/write indices use wrapping arithmetic (`wrapping_add(1)`, `wrapping_sub(r)`). Agents must verify index calculations use unsigned integer wrapping logic.

### 2.2 Performance & Micro-Optimization Protocol
* **Bolt ⚡ Optimization**:
  On hot execution paths, **Bolt** ⚡ verifies that ring buffer capacities are fixed constants (`CAP`), allowing the Rust compiler to emit single-cycle `AND` instructions for array indexing (`idx = w & (CAP - 1)`).

---

## 3. Sample Agent Commands & CLI Interactions

```bash
# Query ring buffer fill level and capacity metrics
sigma-buffer status --channel ipc_main

# Inspect SPSC ring buffer atomic read/write pointer offsets
sigma-buffer inspect-ptrs --buf-id 0x4000

# Benchmark ring buffer throughput under SPSC concurrency
sigma-buffer bench --cap 1024 --iters 1000000
```
