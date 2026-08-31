# SigmaOS File System (VFS & FS Drivers)

This directory implements the filesystem abstractions and format-specific drivers for SigmaOS: VFS abstractions, `ext4` drivers, and `fat32` drivers.

## Sub-Modules

### 1. Virtual File System (`vfs.c`)

Abstracts storage backend structures and maps paths to specific mounted formats. Provides classic interfaces for:

- Inodes and mount points.

- Operations: `open`, `close`, `read`, `write`.

### 2. Ext4 Driver (`ext4.c`)

Lightweight standard implementation of the Fourth Extended Filesystem (ext4):

- Parses Ext4 Superblock (`0xEF53`) structure.

- Resolves Ext4 Inodes.

- Simulates read/write paths for standard nodes (e.g. `/etc/hostname`, `/var/log/syslog`).

### 3. FAT32 Driver (`fat32.c`)

Basic driver for File Allocation Table (FAT32) formats:

- Validates Extended Boot Record (EBR) sector configurations.

- Handles Directory Entry lookups and file size properties.

- Maps sequential cluster lists for cluster chain traversal.
