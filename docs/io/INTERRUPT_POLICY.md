# SigmaOS Interrupt Management Policy

## Objective

Ensure interrupt handling is deterministic, race-free, bounded, and recoverable across hardware IRQs and MSI/MSI-X vectors.

## Requirements and Guarantees

### 1. Race-Free Enqueue and Dequeue
- Handler registration and unregistration use synchronized or mutually exclusive locks.
- Dispatch paths look up handlers without deadlocks or data races between top-half interrupt context and bottom-half worker threads.

### 2. Handler Reuse Policy
- Handler descriptors are reused only after complete deregistration and reclamation.
- Handlers are assigned unique 64-bit monotonic `HandlerId` identifiers to prevent ABA reuse hazards.

### 3. Bounded Handler Metadata
- Maximum capacity of the interrupt table is strictly bounded (e.g. 256 vector slots).
- Handler metadata (name, priority, mode, trigger count) is preallocated or bounded.

### 4. Lost-Interrupt Prevention
- Handlers acknowledge hardware interrupt lines immediately.
- A spurious interrupt counter tracks unmapped IRQ firings (`IrqNotAssigned`).
- Periodic watchdog audits detect lost or unhandled interrupts.

### 5. Double-Handler Prevention
- Registering a handler on an already active IRQ vector without explicit sharing flags returns `InterruptError::AlreadyRegistered`.

### 6. Observable OOM Enforcement
- If the interrupt table reaches maximum capacity, registration returns `InterruptError::TableFull`.
- OOM state is measurable and exposed in subsystem diagnostics.
