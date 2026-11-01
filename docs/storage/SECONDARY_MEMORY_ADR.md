# ADR 0004: SigmaOS Secondary Memory Lifecycle and Architecture

## Status
Accepted

## Context
SigmaOS requires a robust, observable, and recoverable secondary memory subsystem to handle block storage,
caching, filesystems, and swap/paging. Historically, storage components existed as isolated models,
lacking unified lifecycle state machine guarantees, bounds on kernel memory queues, and fail-safe recovery
contracts. Without a formal lifecycle and clear boundaries, system hangs, double-completions, lost writebacks,
and filesystem corruption could occur under hardware failure or memory pressure.

## Decision
1. **Authoritative 11-Stage Storage Lifecycle**:
   Enforce a deterministic lifecycle sequence:
   `probe -> identify -> block layer -> cache -> filesystem -> mount -> read/write -> flush -> unmount -> detach -> error/recovery`.
   Every subsystem component must adhere to its stage-specific contracts.

2. **Bounded Block Device Management**:
   Block device request queues must be bounded. Request allocation must be race-free, and completions must
   occur exactly once. Duplicate completions (`BlockError::DoubleCompletion`) and request exhaustion
   (`BlockError::RequestExhaustion`) are explicitly rejected and typed.

3. **Hybrid Page/Buffer Cache with LRU and OOM Observability**:
   Block caching is bounded by maximum page limits. Dirty pages are committed prior to eviction to prevent
   lost updates. Cache pressure is categorized into observable watermarks (`Normal`, `PressureWarn`, `OomCritical`).

4. **Failure-Safe Mount and Transactional Filesystem Core**:
   Filesystem allocations must be checked against hard volume boundaries. Filesystem transactions prevent lost
   updates by validating transaction IDs at commit time. Mount points are tracked in a centralized table
   preventing duplicate mounts and double unmounts.

## Consequences

### Positive
- Predictable and bounded kernel memory utilization under heavy I/O loads.
- Complete observability into request states, cache watermarks, and filesystem health.
- Clear error propagation and fail-safe recovery preventing persistent filesystem corruption.
- Full compliance with `STORAGE_RULES.toml` and pull request verification standards.

### Negative
- Requests exceeding bounded queue depth will fail or require upper-layer retry/throttling.
- Slight metadata overhead to track transaction logs and cache page timestamps.
