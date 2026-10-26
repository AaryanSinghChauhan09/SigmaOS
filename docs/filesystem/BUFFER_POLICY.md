# SigmaOS Block Buffer Cache Policy

## Objectives
Govern block buffer allocation, access tracking, LRU eviction, and dirty buffer flushing for block storage devices.

## Invariants & Requirements
1. **LRU Eviction Semantics**:
   - When the buffer pool reaches `max_buffers`, allocating a new block triggers LRU eviction of clean buffers.
   - Locked buffers (`BufferState::Locked` or `InFlight`) CANNOT be evicted; if no evictable buffer exists, allocation fails with `PoolExhausted`.
2. **Dirty Buffer Integrity**:
   - Marking a buffer dirty updates state to `BufferState::Dirty`.
   - Flushing a dirty buffer writes data back to storage and resets state to `BufferState::Clean`.
3. **Exact-Once Lock/Unlock**:
   - Accessing or modifying buffer contents requires locking the buffer head. Attempting concurrent access on a locked buffer returns `BufferLocked`.
4. **Limits & Bounded Allocation**:
   - Buffer size must match configured device `block_size`. Mismatched requests fail with `InvalidBlockSize`.
