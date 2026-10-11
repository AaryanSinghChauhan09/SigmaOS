# ADR-003: Layered Virtual File System (VFS) Interface

## Status
Accepted

## Upstream study
Linux VFS inode/dentry abstraction and xv6 inode filesystem layer.

## SigmaOS decision
Implement a layered VFS layer with explicit mount points, component validation (255-char max component, 4096-char path limit), and standardized POSIX error codes (`VfsPathError`).

## Scope
- Path resolution (`/`, `.`, `..`, relative and absolute paths)
- Inode operations (`open`, `read`, `write`, `close`, `readdir`)
- Initial RAM-disk (initramfs) root filesystem

## Non-goals
- Persistent filesystem on disk (Btrfs/ZFS deferred to subsequent cycle)
- Network filesystems (NFS/9P)
- Extended file attributes (`xattr`) in first slice

## Validation
- 18 VFS path resolution unit tests
- RAM-disk file read/write integration tests
