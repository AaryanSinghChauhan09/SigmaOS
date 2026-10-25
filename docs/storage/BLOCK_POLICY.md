# SigmaOS Block Device Manager Policy

## Overview

The block device management layer (`src/storage/block.rs` and `src/storage/virtio.rs`) is responsible for block device discovery, request queueing, I/O dispatching, and completion handling across physical and virtual storage devices.

---

## Core Invariants

### 1. Race-Free Enqueue and Dequeue
- **Request Submission**: Block I/O requests are submitted to `BoundedBlockDeviceManager` through `submit_request`. Monotonically increasing atomic request identifiers (`AtomicU64`) guarantee unique request IDs.
- **Queue Synchronization**: Pending request queues (`pending_queue`) operate under explicit lock protection or atomic queue discipline, ensuring concurrent thread-safe submission and dispatching.

### 2. Request Reuse and Recycling
- Requests transition through a strict state machine:
  `Unallocated` -> `Pending` -> `Dispatched` -> `Completed` / `Failed` -> `Freed`
- Request slots cannot be reused or reassigned until explicitly reaped via `reap_completed`. Attempting to reap an active request returns `BlockError::DeviceBusy`.

### 3. Typed Request Exhaustion Error
- Maximum concurrently active requests are hard-bounded by `max_requests`.
- Submitting requests when active request count equals `max_requests` immediately returns `BlockError::RequestExhaustion`. Memory allocations are strictly bounded to prevent kernel stack or heap exhaustion.

### 4. Exactly-Once Request Completion
- Request completion (`complete_request`) transitions request state from `Dispatched` or `Pending` to `Completed` or `Failed`.
- Calling `complete_request` on a request that is already in `Completed` or `Failed` state returns `BlockError::DoubleCompletion`.

### 5. Lost-Completion & Timeout Detection
- Devices track submission timestamp in milliseconds (`submission_time_ms`).
- `detect_lost_completions(current_time_ms, timeout_ms)` periodically scans dispatched requests. Requests exceeding `timeout_ms` are automatically transitioned to `Failed` and flagged with `BlockError::LostCompletion`.

---

## Supported Block Device Classes

- **`SsdBlockDevice`**: SSD simulation featuring FTL wear-leveling cycle tracking and write-blocking enforcement.
- **`NvmeBlockDevice`**: High-performance NVMe controller with multi-queue submission/completion abstractions.
- **`VirtIoBlockDevice`**: QEMU/KVM virtio-blk driver handling DMA ring descriptors.

---

## Error Specification

| Code | Variant | Trigger Condition |
|---|---|---|
| `0` | `Success` | Operation completed successfully |
| `1` | `NotFound` | Specified device or request ID does not exist |
| `2` | `ReadFailed` | Driver failed to complete block read |
| `3` | `WriteFailed` | Driver failed to complete block write |
| `4` | `WriteBlocked` | Write operation attempted on write-protected block device |
| `5` | `OutOfBounds` | Requested block number exceeds device capacity |
| `8` | `RequestExhaustion` | Active requests reach `max_requests` limit |
| `9` | `LostCompletion` | Request timed out in dispatched state |
| `10` | `DoubleCompletion` | Request completed multiple times |
| `14` | `DeviceDetached` | Request dispatched to unmounted or detached device |
