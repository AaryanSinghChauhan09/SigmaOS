# SigmaOS I/O Management Architecture Decision Record (ADR)

## Context

Operating system I/O subsystems are traditionally vulnerable to race conditions, lost completions, unbounded memory allocations, double-completions, and unrecoverable hardware stalls. Existing SigmaOS components (`io_uring`, `eventfd`, `fd_table`, `ioctl`, `pipe`, `socket`, `swap`, `paging`) require a unified, authoritative lifecycle contract to ensure safe, observable, and recoverable execution across real hardware and virtualized environments (QEMU, VirtIO).

## Decision

Adopt the authoritative 10-stage I/O lifecycle state machine:
`submit -> validate -> enqueue -> execute -> interrupt/DMA -> complete -> callback -> reclaim -> free -> error/recovery`

1. **Deterministic Bounded Queues**: Every subsystem queue (Interrupts, DMA, Scheduler, Buffer Cache, Callbacks) must have an explicit maximum bound. OOM is a designed state returning typed errors rather than triggering unbounded heap growth or kernel panics.
2. **Race-Free State Transitions**: State transitions are atomic or protected by documented lock hierarchies. Request descriptors are tracked with monotonic IDs and generation tokens to prevent ABA problems and request reuse races.
3. **Hardware Isolation and Timeout Governors**: Device operations must be armed with explicit timeouts to prevent hung controllers from locking kernel worker threads.
4. **Clean-Room Subsystem Implementation**: Implement native managers for Interrupts (`src/io/interrupt.rs`), DMA (`src/io/dma.rs`), Buffer Cache (`src/fs/buffer.rs`), I/O Scheduling (`src/io/scheduler.rs`), and Callbacks (`src/io/callback.rs`), coordinated by `SovereignIoManagementAndPrSuite`.

## Consequences

- Predictable latency and bounded memory overhead under peak I/O stress.
- Explicit verification through periodic rehearsals and automated unit/integration tests.
- High observability via structured health reports and procfs/sysfs diagnostic hooks.
