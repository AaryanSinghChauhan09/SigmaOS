# SigmaOS Interrupt Manager Policy

## 1. Subsystem Architecture
The Interrupt Manager (`src/io/interrupt.rs` and `src/io/handler.rs`) governs interrupt vector registration, handler lifecycle management, interrupt priority levels, and lost-interrupt mitigation.

## 2. Key Rules & Guarantees

1. **Race-Free Handler Enqueue/Dequeue**:
   - Registration and deregistration of interrupt handlers are protected by internal spinlocks / mutexes.
   - Vector numbers (0–255) are validated against architectural limits (x86_64 IDT vector bounds).

2. **Defined Handler Reuse Policy**:
   - Shared interrupts (e.g. PCI line interrupts) are supported via chained handler lists.
   - Exclusive handlers reject secondary registration attempts on the same vector with `InterruptError::HandlerAlreadyRegistered`.

3. **Bounded Handler Metadata**:
   - Metadata (`HandlerMetadata`) includes handler ID, IRQ vector, priority, invoke count, error count, mode (Exclusive vs Shared), and last activation timestamp.
   - Total registered handlers cannot exceed `MAX_INTERRUPT_HANDLERS` (256 entries).

4. **Lost-Interrupt Prevention**:
   - Edge-triggered and level-triggered interrupts maintain atomic pending flags.
   - Missed interrupt ticks are detected via periodic watchdog polling and re-triggered.

5. **Double-Handler Prevention**:
   - Each handler registration returns a unique `HandlerId`.
   - Deregistering an already removed or invalid `HandlerId` returns `InterruptError::HandlerNotFound`.

6. **OOM & Interrupt Context Safety**:
   - Zero dynamic allocations occur inside top-half handlers.
   - Overflowing the maximum registered handler capacity returns `InterruptError::CapacityExceeded`.
