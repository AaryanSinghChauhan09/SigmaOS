# SigmaOS Interrupt Management Policy

## Objectives
Provide deterministic, race-free interrupt routing, top-half/bottom-half execution, and lost-interrupt detection.

## Invariants & Requirements
1. **Race-Free Handler Lifecycle**: Registration, unregistration, and IRQ dispatch MUST be synchronized without data races or lock inversions.
2. **Double-Handler Prevention**: Registration on an IRQ line already owned by another handler MUST fail unless shared IRQ mode is explicitly configured.
3. **Lost-Interrupt Detection**: Spurious or unassigned IRQ signals increment atomic counters. Reaching configured thresholds triggers `LostInterruptThresholdExceeded`.
4. **Top-Half / Bottom-Half Separation**:
   - Top-half handlers execution MUST be bounded to microsecond scale.
   - Heavy work, memory allocation, or filesystem operations MUST be offloaded to bottom-half queues.
5. **Bounded Table Allocation**: Handler metadata tables are strictly bounded to `max_handlers` to prevent kernel heap exhaustion during IRQ storms.
