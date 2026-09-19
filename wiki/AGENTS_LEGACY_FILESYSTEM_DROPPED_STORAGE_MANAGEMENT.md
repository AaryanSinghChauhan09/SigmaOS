# AI Agent Management: Legacy Filesystems & Dropped Storage Management

This document provides guidelines for AI agents maintaining, parsing, and emulating legacy filesystems and storage formats dropped or deprecated by upstream Linux (e.g., ReiserFS dropped in Linux 6.6, ext2/ext3 deprecated, HFS/HFS+ read-only, Minix VFS) and BSD distributions (e.g., UFS1, FFS, HAMMER1).

---

## 1. Scope & Objective

SigmaOS provides zero-dependency `#![no_std]` VFS driver adapters capable of safely reading, mounting, and converting legacy storage and filesystem formats without kernel panics or buffer overflows.

AI agents MUST follow these specifications when working in `src/vfs/`, `src/storage/`, and `src/tools/archive.rs`.

---

## 2. Legacy Filesystems & Adapter Requirements

| Filesystem Type | Upstream Distro Status | SigmaOS Driver Abstraction | Directives & Guidelines |
| :--- | :--- | :--- | :--- |
| **ReiserFS (Reiser3)** | Removed in Linux 6.6+ | `ReiserFsAdapter` | Parse B*tree journaled inodes, tail packing data blocks, and enforce read-only safety checks. |
| **ext2 / ext3** | Deprecated in Linux (ext4 fallback) | `Ext2Ext3Adapter` | Support direct block pointers, indirect blocks, journal replay (`jbd2`), and 32-bit timestamp bounds. |
| **UFS1 / UFS2 (FFS)** | Deprecated in NetBSD / FreeBSD | `UfsBsdAdapter` | Support cylinder groups, superblock mirrors, FFS inode disk layouts, and soft updates log reconciliation. |
| **HFS / HFS+** | Deprecated in Linux | `HfsPlusAdapter` | Parse B-tree catalog files, resource forks, Mac OS Roman string conversions, and journal log headers. |
| **JFS (IBM)** | Deprecated in Linux | `JfsIbmAdapter` | Support B+tree extent allocation, journal commit blocks, and 64-bit block addressing. |
| **Minix VFS** | Removed in modern distributions | `MinixVfsAdapter` | Support 14-character / 30-character directory entry bounds, 16-bit/32-bit inode tables, and initramfs boot images. |

---

## 3. Storage Safety & Memory Safety Rules

1. **W^X Memory & Bounds Checking:** Inode table parsing must enforce bounds checking against payload sizes (`s.len() <= MAX_PATH`).
2. **Read-Only Fallback for Corrupted Metadata:** If journal replay fails or superblock magic bytes mismatch, automatically downgrade mount to read-only (`O_RDONLY`).
3. **Zero Allocation Leaks:** Use static stack buffers or arena allocators (`alloc::vec::Vec`) compliant with `#![no_std]`.
