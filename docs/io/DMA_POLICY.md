# SigmaOS DMA Management Policy

## Subsystem Overview
The DMA Manager (`src/io/dma.rs`, `src/io/dma_queue.rs`) manages Direct Memory Access buffer allocations, virtual-to-physical address mappings, scatter-gather lists, and cache coherency.

## Key Requirements & Invariants

1. **Race-Free DMA Mapping**: Buffer mapping and unmapping operations are managed atomically using thread-safe state wrappers (`DmaState`).
2. **Double-DMA & Lost-DMA Prevention**: Mapping reuse follows a strict buffer lifecycle (`Unmapped -> Mapping -> Active -> Unmapping -> Unmapped`). Attempting to map an active buffer or double-free a handle returns explicit typed errors.
3. **Bounded Metadata & Alignment**: All DMA allocations enforce physical alignment (e.g., 4096-byte page boundaries for x86_64 / ARM64). Buffer queues enforce maximum capacity constraints (`max_queues`, `max_descriptors`).
4. **OOM & Exhaustion Metrics**: Memory allocation failures return `Err(DmaError::OutOfMemory)`. Queue overflow returns `Err(DmaError::QueueFull)`.
