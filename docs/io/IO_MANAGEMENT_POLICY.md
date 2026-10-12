# SigmaOS I/O Subsystem Management Policy

## Objective

Maintain SigmaOS I/O as a safe, observable, recoverable subsystem: submit, queue, execute, complete, and reclaim I/O requests without leaks, races, corruption, or undefined behavior.

## Authoritative Lifecycle Contract

Every I/O request in SigmaOS follows a 10-stage lifecycle:

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Required Behavior Per Stage

| Stage | Required Behavior |
|---|---|
| **Submit** | Request resolved, validated, accepted; descriptor allocated, permissions checked |
| **Validate** | Parameter boundaries, buffer alignment, target device status, and permissions checked |
| **Enqueue** | Request sorted into priority queue (RealTime, BestEffort, Idle) without locks/races |
| **Execute** | Device operation issued with bounded timeout and retry budget |
| **Interrupt/DMA** | IRQ handler safe, top-half/bottom-half preemption defined, scatter-gather DMA mapped |
| **Complete** | Device status acknowledged, bytes transferred verified, completed exactly once |
| **Callback** | Completion callback and chained callback-callback invoked safely |
| **Reclaim** | DMA mappings, physical buffers, and ring slots reclaimed |
| **Free** | Request descriptor returned to slab/pool for reuse |
| **Error/Recovery** | Hardware/timeout failure isolated, state preserved, error code returned |

## Subsystem Classes & Limits

- **I/O Class**: Request-based, bio-based, and io_uring async rings.
- **Device Class**: NVMe, VirtIO, SATA/AHCI, USB, Serial (UART).
- **Interrupt Model**: Top-half ISR with bottom-half offload; bounded handler table.
- **DMA Model**: Scatter-gather mapping with strict physical address checks and double-DMA guards.
- **Buffer Model**: Multi-device block buffer cache with clean/dirty/locked/in-flight states and LRU eviction.
- **Scheduler Class**: Real-Time, Best-Effort, Idle queue priority scheduling.
