# Swap Subsystem Policy

## Core Semantics
1. **Race-Free Swap-Out / Swap-In**: Swap slot allocation, disk/zswap page writing, and page table swap entry installation are synchronized using lock-backed slot bitmasks and page state locks.
2. **Bounded Swap Metadata**: Swap space is partitioned into fixed-size 4 KiB slots with a bounded bitmap and slot count (`MAX_SWAP_SLOTS`).
3. **Lost-Swap Prevention**: Swapping in a page retains swap slot contents until the page fault is fully resolved and dirty state is evaluated, ensuring data is not lost during transient failures.
4. **Double-Swap Prevention**: Attempting to swap out a page that is already swapped out or in transit is prevented via atomic page flags (`PG_swapbacked`, `PG_swapped`).
5. **Observable OOM Metrics**: When swap space and physical RAM are both exhausted, the swap manager emits structured telemetry events (`SwapEvent::Exhausted`) and notifies the OOM killer.
