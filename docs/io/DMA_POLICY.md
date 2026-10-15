# SigmaOS DMA Management Policy

## 1. Overview
The DMA Manager subsystem (`src/io/dma.rs`, `src/io/dma_queue.rs`) governs physical memory scatter-gather mappings, hardware ring descriptors, cache coherence, and DMA transfer lifetimes.

## 2. Key Invariants and Rules

### 2.1 Queue Depth Limits
- The `DmaManager` operates with a bounded `max_queue_depth`.
- Attempting to map a DMA buffer when the queue is full returns `DmaError::QueueFull`.

### 2.2 Scatter-Gather Validation
- Every DMA mapping request must supply a non-empty list of valid `ScatterGatherElement` structures.
- Empty or invalid scatter-gather lists return `DmaError::InvalidScatterGatherList`.

### 2.3 Double-DMA Mapping Guard
- Re-mapping an already active buffer ID returns `DmaError::DoubleDmaPrevented(buffer_id)`.

### 2.4 Unmapping and Cache Coherence
- Unmapping non-existent buffers returns `DmaError::BufferNotMapped(buffer_id)`.
- Direction-aware cache invalidation and flush barriers (`DmaDirection::ToDevice`, `DmaDirection::FromDevice`, `DmaDirection::Bidirectional`) are enforced upon transfer start and completion.
