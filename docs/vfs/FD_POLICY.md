# File Descriptor Management Policy

## Overview
The File Descriptor (FD) manager in SigmaOS handles per-process FD tables (`src/fs/fd_table.rs` and `src/fs/fd.rs`).

## Key Requirements & Guarantees
1. **Race-Free Allocation**: FD numbers are allocated using atomic search/bitmask operations in the process `ProcessFdTable`.
2. **Defined Reuse Policy**: Lowest available FD index is reused after being closed and verified as unreferenced.
3. **Typed Exhaustion Error**: Requesting an FD beyond `MAX_FDS` returns `FsError::TableFull` (mapped to `EMFILE`/`ENFILE`).
4. **Reserved Descriptor Protection**: FDs 0 (stdin), 1 (stdout), and 2 (stderr) are protected against accidental unmapped closure during system execution.
5. **Exact-Once Freeing**: Closing an FD atomically removes it from the table and drops its reference count. Double-close operations return `FsError::InvalidFd` (`EBADF`).
6. **Close-on-Exec (CLOEXEC)**: Descriptor flags explicitly mark `O_CLOEXEC` handles for automatic teardown during process `exec`.
