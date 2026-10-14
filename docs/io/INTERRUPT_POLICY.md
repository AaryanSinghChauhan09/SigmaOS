# SigmaOS Interrupt Management Policy

## Subsystem Overview
The Interrupt Manager (`src/io/interrupt.rs`, `src/io/handler.rs`) governs hardware IRQ lines, interrupt vector routing, top-half execution, and bottom-half/deferred work dispatch.

## Key Requirements & Invariants

1. **Race-Free Handler Registration**: Handlers are registered and unregistered through `InterruptManager::register_handler` and `unregister_handler` with atomic handler ID generation (`next_handler_id`).
2. **Double-Handler Prevention**: Re-registering an assigned IRQ line returns `Err(InterruptError::AlreadyRegistered(irq))` unless shared IRQ routing is explicitly enabled.
3. **Bounded Handler Metadata**: The interrupt table capacity is bounded (`max_handlers`). Table overflow returns `Err(InterruptError::TableFull)`.
4. **Lost & Spurious Interrupt Prevention**: Spurious or unassigned interrupts increment atomic counters (`spurious_interrupts`, `lost_interrupt_counters`). Exceeding 100 lost interrupts on an active line triggers `Err(InterruptError::LostInterruptThresholdExceeded(irq))`.
5. **Context Safety**: Interrupt handlers must execute with zero allocation. Memory must be pre-allocated during initialization.
