# SigmaOS PID Allocation & Process Table Policy

## Objectives
Ensure PID allocation, lookup, reuse, and process table operations are race-free, bounded, and deterministic under high concurrency.

## PID Allocation Policy

1. **Safety & Concurrency**:
   - PID allocation uses internal atomic tracking or bounded lock guards to prevent race conditions during concurrent `fork`/`clone` calls.
   - PIDs are allocated within a strict numerical range (e.g. `100` to `32768`, with `PID 1` reserved for `init`).

2. **Reuse Strategy**:
   - Releasing a PID occurs strictly after process reaping is complete (`ZOMBIE` -> `REAPED`).
   - Recently freed PIDs are recycled via round-robin or freelist strategies only after verifying stale references are purged.

3. **Exhaustion Handling**:
   - When all table slots or PIDs within the allowed range are allocated, `allocate()` returns a typed error (`PidError::Exhausted`).
   - Callers (`fork`/`spawn`) convert `PidError::Exhausted` into a POSIX-compliant error code (e.g., `EAGAIN`).

## Process Table Policy

1. **Bounded Capacity**:
   - The process table size is strictly capped at initialization to prevent unbounded kernel memory allocation.
   - Attempts to insert new entries beyond capacity return `ProcessTableError::TableFull`.

2. **Safe Concurrent Lookup**:
   - Process lookup by PID returns thread-safe reference handles (`Arc<ProcessEntry>`).
   - Stale entries cannot be accessed or reused before cleanup completes.
