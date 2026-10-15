# SigmaOS Block Buffer Policy

## 1. Overview
The Block Buffer Cache subsystem (`src/fs/buffer.rs`) manages cached disk block buffers across block storage devices in SigmaOS.

## 2. Key Invariants and Operational Rules

### 2.1 Buffer States
Each buffer head transitions between discrete states:
- `Clean`: Buffer contents match disk storage.
- `Dirty`: Buffer modified in memory; requires flushing to disk.
- `Locked`: Buffer currently undergoing active read/write/flush operation.
- `InFlight`: Buffer queued in I/O scheduler for asynchronous transfer.

### 2.2 LRU Eviction and Dirty Flushing
- Buffers are cached up to `max_buffers`.
- Unlocked clean buffers are evicted using Least Recently Used (LRU) policy when allocation is requested.
- `flush_dirty_buffers` writes all dirty, unlocked buffers to the underlying block storage device and transitions them to `Clean`.

### 2.3 Pool Exhaustion and Lock Safety
- If all cached buffers are in `Locked` state and allocation capacity is reached, new allocations fail with `BufferError::PoolExhausted`.
- Operations on locked buffers return `BufferError::BufferLocked(dev_id, block_num)`.
