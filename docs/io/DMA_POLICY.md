# SigmaOS DMA Manager Policy

## Objectives
- Race-free DMA mapping enqueue/dequeue and scatter-gather validation.
- Coherent CPU cache flush prior to device-bound transfer and CPU cache invalidation after device-to-host transfer.
- Double-DMA prevention and lost-completion prevention.
- Bounded descriptor queues with observable resource limits.

## Policy Rules
1. **Scatter-Gather Validation**: Empty SG lists or zero-length entries return `DmaError::InvalidScatterGatherList`.
2. **Double-DMA Protection**: Re-mapping an active in-flight buffer returns `DmaError::DoubleDmaPrevented`.
3. **Queue Limits**: Maximum active mapped buffers bounded by `max_queue_depth`.
4. **Cache Coherence**: CPU data cache flushes (`flush_cpu_cache`) and invalidations (`invalidate_cpu_cache`) executed around DMA lifecycle events.
