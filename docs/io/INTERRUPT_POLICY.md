# Interrupt Manager Policy

## Overview
The Interrupt Manager governs IRQ routing, handler dispatching, double-handler guards, and lost interrupt detection in SigmaOS.

## Requirements
- Handler enqueue/dequeue is race-free.
- Handler reuse follows a defined unregistration policy.
- Handler metadata is bounded.
- Lost-interrupt prevention and spurious interrupt logging are defined.
- Double-handler prevention is enforced per IRQ line.
- Out-Of-Memory (OOM) / Table-Full enforcement returns `InterruptError::TableFull`.
