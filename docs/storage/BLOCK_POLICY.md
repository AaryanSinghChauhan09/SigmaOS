# SigmaOS Block Device Manager Policy Specification

## 1. Overview

The Block Device Manager (`BoundedBlockDeviceManager` in `src/storage/block.rs`) governs physical and virtual block devices, I/O request queues, and hardware completion callbacks.

## 2. Invariants & Guarantees

### 2.1 Bounded Request Queue
- Queue capacity is strictly bounded by `max_requests`.
- Enqueuing when `active_requests.len() >= max_requests` returns `Err(BlockError::RequestExhaustion)`.

### 2.2 Race-Free Enqueue & Dequeue
- Request IDs are generated atomically using `AtomicU64`.
- Requests transition strictly through `Pending` -> `Dispatched` -> `Completed` / `Failed`.

### 2.3 Exact-Once Completion
- A request in state `Pending` or `Dispatched` transitions to `Completed` or `Failed` on `complete_request()`.
- Attempting to complete an already completed or failed request returns `Err(BlockError::DoubleCompletion)`.
- Attempting to complete a non-existent request returns `Err(BlockError::LostCompletion)`.

### 2.4 Timeout & Lost-Completion Protection
- `detect_lost_completions(current_time_ms, timeout_ms)` scans dispatched requests.
- Requests exceeding `submission_time_ms + timeout_ms` are failed and returned for recovery or retry.

### 2.5 Hardware Detach Protection
- Dispatching a request for an unregistered or detached device immediately fails the request with `Err(BlockError::DeviceDetached)`.
