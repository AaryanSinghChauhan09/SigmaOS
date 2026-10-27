# SigmaOS I/O Callback Manager Policy

## Objectives
- Exactly-once execution of registered completion callbacks upon request completion.
- Support for chained callback-callbacks and status propagation.
- Safe teardown and cancellation.

## Policy Rules
1. **Exactly-Once Execution**: On request completion or error recovery, `CallbackManager::invoke_callbacks` invokes and removes registered callback handles.
2. **Status Propagation**: Typed status (`Success`, `Timeout`, `HardwareError`, `Canceled`) delivered to all registered callbacks.
