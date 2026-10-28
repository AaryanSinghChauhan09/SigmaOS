# SigmaOS Filesystem Buffer Cache Policy

## Objectives
- Bounded LRU buffer cache for block-level I/O.
- Safe buffer eviction and dirty page writeback.
- Access semantics, error codes, and exhaustion mitigation.

## Policy Rules
1. **Buffer Representation**: Fixed 4KB or variable block buffer headers.
2. **Eviction Policy**: Least-Recently-Used (LRU) eviction when capacity is reached; dirty buffers written back prior to eviction.
3. **Buffer Exhaustion**: Exhaustion returns `BufferError::CacheFull` or triggers synchronous flush/reclaim.
4. **Lock Ordering**: Page lock -> Buffer header lock -> Block device queue lock.
