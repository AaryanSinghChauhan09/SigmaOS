# Physical Memory Manager Specification

`docs/memory/PHYSICAL_POLICY.md`

## Overview
The Physical Memory Manager (PMM) governs physical frame management in SigmaOS (`src/memory/physical.rs` and `src/memory/frame.rs`). It provides bounded, race-free physical page allocation and deallocation across all kernel and user memory requests.

## Key Invariants

1. **Race-Free Allocation:** Physical frame bitsets and buddy tree nodes are protected by spinlocks or atomic operations. Allocation and deallocation across multicore systems are race-free.
2. **Reserved Region Protection:** Low memory (`0x0..0x100000`), kernel code/data segments, and ACPI memory regions are marked as reserved during boot initialization and can never be allocated or freed.
3. **Double-Free Prevention:** Every physical frame tracks its allocation state in a frame metadata table. Attempting to free an unallocated or already freed frame returns a `PhysicalMemoryError::DoubleFree`.
4. **Typed Frame Exhaustion:** When physical memory is exhausted, the allocator returns `Err(PhysicalMemoryError::Exhausted)` rather than panicking or looping infinitely.
5. **Lost-Wakeup Prevention:** Threads sleeping on frame availability register on a wait queue. When frames are freed or reclaimed, `reclaim_wakeup_notified` flags ensure wakeups are never lost.

## Interface & Data Structures

- `PhysicalMemoryManager`: Core manager tracking total, allocated, reserved, and free frame counts.
- `Frame`: Wrapper type representing a 4096-byte page aligned physical frame.
- `PhysicalMemoryError`:
  - `Exhausted`: No physical frames available.
  - `DoubleFree`: Physical frame was already free.
  - `ReservedRegionViolation`: Target frame belongs to reserved memory.
  - `InvalidFrameNumber`: Frame index out of physical bounds.
