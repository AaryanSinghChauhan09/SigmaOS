# Block Cache Policy Specification

## Cache Lifecycle & Eviction Guarantees

1. **Race-Free Eviction**: Buffer/page cache eviction uses Least Recently Used (LRU) tracking guarded by thread-safe synchronization.
2. **Dirty Page Writeback**: Any cache page marked dirty (`is_dirty == true`) must be flushed to the underlying block device before eviction. Lost writebacks are strictly prohibited.
3. **Double-Eviction Prevention**: Attempting to evict an unmapped or already evicted page returns `CacheError::DoubleEviction` or `CacheError::NotCached`.
4. **Bounded Metadata & OOM Observability**: Cache pages are subject to a maximum capacity threshold. Cache capacity metrics (`CacheOomMetrics`) measure total page count, dirty page count, and eviction events for OOM observability.
