# SigmaOS Block Layer Policy

## Overview
This document specifies the design, invariants, and error semantics of the SigmaOS Block Layer subsystem implemented in `src/storage/block.rs` and `src/storage/virtio.rs`.

## Requirements & Invariants

### 1. Race-Free Request Enqueue and Dequeue
- `BoundedBlockDeviceManager` manages request queues using explicit locking and atomic lifecycle tracking.
- Requests transition through `RequestState`: `Unallocated -> Pending -> InFlight -> Completed -> Reaped`.
- Dequeue via `dispatch_next()` atomically marks requests as `InFlight`.

### 2. Request Reuse Policy
- Request IDs are generated monotonically or pooled from a bounded set.
- A request slot cannot be reused while in `Pending`, `InFlight`, or `Completed` state.
- Slots only become eligible for reuse after explicit reclamation via `reap_completed()`.

### 3. Request Exhaustion Handling
- Total concurrent requests cannot exceed the bounded limit configured in `BoundedBlockDeviceManager::new(max_pending_requests)`.
- When capacity is reached, new submissions return `Err(BlockError::RequestExhaustion)` immediately without blocking or panicking.

### 4. Exactly-Once Request Completion
- Request completion via `complete_request(req_id, success)` validates that the request is currently in the `InFlight` or active state.
- Once marked `Completed`, subsequent calls to `complete_request` for the same request return `Err(BlockError::DoubleCompletion)`.

### 5. Lost-Completion Prevention
- Dispatched requests are tracked in an active dispatch ledger.
- Timeouts, device watchdog interrupts, and heartbeat checks verify that in-flight requests complete within bounded latency. Unresponsive requests trigger controller reset or typed timeout errors.

### 6. Double-Completion Prevention
- State transitions are strictly validated. Any attempt to signal completion for a request that is not `InFlight` is rejected with `BlockError::DoubleCompletion`.

## Device Drivers & virtio-blk

`VirtIoBlockDevice` provides the paravirtualized storage driver interface:
- Supports read (`VIRTIO_BLK_T_IN`), write (`VIRTIO_BLK_T_OUT`), and flush (`VIRTIO_BLK_T_FLUSH`) operations.
- Verifies sector geometry, block boundaries, and queue capacity.
- Manages descriptor tables and status bytes safely without undefined behavior.
