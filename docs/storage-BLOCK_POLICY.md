# Block Device Policy

## Overview
The block device subsystem manages block I/O requests across storage hardware (NVMe, SSD, VirtIO, Loop).

## Invariants & Rules
1. **Bounded Requests**: Request queues are strictly bounded (`max_requests`). Submissions exceeding queue size return `BlockError::RequestExhaustion`.
2. **Exact-Once Completion**: Requests transition through state machine: `Pending -> Dispatched -> Completed/Failed -> Reaped`.
3. **Double Completion Prevention**: Completing an already completed/failed request returns `BlockError::DoubleCompletion`.
4. **Lost Completion Prevention**: Timeouts detect stale dispatched requests and mark them failed (`detect_lost_completions`).
5. **Write Blocking**: Write operations on write-blocked devices return `BlockError::WriteBlocked`.
