# Cache Memory Architecture, LRU Eviction & Package Cache Rules for AI Agents

## Key Invalidation Invariant
- SovereignCacheEngine::set MUST purge pre-existing entries with matching keys via self.entries.retain(|e| e.key != key) before inserting new values
- Implement proper cache eviction policies (LRU, LFU, ARC)
- Use atomic operations for thread-safe cache access

## Cache Architecture
- Implement CPU cache line alignment (64-byte)
- Use lock-free structures for hot cache paths
- Implement cache coherency protocols

## Design Patterns
- **Strategy**: Pluggable eviction policies
- **Observer**: Cache miss/miss notifications
- **Decorator**: Cache layers on existing storage
