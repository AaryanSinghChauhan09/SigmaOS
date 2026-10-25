# Block Device Subsystem Policy

## Overview
The block device manager (`src/storage/block.rs`) handles low-level I/O requests between filesystems/caches and underlying block-oriented storage media (NVMe, VirtIO, SSD, SATA).

## Core Requirements

### 1. Bounded Request Queues
- Request queues must have a configured maximum capacity (`max_requests`).
- Submitting requests when the queue is full must fail cleanly with `BlockError::RequestExhaustion`.
- Unbounded kernel heap allocations during I/O submission are strictly forbidden.

### 2. Race-Free Request Allocation
- Request IDs are generated using atomic counters (`AtomicU64`).
- Transitioning request state (`Pending` -> `Dispatched` -> `Completed` / `Failed`) is synchronized and race-free.

### 3. Exact-Once Completion
- A request must be completed exactly once.
- Submitting a completion for an already completed or failed request must return `BlockError::DoubleCompletion`.
- Completing a non-existent or lost request must return `BlockError::LostCompletion`.

### 4. Lost-Completion Detection
- Dispatched requests track submission timestamps.
- `detect_lost_completions(current_time, timeout)` identifies stalled requests, marks them `Failed`, and returns their IDs for recovery.

### 5. Device Detachment Handling
- Unregistering or physically detaching a storage device must mark pending/dispatched requests as `Failed` with `BlockError::DeviceDetached`.
