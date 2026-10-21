# SigmaOS File Lock Policy (LOCK_POLICY)

## Core Objectives

- **Race-Free Lock Management:** Range locks and full-file advisory/mandatory locks are acquired and released without race conditions or lock table corruption.
- **Lock Overlap Resolution:** Adjacent and overlapping locks from the same process/owner are automatically merged; overlapping write locks from distinct owners are rejected with `FsError::ResourceBusy`.
- **Lock Metadata Bounds:** Per-file range lock tables enforce hard maximum entry limits (`MAX_LOCKS_PER_INODE = 256`). Exceeding this limit returns `FsError::OutOfMemory`.
- **Double-Unlock Prevention:** Releasing an unheld lock range succeeds idempotently without error or table corruption.
- **Automatic Process Cleanup:** Closing all file descriptors associated with an inode automatically releases all range locks held by that process on that inode.

---

## Lock Types & Conflict Rules

| Existing Lock | Requested Shared (Read) | Requested Exclusive (Write) |
|---|---|---|
| **None** | Granted | Granted |
| **Shared (Read)** | Granted | Denied (`FsError::ResourceBusy`) |
| **Exclusive (Write)** | Denied (`FsError::ResourceBusy`) | Denied (`FsError::ResourceBusy`) |

Note: A single process owner may upgrade a held shared lock to exclusive or downgrade an exclusive lock to shared.

---

## Lock Ranges & Operations

- **Full-File Lock:** Range `0` to `u64::MAX`.
- **Byte-Range Lock:** Range `start_offset` to `end_offset`.
- **`flock(fd, operation)`:** Advisory file locking (`LOCK_SH`, `LOCK_EX`, `LOCK_UN`, `LOCK_NB`).
- **`fcntl(fd, F_SETLK/F_SETLKW, lock_struct)`:** POSIX byte-range record locking.
