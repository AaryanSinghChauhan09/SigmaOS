# SigmaOS Buffer Cache Policy

## Objective

Manage block buffer caching between filesystems, page caches, and underlying block devices to ensure low latency, cache consistency, race-free LRU eviction, and deterministic memory limits.

## Requirements and Guarantees

### 1. Buffer Set Representation
- Keyed by `(dev_id, block_num)`.
- Metadata stored in `BufferHead` with size, access count, and buffer state (`Clean`, `Dirty`, `Locked`, `InFlight`).

### 2. Buffer Set and Update Semantics
- Accessing an existing block increments access counters and brings it to the front of LRU order.
- Updates to data mark the buffer `Dirty` until flushed to storage.

### 3. Access Semantics and Concurrency
- `Locked` buffers cannot be concurrently mutated or evicted.
- Reads while buffer is `InFlight` wait or return typed errors.

### 4. Error Semantics and Codes
- `BufferError::PoolExhausted`: Maximum cache pool capacity reached and all existing buffers are dirty/locked.
- `BufferError::BufferLocked(dev_id, block_num)`: Attempted operation on locked buffer.
- `BufferError::NotFound(dev_id, block_num)`: Requested buffer is not cached.

### 5. Limits and Eviction
- Pool size is bounded by `capacity`.
- Eviction selects the least recently accessed `Clean` buffer.
- If all candidates are `Dirty`, flush must be triggered before eviction succeeds.

### 6. NUMA Behavior
- Buffer caches are partitioned per NUMA node or allocate physical blocks from local node memory to minimize cross-node memory bus penalties.

### 7. Behavior Under Buffer Exhaustion
- Returns `BufferError::PoolExhausted`.
- Triggers background flush worker threads to write out dirty blocks and free clean pages.
