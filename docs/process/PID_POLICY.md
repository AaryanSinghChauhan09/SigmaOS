# SigmaOS PID Allocation & Process Table Policy

## Overview

This document specifies the PID lifecycle, process table boundaries, lookup safety, and error semantics for SigmaOS process management.

## Allocation & Reuse Rules

1. **Race-Free PID Allocation:** PID allocation and deallocation operations are protected by atomic bitmasks or synchronized spinlocks, ensuring thread-safe concurrent `fork` and `clone` invocations.
2. **PID Recycling Policy:** A PID is eligible for reuse only after the process has reached the `REAPED` state and its process table entry has been fully cleared. Stale entries cannot be reused before cleanup completes.
3. **PID Capacity Bounds:** Process table capacity is bounded (e.g. max 32,768 or configuration-defined max entries). PID exhaustion returns a typed `PidError::Exhausted` error rather than crashing or overflowing.
4. **Reserved PIDs:**
   - PID 0: Kernel Idle Process.
   - PID 1: System Init / Primary Orphan Reaper.

## Concurrent Lookup & Safety

- PID-to-process lookups must remain valid under concurrent `fork`, `exit`, `wait`, and signal operations.
- Table lookups return strong references (`Arc<ProcessEntry>`) or safe process handles (`PidFd`) to prevent use-after-free or PID race hazards.
