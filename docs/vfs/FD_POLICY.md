# File Descriptor Policy (FD_POLICY.md)

## Overview
The File Descriptor (FD) manager in SigmaOS provides process-local file descriptor table isolation, race-free allocation, handle tracking, and lifecycle cleanup.

## Requirements & Enforcements
- **Race-Free Allocation**: `ProcessFdTable` allocates the lowest available integer descriptor using internal mutexes / spinlocks.
- **FD Reuse**: When an FD is closed, its slot is cleared and made eligible for subsequent allocation requests.
- **Reserved Descriptors**: FDs 0 (`STDIN_FILENO`), 1 (`STDOUT_FILENO`), and 2 (`STDERR_FILENO`) are reserved upon process table initialization.
- **FD Exhaustion**: When a process reaches its configured maximum descriptor count (`DEFAULT_MAX_FDS = 1024`), requests fail with `FsError::FdTableFull` or `FsError::TooManyOpenFiles`.
- **Double-Free & Lost-Completion Prevention**: `ProcessFdTable::close(fd)` removes the descriptor entry and returns `Arc<FileHandle>`. Reference count drop triggers exact-once cleanup of underlying inode references.
- **Close-on-Exec (`FD_CLOEXEC`)**: Semantics are explicitly tracked via descriptor flags (`FD_CLOEXEC = 1`) and checked during `execve` image replacement to auto-close designated file descriptors.
