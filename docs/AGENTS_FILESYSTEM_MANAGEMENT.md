# SigmaOS AI Agent Filesystem Management Specification

This document specifies mandatory filesystem invariants, Copy-On-Write (CoW) snapshot rules, atomic journal transaction guidelines, and path sandboxing standards for autonomous AI engineering agents (Jules, Sentinel, Palette, Bolt) contributing to SigmaOS.

## 1. Virtual File System (VFS) & Mount Namespace Architecture
- **VFS Inode Caching (`docs/filesystem.md`)**:
  - Virtual File System inodes must be pinned in memory caches before read/write transactions.
  - Mount namespaces (`src/filesystem/mount_namespace.rs`) must isolate process file visibility based on container profiles.

## 2. Copy-On-Write (CoW) & Atomic Snapshot Mechanics
- **CoW Snapshots & Extent Trees (`src/filesystem/cow_snapshot.rs`, `src/filesystem/btrfs_inspired.rs`)**:
  - Modifications to snapshot subvolumes must duplicate extent pointers rather than mutating blocks in-place.
  - Snapshot generations must be monotonically incremented to support one-click rollback gates.
- **HAMMER2 PFS Multi-Volume Deduplication (`src/unimplemented_features.rs`)**:
  - CoW blocks must be verified via CAS hash IDs before deduplication linking.

## 3. Journaling & Ext4 Compatibility
- **JBD2 Transaction Logging (`src/filesystem/ext4.rs`)**:
  - All file metadata writes must commit log entries to the JBD2 journal before updating block allocations.

## 4. Path Sandboxing & Security Enforcers
- **OpenBSD Unveil Path Sandbox (`src/security/sigma_unveil.rs`)**:
  - Filesystem access must enforce OpenBSD `unveil(2)` path restrictions (`r` read, `w` write, `c` create, `x` execute).
  - Attempts to traverse outside unveiled directory trees must raise immediate capability access violations.

## 5. AI Agent Filesystem Directives
1. **Atomic File Writes**: Use staged temporary write buffers followed by atomic rename operations to prevent partial file corruption.
2. **Path Sanitization**: Validate all path inputs against `..` traversal and null byte injections.
