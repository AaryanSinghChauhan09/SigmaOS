# SigmaOS I/O Subsystem Management Policy

## 1. Overview & Objective

The objective of the SigmaOS I/O Subsystem Management Policy is to establish an authoritative, safe, observable, and recoverable subsystem for managing device I/O, interrupts, Direct Memory Access (DMA), buffer caches, and I/O scheduling.

SigmaOS manages I/O as a unified lifecycle subsystem ensuring requests transition safely without resource leaks, races, corruption, or undefined behavior.

## 2. Authoritative I/O Lifecycle Contract

Every I/O operation within SigmaOS adheres to a single 10-stage lifecycle contract:

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA -> complete -> callback -> reclaim -> free -> error/recovery
```

### Stage Contracts & Guarantees

1. **Submit**: Request parameters (device ID, buffer pointers, lengths, flags) are passed from user or kernel context. Descriptor is allocated; caller permissions and capabilities are verified.
2. **Validate**: Bounds checking, alignment verification, address space validation (user vs kernel mapping), and resource limit checks are performed.
3. **Enqueue**: The validated request is placed into an I/O scheduling queue (e.g. priority queue, FIFO, or deadline scheduler) in a race-free manner.
4. **Execute**: The I/O scheduler selects the request and issues the device-specific hardware or ring buffer operation with a strict, non-zero hardware timeout.
5. **Interrupt/DMA**: Hardware signals completion via interrupt or DMA status register update. Top-half handlers execute in interrupt context without memory allocation; bottom-half deferred work is enqueued.
6. **Complete**: The request transitions to completed state. CQE (Completion Queue Entry) or completion token is updated exactly once.
7. **Callback**: Registered completion callbacks and chained callback-callbacks are invoked deterministically.
8. **Reclaim**: DMA mappings are unmapped/invalidated; hardware resources and buffer locks are released.
9. **Free**: Request descriptors, callback handles, and metadata nodes are returned to bounded slab/pool allocators.
10. **Error / Recovery**: Any stage failure transitions the request to an explicit error state, logs structured diagnostic telemetry, triggers device-level recovery/reset if required, and frees associated resources without corrupting system state.

## 3. Subsystem Architecture & Classification

### 3.1 I/O Classes
* **Request-Based I/O**: High-level block and character requests (`IoRequest`) encapsulated with explicit request IDs, priority levels, and hardware timeout parameters.
* **Bio-Based I/O**: Direct page/buffer vector I/O (`ScatterGatherElement`) for raw disk and block storage drivers.
* **Asynchronous I/O**: `io_uring` and `io_uring_v2` submission and completion ring infrastructure supporting multishot, fixed buffers, registered file descriptors, and zero-copy operations.

### 3.2 Device Classes
* **NVMe**: PCIe high-throughput NVMe controllers using submission/completion queue pairs and MMIO doorbells.
* **VirtIO**: VirtIO-block, VirtIO-net, and VirtIO-console devices using split and packed virtqueue rings.
* **USB**: xHCI host controllers, transfer rings, and USB endpoint management.
* **Serial**: 16550 UART early serial console and character devices for diagnostics.

### 3.3 Interrupt, DMA, Buffer, & Callback Models
* **Interrupt Model**: Race-free bounded handler registration with strict interrupt priority levels (Low, Normal, High, Critical), nested interrupt safety, and lost-interrupt detection.
* **DMA Model**: Bounded DMA mapping manager supporting physical scatter-gather element vectors, explicit cache coherency directions (HostToDevice, DeviceToHost, Bidirectional), and zero-copy transfer validation.
* **Buffer Model**: Bounded LRU buffer set manager (`BufferSetManager`) enforcing allocation limits, dirty page flushing, exact eviction semantics, and OOM resilience.
* **Callback Model**: High-performance completion callback engine (`CallbackManager`) executing chained callbacks with atomic double-execution prevention.

## 4. Resource Limits & OOM Policy

* **Bounded Allocations**: All queue depths, descriptor pools, interrupt tables, DMA tracking slots, and callback handle tables are statically capped or bounded.
* **OOM Enforcement**: Upon reaching resource limits, new submissions are rejected with typed errors (`SchedulerError::QueueFull`, `InterruptError::CapacityExceeded`, `DmaError::OutOfDescriptors`).
* **No Panics in I/O Path**: Memory allocation failures in I/O paths return typed errors and trigger graceful request rejection or buffer eviction rather than panicking.
* **Interrupt Context Discipline**: Zero dynamic allocations are permitted within top-half interrupt context handlers.

## 5. Security & NUMA / Hotplug Policy

* **Capability Verification**: Privileged I/O operations require explicit process capabilities (e.g., `CAP_SYS_RAWIO`).
* **Hotplug Handling**: Device disconnect events gracefully cancel pending requests, flush wait queues, invoke error callbacks, and reclaim descriptor structures.
* **NUMA Behavior**: Memory allocations for device rings and DMA buffers prioritize node-local memory pages when NUMA topology is active.
