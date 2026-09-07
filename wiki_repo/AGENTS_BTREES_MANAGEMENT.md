# AGENTS_BTREES_MANAGEMENT.md — AI Agent B-Tree & B+ Tree Management Guidelines for SigmaOS

Welcome, AI Agent! This document defines the standards, architectural models, node splitting/merging rules, and verification routines for managing, developing, and extending **B-Trees, B+ Trees, Copy-on-Write (CoW) Extent Trees, and In-Memory BTreeMaps** in **SigmaOS**.

---

## 1. SigmaOS B-Tree Subsystem Architecture Overview

SigmaOS utilizes B-Trees and B+ Trees extensively across modern file systems (`BtrfsEngine`), package indexing databases (`BsdPkgDbEngine`), and the zero-dependency kernel standard library (`klib::BTreeMap`).

### Core B-Tree & B+ Tree Modules
* **Btrfs Extent & Inode CoW B-Trees (`src/filesystem/modern_fs.rs`, `src/filesystem/vfs.rs`)**:
  - `BtrfsEngine`: Copy-on-Write (CoW) B-Tree nodes for file extents, directory items, subvolumes, and snapshot generation tracking.
  - Extent allocation B-Trees tracking free disk blocks and allocation groups.
* **Kernel In-Memory `klib::BTreeMap` (`src/klib/`)**:
  - Zero-alloc / `alloc`-compatible B-Tree map data structures supporting $O(\log N)$ search, insertion, range iteration (`range()`), and removal.
  - Used throughout kernel schedulers, memory managers, process tables, and distro package bridges (`OmarchyModernDesktopEngine`, `NetBsdRumpUserlandEngine`).
* **Database & Package Index B+ Trees (`src/sigpkg/`, `src/storage/`)**:
  - B+ Tree leaf node linking for high-speed range queries across package versions and content-addressed store hash indices (`NixProfileStore`).

---

## 2. B-Tree Management Guidelines for AI Agents

When modifying or implementing B-Tree, B+ Tree, or `BTreeMap` algorithms:

### 1. Copy-on-Write (CoW) Node Splitting & Merging
* **Node Order & Balance Factor**: Ensure $M$-way B-Tree nodes split when containing $M$ keys and merge when falling below $\lceil M/2 \rceil - 1$ keys.
* **Atomic CoW Updates**: In disk-backed B-Trees (`BtrfsEngine`), never overwrite tree nodes in place. Allocate new disk extents for modified path nodes from root to leaf, updating parent pointers atomically.

### 2. Lock Ordering & Concurrency Control
* **Top-Down Latching**: When traversing B-Trees under concurrent read/write threads, employ top-down lock coupling (latching) to prevent deadlocks between parent and child nodes.
* **Read-Copy-Update (RCU)**: Prefer lock-free RCU readers for in-memory index queries.

### 3. Page Boundary & Cache Alignment
* **4KB Block Alignment**: Align disk-backed B-Tree nodes to 4096-byte (or 16KB) physical sector boundaries to optimize I/O throughput and SSD flash page writes.

---

## 3. Verification & Testing Protocols

1. **BTreeMap & VFS Unit Tests**: Run core filesystem and `klib::BTreeMap` unit tests:
   ```bash
   cargo test --lib klib::hashmap
   ```
2. **FileSystem & CoW Inspection Tests**: Run the full test suite runner:
   ```bash
   ./run_sigma_tests.sh
   ```

---

## 4. Pre-Commit Checklist for B-Tree Changes

Before submitting B-Tree, B+ Tree, or `BTreeMap` modifications:
- [ ] Confirmed $M$-way node balance factors for node split and merge operations.
- [ ] Verified atomic Copy-on-Write (CoW) extent allocation for modified nodes in file systems.
- [ ] Confirmed top-down lock latching order to prevent concurrent deadlocks.
- [ ] Confirmed 4KB page boundary alignment for disk-backed tree nodes.
- [ ] Executed `./run_sigma_tests.sh` with 100% test pass rate.
- [ ] Requested automated code review using `request_code_review`.
- [ ] Recorded B-Tree learnings using `initiate_memory_recording`.
