# Block Cache Manager Policy

## Overview
The Block Cache Manager (`BlockCacheManager`) in `src/fs/cache.rs` provides cached page management, writeback queueing, and OOM pressure watermarks for block I/O operations.

## Requirements & Rules

1. **Race-Free Eviction & LRU Selection**:
   - Cache page eviction selects the Least Recently Used (LRU) block based on `last_access_ms`.
   - Double-eviction attempts return `CacheError::DoubleEviction`.

2. **Lost-Writeback Prevention**:
   - When a dirty page is evicted, its data is synchronized to persistent storage before buffer removal.
   - Flush operations (`flush_writeback_queue`) write dirty blocks to disk and mark page state as `Active`.

3. **Observable OOM Watermarks**:
   - Cache capacity is bounded by `max_pages`.
   - Watermarks automatically transition:
     - `Normal`: Page count < 75% max pages.
     - `HighPressure`: Page count >= 75% max pages.
     - `OomCritical`: Page count >= 90% max pages.
   - Metrics (`CacheOomMetrics`) track total pages, dirty pages, hits, misses, evictions, and reclaimed bytes.
