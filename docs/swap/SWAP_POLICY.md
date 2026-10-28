# Swap Management Policy

## 1. Swap Subsystem Specifications

SigmaOS swap management (`src/swap/core.rs`, `src/swap/enqueue.rs`) governs anonymous page swap-out and swap-in operations:

1. **Race-Free Swap Operations**: Swap-out and swap-in operations synchronize swap slot bitmasks and page state locks to prevent duplicate allocation or concurrent page corruption.
2. **Bounded Swap Metadata**: Swap slot allocation bitmap and priority queue sizes are strictly bounded by configured swap device total sector capacity.
3. **Lost-Swap Prevention**: Written swap slots require synchronous or flushed write verification before page table entries are converted from Present to Swapped state.
4. **Double-Swap Prevention**: Unused or freed swap slots cannot be double-freed or allocated to multiple pages simultaneously.
5. **Observable OOM Enforcement**: Complete exhaustion of physical frames and swap slots transitions the subsystem into an observable OOM state, notifying diagnostics and triggering process termination.
