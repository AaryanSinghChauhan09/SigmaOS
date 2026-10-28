# SigmaOS File Management Policy

## Architectural Overview

SigmaOS treats **file management** (VFS, file descriptors, inodes, directories, permissions, locking, and file lifecycle) as an integrated, safe, observable, and recoverable lifecycle subsystem.

### Class Architecture
- **VFS Class**: Hybrid request-based and inode-based virtual filesystem router.
- **File Descriptor Class**: Handle-based process-local descriptor table with atomic handle references.
- **Inode Class**: Inode-based lifecycle management with explicit state transitions (`Allocated` -> `Active` -> `Unlinked` -> `Reclaimed`).
- **Directory Semantics**: Bounded capacity (`MAX_DIR_ENTRIES = 4096`), race-safe entry insertion and removal, double-unlink prevention.
- **Permission Semantics**: POSIX mode bits (`0o777` mask), process credential verification (`uid`, `gid`, `euid`, `egid`), and capability-based overrides (`CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, `CAP_FOWNER`).
- **Lock Semantics**: POSIX advisory (`flock`) and byte-range locks (`fcntl`), non-blocking lock trials, lock queue bound enforcement, lost-lock/double-unlock prevention.
- **Metadata Semantics**: Coherent update propagation across timestamps (`atime`, `mtime`, `ctime`), refcounts, and file sizes.
- **Error Policy**: Typed `FsError` return codes (`NotFound`, `PermissionDenied`, `InvalidFileDescriptor`, `NoFileDescriptorsAvailable`, `FileAlreadyExists`, `IsADirectory`, `NotADirectory`, `DirectoryNotEmpty`, `ResourceBusy`, `StorageFull`, `OutOfMemory`, `IoError`, `BadOffset`, `InvalidOperation`).
- **Limits & Bounds**: Strictly enforced static allocation limits preventing kernel queue or cache memory exhaustion.
- **NUMA & Memory Domain**: Core memory structures avoid cross-NUMA socket allocations.
- **Hotplug & Detachment**: Graceful handling of storage hotplug detachments without corrupting kernel state or deadlocking lock queues.

## Single Authoritative File Lifecycle

```text
open -> read/write -> seek -> lock -> metadata -> close -> cleanup
```

| Lifecycle State | Enforced Invariants |
|---|---|
| Open | Race-free fd table allocation, inode resolution, credential/capability check |
| Read/Write | Validated request offsets, bounded transfer buffers, exactly-once completion |
| Seek | Race-safe atomic offset adjustments with `SeekFrom::Start`, `Current`, `End` |
| Lock | Bounded range lock tracking, non-deadlocking lock trials, automatic release on close |
| Metadata | Coherent inode attribute updates, refcount modification |
| Close | Single release of descriptor slots, atomically decrementing inode/handle refcounts |
| Close-on-exec | Descriptor flag `FD_CLOEXEC` evaluated during `exec` transitions |
| Error | Failure does not corrupt filesystem state or leak descriptor slots |
