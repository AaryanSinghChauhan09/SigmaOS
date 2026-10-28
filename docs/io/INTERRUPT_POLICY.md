# SigmaOS Interrupt Manager Policy

## Objectives
- Race-free handler enqueue, registration, and dispatch.
- Prevention of lost and spurious interrupts.
- Prevention of double-handler registration for the same vector.
- Bounded metadata allocations and observable OOM/exhaustion metrics.

## Policy Rules
1. **Race-Free Enqueue**: Interrupt vector dispatch table uses lock-free atomic references or bounded spinlocks.
2. **Double-Registration Prevention**: Attempting to register a handler on an occupied IRQ vector without explicit `shared` mode returns `InterruptError::HandlerAlreadyExists`.
3. **Lost Interrupt Prevention**: Pending interrupt bitmap tracks unserviced IRQs and re-triggers dispatch upon handler unmask.
4. **Interrupt Context Constraints**: Interrupt handlers MUST NOT allocate heap memory or acquire nested mutexes.
