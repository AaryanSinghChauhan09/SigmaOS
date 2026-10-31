# SigmaOS Block Cache Policy

## Overview
This document specifies the architecture, invariants, and eviction policies of the SigmaOS Block Cache Manager subsystem implemented in `src/fs/cache.rs`.

## Requirements & Invariants

### 1. Race-Free Cache Eviction
- Cache page lookups, insertions, and evictions are executed atomically under safe concurrency controls.
- Pages maintain lifecycle states: `Active -> Evicting -> Evicted` (or `Flushing -> Active`).
- Only clean pages in `Active` state or successfully flushed dirty pages may transition to `Evicted`.

### 2. Cache Reuse Policy
- Cache space is governed by Least Recently Used (LRU) tracking with access timestamps and frequency counters.
- When new blocks are inserted into a full cache, the page with the oldest access timestamp is selected for eviction.

### 3. Bounded Cache Metadata
- The maximum number of cached pages is fixed upon instantiation (`BlockCacheManager::new(max_pages)`).
- Metadata tables (page indexes, hash tables, LRU queues) occupy bounded memory proportional to `max_pages`.

### 4. Lost-Writeback Prevention
- If an LRU page selected for eviction is marked dirty (`is_dirty == true`), eviction automatically triggers a synchronous or asynchronous writeback to backing storage prior to releasing or overwriting the page buffer.
- Eviction does not complete until the backing store acknowledges data durability.

### 5. Double-Eviction Prevention
- A page undergoing eviction is removed from the active cache lookup table.
- Subsequent calls to `evict_page(device_id, block_num)` for an already evicted page return `Err(CacheError::NotCached)` or `Err(CacheError::DoubleEviction)`.

### 6. Observable OOM and Watermark Metrics
- `BlockCacheManager` tracks real-time memory metrics (`CacheOomMetrics`):
  - `total_pages`: Total pages currently allocated in cache.
  - `max_pages`: Hard limit of pages allowed.
  - `dirty_pages`: Number of modified pages awaiting writeback.
  - `evictions_count`: Cumulative page evictions.
  - `watermark_level`: Current pressure level (`Normal`, `HighPressure`, `OomCritical`).
- When page allocation exceeds 90% of capacity, pressure escalates to `OomCritical`, triggering aggressive writeback and blocking speculative allocations.
