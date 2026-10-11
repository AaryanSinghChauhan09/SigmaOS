# File Lock Policy (LOCK_POLICY)

## Subsystem Architecture
SigmaOS file locking is managed by `FileLockManager` (`src/fs/flock.rs` and `src/fs/lock.rs`).

## Lock Types & Semantics
1. **BSD Process Locks (`flock`)**:
   - `LOCK_SH` (`F_RDLCK`): Shared lock. Multiple processes may hold shared locks on the same file concurrently.
   - `LOCK_EX` (`F_WRLCK`): Exclusive lock. Only one process may hold an exclusive lock; conflicts with shared and exclusive locks.
   - `LOCK_UN` (`F_UNLCK`): Unlock. Releases held locks.
   - `LOCK_NB`: Non-blocking flag. If a lock conflict occurs, returns `Err(FsError::ResourceBusy)` immediately instead of blocking.

2. **Byte-Range Locks (`FileRangeLock`)**:
   - Supports region-specific locking defined by `start`, `len`, `lock_type`, and `pid`.
   - Overlapping exclusive ranges trigger `Err(FsError::ResourceBusy)`.

## Concurrency & Cleanup Safety
- **Double-Unlock Prevention**: Unlocking a descriptor that is not locked returns `Ok(())` (idempotent release).
- **Process Teardown**: When a process closes a file descriptor or terminates, `FileLockManager::release_locks_for_pid` releases all associated locks automatically, preventing stale lock deadlocks.
- **OOM & Bounded Storage**: Lock tables operate with pre-allocated vectors to prevent kernel memory exhaustion under heavy locking stress.
