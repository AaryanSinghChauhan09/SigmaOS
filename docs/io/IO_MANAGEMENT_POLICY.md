# SigmaOS I/O Management Policy

## Objective

Maintain SigmaOS I/O as a safe, observable, recoverable subsystem: submit, queue, execute, complete, and reclaim I/O requests without leaks, races, corruption, or undefined behavior.

The first dependency is a validated firmware-to-interactive-shell path. SigmaOS identifies boot-to-shell validation, recoverable installer/update paths, and connecting subsystem models to runtime paths as near-term priorities.

## Authoritative I/O Lifecycle Contract

Define one authoritative lifecycle:

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Stage Contracts and Required Behaviors

| Stage | Required Behavior | Invariants / Guarantees |
|---|---|---|
| **Submit** | Request resolved, validated, and accepted | Descriptor allocated, permissions checked. Request parameters (buffer, length, offset) established. |
| **Validate** | Memory safety and capability checks | Validates user-space address ranges, scatter-gather list limits, device bounds, and task permissions. |
| **Enqueue** | Race-free scheduling queue placement | Places request into bounded multi-queue priority rings (RealTime, BestEffort, Idle). Lost-I/O prevented. |
| **Execute** | Device operation issued with timeout | Command sent to hardware/virtual device ring. Watchdog timers armed to detect hung or stalled hardware. |
| **Interrupt / DMA** | Handler safe, preemption defined | Top-half / bottom-half interrupt handling. Coherent scatter-gather DMA mappings. Double-DMA prevented. |
| **Complete** | Exactly-once completion and status update | CQE posted to completion queue. Status recorded, error types mapped without corrupting internal state. |
| **Callback** | Callback and callback-callback defined | Chained completion callbacks executed race-free. Failures in callbacks isolated from core queue. |
| **Reclaim** | Descriptor and buffer release | DMA mappings unmapped, scatter-gather lists torn down, buffer cache reference counts decremented. |
| **Free** | Memory returned to bounded pools | Allocation slots returned to free list. Prevents descriptor leakage under high load or panic. |
| **Error / Recovery** | Failure does not corrupt state | Errors typed (`InterruptError`, `DmaError`, `BufferError`, `SchedulerError`). Device ring reset or retry. |

## Subsystem Architecture Classifications

### 1. I/O Class
- **Request-Based I/O**: Structured I/O packets with explicit priority, timeouts, and scatter-gather lists.
- **Bio-Based I/O**: Block-level transfers operating directly against the LRU buffer cache and filesystem inodes.
- **Async I/O**: High-performance completion-ring asynchronous I/O modeled after `io_uring` with SQPOLL and CQE rings.

### 2. Device Class
- **NVMe**: PCIe Non-Volatile Memory Express controller interface with multi-queue MMIO doorbell rings.
- **VirtIO**: Paravirtualized block (`virtio-blk`), network (`virtio-net`), and console devices for virtual machine hypervisors.
- **USB**: Universal Serial Bus host controller interface (xHCI) supporting hotpluggable storage and HID devices.
- **Serial**: Standard 16550 UART early boot serial console for kernel logging, panic dumps, and interactive shell debugging.

### 3. Interrupt Model
- Bounded interrupt vector table (default 256 vectors).
- Split-driver architecture: Top-half executes in hardware interrupt context with interrupts masked; bottom-half deferred work queued to worker threads.
- Lost-interrupt detection via spurious counter and periodic ring polling.
- Double-handler registration strictly prevented with typed `AlreadyRegistered` errors.

### 4. DMA Model
- Bounded DMA descriptor queue depths.
- Physical address scatter-gather list validation (non-empty, non-zero lengths, memory boundary alignment).
- Explicit cache coherence barriers before device submission (`ToDevice`) and after completion (`FromDevice`).
- Double-DMA prevention: Enforces single active DMA mapping per buffer identifier.

### 5. Buffer Model
- LRU (Least Recently Used) block buffer cache with fixed capacity.
- Explicit buffer states: `Clean`, `Dirty`, `Locked`, `InFlight`.
- Dirty writeback synchronization to backing storage.
- Observable pressure watermarks (`Normal`, `HighPressure`, `OomCritical`) with legal eviction policies.

### 6. I/O Scheduling Class
- Multi-queue priority scheduling:
  - **RealTime (RT)**: Highest priority, preemptive dispatch for low-latency audio, display, or control loops.
  - **BestEffort (BE)**: Standard priority with round-robin or deadline fairness.
  - **Idle**: Background batch tasks executed only when RealTime and BestEffort queues are empty.
- Strict bounded queue limits to enforce OOM determinism.

### 7. I/O Callback Model
- Bounded callback queues.
- Asynchronous notification upon request completion.
- Support for chained callbacks (callback-callback) for secondary completion processing (e.g. journaling update -> fsync notifier).
- Status-based dispatch (`Success`, `TimedOut`, `Aborted`, `DeviceError`).

### 8. I/O Error Policy
- All errors are typed, observable, and propagated without kernel panic.
- Failure of an I/O operation does not leak memory descriptors or leave device rings in an inconsistent state.
- Hardware timeouts trigger error transitions and device reset recovery procedures.

### 9. Limits and Resource Caps
- Max interrupt vectors: 256.
- Max concurrent DMA mappings: Configurable per controller (default 1024).
- Max I/O scheduler queue depth: Configurable per priority class (default 4096).
- Max buffer cache size: Bounded pool allocated at system initialization.

### 10. NUMA Behavior
- DMA buffers and I/O scheduler queues are allocated from the local NUMA node nearest to the PCIe root complex or controller.
- Memory affinity reduces inter-socket QPI/UPI bus traffic.

### 11. Behavior Under I/O Hotplug
- Device detachment marks controller offline.
- In-flight requests gracefully failed with typed `DeviceDetached` errors.
- Device descriptors and interrupt lines safely unregistered and reclaimed without memory leaks.
