# Swap Subsystem Policy (`SWAP_POLICY.md`)

## 1. Overview & Objectives
The Swap Subsystem (`SwapManager` in `src/swap/core.rs` and `SwapQueueManager` in `src/swap/enqueue.rs`) provides secondary storage page eviction and restoration.

Key Requirements:
- **Race-Free Swap-Out and Swap-In**: Slot allocation and lookup are protected by Mutex/RefCell locks with atomic slot tracking.
- **Priority Queueing**: Asynchronous swap requests are ordered by `SwapPriority` (`High`, `Normal`, `Low`).
- **Double-Swap Protection**: Swapping out an already swapped virtual address returns `SwapError::AlreadySwapped(virt_addr)`. Swapping in an unswapped address returns `SwapError::NotSwapped(virt_addr)`.
- **Observable OOM Metrics**: When swap slot capacity is exhausted, `swap_out_page` returns `SwapError::OutOfSwapSlots` and sets `oom_triggered.store(true, Ordering::SeqCst)` for system diagnostics.

---

## 2. Invariants & Rules

1. **Slot Uniqueness**: Each swapped virtual page maps to exactly one active swap slot index.
2. **Double-Eviction Prevention**: Attempting `swap_out_page` on a virtual address that already exists in the swap table returns `SwapError::AlreadySwapped`.
3. **Double-In Prevention**: Attempting `swap_in_page` on a non-swapped virtual address returns `SwapError::NotSwapped`.
4. **Queue Ordering**: High priority requests are processed before Normal or Low priority requests in `process_next()`.
5. **OOM Observability**: Slot exhaustion triggers the `oom_triggered` flag, readable via `is_oom_triggered()`.

---

## 3. Implementation Verification
Verified by unit tests in `tests/swap/core.rs`:
- `test_swap_manager_swap_out_in_basic`
- `test_swap_manager_oom_on_exhaustion`
- `test_swap_queue_priority_and_processing`
