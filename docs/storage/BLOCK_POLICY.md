# Block Device Manager Specification and Policy

## 1. Overview

The Block Device Manager (`src/storage/block.rs`) abstracts physical and virtual block storage devices (NVMe, VirtIO block, SSD, HDD, loop devices) and provides safe request enqueueing, dispatching, execution, and completion.

## 2. Queue & Request Lifecycle

Block requests transition through explicit state machine stages:

```text
Unallocated -> Pending -> Dispatched -> Completed / Failed -> Freed
```

### 2.1 Request Allocation and Bounding
- The manager maintains a fixed maximum queue capacity (`max_queue_depth`).
- Submitting a request when active requests reach capacity returns `Err(BlockError::RequestExhaustion)`.
- Requests receive monotonically increasing sequence identifiers (`RequestID`).

### 2.2 Dispatch and Concurrency
- `dispatch_next()` retrieves the oldest `Pending` request and marks its state as `Dispatched`.
- Dispatching is race-free and guarded by internal synchronization mechanisms.

### 2.3 Completion Semantics
- Executing `complete_request(request_id, success)` transitions a `Dispatched` request to `Completed` or `Failed`.
- Re-completing an already completed or freed request returns `Err(BlockError::DoubleCompletion)`.
- Attempting to complete non-existent or unallocated request IDs returns `Err(BlockError::InvalidRequest)`.

## 3. Wear-Leveling and Device Specialization

- SSD devices track write cycle counts per block for wear-leveling diagnostics.
- Write operations to write-protected devices fail cleanly with `BlockError::WriteBlocked`.
- Out-of-bounds sector accesses fail cleanly with `BlockError::OutOfBounds`.
