# SigmaOS File Lock Manager Policy

## Overview

The file lock manager (`FileLockManager`, `FileRangeLock`) provides BSD `flock` whole-file locking and POSIX byte-range file locking across open file handles.

## Key Policy Directives

1. **Lock Types**:
   - Shared Locks (`LOCK_SH` / `F_RDLCK`): Allow concurrent readers.
   - Exclusive Locks (`LOCK_EX` / `F_WRLCK`): Block all other readers and writers.
   - Unlock (`LOCK_UN` / `F_UNLCK`): Releases previously acquired lock.
2. **Race-Free Lock Enqueue & Conflicts**:
   - `flock()` checks active file locks. Conflicting exclusive requests return `FsError::ResourceBusy` when non-blocking (`LOCK_NB`) is specified.
   - POSIX range locking (`FileRangeLock`) evaluates offset ranges (`start` to `end`). Overlapping ranges with conflicting lock types are rejected.
3. **Double-Unlock Prevention**: Unlocking a range or descriptor that holds no active lock is a safe no-op returning `Ok(())`.
4. **Lost-Lock Prevention & Clean Closure**:
   - Process termination or descriptor closure automatically releases all locks associated with the closed process/handle, preventing orphaned locks.
