# SigmaOS PID Allocation and Process Table Policy

## Overview

The Process ID (PID) allocator and Process Table manager in SigmaOS provide a race-free, bounded, and deterministic identity model for process management.

## PID Allocation Policy

1. **Race-Free Operations:** PID allocation and deallocation are guarded by thread-safe atomic operations or process table mutexes.
2. **PID Space Boundaries:**
   - Reserved PIDs: `PID 0` (Idle/Kernel), `PID 1` (Init process).
   - User PIDs: Allocated sequentially within bounded ranges (default `2..32768` or `2..MAX_PID`).
3. **PID Cooldown & Reuse:**
   - PIDs are not immediately reused upon `exit`. A PID is only freed once the process has been explicitly reaped (`REAPED` state).
   - Reused PIDs undergo validation to prevent race conditions with stale process handles or `pidfd` descriptors.
4. **Exhaustion Handling:**
   - If no PID is available within the valid range, `PidAllocator::allocate()` returns a typed `PidError::PidExhausted` error.

## Process Table Policy

1. **Bounded Capacity:** The process table is bounded to prevent kernel heap exhaustion.
2. **Atomic Lookup:** Process lookup by PID (`get_process`) is safe under concurrent `fork`, `exec`, `exit`, and `wait` calls.
3. **Stale Entry Protection:** Process table slots cannot be overwritten or reused until complete cleanup and reaping have occurred.
