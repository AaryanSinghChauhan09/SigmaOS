# SigmaOS I/O Management Policy

## Subsystem Objective
Maintain SigmaOS I/O as a safe, observable, recoverable subsystem: submit, queue, execute, complete, and reclaim I/O requests without leaks, races, corruption, or undefined behavior.

## Standard I/O Lifecycle Contract
Every I/O request in SigmaOS follows an authoritative 10-stage lifecycle:
```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Stage Requirements
| Stage | Required Behavior |
|---|---|
| Submit | Request resolved, validated, and accepted into scheduler queue |
| Validate | Descriptor allocated, permissions checked, bounds checked |
| Enqueue | Placed into appropriate priority queue (RealTime, BestEffort, Idle) |
| Execute | Issued to underlying hardware or driver with deterministic timeout |
| Interrupt / DMA | Hard IRQ / DMA engine processes transfer; preemption and safety rules enforced |
| Complete | Hardware flags completion; completion queue entry recorded exactly once |
| Callback | Primary callback and secondary callback-callback handles invoked |
| Reclaim | Resources, physical buffers, and DMA mappings marked for reclaim |
| Free | Ring buffers and queues freed back to memory pools |
| Error / Recovery | Errors classified into typed variants; recovery preserves subsystem integrity |

## Device and I/O Classes
- **I/O Classes**: Request-based (block/queue), bio-based (direct filesystem bio), async (`io_uring` v1 & v2).
- **Device Classes**: NVMe 1.4+ (MMIO queues), VirtIO 1.2+ (virtqueues), USB xHCI 1.1+, Serial UART (16550).
- **Interrupt Model**: Top-half / Bottom-half offloading, bounded table capacity, lost interrupt detection.
- **DMA Model**: Coherent physical scatter-gather mapping, ring descriptors, double-DMA prevention.
- **Buffer Model**: Bounded block buffer cache (`BufferCache`), LRU eviction, dirty flushing.
- **I/O Scheduler**: Multi-queue priority classes (RealTime, BestEffort, Idle), starvation prevention.
- **Error Policy**: Typed `IoError`, `InterruptError`, `DmaError`, `SchedulerError`, `BufferError`. Zero panic policy under stress.
