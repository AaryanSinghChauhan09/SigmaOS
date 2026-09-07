# SigmaOS AI Agent Basic File System Management Guidelines

## 1. Overview
SigmaOS implements POSIX and BSD Virtual File System (VFS) abstractions managed autonomously or interactively by AI agents (such as `BasicFilesystemGovernor`, `VfsInodeLookupEngine`, and `FileDescriptorTableManager`). These guidelines define VFS directory and inode lookups, standard POSIX file I/O operations (`open`, `read`, `write`, `close`, `lseek`, `stat`), file descriptor tables, and basic file system mounts.

## 2. Core Basic File System Management Principles

### 2.1 VFS Inodes & Dentry Lookup Cache
- **VFS Inode Table**: Every file and directory is represented by a unique VFS Inode (`Inode` struct in `src/kernel/vfs/vfs.rs`) containing file metadata, file size, permissions (`mode_t`), owner UID/GID, and block pointers.
- **Directory Entry Cache (`dcache`)**: Frequent path lookups (e.g. `/usr/bin/python3`) hit the in-memory `dcache` tree for $O(1)$ path-to-inode resolution.

### 2.2 POSIX File System System Calls
- **File Descriptor Tables**: Every process PCB maintains a per-process file descriptor table (`fd_table`).
- **Atomic Operations**: File creation (`openat` with `O_CREAT | O_EXCL`) and atomic file replacement (`renameat2` with `RENAME_EXCHANGE`) prevent TOCTOU file system race conditions.

### 2.3 Basic File System Mounts
- **Mount Tree**: Root (`/`), `/proc`, `/sys`, `/dev`, `/tmp`, and `/home` mount points are managed via `SovereignMountManager`.
- **Mount Options**: Standard mount flags (`MS_NODEV`, `MS_NOEXEC`, `MS_NOSUID`, `MS_READONLY`) isolate system partitions from unauthorized script execution.

---
*Maintained by the SigmaOS Storage & VFS Steering Committee.*
