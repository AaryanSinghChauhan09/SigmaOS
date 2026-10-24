# SigmaOS Block Device Layer Policy

## Overview
The block device layer (`src/storage/block.rs` and `src/storage/virtio.rs`) provides safe, race-free block I/O scheduling and request queue management for physical and virtual block devices.

## Requirements

1. **Request Queue Bounds**: All submission queues must have explicit upper size limits (`max_pending_requests`).
2. **Request Exhaustion**: When a request queue reaches capacity, `BlockError::RequestExhaustion` or `BlockError::QueueFull` is returned immediately without blocking or panicking.
3. **Atomic State Invariants**: Each request moves sequentially through `Unallocated -> Pending -> Dispatched -> Completed / Failed -> Freed`.
4. **Exactly-Once Completion**: Double-completion attempts return `BlockError::DoubleCompletion`. Lost completion checks verify dispatch timer bounds.
5. **VirtIO Block Compatibility**: VirtIO devices implement virtqueue request submission with header validation and flush descriptor handling.
