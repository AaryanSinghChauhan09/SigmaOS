# Architectural Decision Record: SigmaOS Unified I/O Management Subsystem

## Status
Accepted

## Context
SigmaOS requires a high-performance, robust, and maintainable I/O subsystem capable of handling device I/O, interrupts, DMA, buffering, and scheduling across diverse workloads. Previously, I/O logic was distributed across experimental components (`io_uring`, `eventfd`, `fd_table`, `ioctl`). To ensure zero memory leaks, race-free operation, and strict error recovery, a authoritative I/O lifecycle contract must be defined and enforced across the codebase.

## Decision
We establish the authoritative 10-stage I/O lifecycle:
`submit -> validate -> enqueue -> execute -> interrupt/DMA -> complete -> callback -> reclaim -> free -> error/recovery`

To support this lifecycle, we define seven dedicated policy modules and native Rust management engines:
1. **Interrupt Manager (`src/io/interrupt.rs`, `src/io/handler.rs`):** Race-free IRQ handler routing, double-handler guards, lost-interrupt detection.
2. **DMA Manager (`src/io/dma.rs`, `src/io/dma_queue.rs`):** Physical memory scatter-gather mapping, cache coherence, double-DMA mapping prevention.
3. **Block Buffer Cache (`src/fs/buffer.rs`):** Multi-device block cache, LRU eviction, dirty page flushing, lock-safety.
4. **I/O Scheduler (`src/io/scheduler.rs`, `src/io/scheduler_queue.rs`):** Bounded multi-queue scheduling (RealTime, BestEffort, Idle) and stage transition tracking.
5. **Completion Callback Manager (`src/io/callback.rs`):** Registration and single-invocation completion callback handles.
6. **Component Rules (`IO_RULES.toml`):** Automated governance rules for I/O safety, completeness, OOM metrics, and boot evidence.
7. **PR Validation Standard (`.github/PULL_REQUEST_TEMPLATE.md`):** Mandatory checklist for all I/O, storage, and driver pull requests.

## Consequences
- **Safety:** All I/O queues and buffer sets have bounded limits. Out-of-memory or capacity exhaustion conditions return typed error codes (`QueueFull`, `PoolExhausted`, `TableFull`) without panic or corruption.
- **Observability:** Every stage transition in the 10-stage pipeline is inspectable and audited via tests and diagnostics.
- **Maintainability:** Clear separation between scheduling, DMA mapping, interrupt routing, and buffer management simplifies subsystem evolution and hardware driver additions.
