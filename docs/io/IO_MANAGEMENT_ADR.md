# Architecture Decision Record (ADR): Unified I/O Lifecycle Subsystem

## Status
Accepted

## Context
SigmaOS requires a unified, safe, recoverable, and observable I/O subsystem across NVMe, VirtIO, PCI, and bare-metal architectures. Previous I/O implementations lacked strict lifecycle state transitions, leading to potential request leaks or race conditions under high concurrency.

## Decision
Adopt an authoritative 10-stage I/O request lifecycle (`submit -> validate -> enqueue -> execute -> interrupt/DMA -> complete -> callback -> reclaim -> free -> error/recovery`) with bounded managers for:
1. **Interrupt Manager**: Bounded IRQ table, double-handler guard, lost interrupt detection.
2. **DMA Manager**: Physical address scatter-gather list mapping, total byte accounting, and double-DMA guard.
3. **Buffer Cache**: Multi-device block buffer cache with LRU eviction and locked/dirty buffer semantics.
4. **I/O Scheduler**: Priority-based multi-queue scheduler (RealTime, BestEffort, Idle).
5. **Callback Manager**: Exactly-once completion callback router.

## Consequences
- Prevents leaks, races, and double-completion errors.
- All I/O errors are typed, diagnosable, and propagated.
- Out-of-memory and buffer exhaustion are handled as designed states without panic.
