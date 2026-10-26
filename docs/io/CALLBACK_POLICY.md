# SigmaOS I/O Callback Policy

## Objectives
Manage I/O completion callbacks safely, ensuring single-invocation guarantees, zero memory leaks, and error-resilient callback dispatching.

## Invariants & Requirements
1. **Single Invocation Guarantee**: Registered completion callback handles for a request ID MUST be invoked exactly once upon stage completion (`Complete` / `Callback`).
2. **Handle Cleanup**: Invoking callbacks removes all registered handle pointers from the internal callback tracking map, preventing memory leaks or dangling handles.
3. **Multi-Callback Dispatch**: Multiple callbacks registered for a single request ID are executed sequentially in order of registration.
4. **Status Propagation**: Callbacks receive explicit `CallbackStatus` variants (`Success`, `Timeout`, `HardwareError`, `Canceled`).
