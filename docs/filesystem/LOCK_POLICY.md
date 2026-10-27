# File Lock Policy

## Overview
SigmaOS file lock manager (`src/fs/flock.rs` / `src/fs/lock.rs`) enforces shared (`LOCK_SH`), exclusive (`LOCK_EX`), and unlock (`LOCK_UN`) semantics.

## Specifications
1. **Race-Free Enqueue/Dequeue**: Lock table modifications are protected against concurrent modification races.
2. **Lock Reuse & Lifetime**: Locks are tied to process file descriptors and inode handles; closing all descriptors associated with a lock releases it automatically.
3. **Lost-Lock Prevention**: Pending lock requests queued on blocking locks are cancelled cleanly if the waiting process receives an interrupting signal or dies.
4. **Double-Unlock Prevention**: Unlocking a file descriptor that does not hold a lock succeeds idempotently without corrupting lock table state.
5. **Bounded Metadata**: The maximum active locks per inode and per process are capped to guarantee lock table memory safety.
