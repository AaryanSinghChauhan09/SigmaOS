# Architecture Decision Record (ADR 001): SigmaOS Subsystem-Wide I/O Lifecycle Engine

## Status
Accepted

## Context
SigmaOS previously contained isolated I/O experimental components (`io_uring`, `fd_table`, `pipe`, `socket`, `procfs`, `sysfs`). However, as stated in system governance rules, "code present" and "unit tests pass" do not equate to subsystem integration or hardware support. To guarantee zero leaks, zero race conditions, and complete recoverability under heavy load or hardware failure, SigmaOS requires one authoritative I/O lifecycle contract across all device types and kernel subsystems.

## Decision
We adopt a unified, 10-stage stateful I/O lifecycle subsystem contract:

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA -> complete -> callback -> reclaim -> free -> error/recovery
```

### Key Architectural Principles:

1. **Centralized Lifecycle Tracking**: Every I/O request (`IoRequest`) is assigned a unique 64-bit ID and tracked explicitly through valid state transitions (`Submit` -> `Validate` -> `Enqueue` -> `Execute` -> `Interrupt` / `Dma` -> `Complete` -> `Callback` -> `Reclaim` -> `Free`).
2. **Deterministic Locking Discipline**:
   - Interrupt Manager lock (`Spinlock`) -> DMA Manager lock -> Buffer Manager lock -> Scheduler lock.
   - Lock nesting must strictly follow this order to eliminate deadlocks.
3. **No Allocation in Interrupt Context**:
   - Top-half interrupt handlers manipulate pre-allocated atomic descriptors and atomic ring buffers.
   - Deferred work is delegated to bottom-half worker queues.
4. **Typed Diagnostic Errors**:
   - Error types (`InterruptError`, `DmaError`, `SchedulerError`, `BufferError`) provide clear, actionable status codes without panicking.
5. **Double-Execution & Lost-Update Prevention**:
   - Callbacks and completions use atomic status checks (`CallbackStatus::Executing`, `CallbackStatus::Completed`) to prevent double invocation or lost wakeups.

## Consequences

### Positive
- Zero memory leaks under continuous stress testing.
- Predictable, bounded memory consumption even under OOM stress.
- Full observability of request stages via diagnostic inspection.
- POSIX-compliant error handling and ABI stability.

### Negative / Trade-offs
- Slight allocation overhead for descriptor tracking metadata.
- Strict state checking adds minimal runtime assertions in debug and release builds.
