# SigmaOS File Management Policy

## 1. Overview and Objective

SigmaOS manages file management—including VFS, file descriptors, inodes, directories, permissions, advisory and range locking, and file lifecycle—as a maintainable, observable, and recoverable subsystem governed by `VFS_RULES.toml`.

> **Objective:** Maintain SigmaOS file management as a safe, observable, recoverable subsystem: validate paths, enforce permissions, manage file descriptors and inodes, handle locks and events, and release resources exactly once.

---

## 2. File Lifecycle Contract

The file management subsystem enforces a single, authoritative file lifecycle state machine:

```text
open -> read/write -> seek -> lock -> metadata -> close -> cleanup
```

### State Contract Specification

| State | Required Behavior & Invariants |
| :--- | :--- |
| **Open** | File descriptor allocated from process-local table; inode resolved; permissions validated against `ProcessCredentials`; flags processed (`O_RDONLY`, `O_WRONLY`, `O_RDWR`, `O_CREAT`, `O_EXCL`, `O_TRUNC`, `O_APPEND`, `O_NONBLOCK`, `O_CLOEXEC`). |
| **Read / Write** | I/O requests validated against bounds and file offset; executed exactly once; reference counters updated atomically; errors return typed `FsError`. |
| **Seek** | Offset updated race-safely via atomic `AtomicU64` or lock-protected `SeekFrom` (`Start`, `Current`, `End`); checked overflow prevention returning `FsError::BadOffset`. |
| **Lock** | Advisory (`flock`) and POSIX byte-range (`fcntl`) locks evaluated for conflicts; non-blocking failures return `FsError::ResourceBusy`; lock table tracking prevents lost-lock and double-unlock defects. |
| **Metadata** | Inode metadata (`atime`, `mtime`, `ctime`, `mode`, `uid`, `gid`, `nlink`, `size`) updated coherently without lost updates. |
| **Close** | Descriptor slot cleared in process fd table; file handle reference count decremented; held locks for fd released exactly once. |
| **Close-on-exec** | Descriptors marked with `FD_CLOEXEC` are automatically released and closed upon process `exec` lifecycle transition. |
| **Error / Recovery**| Subsystem errors return typed `FsError` values; failures preserve filesystem state without corrupting metadata or triggering kernel panics. |

---

## 3. Subsystem Architectural Design

### Class Specifications

- **VFS Class:** Request-based hybrid layer mapping virtual file paths to underlying mounted storage backends (`SigmaFs`, `Btrfs`, `XFS`, `bcachefs`, `OverlayFs`, `tmpfs`, `procfs`, `sysfs`).
- **File Descriptor Class:** Handle-based, process-local table (`ProcessFdTable`) storing shared `FileHandle` descriptors pointing to atomic offsets and inode references.
- **Inode Class:** Hybrid numeric/UUID inode manager (`InodeManager`) tracking in-memory `Inode` instances with atomic reference counts (`ref_count`, `nlink`) and active lock lists.
- **Directory Semantics:** Bounded directory tables (`Directory`) supporting lookup, entry addition, entry removal, and directory entry validation up to `MAX_DIR_ENTRIES` (4096) and `MAX_FILENAME_LEN` (255 chars).
- **Permission Semantics:** Capability-aware DAC permission engine (`PermissionEngine`) evaluating EUID/EGID, owner/group/other mode bits (`rwx`), `CAP_DAC_OVERRIDE`, `CAP_DAC_READ_SEARCH`, and `CAP_FOWNER`.
- **Lock Semantics:** Range lock manager (`FileLockManager`) tracking process and descriptor advisory locks with overlap detection and automatic release on process termination or descriptor close.
- **Metadata Semantics:** Coherent timestamp and file size synchronization backed by transactional journaling or CoW extent updates.
- **Error Policy:** Explicit typed return values (`FsError`); no panics on invalid user inputs, broken descriptors, or filesystem I/O errors.
- **Limits:** Hard capacity limits for open descriptors per process (`DEFAULT_MAX_FDS = 1024`), inode table entries, directory entries, and buffer cache size.
- **NUMA Behavior:** Domain-local memory allocation for buffer cache pages and virtual memory mappings, respecting NUMA page affinity policies.
- **Hotplug Behavior:** Dynamic detachment handling for block storage devices, flushing pending write buffers and transitioning open handles to `FsError::IoError`.
