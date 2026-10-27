# Physical Memory Allocation Policy

## 1. Frame Allocator Policy

SigmaOS physical memory allocation (`src/memory/physical.rs`, `src/memory/frame.rs`) adheres to the following strict operational rules:

1. **Race-Free Frame Allocation**: All physical frame allocation and deallocation operations use atomic state bitmasks or spinlock synchronization to guarantee thread-safe frame acquisition.
2. **Defined Frame Reuse Policy**: Freed physical frames are returned to the buddy allocator freelist at their appropriate block order, making them immediately eligible for reallocation without fragmentation leaks.
3. **Typed Exhaustion Handling**: When no physical frames are available, the physical manager returns `PhysicalMemoryError::Exhausted` rather than panicking or looping infinitely.
4. **Reserved Region Protection**: Firmware, ACPI, kernel image, and I/O mapped memory regions are marked as reserved during boot discovery and are permanently excluded from physical frame allocation.
5. **Double-Free Protection**: Each frame tracks its allocation state (`Allocated` vs `Free`). Attempting to free an unallocated frame returns `PhysicalMemoryError::DoubleFree`.
6. **Lost-Wakeup Prevention**: Wait queues associated with memory reclaim (`kswapd` / reclaim threads) use atomic sequence counter updates to ensure no wakeup signal is lost during memory pressure recovery.
