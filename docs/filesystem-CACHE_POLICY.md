# Block Cache Policy

## Overview
The block cache manager (`BlockCacheManager`) provides high-performance cached I/O while guaranteeing data durability and observable OOM metrics.

## Policy Rules
1. **LRU Eviction**: Cache capacity is bounded (`max_pages`). Over-capacity triggers LRU eviction of the least-recently used clean or flushed page.
2. **Lost-Writeback Prevention**: Dirty pages marked for eviction or flush are written back to storage before releasing memory.
3. **Double Eviction Prevention**: Evicting an un-cached or already evicted page returns `CacheError::DoubleEviction` or `CacheError::NotCached`.
4. **OOM Watermarks**:
   - `< 75% capacity`: `WatermarkLevel::Normal`
   - `75% - 89% capacity`: `WatermarkLevel::HighPressure`
   - `>= 90% capacity`: `WatermarkLevel::OomCritical`
