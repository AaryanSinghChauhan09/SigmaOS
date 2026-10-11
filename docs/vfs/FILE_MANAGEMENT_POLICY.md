# SigmaOS File Management Policy

## Architectural Classification
- **VFS Class**: Hybrid Request/Inode-based VFS model. Requests are validated through permission and lookup layers before resolving to active Inode references and underlying filesystem operations.
- **File Descriptor Class**: Hybrid File Descriptor Table and File Handle architecture (`ProcessFdTable` mapping integer `fd` to shared `FileHandle` objects referencing `Inode` instances).
- **Inode Class**: Inode-based lifecycle management (`Inode` and `InodeManager`) with explicit reference counting (`ref_count`), link tracking (`nlink`), and atomic mutation state (`InodeState`).

## File Lifecycle Contract
SigmaOS enforces one authoritative lifecycle state machine for all storage operations:
`open -> read/write -> seek -> lock -> metadata -> close -> cleanup`

| State | Required Behavior |
|---|---|
| Open | Race-free allocation of file descriptor in `ProcessFdTable`, path resolution via `Directory`, permission validation via `PermissionEngine`, and `FileHandle` instantiation. |
| Read/Write | I/O requests checked against open flags (`O_RDONLY`, `O_WRONLY`, `O_RDWR`), queued, and executed with exact-once completion semantics. |
| Seek | Offset state (`file_offset`) modified via thread-safe atomic operations or guarded mutexes without racing concurrent I/O operations. |
| Lock | Lock acquisition/release via `FileLockManager` ensuring failure-safe rollbacks on interrupt or signal abort. |
| Metadata | Atomic updates to `InodeMetadata` (timestamps, size, flags) maintaining cached and persistent filesystem consistency. |
| Close | Process fd entry cleared and underlying `FileHandle`/`Inode` reference count decremented exactly once. |
| Close-on-exec | File descriptors with `FD_CLOEXEC` flags cleared during `execve` image replacement (`handle_exec`). |
| Error | All subsystem errors return typed `FsError` codes without corrupting kernel state or dangling resources. |

## Subsystem Boundaries & Semantics
- **Directory Semantics**: Bounded capacity (`MAX_DIR_ENTRIES`), atomic creation/unlinking, lost-update prevention via mutex guards, double-unlink protection.
- **Permission Semantics**: POSIX standard `rwxrwxrwx` bits with Root (`uid = 0`) and POSIX Capability overrides (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
- **Lock Semantics**: BSD `flock` (shared/exclusive) and byte-range locking (`FileRangeLock`) with deadlock detection and automatic cleanups on process teardown.
- **Metadata Semantics**: In-memory metadata (`atime`, `mtime`, `ctime`, `size`, `mode`) synchronized with journaling backing stores.

## Resource Limits & Error Handling
- Default process file descriptor capacity: `DEFAULT_MAX_FDS` (1024 descriptors).
- Maximum directory entries: 4096 entries per directory node.
- Typed error propagation via `FsError`: `NotFound`, `PermissionDenied`, `InvalidFileDescriptor`, `NoFileDescriptorsAvailable`, `FileAlreadyExists`, `IsADirectory`, `NotADirectory`, `DirectoryNotEmpty`, `ResourceBusy`, `StorageFull`, `OutOfMemory`, `IoError`, `BadOffset`, `InvalidOperation`.

## Hardware & Environment Support
- **NUMA Behavior**: Inode metadata and buffer cache allocations prioritize local memory node affinity where applicable.
- **Hotplug Behavior**: Dynamic storage device attachments/detachments gracefully handle unmount transitions and invalidate open file descriptors returning `FsError::IoError`.
