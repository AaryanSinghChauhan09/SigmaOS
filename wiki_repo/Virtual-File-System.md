# Sovereign Virtual File System (VFS)

The Virtual File System is the abstraction layer separating file system implementations from the POSIX-style system calls used by the kernel and userland.

## Core Concepts

- **Inodes**: Global descriptors containing file metadata (size, permissions, timestamps, device mapping).
- **File Descriptors**: Per-process handles that maintain independent state (e.g., offset) while pointing to a common inode.
- **Mount Points**: The VFS supports dynamic mounting of devices. Currently mocked mounts include `/dev/null` and `/dev/zero`.

## Architecture

The VFS interacts directly with the `Device Manager` to route reads and writes to physical block devices. 
It natively supports a unified namespace tree, beginning at `/`.
