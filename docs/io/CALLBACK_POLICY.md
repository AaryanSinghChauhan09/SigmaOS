# SigmaOS I/O Callback Policy

## 1. Subsystem Architecture
The I/O Callback Manager (`src/io/callback.rs`) manages asynchronous completion callbacks, chained callback execution, and double-execution guards for I/O requests.

## 2. Key Rules & Guarantees

1. **Deterministic Execution**:
   - Callbacks registered for completed requests execute in registration order or upon explicit trigger.

2. **Atomic Double-Execution Guard**:
   - Each callback handle tracks execution status (`CallbackStatus::Pending`, `CallbackStatus::Executing`, `CallbackStatus::Completed`).
   - Invoking a callback transitions status atomically; subsequent invocation attempts are ignored or return an error.

3. **Chained Callback Safety**:
   - Callback-callbacks (secondary callbacks triggered by primary completion) are supported up to a max recursion depth of 4 to prevent stack overflow.

4. **Resource Management**:
   - Callback handles are managed in a bounded table (`MAX_CALLBACK_HANDLES = 1024`).
   - Handles are reclaimed automatically upon completion or error recovery.
