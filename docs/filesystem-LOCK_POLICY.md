# SigmaOS File Lock Subsystem Policy

## Objective

Govern BSD `flock` and POSIX byte-range file locks in `src/fs/flock.rs` and `src/fs/lock.rs`.

## Requirements & Rules

1. **Lock Types & Compatibility**:
   - `Shared` (`LOCK_SH` / `F_RDLCK`): Multiple processes/threads can hold shared read locks concurrently on the same inode or range.
   - `Exclusive` (`LOCK_EX` / `F_WRLCK`): Only one owner can hold an exclusive write lock. Blocks other shared or exclusive requests.
   - `Unlock` (`LOCK_UN` / `F_UNLCK`): Releases held locks.

2. **File Lock Manager (`FileLockManager`)**:
   - Manages process-level `flock` locks per inode.
   - Non-blocking attempts (`LOCK_NB`) return `Err(FsError::ResourceBusy)` if the requested lock conflicts with an existing lock.

3. **Byte-Range Locks (`FileRangeLock`)**:
   - Bounded range lock descriptors specifying `inode_id`, `pid`, `LockType`, `start_offset`, and `length`.
   - Overlapping range check (`overlaps()`) verifies whether range `[s1, s1+l1)` intersects `[s2, s2+l2)`.

4. **Double-Unlock & Lost-Lock Prevention**:
   - Unlocking a region or inode that holds no active lock is safe and idempotent.
   - Releasing or closing a file descriptor automatically cleans up all byte-range and `flock` locks held by that process on the associated inode, preventing lost/orphaned locks.

5. **Metadata Bounds & Safety**:
   - Lock queues and active lock lists are bounded to prevent memory allocation denial-of-service under heavy locking stress.
