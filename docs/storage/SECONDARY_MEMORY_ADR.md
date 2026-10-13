# ADR 0002: Secondary Memory Subsystem Lifecycle & Architecture

## Status
Accepted

## Context
SigmaOS requires a safe, observable, and recoverable secondary storage lifecycle to prevent data corruption, resource leaks, lost-update races, and double-unmount failures across persistent storage devices, block I/O drivers, page caches, and filesystems.

## Decision

We define an authoritative 11-stage secondary storage lifecycle state machine:

```text
probe -> identify -> block layer -> cache -> filesystem -> mount ->
read/write -> flush -> unmount -> detach -> error/recovery
```

### Stage Requirements

| Stage | Required Behavior |
|---|---|
| **Probe** | Device discovered, hardware signatures checked, capabilities reported. |
| **Identify** | Geometry, sector size, queue depth limits, and wear-leveling state established. |
| **Block Layer** | Requests enqueued/dequeued race-free, bounded allocation enforced, completed exactly once. |
| **Cache** | LRU page eviction with automatic dirty writebacks and observable OOM watermarks. |
| **Filesystem** | Core filesystem structures initialized with transactional logging and capacity bounds. |
| **Mount** | Mount entry attached safely with rollback on failure; double-unmount prevented. |
| **Read / Write** | Operations clamped within boundaries; concurrent mutations guarded against lost updates. |
| **Flush** | Durability semantics explicit (`BlockOpCode::Flush`); dirty cache pages synchronized to disk. |
| **Unmount** | File references and buffers released exactly once; state set to `Unmounted`. |
| **Detach** | Device safely removed (`BlockError::DeviceDetached`); pending requests failed and resources freed. |
| **Error / Recovery** | Fault injection or hardware failure triggers recovery path without corruption or undefined behavior. |

## Consequences

1. **Typed Error Guarantee**: All failure modes return explicit typed errors (`BlockError`, `CacheError`, `FsCoreError`, `MountError`).
2. **Deterministic Resource Management**: Bounded request queues and cache pages eliminate kernel memory leaks under heavy I/O stress.
3. **Observability**: Storage metrics expose active requests, page cache watermarks, and mount states for health monitoring and diagnostics.
