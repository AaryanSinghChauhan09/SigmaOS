# Block Buffer Cache Policy

## Overview
The Block Buffer Cache manages in-memory cached storage blocks across multiple devices with clean, dirty, locked, and in-flight states.

## Specifications
- **Buffer Set Representation**: Map keyed by `(device_id, block_number)`.
- **Access Semantics**: Re-reads update access counters and LRU position.
- **Eviction Strategy**: Least Recently Used (LRU) eviction of clean buffers when pool is full.
- **Locked Buffers**: Locked buffers cannot be evicted or marked dirty; attempts return `BufferError::BufferLocked`.
- **Exhaustion**: When all buffers are locked or dirty during allocation, returns `BufferError::PoolExhausted`.
