# Lock Policy (LOCK_POLICY)

## File Lock Types & Semantics
- Advisory locks (`flock`) support `LOCK_SH` (Shared), `LOCK_EX` (Exclusive), `LOCK_NB` (Non-blocking), and `LOCK_UN` (Unlock).
- Range locks (`fcntl`) support `F_RDLCK` (Read lock), `F_WRLCK` (Write lock), and `F_UNLCK` (Unlock) over offset sub-ranges `[start, start + len)`.

## Conflict & Verification Discipline
- Multiple shared/read locks may overlap on the same file/range.
- An exclusive/write lock conflicts with any overlapping lock from a different owner process (`owner_pid`).
- Lock requests in non-blocking mode return `FsError::ResourceBusy` immediately on conflict.
- When a process closes a descriptor or terminates, all associated range locks held by that process are automatically released.
- Double-unlock requests or unlocks on non-existent locks return success without altering lock table state.
