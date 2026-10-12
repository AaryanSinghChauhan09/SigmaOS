# Swap Subsystem Policy

## Scope

This document specifies secondary storage swap slot allocation, request queueing, and OOM observability for SigmaOS.

## Principles

1. **Race-Free Swap Operations**: Swap slot acquisition and page-out/page-in operations are thread-safe and protected against double-swap or double-free conditions.
2. **Bounded Swap Metadata**: Swap slots are indexed via bitmask allocators and bounded metadata structures to avoid uncontrolled kernel memory consumption.
3. **Priority Queueing**: Swap requests are categorized into `High`, `Normal`, and `Low` priority levels. System critical pages receive high priority dispatch.
4. **Observable OOM Metrics**: When all physical RAM and swap slots are fully committed, `SwapError::OutOfSwapSlots` is logged and transmitted to the kernel OOM killer service.
