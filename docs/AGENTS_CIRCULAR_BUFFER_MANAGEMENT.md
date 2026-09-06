# SigmaOS Circular Buffer, Lock-Free Ring Buffer & Zero-Copy Streaming Guide for AI Agents

This guide provides technical specifications, atomic pointer invariants, lock-free ring buffer concurrency semantics, wrap-around index calculations, and overflow handling rules for AI agents managing circular buffers in SigmaOS.

---

## 1. Zero-Dependency `#![no_std]` Circular Buffer Architecture

SigmaOS utilizes lock-free circular buffers and ring buffer structures across kernel, driver, media, and networking subsystems (`src/klib/ring_buffer.rs`, `src/klib/ringbuf.rs`, `src/media/sovereign_video_player.rs`):

* **Lock-Free Zero-Copy Frame Ring Buffers (`VlcLightweightMediaPipeline` in `src/media/sovereign_video_player.rs`):**
  Single-producer single-consumer (SPSC) lock-free ring buffer for real-time video frame rendering and audio stream processing without locks or heap allocation during playback.
* **Kernel & Subsystem Ring Buffers (`src/klib/ring_buffer.rs` & `src/klib/ringbuf.rs`):**
  Provides bounded lock-free character queues, I/O ring buffers, and event notification streams with power-of-two capacity optimization.

---

## 2. Concurrency & Wrap-Around Index Calculation Rules

When implementing or modifying circular buffers:

1. **Power-of-Two Capacity Rule:**
   Ring buffer capacities SHOULD be powers of two ($N = 2^k$) to enable bitwise modulo wrap-around indexing via `idx & (capacity - 1)` instead of modulo division `% capacity`.
2. **Atomic Head & Tail Pointer Invariants:**
   * `head` (producer write offset) and `tail` (consumer read offset) MUST be modified using atomic operations (`AtomicUsize` with `Ordering::Acquire` / `Ordering::Release`).
   * A empty ring buffer condition is indicated when `head == tail`.
   * A full ring buffer condition is indicated when `(head + 1) & (capacity - 1) == tail`.
3. **Overwrite vs. Drop Policy:**
   * In media frame pipelines (`VlcLightweightMediaPipeline`), if the frame ring buffer is full, the producer MUST overwrite the oldest unread frame or drop late frames gracefully to maintain real-time playback synchronization.
   * In reliable I/O queues, push operations MUST return an overflow error (`BufferFull`) rather than silently discarding unread data.

---

## 3. Checklist for AI Agents Managing Circular Buffers

1. **Verify Power-of-Two Capacity:** Ensure ring buffer sizes are bitwise masked for $O(1)$ index wrapping.
2. **Test Concurrency & Wrap-Around Logic:**
   Run ring buffer unit tests:
   ```bash
   cargo test --lib -- klib::ring_buffer::tests
   cargo test --lib -- klib::ringbuf::tests
   ./run_sigma_tests.sh
   ```
