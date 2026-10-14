# SigmaOS Block Layer & Device Driver Policy

## 1. Overview

This document specifies the block layer concurrency, request allocation, and I/O completion rules enforced by `src/storage/block.rs` and driver implementations such as `src/storage/virtio.rs`.

---

## 2. Request Allocation & Lifecycle

### 2.1 Concurrency & State Machine
Every block I/O request transitions through the following atomic states:

```text
Unallocated -> Pending -> Dispatched -> Completed / Failed -> Freed
```

- **Race-Free Enqueue/Dequeue**: Request queues (`BoundedBlockDeviceManager`) use bounded array storage with atomic index tracking or locked inner queues.
- **Request Reuse**: A request slot may only be reused after its state is explicitly transitioned to `Freed` or `Unallocated`.

### 2.2 Exhaustion & Error Handing
- Queue capacity is strictly bounded (e.g. 1024 slots).
- When queue capacity is reached, new requests return `BlockError::RequestExhaustion` or `BlockError::QueueFull`.
- Heap allocation in interrupt context during request allocation is strictly forbidden.

---

## 3. Completion Safety Guarantees

### 3.1 Exact-Once Completion
Every dispatched request must receive exactly one completion event from the underlying hardware or driver.

### 3.2 Lost-Completion Prevention
- Devices maintain a completion timer / watchdog.
- If a request remains in `Dispatched` state longer than the timeout period, the block layer flags `BlockError::LostCompletion` and initiates recovery or retry.

### 3.3 Double-Completion Prevention
- Request state transitions from `Dispatched` to `Completed`/`Failed` use atomic compare-and-swap (CAS) operations or locked state updates.
- Second attempts to complete an already completed request return `BlockError::DoubleCompletion` and log a diagnostic warning.

---

## 4. Hardware Driver Contracts (VirtIO, NVMe, SATA)

1. Drivers must validate I/O block boundaries (`start_lba + block_count <= total_blocks`).
2. Out-of-bounds requests return `BlockError::OutOfBounds`.
3. Flush and barrier requests must be passed directly to controller hardware flush queues.
4. Hotplug detachment transitions device status to `DeviceDetached` and cancels pending requests with `BlockError::DeviceDetached`.
