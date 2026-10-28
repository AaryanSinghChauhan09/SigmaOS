# Block Device Manager Policy

## Overview
The Block Device Manager (`BoundedBlockDeviceManager`) in `src/storage/block.rs` governs block I/O requests, device registration, and request dispatching across NVMe, virtio-blk, SSD, and RAM disk storage devices.

## Requirements & Rules

1. **Request Enqueue / Dequeue Discipline**:
   - Enqueue operations validation device presence and queue capacity limits (`max_requests`).
   - Request allocation returns `BlockError::RequestExhaustion` when queue limits are reached.

2. **Exact-Once Completion Guarantee**:
   - Requests transition atomically from `Pending` -> `Dispatched` -> `Completed` / `Failed`.
   - Secondary completion calls for the same request return `BlockError::DoubleCompletion`.
   - Reaping uncompleted requests returns `BlockError::DeviceBusy`.

3. **Lost-Completion Prevention**:
   - Dispatched requests track `submission_time_ms`.
   - `detect_lost_completions(current_time_ms, timeout_ms)` scans for timed-out requests and transitions them to `Failed` to prevent kernel hangs.

4. **Typed Errors**:
   - All operations return explicit `BlockError` codes: `RequestExhaustion`, `LostCompletion`, `DoubleCompletion`, `QueueFull`, `DeviceBusy`, `InvalidRequest`, `DeviceDetached`, `WriteBlocked`, `OutOfBounds`.
