# Physical Memory Manager Policy (`PHYSICAL_POLICY.md`)

## 1. Overview & Objectives
The Physical Memory Manager (`PhysicalMemoryManager` in `src/memory/physical.rs` and `FrameDescriptor`/`Frame` in `src/memory/frame.rs`) manages physical RAM page frames (default 4KiB, 2MiB, 1GiB).

Key Requirements:
- **Race-Free Frame Allocation**: Protected free lists and atomic reference counts (`free_count`, `allocated_count`, `reserved_count`).
- **Defined Frame Reuse Policy**: Returned frames are zeroed/sanitized and pushed back onto the free list.
- **Typed Error Handling**: Returns `FrameAllocationError` (`OutOfMemory`, `DoubleFree`, `ReservedFrame`, `InvalidFrameIndex`, `CapacityExceeded`).
- **Reserved Memory Protection**: Firmware/MMIO ranges are flagged `FrameState::Reserved` and strictly protected against allocation and free.
- **Double-Free Protection**: Attempting to free an unallocated or already free frame returns `FrameAllocationError::DoubleFree`.
- **Lost-Wakeup Prevention**: Atomic flag `reclaim_wakeup_notified` notifies waiting threads in reclaim paths when frames are freed.

---

## 2. Invariants & Rules

1. `free_count + allocated_count + reserved_count == total_frames` at all times.
2. Reserved frames must never enter the free list or be allocated to user/kernel processes.
3. Every call to `allocate_frame()` increments `allocated_count` and decrements `free_count` atomically.
4. Every valid call to `free_frame()` decrements `allocated_count` and increments `free_count` atomically.
5. Attempting to free a frame with `FrameState::Free` returns `Err(FrameAllocationError::DoubleFree)`.
6. Freeing a frame triggers `reclaim_wakeup_notified.store(true, Ordering::SeqCst)` to guarantee lost-wakeup prevention.

---

## 3. Implementation Verification
Verified by unit tests in `tests/memory/physical.rs`:
- `test_physical_memory_manager_basic_allocation`
- `test_physical_memory_manager_double_free_protection`
- `test_physical_memory_manager_exhaustion_error`
- `test_physical_memory_manager_reserved_region_protection`
- `test_reclaim_wakeup_notification`
