# Architecture Decision Record (ADR): VFS File Management Subsystem

## Status
Accepted

## Context
SigmaOS requires an authoritative, safe, observable, and recoverable file management subsystem. Previous experiments established unit-level tests for components like `fd_table`, `flock`, `directory`, and `permission`, but lacked unified VFS lifecycle integration and explicit architecture policy documentation.

## Decision
We establish a hybrid Request-Based and Inode-Based Virtual File System (VFS) architecture in `src/fs/`.

1. **File Descriptor Table**: Process-local bounded descriptor tables (`ProcessFdTable`) managing safe atomic descriptor allocations, `dup`/`dup2` reference sharing, reserved stdio descriptors (`0, 1, 2`), and `FD_CLOEXEC` handling.
2. **File Handle and Lifecycle**: Shared `Arc<FileHandle>` tracking underlying inode references, access mode flags, and atomic file offset seeking (`SeekFrom::Start`, `Current`, `End`).
3. **Inode and Directory Manager**: Explicit `Inode` state machine (`Allocated -> Active -> Unlinked -> Reclaimed`), reference counting (`nlink`), and bounded `Directory` tables with double-unlink and lost-update prevention.
4. **Permission Engine**: POSIX-compatible standard octal bitmask (`0o777`) checking owner/group/other access combined with Linux capability overrides (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
5. **Locking Engine**: Advisory BSD `flock` and POSIX byte-range lock manager (`FileRangeLock`) with overlapping range splitting/merging and double-unlock prevention.

## Consequences
- **Safety**: Single allocation and single release guarantees for file descriptors and inodes prevent double-free bugs and leaks under load.
- **Observability**: All errors return strongly-typed `FsError` enum variants.
- **Limits**: Hard upper bounds on maximum process file descriptors and directory entry capacities prevent OOM panics.
