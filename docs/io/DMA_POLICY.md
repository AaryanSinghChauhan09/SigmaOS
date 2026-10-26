# SigmaOS DMA Management Policy

## Objectives
Ensure safe, coherent, zero-copy Direct Memory Access (DMA) transfers between device controllers and host physical memory.

## Invariants & Requirements
1. **Scatter-Gather Validation**: Every DMA mapping MUST supply a valid, non-empty list of physical scatter-gather elements. Empty lists trigger `InvalidScatterGatherList`.
2. **Double-DMA Protection**: Mapping an already active physical buffer ID without an intermediate unmap cycle MUST fail with `DoubleDmaPrevented`.
3. **Coherence & Direction Discipline**: DMA direction (`ToDevice`, `FromDevice`, `Bidirectional`) dictates cache flush and invalidate barriers before and after hardware execution.
4. **Bounded Queue Depth**: Pending DMA ring descriptors are bounded by `max_queue_depth`. Queue saturation triggers `QueueFull` back-pressure.
5. **Exact-Once Completion**: Unmapping or completing a DMA transfer moves the descriptor to completed records exactly once and releases buffer ownership.
