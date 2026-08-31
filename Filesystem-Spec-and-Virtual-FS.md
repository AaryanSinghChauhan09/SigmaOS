# SigmaFS and Virtual Filesystem Spec

SigmaOS implements Virtual Filesystem (VFS) mappings to coordinate multiple storage engines.

## Storage Backends

*   **SigmaFS**: A custom lightweight metadata-journaled filesystem.
*   **Btrfs Integration**: Support for subvolumes, snapshots, and LZO compression.

## Mount Architecture

Filesystem mounts are protected by capabilities:

    /           -> Root RAMFS (Read-only)
    /etc        -> Configuration space
    /proc       -> Linux-style process tracking
    /sys        -> Hardware interface maps
    ```\n
