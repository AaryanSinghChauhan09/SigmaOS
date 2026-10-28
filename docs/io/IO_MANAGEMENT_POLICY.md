# SigmaOS I/O Management Subsystem Policy Specification

## 1. Subsystem Objective

Maintain SigmaOS I/O as a safe, observable, recoverable subsystem: submit, queue, execute, complete, and reclaim I/O requests without leaks, races, corruption, or undefined behavior.

## 2. Authoritative 10-Stage Lifecycle Contract

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

| Stage | Required Behavior | Enforcement |
|---|---|---|
| Submit | Request resolved, validated, and accepted; descriptor allocated, permissions checked. | `IoScheduler::submit_request` |
| Validate | Request parameters, buffer bounds, and device permissions verified. | `IoRequest::advance(Validate)` |
| Enqueue | Priority queue insertion (RealTime, BestEffort, Idle) with bounded queue depth. | `IoScheduler` multi-queue |
| Execute | Device operation issued with timeout tracking and active mapping. | `IoScheduler::dispatch_next` |
| Interrupt/DMA | Scatter-gather DMA mapping flushed/invalidated, race-free interrupt dispatch. | `DmaManager` & `InterruptManager` |
| Complete | Status code set, total byte count calculated, completion queue updated. | `IoScheduler::complete_request` |
| Callback | Registered completion callbacks invoked exactly once; callback-callback chain defined. | `CallbackManager::invoke_callbacks` |
| Reclaim | Active resources, scatter-gather lists, and ring buffers reclaimed. | Cache & descriptor reclamation |
| Free | Request descriptor freed back to slab/pool for safe reuse. | Atomic ID & pool release |
| Error/Recovery | Typed failure isolation preventing memory/state corruption. | `SchedulerError` / `DmaError` / `InterruptError` |

## 3. I/O Architecture & Device Classes

- **I/O Class**: Request-based (block/storage), bio-based (buffer cache), and async (`io_uring`/`io_uring_v2`).
- **Device Class**: NVMe, VirtIO (block, net, GPU), USB, serial (16550 UART), AHCI SATA.
- **Interrupt Model**: Race-free handler dispatch with strict priority levels and lost/double-handler prevention.
- **DMA Model**: Scatter-gather mapping, coherent cache flush/invalidation, bounded descriptor pools.
- **Buffer Model**: LRU buffer cache with bounded capacity, lock ordering, and explicit eviction error handling.
- **I/O Scheduling Class**: Multi-queue priority scheduling (RealTime, BestEffort, Idle) with bounded depth.
- **I/O Error Policy**: Typed errors (`QueueFull`, `ResourceExhausted`, `DoubleDmaPrevented`, `InvalidScatterGatherList`), zero panics in kernel mode.
- **Limits**: Bounded queue depths, explicit OOM handling, no unbounded recursion or allocations in interrupt context.
- **NUMA Behavior**: NUMA-aware queue allocation where available.
- **Hotplug Behavior**: Graceful tear-down and callback cancellation on device removal.
