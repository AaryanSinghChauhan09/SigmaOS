# File Lock Management Policy

## Overview

The file lock manager (`src/fs/flock.rs` and `src/fs/lock.rs`) implements process advisory locks (`flock`) and byte-range locking (`FileRangeLock`).

## Key Rules & Invariants

1. **Lock Modes**:
   - `Shared` (`LOCK_SH`, `F_RDLCK`): Multiple processes may hold read/shared locks on overlapping byte ranges.
   - `Exclusive` (`LOCK_EX`, `F_WRLCK`): Only one process may hold a write/exclusive lock on a byte range. Exclusive locks conflict with all other shared and exclusive locks on overlapping ranges.
   - `Unlock` (`LOCK_UN`, `F_UNLCK`): Releases locks held on specified byte ranges or descriptors.
2. **Non-Blocking Mode (`LOCK_NB`)**:
   - Acquiring a lock that conflicts with an existing held lock in non-blocking mode immediately fails and returns `FsError::ResourceBusy`.
3. **Range Overlaps**:
   - Range locks evaluate start offsets (`start`) and lengths (`len`). A length of `0` indicates locking through the end of file (`u64::MAX`).
4. **Cleanup & Double-Unlock Safety**:
   - Closing a file descriptor or terminating a process automatically clears all held range locks associated with that process descriptor.
   - Unlocking a region with no active lock succeeds harmlessly (idempotent unlock), preventing double-unlock panics or corruption.

## Testing & Verification

File locking semantics and conflict detection are verified via `tests/filesystem/lock.rs`.
