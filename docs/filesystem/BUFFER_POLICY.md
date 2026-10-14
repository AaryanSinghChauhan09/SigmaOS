# SigmaOS Buffer Management Policy

## Subsystem Overview
The Buffer Cache Manager (`src/fs/buffer.rs`) manages in-memory block buffers for file systems and raw block devices.

## Key Requirements & Invariants

1. **Buffer Set Representation**: Buffers are tracked in fixed-size block pools identified by device ID and block number.
2. **Access & Eviction Semantics**: LRU (Least Recently Used) cache eviction policy. Dirty buffers are locked and flushed before eviction.
3. **Buffer Exhaustion**: When capacity is reached, clean buffers are evicted. If all buffers are locked/dirty, allocation returns `Err(BufferError::CacheFull)`.
4. **Error Codes**:
   - `CacheFull`: Cache memory capacity reached.
   - `BlockLocked`: Block currently pinned by concurrent I/O.
   - `IoError`: Device read/write failure during flush/fill.
   - `InvalidBlock`: Block number out of physical device bounds.
