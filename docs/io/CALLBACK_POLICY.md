# SigmaOS I/O Callback Policy

## Subsystem Overview
The Callback Manager (`src/io/callback.rs`) coordinates completion notifications and deferred callback-callback execution following request completion.

## Key Requirements & Invariants

1. **Double-Callback Prevention**: Callbacks are registered with unique IDs and invoked exactly once upon I/O completion.
2. **Stack Depth & Reciprocation Bounds**: Recursive callback execution is bounded to prevent kernel stack overflow (maximum recursion depth: 4).
3. **Error Propagation**: Non-zero I/O completion statuses or hardware faults are delivered directly to the callback context, allowing clean resource unmap and retry logic.
