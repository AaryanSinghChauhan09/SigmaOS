# SigmaOS DMA Manager Policy

## 1. Subsystem Architecture
The DMA Manager (`src/io/dma.rs` and `src/io/dma_queue.rs`) manages direct memory access buffer allocations, scatter-gather lists, memory coherency synchronization, and descriptor queues.

## 2. Key Rules & Guarantees

1. **Race-Free DMA Allocation**:
   - DMA descriptor allocation and deallocation operate under strict synchronization.
   - Physical address bounds are validated against 32-bit / 64-bit hardware DMA capabilities.

2. **Defined DMA Reuse Policy**:
   - Released DMA descriptors (`DmaDescriptor`) are sanitized (zeroed or invalidated) before returning to the pool.
   - Double-free or reuse of active DMA descriptors is explicitly prevented.

3. **Bounded Metadata Limits**:
   - The DMA manager maintains a maximum capacity of active DMA channels and scatter-gather lists (`MAX_DMA_DESCRIPTORS = 1024`).

4. **Lost-DMA Prevention**:
   - Active DMA operations are tracked with hardware status flags and timeout timers.
   - Stalled transfers trigger driver recovery and DMA queue flushing.

5. **Double-DMA Prevention**:
   - Mapping an already active buffer or submitting duplicate DMA transfer requests is rejected with `DmaError::DescriptorInUse`.

6. **OOM & Memory Isolation**:
   - Out-of-descriptor conditions yield `DmaError::OutOfDescriptors` without crashing the kernel.
   - Physical memory addresses are checked to prevent DMA operations into kernel code/stack regions.
