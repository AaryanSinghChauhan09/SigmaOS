# SigmaOS I/O Management Policy

## Objective
Maintain SigmaOS I/O as a safe, observable, recoverable subsystem: submit, queue, execute, complete, and reclaim I/O requests without leaks, races, corruption, or undefined behavior.

## Authoritative I/O Lifecycle Contract

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior |
|---|---|
| Submit | Request resolved, validated, descriptor allocated, permissions checked |
| Validate | Command opcode, buffer bounds, and device readiness verified |
| Enqueue | RealTime, BestEffort, or Idle multi-queue placement, bounded queue depth |
| Execute | Device operation issued with timeout tracking and status checks |
| Interrupt / DMA | Race-free IRQ/DMA handling, preemption defined, scatter-gather list validation |
| Complete | Executed exactly once, CQE generated, state transitioned to Complete |
| Callback | Registered completion callbacks executed safely without double-invocation |
| Reclaim | Resources reclaimed, descriptors recycled without memory leaks |
| Free | Active request table cleared, memory freed |
| Error / Recovery | Failure does not corrupt state; typed errors logged and propagated |

## Subsystem Parameters

- **I/O Classes**: Request-based, Bio-based, Async (`io_uring`, `io_uring_v2`).
- **Device Classes**: NVMe, VirtIO, USB, Serial.
- **Interrupt Model**: Race-free top-half/bottom-half handlers with lost-interrupt tracking.
- **DMA Model**: Coherent physical scatter-gather list mapping with double-DMA guards.
- **Buffer Model**: Block buffer cache (`BufferHead`) with clean/dirty/locked/in-flight states and LRU eviction.
- **I/O Scheduler**: Multi-queue scheduler with priority queues (RT, BE, Idle).
- **Callback Model**: Bounded multi-callback registration with strict status reporting.
- **Limits & Hotplug**: Explicit queue depth bounds, OOM handling, hotplug notification hooks.
