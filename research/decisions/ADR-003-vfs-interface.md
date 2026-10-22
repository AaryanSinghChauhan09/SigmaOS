# ADR-003: Trait-Based Virtual Filesystem (VFS) Interface

## Status
Accepted

## Context
SigmaOS requires a filesystem abstraction supporting multiple backends, mount namespaces, and path resolution.

## Upstream Study
Linux VFS separates generic inode/dentry operations from filesystem implementations; xv6 provides simple buffer caches.

## Decision
Adopt safe Rust traits (`VFileSystem`, `VNode`) with explicit path traversal checks, mount points, and POSIX errnos.

## Consequences
- Backends (RAM disk, ext4, virtio-fs) can be added independently.
- Path resolution prevents symlink loops and relative traversal escapes.
- Memory safety guaranteed without raw C pointers.

## Validation
- VFS mount unit tests and RAM disk integration test suite.
