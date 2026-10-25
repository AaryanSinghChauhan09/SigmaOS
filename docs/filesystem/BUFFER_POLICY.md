# SigmaOS Block Buffer Cache Policy

## Overview
The Block Buffer Cache subsystem governs device block buffer allocation, access, locking, dirty block flushing, and LRU eviction.

## Requirements & Enforcements
- **Buffer State Machine**: Buffers transition between `Clean`, `Dirty`, `Locked`, and `InFlight`.
- **Legal Eviction**: Locked or in-flight buffers cannot be evicted (`BufferError::BufferLocked`).
- **Exhaustion & Pool Limits**: Bounded buffer pool size. Exhaustion returns `BufferError::PoolExhausted`.
- **Dirty Block Flushing**: Dirty buffers are marked in-flight prior to flush and transition to clean upon completion.
- **Access Semantics**: Thread-safe retrieval and LRU access tracking (`access_count`).
