# SigmaOS I/O Management Policy

## Subsystem Objective

Maintain SigmaOS I/O as a safe, observable, recoverable subsystem: submit, queue, execute, complete, and reclaim I/O requests without leaks, races, corruption, or undefined behavior.

## Standard I/O Lifecycle Contract

Every I/O transaction across SigmaOS must adhere to the 11-stage authoritative lifecycle contract:

```text
submit -> validate -> enqueue -> execute -> interrupt/DMA ->
complete -> callback -> reclaim -> free -> error/recovery
```

### Stage Requirements Matrix

| Stage | Behavior & Guarantee |
|---|---|
| **Submit** | Request parameters resolved, validated, and accepted by kernel entry gate. Descriptor allocated, permissions and capabilities checked. |
| **Validate** | Memory regions, buffer alignment, boundary checks, and request parameters verified before scheduling. |
| **Enqueue** | Bounded placement into priority/scheduler queue without lock contention or allocation under spinlock. |
| **Execute** | Issued to target hardware device or synthetic driver with strictly bounded execution timeout. |
| **Interrupt / DMA** | Race-free interrupt handler dispatch, top-half queueing, and hardware DMA memory transfer with cache coherency barriers. |
| **Complete** | Completed exactly once; status code, byte count, and completion events recorded. |
| **Callback** | Completion notification dispatched to registered top-half/bottom-half or async ring listeners. |
| **Reclaim** | Hardware descriptors, DMA buffer maps, and ring slots returned to recycling pools. |
| **Free** | Allocated memory and temporary tracking structures returned to system allocators without memory leaks. |
| **Error / Recovery** | Typed, diagnosable error propagation; device resets or queue flushing triggered without corrupting subsystem state. |

## I/O & Device Classes

1. **Request-Based I/O**: Block devices, NVMe submission/completion queues, virtio-blk queues.
2. **Bio-Based I/O**: Direct file system block buffers (`src/fs/buffer.rs`), page-cache flush/sync, raw disk sector I/O.
3. **Async Ring I/O**: Zero-copy async kernel submit/completion rings (`src/io/io_uring.rs`, `src/io/io_uring_v2.rs`).

## Supported Hardware & Peripheral Classes

- **NVMe Subsystem**: PCIe NVMe controller queues, submission/completion doorbell registers, admin/IO ring pairs.
- **VirtIO Devices**: `virtio-blk`, `virtio-net`, `virtio-console`, virtqueue split/packed ring descriptors.
- **USB & Serial**: xHCI host controllers, USB mass storage, COM1 0x3F8 UART serial console transmitter.
- **Block & Network**: Disk sector devices, virtio-net Ethernet frames, eBPF XDP socket buffers.

## Hard Operating Limits & Safety Bounds

- Maximum Concurrent In-Flight I/O Descriptors: 8,192 per device controller.
- Timeout Default: 5,000ms per request (hardware reset triggered upon breach).
- Maximum DMA Scatter-Gather Segments: 128 vectors per request.
- Allocation in Interrupt Context: Strictly forbidden (pre-allocated ring pools required).

## Hotplug & NUMA Policy

- **NUMA Local Allocation**: Ring buffers and DMA descriptor pools are allocated from the NUMA node local to the target PCIe bus root complex.
- **Hotplug Invalidation**: Upon device unplug, all in-flight requests transition to `Err(IoError::DeviceDisconnected)`, wait queues are unblocked, and resources are safely reclaimed.
