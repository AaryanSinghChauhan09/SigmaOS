# AI Agent Circular Buffer Management Architecture (`docs/AGENTS_CIRCULAR_BUFFER.md`)

This guide details the architectural design, lock-free queue abstractions, and AI agent monitoring protocols for circular buffer management in SigmaOS.

---

## 1. Subsystem Architecture

SigmaOS implements lock-free circular buffers across kernel, network, and memory subsystems:

### A. Lock-Free Core Ring Buffers (`RingBuf`, `RingBuffer`)
- Located in `src/klib/ringbuf.rs` and `src/klib/ring_buffer.rs`.
- Implements SPSC (Single-Producer Single-Consumer) and MPSC (Multi-Producer Single-Consumer) lock-free atomic circular queues.
- Uses atomic head and tail pointers with power-of-two index masking (`ptr & (CAP - 1)`).

### B. Network & IPC Packet Ring Buffers
- Located in `src/network/ring_buffer_stack.rs` and `src/kernel/ipc.rs`.
- Manages high-speed packet ingestion and inter-process message passing queues with bounded capacity checks.

### C. DMA & Driver Ring Buffers
- Located in `src/kernel/memory/resource_allocator.rs` and driver suites.
- Manages DMA ring buffer allocation for hardware devices and audio/video frame streaming pipelines.

---

## 2. AI Agent Operational Directives

1. **Capacity Validation:** Ensure static and const-generic circular buffer capacities enforce power-of-two bounds checks.
2. **Concurrency Audit:** Verify atomic load/store operations on head and tail variables use correct `Acquire`/`Release` memory orderings.
3. **Automated Verification:** Execute `./run_sigma_tests.sh` to confirm ring buffer unit tests pass.
