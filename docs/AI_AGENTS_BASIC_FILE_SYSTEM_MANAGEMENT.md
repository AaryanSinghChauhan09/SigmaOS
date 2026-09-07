# 📁 AI Agents Basic File System Management Specification (`docs/AI_AGENTS_BASIC_FILE_SYSTEM_MANAGEMENT.md`)

This specification defines Virtual File System (VFS) operations, inode abstractions, file permissions, directory traversal protocols, and content-addressed storage (CAS) management for autonomous AI agents (Bolt ⚡, Palette 🎨, Sentinel 🛡️) in SigmaOS.

---

## 1. Virtual File System (VFS) Layer & Inodes (`src/fs/vfs.rs`)

AI agents manage Virtual File System abstractions:
- **Mount Point Hierarchy**: Root filesystem (`/`), `/sys/`, `/proc/`, `/dev/`, and `/sigma/store/` mount management.
- **Inode & Dentry Caches**: Inode metadata lookup (`Inode`) and directory entry (`Dentry`) cache resolution.
- **File System Operations**: Standard `open`, `read`, `write`, `seek`, `readdir`, `mkdir`, and `unlink` VFS operations.

---

## 2. File System Drivers (`src/fs/sigmafs.rs`, `src/fs/btrfs.rs`, `src/fs/xfs.rs`)

- **Native SigmaFS (`src/fs/sigmafs.rs`)**: Sovereign, zero-dependency filesystem driver optimized for AI agent state trees.
- **Btrfs CoW Driver (`src/fs/btrfs.rs`)**: Copy-On-Write filesystem supporting subvolumes, extents, and sub-second snapshots.
- **XFS Driver (`src/fs/xfs.rs`)**: Scalable, allocation-group-based filesystem handling high-throughput log and bulk storage.

---

## 3. Content-Addressed Storage (CAS) (`src/fs/sigmacas.rs`)

- **`/sigma/store/` CAS Storage**: Immutable, content-hashed binary store where files are indexed by SHA-256 / BLAKE3 digests.
- **Deduplication & Symlink Reflinks**: Zero-cost hardlinking and reflinking across installed packages and userland applications.

---

## 4. AI Agent Basic File System Responsibilities

- **⚡ Bolt**: Profiles VFS file I/O latency, monitors dentry cache hit rates, and optimizes file descriptor lookup pipelines.
- **🎨 Palette**: Manages graphical file manager directory rendering, icon previews, and file tree visual representations.
- **🛡️ Sentinel**: Enforces POSIX mode bits (`0755`/`0600`), validates OpenBSD `unveil` file path restrictions, and audits immutable file attributes.
