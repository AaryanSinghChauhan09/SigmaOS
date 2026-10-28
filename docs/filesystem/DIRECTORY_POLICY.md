# Directory & Inode Policy (Filesystem)

## Overview
Re-exported directory and inode specification for filesystem drivers and mount targets.

## Policy Rules
- **Directory Capacity**: Maximum child entries per directory are enforced at VFS and driver levels.
- **Atomic Operations**: `mkdir`, `rmdir`, `unlink`, and `rename` operations guarantee filesystem state consistency.
- **Double Unlink Protection**: Inode deletion only triggers when link count drops to 0 AND open reference count reaches 0.
