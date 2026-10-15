# SigmaOS I/O Management Policy

## 1. Overview and Subsystem Objective
SigmaOS manages I/O as an authoritative, safe, observable, and recoverable lifecycle subsystem. The primary objective of the subsystem is to:
> **Maintain SigmaOS I/O as a safe, observable, recoverable subsystem: submit, queue, execute, complete, and reclaim I/O requests without leaks, races, corruption, or undefined behavior.**

## 2. Mandatory I/O Lifecycle Contract
Every I/O operation in SigmaOS must strictly adhere to the 10-stage unified lifecycle:

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior | Failure / Recovery Mode |
|---|---|---|
| **Submit** | Operation parameters specified; descriptor allocated, permissions checked. | `ResourceExhausted` if queue full; descriptor freed on error. |
| **Validate** | Parameter sanity check (block ranges, non-zero lengths, memory permissions). | Immediate return of typed error (`InvalidStageTransition`, `InvalidScatterGatherList`). |
| **Enqueue** | Placed into multi-queue scheduler (`RT`, `BE`, `Idle`) based on `IoPriority`. | Queue depth bounded; rejection if capacity exceeded. |
| **Execute** | Issued to underlying device driver with deterministic timeout. | Driver timeout triggers hardware reset and recovery path. |
| **Interrupt / DMA** | Interrupt handler safely triggered; DMA channel programmed with physical memory SG list. | Coherence check; lost interrupt/DMA detection via watchdog timer. |
| **Complete** | Device confirms execution status; request transitions to completion state. | Double-completion guarded by atomic stage transitions. |
| **Callback** | Registered completion handle invoked exactly once with status (`Success`, `Timeout`, `HardwareError`, `Canceled`). | Isolated callback execution; handle detached after invocation. |
| **Reclaim** | DMA mappings unmapped; buffer locks released; state clean-up verified. | Safe resource reclamation even under error conditions. |
| **Free** | Descriptor returned to global freelist or deallocated. | Zero-leak guarantee. |
| **Error / Recovery** | Fault diagnosed, typed error returned, device re-initialized if necessary. | No global state corruption; isolated error propagation. |

## 3. Subsystem Architecture Specifications

### 3.1 I/O Classes
- **Request-Based:** Block and async device requests structured into `IoRequest` descriptors.
- **Bio-Based:** Direct block buffer access managed via `BufferCache` with LRU eviction and dirty page flushes.
- **Async I/O:** High-throughput `io_uring` and `io_uring_v2` multishot / zero-copy submission & completion rings.

### 3.2 Device Classes Supported
- **NVMe:** Direct queue pair handling and NVMe passthrough commands.
- **VirtIO:** VirtIO block and net virtqueue submission paths.
- **Serial / Console:** Bounded UART IRQ delivery and ring buffer transmitter.
- **USB:** xHCI host controller transfers.

### 3.3 Interrupt Model
- Bounded IRQ routing table (`InterruptManager`).
- Double-handler registration guards.
- Bounded lost/spurious interrupt counters with threshold enforcement.

### 3.4 DMA Model
- Bounded physical DMA descriptor queue (`DmaManager`).
- Validation of `ScatterGatherElement` lists before mapping.
- Double-DMA mapping prevention and cache coherence tracking.

### 3.5 Buffer Model
- Multi-device block buffer cache (`BufferCache`).
- LRU eviction policy; dirty buffer flush pipelines.
- Pool exhaustion protection for locked buffers.

### 3.6 Limits and OOM Behavior
- Strict allocation bounds on queues, handler maps, buffer pools, and callbacks.
- Rejection of incoming requests when limits are reached with typed errors (`QueueFull`, `PoolExhausted`, `TableFull`).
- OOM treated as a normal, handled condition rather than causing kernel panic.

### 3.7 NUMA and Hotplug Behavior
- NUMA-aware buffer allocation strategy when enabled.
- Safe device detachment during hotplug events: in-flight requests canceled and completed with `Canceled` status; resources deallocated safely.
