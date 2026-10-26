# SigmaOS Block Device Subsystem Policy

## 1. Overview

The SigmaOS block device manager (`src/storage/block.rs` and `src/storage/virtio.rs`) provides safe, race-free request queuing and completion handling for block-oriented storage devices.

## 2. Invariants and Safety Guarantees

### 2.1 Race-Free Request Allocation
- Enqueue and dequeue operations are protected by atomic state variables (`RequestState`) or internal locks.
- Request slots are managed via bounded pools.

### 2.2 Request Exhaustion
- If the request queue reaches capacity, the allocator returns `BlockError::RequestExhaustion`.
- Unbounded queue growth and memory allocation during request submission are explicitly forbidden.

### 2.3 Request Completion Guarantees
- Every submitted block request transitions strictly from `Pending` -> `Dispatched` -> `Completed` or `Failed`.
- **Exactly-Once Completion**: Block drivers must complete each request exactly once.
- **Double-Completion Prevention**: Completing an already completed or freed request returns `BlockError::DoubleCompletion`.
- **Lost-Completion Prevention**: Active requests in `Dispatched` state maintain timeout tracking; lost completions trigger diagnostic recovery and return `BlockError::LostCompletion`.

## 3. Device Support and Hardware Abstraction

- **VirtIO Block**: Uses VirtIO split/packed queues with descriptors (`VirtQueueDesc`) and request headers (`VirtIoBlockHeader`).
- **SSD / FTL Wear Leveling**: Supports FTL wear leveling, TRIM/discard operations, and bad-block map management.
