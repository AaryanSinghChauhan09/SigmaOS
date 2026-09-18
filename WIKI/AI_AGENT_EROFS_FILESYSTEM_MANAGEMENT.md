# AI Agent Directive: EROFS Read-Only Filesystem Subsystem Management

## Overview

The `ErofsEngine` (`src/filesystem/erofs.rs`, re-exported in `src/filesystem/mod.rs`) implements a zero-dependency, `#![no_std]` native Rust implementation of Linux EROFS (Enhanced Read-Only File System) for SigmaOS.

## Key Architectural Structures

1. **`ErofsSuperblock`**:
   - Parses EROFS superblock structures located at 1024-byte offset.
   - Validates `EROFS_SUPER_MAGIC` (`0xE0F5E1E2`).
   - Calculates block sizes (`1 << blkszbits`), inode counts, and metadata block addresses.

2. **`ErofsInode`**:
   - Supports compact (32-byte) and extended (64-byte) EROFS inode headers.
   - Tracks node IDs (`nid`), permissions (`mode`), and block address locations (`data_blkaddr`).

3. **`ErofsEngine`**:
   - Manages mounted image state and provides fast `lookup_inode` routines.

## Directives for AI Agents

- **Zero-Dependency Rule**: Do NOT import external C compression libraries or FUSE layers into core kernel/fs paths.
- **`#![no_std]` Compatibility**: Maintain strict compatibility with `#![no_std]` alloc primitives.
- **Verification**: Run standalone unit tests using:
  ```bash
  rustc --test --edition 2021 src/filesystem/erofs.rs -o build/erofs_test && ./build/erofs_test
  ```
