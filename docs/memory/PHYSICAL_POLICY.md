# Physical Memory Manager Policy

## Scope

This document specifies physical memory frame allocation, tracking, reservation protection, and double-free guard policies for SigmaOS.

## Principles

1. **Race-Free Frame Allocation**: All physical frame allocations are guarded by spinlocks/mutexes and atomic frame state flags.
2. **Reserved Memory Protection**: Physical RAM regions marked as BIOS/ACPI/MMIO reserved cannot be allocated to user processes or kernel heaps.
3. **Double-Free Guarding**: Releasing a frame that is already marked as `Free` returns a explicit `FrameAllocationError::DoubleFree` error and prevents memory metadata corruption.
4. **Typed Frame Exhaustion**: When all non-reserved physical frames are allocated, the allocator returns `FrameAllocationError::OutOfMemory` to trigger page reclaim or OOM killer action.
5. **Lost-Wakeup Prevention**: Reclaim wait queues notify blocked allocation threads upon frame release.
