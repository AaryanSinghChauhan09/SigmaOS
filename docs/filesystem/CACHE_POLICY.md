# Block Cache Manager Specification and Policy

## 1. Overview

The Block Cache Manager (`src/fs/cache.rs`) provides in-memory page and block caching between high-level filesystem layers and low-level block devices.

## 2. Cache Page Lifecycle & Eviction

### 2.1 Capacity Bounds and LRU Eviction
- Cache capacity is bounded by `max_pages`.
- Insertion into a full cache triggers automatic Least-Recently-Used (LRU) eviction based on access timestamps (`last_access_ms`).

### 2.2 Dirty Page Writeback & Lost-Writeback Prevention
- Cache pages modified by write operations are marked dirty (`is_dirty = true`).
- Evicting a dirty page invokes underlying block device writeback before freeing the cached page, preventing lost writeback data.
- Flush operations (`flush_dirty_pages()`) synchronously commit all dirty pages to storage.

### 2.3 Double-Eviction Prevention
- Attempting to evict a page that is not currently cached returns `Err(CacheError::NotCached)`.
- Re-evicting pages in `Evicting` or `Evicted` states is rejected.

## 3. OOM Observability and Watermarks

Cache pressure is monitored via `CacheOomMetrics` and `WatermarkLevel`:
- **Normal**: Cache utilization below 80%.
- **HighPressure**: Cache utilization between 80% and 95%. Eviction rate increases.
- **OomCritical**: Cache utilization exceeds 95%. Emergency synchronous page reclamation triggered.
