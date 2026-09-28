# ⚡ Bolt's Performance Journal

## 2026-03-31 - Zero-Byte Linear Scans in Fixed Buffer Arrays
**Learning:** Performing unaligned linear scans over large fixed byte buffers in hot loop paths introduces unnecessary L1/L2 cache misses and CPU cycle bloat.
**Action:** Always use SIMD-vectorized iterators or 64-bit aligned memory chunks for payload inspection in `SovereignRingBuffer` and `SovereignIoUring`.

## 2026-04-01 - Thread-Local Buffer Reuse in Hot Allocation Loops
**Learning:** Repeatedly allocating and freeing temporary heap buffers in high-frequency execution loops (such as eBPF instruction verification and syscall dispatching) creates significant memory allocator contention and GC pressure.
**Action:** Reuse thread-local scratch buffers (`thread_local!`) in hot path verification loops to minimize heap allocation overhead and improve cache locality.
