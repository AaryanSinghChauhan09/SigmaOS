# Block Device Manager Policy

## 1. Scope and Overview

The Block Device Manager (`src/storage/block.rs`, `src/storage/virtio.rs`) is responsible for block device discovery, capacity identification, request allocation, I/O queue management, and exact-once request completion across all storage media in SigmaOS.

## 2. Block Layer Lifecycle & Request State Machine

Requests in the block layer transition through an explicit 6-state lifecycle:

```text
Unallocated -> Pending -> Dispatched -> Completed/Failed -> Freed
```

### Request State Rules
- **Unallocated**: Request slot in `BoundedBlockDeviceManager` is free for reuse.
- **Pending**: Request created, validated against LBA boundary limits, and placed into the device queue.
- **Dispatched**: Request sent to physical or virtual hardware controller (e.g. VirtIO ring, NVMe submission queue).
- **Completed / Failed**: I/O processing finished by host controller; completion status recorded.
- **Freed**: Request slot returned to unallocated pool.

## 3. Queue Bounds & Resource Limits

1. **Queue Capacity**: Each block device manager instance is constructed with a fixed request pool capacity (`max_requests`).
2. **Exhaustion Handling**: When the pending/dispatched queue reaches maximum capacity, `submit_request` fails immediately with `Err(BlockError::RequestExhaustion)`.
3. **LBA Validation**: Sector offsets and block counts are validated prior to queue insertion. Out-of-bounds requests return `Err(BlockError::OutOfBounds)`.

## 4. Exact-Once Completion Guarantees

1. **Single Completion**: Calling `complete_request(req_id, success)` marks a `Dispatched` request as `Completed` or `Failed`.
2. **Double Completion Prevention**: Any subsequent attempt to complete the same `req_id` returns `Err(BlockError::DoubleCompletion)`.
3. **Lost Completion Prevention**: The block manager tracks dispatched timestamps. Stalled requests are detected via watchdog audits and failed gracefully with `BlockError::LostCompletion`.

## 5. Concurrency and Lock Discipline

- The block device registry and request queues use strict mutex/spinlock lock ordering:
  `BlockDeviceManager Lock -> Controller Ring Lock -> Hardware Interrupt Lock`.
- Lock holds during physical DMA register access are minimized to prevent interrupt latency spikes.
