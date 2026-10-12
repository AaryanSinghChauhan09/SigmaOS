# Block Device Policy Specification

## Enqueued & Dequeued Request Guarantees

1. **Race-Free Request Allocator**: Request slots are allocated from a bounded array with explicit locking/atomic synchronization.
2. **Request Exhaustion**: When the request queue reaches capacity (default 64 requests), submission returns `BlockDeviceError::QueueFull`. Unbounded queue expansion is prohibited.
3. **Exact-Once Completion**: Every enqueued request transitions through `Pending -> Processing -> Completed` exactly once.
4. **Double-Completion Prevention**: Completing an already completed request or completing an invalid ID returns `BlockDeviceError::DoubleCompletion` or `BlockDeviceError::InvalidRequest`.
5. **Lost-Completion Prevention**: Requests carry completion notification flags and timeout monitors to prevent hanging I/O operations.
