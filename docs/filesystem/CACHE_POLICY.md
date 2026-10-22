# Block Cache Policy

## 1. Scope and Overview

The Block Cache Manager (`src/fs/cache.rs`) manages memory-backed caching of secondary storage blocks, maintaining page cache coherence, Least Recently Used (LRU) eviction, dirty writebacks, and memory pressure observability.

## 2. Page Lifecycle State Machine

Cache pages (`CachePage`) transition through 4 explicit operational states:

```text
Active -> Flushing -> Evicting -> Evicted
```

### State Definitions
- **Active**: Page contains valid block data, accessible for reads and writes.
- **Flushing**: Dirty page is actively undergoing I/O writeback to persistent block storage.
- **Evicting**: Clean or flushed page is being unmapped and removed from the cache registry.
- **Evicted**: Page buffer returned to kernel memory pool.

## 3. LRU Eviction & Writeback Policy

1. **LRU Selection**: When the total cache page count reaches `max_pages`, inserting a new block triggers LRU page selection based on `last_access_ms` and `access_count`.
2. **Dirty Page Writeback**: If the selected LRU page is dirty (`is_dirty == true`), the block cache manager invokes synchronous writeback to persistent storage before unmapping.
3. **Double Eviction Protection**: Evicting a page that is already evicted or not cached returns `Err(CacheError::NotCached)` or `Err(CacheError::DoubleEviction)`.

## 4. Watermark Monitoring & OOM Metrics

The cache manager calculates system pressure across three watermark levels based on cache occupancy percentage:

- **Normal (< 80%)**: Standard cache allocation and reads.
- **HighPressure (80% - 95%)**: Asynchronous background flush worker threads activated.
- **OomCritical (> 95%)**: Immediate synchronous eviction of clean pages and suppression of non-essential read-ahead.

Real-time cache metrics (`CacheOomMetrics`) record:
- `total_pages`, `max_pages`, `dirty_pages`
- `cache_hits`, `cache_misses`
- `eviction_count`, `reclaimed_bytes`
