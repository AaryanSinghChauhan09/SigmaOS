# Swap Manager Specification

`docs/swap/SWAP_POLICY.md`

## Overview
The Swap Subsystem (`src/swap/core.rs` and `src/swap/enqueue.rs`) provides anonymous page swapping to disk or zram/zswap compressed memory backing stores in SigmaOS.

## Specification

### Race-Free Swap Operations
- Swap slot allocation and deallocation use atomic bitsets protected by `SwapManager` spinlocks.
- Concurrent swap-out and swap-in operations on the same slot are synchronized to prevent data corruption or race conditions.

### Priority Queueing
- Swap requests are enqueued into a priority-aware manager (`SwapQueueManager`) supporting `SwapPriority::High`, `SwapPriority::Normal`, and `SwapPriority::Low`.
- Critical kernel swap requests (e.g., active user stack preservation) take precedence over background process swap-out requests.

### Lost-Swap & Double-Swap Prevention
- Swap slots track validity states (`Used`, `Free`, `PendingIO`).
- Freeing an unallocated or already freed swap slot returns `SwapError::DoubleSwap` or `SwapError::InvalidSlot`.
- Failed I/O operations maintain slot validity until retry or explicit abort to prevent lost swap data.

### Observable OOM Metrics
- Swap slot utilization, queue lengths, and exhaustion events emit structured diagnostics.
- When swap slot capacity is exhausted during swap-out under memory pressure, `SwapManager` returns `SwapError::OutOfSwap`, signaling the OOM killer to terminate a candidate process.
