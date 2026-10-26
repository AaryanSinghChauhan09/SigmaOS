# Block Layer Policy Specification

## Architectural Overview

The SigmaOS Block Layer provides a race-free, bounded block I/O processing framework for storage devices.

## Core Requirements & Rules

### 1. Request Enqueue and Dequeue
- Enqueueing requests into `BoundedBlockDeviceManager` must be race-free and atomic.
- Enqueue operations verify device availability before allocating a request slot.

### 2. Request Reuse and Exhaustion
- Request slots are managed in a bounded pool (`max_requests`).
- When request slots are exhausted, `submit_request` immediately returns `BlockError::RequestExhaustion`.

### 3. Exact-Once Completion
- Every request transitions through defined states: `Pending -> Dispatched -> Completed | Failed`.
- Attempting to complete a request already marked `Completed` or `Failed` returns `BlockError::DoubleCompletion`.
- Completing an unknown request ID returns `BlockError::LostCompletion`.

### 4. Timeout & Lost-Completion Prevention
- Dispatched requests track submission timestamps.
- `detect_lost_completions(current_time_ms, timeout_ms)` scans dispatched requests, flags timed-out items as `Failed`, and returns affected request IDs for recovery.
