# SigmaOS Block Cache Policy

## Overview

The block cache subsystem (`src/fs/cache.rs`) provides in-memory caching for block device sectors and page frames. It reduces physical I/O latency, coordinates buffer coherence, and manages page writebacks to secondary storage.

## Cache Page Lifecycle

Cache pages transition through strict state boundaries:

```text
Uncached ---> Allocate ---> Clean ---> Read/Write Modify ---> Dirty ---> Flush/Writeback ---> Evict ---> Uncached
```

### Page States

- **Clean**: Page contents are synchronized with physical block storage.
- **Dirty**: Page contents have been modified in memory and require writeback before eviction.
- **Flushing**: Writeback I/O is currently in-flight to the block device.
- **Evicting**: Page is being unmapped and returned to the free pool.

## Eviction & LRU Policy

1. **LRU Ordering**: Pages are tracked in a Least Recently Used (LRU) list. Cache hits update the access timestamp and move the page to the head of the LRU list.
2. **Double Eviction Prevention**: An atomic state check verifies that a page is in `Clean` state and not currently referenced before eviction. Attempting to evict an already evicted or active page yields `CacheError::DoubleEviction`.
3. **Lost-Writeback Prevention**: Dirty pages cannot be evicted directly. Eviction requests on dirty pages automatically schedule background writebacks (`sync_page`). The page remains in cache until writeback completes successfully.

## Memory Bounding & OOM Metrics

To prevent unbounded cache growth from starving the kernel or applications:
- **Maximum Pages Boundary**: Total cache pages cannot exceed the declared `max_pages` limit.
- **Watermark Thresholds**:
  - High Watermark (80% capacity): Triggers asynchronous background writeback and eviction of clean pages.
  - Critical Watermark (95% capacity): Forces synchronous eviction for incoming cache allocation requests.
- **OOM Observability**: The cache manager exposes `CacheOomMetrics`:
  - `total_pages`: Total allocated cache pages.
  - `max_pages`: Maximum permitted cache pages.
  - `dirty_pages`: Number of pending dirty pages awaiting flush.
  - `eviction_count`: Total evicted pages since boot.
  - `cache_hit_ratio`: Ratio of cache hits vs total lookup requests.

## Error Policy

Cache operations return typed `CacheError` variants:
- `CacheError::NotCached`: Block not present in cache.
- `CacheError::CacheFull`: Cache reached capacity and eviction failed to free pages.
- `CacheError::DoubleEviction`: Page eviction race detected and prevented.
- `CacheError::WritebackFailed`: Physical I/O writeback error.
- `CacheError::InvalidPage`: Invalid page parameter or memory handle.
