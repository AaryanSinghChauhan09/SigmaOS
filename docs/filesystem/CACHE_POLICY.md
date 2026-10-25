# SigmaOS Block Cache Manager Policy

## Overview

The block cache subsystem (`src/fs/cache.rs`) manages memory-backed page caching for filesystem blocks, balancing I/O performance against kernel memory constraints through observable watermark levels and LRU eviction.

---

## Core Invariants

### 1. Race-Free Eviction and Access
- Cache access updates last-accessed timestamps (`last_accessed_ms`) atomically during `get_page` operations.
- Page insertion (`insert_page`) checks page capacity bounds (`max_pages`). When capacity is reached, the least recently used (LRU) clean or dirty page is selected for eviction.

### 2. Dirty Writeback and Lost-Writeback Prevention
- Modifications mark pages as dirty via `mark_dirty`.
- Page eviction (`evict_page`) checks `is_dirty`. If the page is dirty, dirty data is automatically written back to the underlying block device before freeing the cache frame. This prevents lost writebacks on page eviction.

### 3. Double-Eviction Prevention
- Attempting to evict a page that is not present in the cache table returns `CacheError::NotCached`.
- Page eviction removes the target `(device_id, block_num)` key atomically from the cache map, ensuring concurrent or repeated eviction calls fail cleanly without corrupting memory or data.

### 4. Bounded Cache Metadata
- Cache frame allocations are strictly bounded by `max_pages`.
- Each cache entry stores fixed metadata (device ID, block number, dirty bit, timestamp, pinned ref count), ensuring O(1) space complexity per page frame and preventing unbounded kernel metadata expansion.

### 5. Observable OOM Watermarks
Cache memory usage is monitored across four observable watermark tiers:

| Tier | Utilization Threshold | Behavior & Reclaim Action |
|---|---|---|
| `Low` | < 50% capacity | Normal page allocation and caching |
| `Medium` | 50% - 80% capacity | Background eviction preparation |
| `High` | 80% - 95% capacity | Proactive clean page eviction enabled |
| `OomCritical` | >= 95% capacity | Synchronous dirty page writeback & aggressive LRU reclaim |

---

## Error Specification

| Code | Variant | Description |
|---|---|---|
| `0` | `Success` | Cache operation completed successfully |
| `1` | `NotCached` | Block not found in cache map or already evicted |
| `2` | `CacheFull` | Cache capacity exceeded and LRU reclaim failed |
| `3` | `WritebackFailed` | Underlying block device flush or write failed during eviction |
| `4` | `PinnedBlock` | Page cannot be evicted because active references are pinned |
| `5` | `DirtyDataLossRisk` | Operation would discard unflushed dirty data |
