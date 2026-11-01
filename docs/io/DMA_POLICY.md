# SigmaOS DMA Management Policy

## Objective

Govern physical memory address mapping, scatter-gather lists, and cache coherence for high-speed device DMA without data corruption, lost completions, or double-mapping bugs.

## Requirements and Guarantees

### 1. Race-Free Mapping and Unmapping
- Descriptor allocations and lookups operate under thread-safe data structures.
- Completion and unmapping verify ownership before freeing resources.

### 2. DMA Descriptor Reuse Policy
- Descriptors are assigned unique monotonic 64-bit IDs.
- Completed transfers are tracked in history buffers to prevent double-completion or re-entry races.

### 3. Bounded DMA Metadata
- Queue depth is bounded by `max_queue_depth`.
- Scatter-gather element counts per transaction cannot exceed architecture limits (e.g. 1024 chunks).

### 4. Lost-DMA Prevention
- Mapped buffers are tracked until explicit completion or timeout abort.
- Device timeout governors detect non-responsive DMA engines.

### 5. Double-DMA Prevention
- Attempting to map an active `buffer_id` that is already in-flight returns `DmaError::DoubleDmaPrevented`.

### 6. Observable OOM Enforcement
- Queue overflow returns `DmaError::QueueFull`.
- Invalid or empty scatter-gather lists return `DmaError::InvalidScatterGatherList`.
- DMA pressure is observable in subsystem telemetry.
