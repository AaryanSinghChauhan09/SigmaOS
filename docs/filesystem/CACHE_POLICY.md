# SigmaOS Block Cache Policy

## Overview
The block cache manager (`src/fs/cache.rs`) maintains in-memory copies of disk blocks and pages, managing dirty writebacks and cache eviction under memory pressure.

## Requirements

1. **Race-Free Eviction**: Eviction selects clean pages first under LRU policy. Evicting dirty pages forces a synchronous writeback before eviction.
2. **Double-Eviction Prevention**: Attempting to evict an already evicted page returns `CacheError::DoubleEviction`.
3. **Lost-Writeback Prevention**: Pages undergoing flush transition to `CachePageState::Flushing` to prevent concurrent write modifications or lost updates.
4. **OOM Observability**: Cache OOM metrics (`CacheOomMetrics`) track active pages, dirty page counts, hit/miss ratios, total evictions, and total reclaimed bytes.
