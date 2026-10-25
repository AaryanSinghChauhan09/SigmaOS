# SigmaOS Interrupt Management Policy

## Overview
The Interrupt Manager (`InterruptManager`) governs IRQ routing, top-half and bottom-half handler dispatching, lost interrupt detection, and double-handler prevention.

## Requirements & Enforcements
- **Race-Free Handler Lifecycle**: Enqueue and dequeue operations are safe and synchronized.
- **Double-Handler Prevention**: Returning `InterruptError::AlreadyRegistered` if an IRQ is already assigned without explicit unregistering.
- **Lost Interrupt Detection**: Tracks lost/unhandled interrupt counts per IRQ. Exceeding threshold triggers `LostInterruptThresholdExceeded`.
- **Bounded Table & Metadata**: Upper limit (`max_handlers`) enforced on total registered handlers (`InterruptError::TableFull`).
- **OOM & Allocation Enforcement**: Zero dynamic allocations inside top-half IRQ handlers.
