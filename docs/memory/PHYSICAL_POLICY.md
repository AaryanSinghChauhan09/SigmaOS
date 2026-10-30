# Physical Memory Management Policy

## Architectural Rules
1. **Race-Free Frame Allocation**: Physical frame allocation and deallocation are guarded by mutual exclusion locks or atomic bit manipulation primitives.
2. **Reuse Policy**: Frames are zeroed upon deallocation prior to re-entering the available pool to prevent cross-process data leakage.
3. **Typed Frame Exhaustion**: Out-of-memory physical allocation attempts return explicit typed error codes (`FrameAllocationError::Exhausted` or `OutOfMemory`) rather than causing accidental kernel panics.
4. **Reserved Region Protection**: Kernel executable code, initrd, ACPI tables, and low 1MB memory regions are registered as reserved frames during boot discovery and cannot be allocated or freed.
5. **Double-Free Prevention**: Frame deallocation validates frame allocation state in the bitmap/buddy structure; attempting to free an unallocated frame returns a double-free error (`FrameAllocationError::DoubleFree`).
6. **Lost-Wakeup Prevention**: Wait queues associated with frame reclaimers use lock-backed notification mechanisms to ensure threads waiting for available frames are deterministically awakened upon frame release.
