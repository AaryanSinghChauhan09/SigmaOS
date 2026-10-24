# File Locking Policy & Advisory Semantics

## 1. Overview

SigmaOS supports both BSD-style advisory file locks (`flock`) and POSIX byte-range locks (`fcntl`). File lock management is coordinated by `FileLockManager` in `src/fs/flock.rs`.

---

## 2. Lock Types and Conflict Rules

### Lock Modes (`LockType`)

- **Shared Lock (`LockType::Shared` / `LOCK_SH` / `F_RDLCK`):** Multiple processes may hold overlapping shared locks simultaneously for reading.
- **Exclusive Lock (`LockType::Exclusive` / `LOCK_EX` / `F_WRLCK`):** Only one process may hold an exclusive lock for writing; conflicts with both shared and exclusive lock requests.

### Conflict Matrix

| Existing Lock | Requested Shared | Requested Exclusive |
| :--- | :--- | :--- |
| **None** | Granted | Granted |
| **Shared (Same PID)** | Granted | Granted (Upgrade) |
| **Shared (Other PID)** | Granted | Denied (`FsError::ResourceBusy`) |
| **Exclusive (Same PID)** | Granted (Downgrade) | Granted |
| **Exclusive (Other PID)** | Denied (`FsError::ResourceBusy`) | Denied (`FsError::ResourceBusy`) |

---

## 3. Byte-Range Locks & Range Overlaps

1. **Range Locks (`FileRangeLock`):**
   - Specified by `(start, len)`. If `len == 0`, the lock extends to the end of file (`u64::MAX`).
   - Overlap checking determines whether `self.start < other_end && start < self_end`.
2. **Lock Table Invariants:**
   - Applying a lock replaces or updates existing locks held by the same process for the same file range or fd.
   - Closing a file descriptor or process exit automatically triggers lock cleanup (`release_locks_for_fd` or `release_locks_for_process`), ensuring zero orphaned locks.
