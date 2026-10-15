# SigmaOS Interrupt Management Policy

## 1. Overview
The Interrupt Management subsystem (`src/io/interrupt.rs`, `src/io/handler.rs`) provides race-free interrupt registration, dispatching, and monitoring for SigmaOS hardware device drivers.

## 2. Key Invariants and Rules

### 2.1 Bounded Handler Capacity
- The `InterruptManager` is instantiated with a fixed maximum capacity (`max_handlers`).
- Attaching a handler when capacity is exceeded returns `InterruptError::TableFull`.

### 2.2 Double-Handler Prevention
- An IRQ line can be mapped to at most one handler at a time unless explicit handler sharing is configured.
- Registering a second handler on an occupied IRQ without releasing the existing one returns `InterruptError::AlreadyRegistered(irq)`.

### 2.3 Lost and Spurious Interrupt Detection
- Each IRQ line tracks lost interrupts via an atomic counter (`ArcAtomicU32`).
- If an IRQ fires without an active handler registered, it is counted as a spurious interrupt.
- Exceeding the lost-interrupt threshold returns `InterruptError::LostInterruptThresholdExceeded(irq)`.

### 2.4 Race-Free Execution
- Handlers are assigned unique `HandlerId` descriptors.
- Lookup and dispatch operations execute atomically, preventing races between handler unregistration and interrupt arrival.
