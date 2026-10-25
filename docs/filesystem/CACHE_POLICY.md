# Block Cache Policy

## Overview
The block cache manager (`src/fs/cache.rs`) manages in-memory caching of storage blocks to minimize physical I/O latency while guaranteeing data consistency.

## Core Requirements

### 1. LRU Eviction & Race-Free Reuse
- Pages are tracked in an LRU structure.
- Evicting clean pages immediately returns memory to the pool.
- Evicting dirty pages forces an atomic writeback to the underlying block device before freeing.

### 2. Double-Eviction Prevention
- Evicting a block page that is not present in the cache or already evicted returns `CacheError::DoubleEviction` or `CacheError::NotCached`.

### 3. Lost-Writeback Prevention
- Modifications to cached blocks set the `dirty` flag.
- During page eviction or cache sync/flush, dirty pages are flushed to storage using `BlockOrientedDevice::write_block()`.
- Flushes clear the `dirty` flag only upon successful completion.

### 4. Watermarks & OOM Observability
- High and low memory watermarks dictate cache pressure.
- Under high memory pressure, `reclaim_memory()` automatically flushes dirty pages and releases clean pages.
- OOM metrics record total cache hits, misses, evictions, writeback failures, and reclaimed page counts.
