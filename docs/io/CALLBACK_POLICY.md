# SigmaOS I/O Callback Policy

## Objective

Manage completion callbacks and callback-callback chaining for asynchronous I/O operations without reentrancy panics, stack overflows, or lost notifications.

## Requirements and Guarantees

### 1. Bounded Callback Registration
- Registered callbacks are mapped to unique request IDs.
- Prevents unbounded memory growth in waiting lists.

### 2. Chained Callback-Callback Safety
- Callbacks can register secondary callbacks for hierarchical or multi-stage notifications.
- Execution is isolated so an exception or panic in one callback does not abort other registered handlers.

### 3. Status Reporting
- Callbacks receive typed statuses: `CallbackStatus::Success`, `CallbackStatus::Failed`, `CallbackStatus::Cancelled`.

### 4. Race-Free Invocation and Reclamation
- Callbacks are removed from pending tables prior to invocation (at-most-once execution).
- Resources associated with callback handles are reclaimed immediately after execution.
