# SigmaOS AI Agent Circular Buffer Management Directive (`AGENTS_CIRCULAR_BUFFER.md`)

This document specifies technical directives, lock-free synchronization rules, and operational guidelines for AI agents managing circular buffer (ring buffer) structures in SigmaOS.

---

## 1. Core Principles for Circular Buffer Management

Circular ring buffers (`RingBuf`, `RingBuffer`) are heavily utilized across high-throughput IPC, kernel trace logging, network packet processing, and DMA drivers in SigmaOS. AI agents modifying or creating circular buffers must observe the following rules:

1. **Power-of-Two Capacity Alignment:**
   - Circular buffer capacities (`CAP`, `N`) must strictly be powers of two (e.g., 64, 1024, 4096).
   - Use bitwise masking (`head & (CAP - 1)`) instead of modulo operations (`head % CAP`) for index wrapping to optimize performance and prevent division instructions in hot paths.

2. **Lock-Free Atomic Head & Tail Ordering:**
   - Single-Producer Single-Consumer (SPSC) and Multi-Producer Single-Consumer (MPSC) circular queues must update head and tail indices using atomic variables (`AtomicUsize`).
   - Push operations must store items before advancing tail pointers with acquire-release semantics (`Ordering::Release`).
   - Pop operations must load items before advancing head pointers with acquire-release semantics (`Ordering::Acquire`).

3. **Overflow & Underflow Guardrails:**
   - Enqueue operations must explicitly check for full capacity conditions (`tail - head == CAP`) and handle drop/overwrite or backpressure policies gracefully.
   - Dequeue operations must explicitly check for empty buffer conditions (`head == tail`) before attempting slot read or removal.

4. **Zero-Dependency `#![no_std]` Compatibility:**
   - Use native `klib::ringbuf` or `klib::ring_buffer` structures avoiding external dependencies.

---

## 2. Pre-Commit Circular Buffer Verification Checklist

Before submitting code modifications, AI agents must verify:
- [ ] Circular buffer capacity parameters enforce power-of-two constraints via const assertions.
- [ ] Atomic memory orderings on head/tail pointers prevent race conditions or stale reads across CPU cores.
- [ ] Overflow conditions handle drops or error propagation safely without panics.
- [ ] `./run_sigma_tests.sh` executes with 100% test pass rate.
