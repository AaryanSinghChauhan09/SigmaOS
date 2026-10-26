# SigmaOS I/O Management Architecture Decision Record (ADR)

## Status
Accepted

## Context
SigmaOS requires a unified, safe, and observable subsystem for device I/O, interrupts, DMA, buffering, and scheduling. Code presence alone is insufficient for hardware stability; I/O operations must adhere to strict correctness, concurrency, and memory safety invariants.

## Decision
1. **Authoritative 10-Stage Lifecycle**: Standardize all I/O handling on the 10-stage contract: `submit -> validate -> enqueue -> execute -> interrupt/DMA -> complete -> callback -> reclaim -> free -> error/recovery`.
2. **Bounded Structures**: All queues, interrupt tables, DMA descriptor rings, and buffer caches MUST have configurable maximum limits (`max_queue_depth`, `max_handlers`, etc.) and return typed exhaustion errors upon saturation.
3. **Double-Operation Guards**: Explicitly track active descriptors and mapped buffer IDs to prevent double-completion, double-handler registration, and double-DMA mappings.
4. **Race-Free Interrupt and DMA Dispatch**: Implement lock ordering and atomic invocation tracking across hard IRQ top-halves and deferred bottom-halves.

## Consequences
- Prevents resource leaks, lost updates, and memory corruption during high-throughput stress or fault conditions.
- Facilitates integration with `io_uring`, VirtIO block drivers, and NVMe controllers.
