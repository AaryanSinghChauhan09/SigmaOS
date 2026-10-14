# ADR 001: SigmaOS Unified I/O Lifecycle Subsystem Architecture

## Status
Accepted (Phase 9 Core Kernel & Storage Parity)

## Context
SigmaOS integrates high-performance asynchronous I/O (`io_uring_v2`), block buffer caching (`src/fs/buffer.rs`), DMA management (`src/io/dma.rs`), interrupt management (`src/io/interrupt.rs`), and I/O scheduling (`src/io/scheduler.rs`). Previously, I/O components were tested in model isolation, leading to potential gaps between model tests and real hardware/firmware execution.

## Decision
Establish an authoritative, unified I/O management subsystem adhering strictly to the 11-stage lifecycle:

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Key Architectural Invariants
1. **Strict Non-Blocking Core**: No spinlock or blocking operation may occur in interrupt context. Memory allocation during IRQ handling is prohibited; pre-allocated pools must be utilized.
2. **Exactly-Once Completion**: Every request submitted to the subsystem must terminate in either `complete` or `error/recovery` exactly once. Double-completion or abandoned request descriptors are treated as critical kernel panics or typed errors.
3. **Bounded Queues & Resource Observability**: Every ring, queue, and table (Interrupt Table, DMA Queue, Scheduler Queue, Callback Queue, Buffer Set) must define explicit upper capacity bounds and measure OOM states.
4. **POSIX & Modern Async API Stability**: Maintain backwards compatibility with standard POSIX synchronous file/socket descriptor calls while supporting zero-copy `io_uring_v2` submission/completion rings.

## Consequences
- All I/O components must validate their state transitions against the authoritative lifecycle.
- CI and test harnesses must enforce `IO_RULES.toml` rules (`io_completeness`, `no_leak`, `oom_metric`, `interrupt_integrity`, `buffer_semantics`).
