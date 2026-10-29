# Physical Memory Manager Policy (PHYSICAL_POLICY.md)

## Requirements & Standards

### 1. Concurrency & Race-Free Allocation
- Physical frame allocations and frees must be atomic and race-free.
- Concurrency control is implemented via spinlocks or lock-free bitsets.
- Allocation in interrupt context is strictly bounded to pre-reserved NonPaged pools.

### 2. Frame Reuse & Double-Free Prevention
- Frame tracking maintains explicit allocation bits and refcounts.
- Attempting to free an unallocated frame or double-free a frame returns `MemoryError::DoubleFree` or `MemoryError::InvalidFrame`.
- Allocated frames are zeroed or redzone-sanitized prior to reuse.

### 3. Exhaustion & Bounded Capacity
- System frame capacity is fixed at boot based on firmware e820/UEFI memory maps.
- Frame exhaustion returns `Err(MemoryError::OutOfMemory)`.
- Physical allocators must never trigger recursive allocations or panics when depleted.

### 4. Reserved Memory Protection
- Reserved physical regions (ACPI tables, MMIO ranges, UEFI runtime services, kernel image text/data) are marked permanently unavailable during initialization.
- Reclaiming or reallocating reserved physical ranges is explicitly rejected.

### 5. Lost-Wakeup Prevention
- Thread wait queues waiting for available frames (e.g., during memory reclaim) use atomic notification state machines to guarantee no wakeups are missed when frames are freed.
