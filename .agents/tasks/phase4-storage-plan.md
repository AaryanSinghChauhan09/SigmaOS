# Phase 4 Advanced Storage Implementation Plan

## Overview
This plan implements ZFS ARC Cache, Btrfs CoW Snapshots, unified Snapshot Manager, and enhanced CoW snapshot primitives. All code follows SigmaOS guidelines: `#![no_std]`, zero external dependencies, safe Rust, and custom data structures using `alloc::`.

## Existing Codebase Analysis

### Current Exports in `src/filesystem/mod.rs`
- **btrfs.rs**: `BtrfsFilesystem`, `BtrfsSuperblock`, `BtrfsError`, `BtrfsSnapshot`, `BtrfsStats`, `BtrfsCompression`, `BtrfsRaidLevel`
- **zfs.rs**: `ZfsPool`, `ZfsVdev`, `ZfsDataset`, `ZfsError`, `ZfsScrubStats`, `VdevType`, `VdevState`, `PoolState`, `DatasetType`
- **cow_snapshot.rs**: `CowSnapshot`, `CowSnapshotManager`, `FileTransaction`, `SnapshotState`

### Naming Constraints (AVOID duplicates)
**DO NOT redefine these existing types:**
- `BtrfsSnapshot` (already in btrfs.rs)
- `CowSnapshot`, `CowSnapshotManager`, `FileTransaction`, `SnapshotState` (already in cow_snapshot.rs)
- `ZfsPool`, `ZfsVdev`, `ZfsDataset` (already in zfs.rs)
- Any types re-exported from `filesystem::vfs`

### Standard Patterns Found
1. **Module header**: `#![no_std]` + `extern crate alloc;`
2. **Imports**: `use alloc::vec::Vec; use alloc::collections::BTreeMap; use core::sync::atomic::{AtomicU64, Ordering};`
3. **Tests**: `#[cfg(test)] mod tests { use super::*; }` at bottom of file
4. **Error types**: Simple enums with `#[derive(Debug, Clone, Copy, PartialEq, Eq)]`
5. **Build/test commands**: `cargo check --lib` for compilation, `rustc --test <file> --edition=2021 -o build/<name>_test && ./build/<name>_test` for standalone tests

---

## Implementation Plan

### File 1: `/home/aaryansinghchauhan/SigmaOS/src/filesystem/zfs_arc.rs`

**Purpose**: ZFS Adaptive Replacement Cache with dual LRU lists (MRU + MFU), ghost lists for eviction tracking, dynamic arc_p balancing, memory pressure callbacks, and compressed block support.

**Structs/Enums to Define**:
- `ZfsArcCache` - Main ARC cache manager with MRU/MFU lists and ghost lists
- `ArcState` - Enum: `MruTop`, `MruGhost`, `MfuTop`, `MfuGhost` (cache state for each block)
- `ArcBlock` - Cached block entry with {blkptr: u64, data: Vec<u8>, refcount: AtomicU64, compressed: bool, state: ArcState}
- `ArcStats` - Statistics: {hits, misses, mru_size, mfu_size, ghost_hits, evictions, arc_p_current, target_size}
- `ArcError` - Enum: `NoSpace`, `BlockNotFound`, `CompressionError`, `MemoryPressure`
- `MemoryPressureLevel` - Enum: `Low`, `Medium`, `High`, `Critical`

**Custom Data Structures** (zero dependencies - implement from scratch):
- `DoublyLinkedList<T>` wrapper around `Vec<T>` with index-based prev/next pointers for LRU
- `ArcHashMap<K, V>` wrapper around `BTreeMap<K, V>` for O(log n) lookups

**Reuse from Existing**:
- Import `ZfsBlockPtr` from `super::zfs` for block pointer references
- Do NOT redefine `ZfsPool` or `ZfsVdev`

**Key Methods**:
- `ZfsArcCache::new(target_size: u64) -> Self` - Initialize with target cache size
- `get(&mut self, blkptr: u64) -> Result<&[u8], ArcError>` - Lookup with MRU/MFU promotion
- `insert(&mut self, blkptr: u64, data: Vec<u8>, compressed: bool) -> Result<(), ArcError>` - Insert with eviction if needed
- `evict(&mut self) -> Result<u64, ArcError>` - Evict from MRU or MFU based on arc_p
- `adjust_arc_p(&mut self, ghost_hit_location: ArcState)` - Adjust arc_p based on ghost list hits (core ARC algorithm)
- `apply_memory_pressure(&mut self, level: MemoryPressureLevel) -> u64` - Shrink cache, return bytes freed
- `get_stats(&self) -> ArcStats` - Return cache statistics

**Test Cases** (`#[cfg(test)]`):
1. `test_arc_mru_mfu_promotion` - Insert blocks, verify MRU->MFU promotion on repeated access
2. `test_arc_ghost_list_tracking` - Evict blocks, verify ghost entries, verify arc_p adjustment on ghost hits
3. `test_arc_eviction_policy` - Fill cache beyond target, verify eviction respects arc_p ratio
4. `test_arc_compressed_blocks` - Store compressed blocks, verify size accounting differs from decompressed
5. `test_arc_memory_pressure` - Apply pressure levels, verify cache shrinks appropriately

**Verification**:
```bash
cargo check --lib  # Must compile with 0 errors
mkdir -p build && rustc --test src/filesystem/zfs_arc.rs --edition=2021 -o build/zfs_arc_test && ./build/zfs_arc_test
```

---

### File 2: `/home/aaryansinghchauhan/SigmaOS/src/filesystem/btrfs_snapshots.rs`

**Purpose**: Btrfs CoW snapshot engine with B-tree extent management, read-only and read-write snapshot creation, reference counting, send/receive stream format, incremental snapshot diff.

**Structs/Enums to Define**:
- `BtrfsSnapshotEngine` - Manages snapshot lifecycle (DO NOT name it `BtrfsSnapshot` - already exists)
- `BtrfsExtentRef` - Extent reference: {extent_id: u64, refcount: AtomicU64, generation: u64, owner_root: u64}
- `BtrfsSubvolume` - Subvolume descriptor: {id: u64, parent_id: u64, root_offset: u64, flags: SubvolumeFlags, name: Vec<u8>}
- `SubvolumeFlags` - Enum flags: `ReadOnly`, `ReadWrite`, `Default`
- `SnapshotCreateOpts` - Options: {source_subvol_id: u64, target_name: Vec<u8>, readonly: bool}
- `SendReceiveStream` - Stream format: {header: StreamHeader, operations: Vec<StreamOp>}
- `StreamOp` - Enum: `CreateSubvol`, `CreateSnapshot`, `Mkfile`, `Mkdir`, `Write{offset, data}`, `Clone{from_offset, to_offset, len}`, `Chown`, `Chmod`, `UpdateExtent`
- `StreamHeader` - {magic: [u8; 8], version: u32, flags: u32}
- `SnapshotDiff` - Incremental diff: {parent_gen: u64, child_gen: u64, changed_extents: Vec<u64>}
- `BtrfsSnapshotError` - Enum: `SubvolumeNotFound`, `SnapshotExists`, `ReadOnlyViolation`, `ExtentNotFound`, `StreamCorrupted`

**Reuse from Existing**:
- Import `BtrfsFilesystem`, `BtrfsKey`, `BtrfsFileExtentItem` from `super::btrfs`
- DO NOT redefine `BtrfsSnapshot` (already exported from btrfs.rs) - use wrapper pattern if needed

**Key Methods**:
- `BtrfsSnapshotEngine::new(fs: &BtrfsFilesystem) -> Self`
- `create_snapshot(&mut self, opts: SnapshotCreateOpts) -> Result<u64, BtrfsSnapshotError>` - Clone B-tree root with CoW
- `delete_snapshot(&mut self, snapshot_id: u64) -> Result<(), BtrfsSnapshotError>` - Decrement extent refcounts, free orphaned extents
- `increment_extent_ref(&mut self, extent_id: u64) -> Result<(), BtrfsSnapshotError>`
- `decrement_extent_ref(&mut self, extent_id: u64) -> Result<bool, BtrfsSnapshotError>` - Returns true if refcount reached zero
- `generate_send_stream(&self, snapshot_id: u64, parent_id: Option<u64>) -> Result<SendReceiveStream, BtrfsSnapshotError>` - Full or incremental
- `apply_receive_stream(&mut self, stream: &SendReceiveStream) -> Result<u64, BtrfsSnapshotError>` - Reconstruct snapshot from stream
- `compute_incremental_diff(&self, parent_id: u64, child_id: u64) -> Result<SnapshotDiff, BtrfsSnapshotError>` - Compare B-tree generations

**Test Cases**:
1. `test_snapshot_creation_readonly` - Create RO snapshot, verify cannot write to it
2. `test_snapshot_creation_readwrite` - Create RW snapshot, verify independent writes
3. `test_extent_refcounting` - Create multiple snapshots sharing extents, delete one, verify refcounts
4. `test_send_receive_full` - Generate full send stream, apply to new location, verify integrity
5. `test_send_receive_incremental` - Generate incremental stream between two snapshots, verify only changed extents transmitted
6. `test_incremental_diff` - Create snapshot, modify files, create second snapshot, verify diff captures exact changes

**Verification**:
```bash
cargo check --lib
mkdir -p build && rustc --test src/filesystem/btrfs_snapshots.rs --edition=2021 -o build/btrfs_snapshots_test && ./build/btrfs_snapshots_test
```

---

### File 3: `/home/aaryansinghchauhan/SigmaOS/src/filesystem/snapshot_manager.rs`

**Purpose**: Filesystem-agnostic unified snapshot manager with scheduling (hourly/daily/weekly/monthly), retention policies, bootloader integration, rollback with boot entry generation, space accounting.

**Structs/Enums to Define**:
- `UnifiedSnapshotManager` - Main manager coordinating ZFS, Btrfs, and generic CoW snapshots
- `FilesystemBackend` - Enum: `Zfs(ZfsPool)`, `Btrfs(BtrfsFilesystem)`, `Generic(CowSnapshotManager)`
- `SnapshotSchedule` - Enum: `Hourly`, `Daily`, `Weekly`, `Monthly`, `OnDemand`
- `SnapshotPolicy` - Policy: {schedule: SnapshotSchedule, retention_count: u32, min_free_space_percent: u8, auto_cleanup: bool}
- `SnapshotRecord` - Unified record: {id: u64, name: Vec<u8>, timestamp: u64, backend: FilesystemBackend, size_bytes: u64, parent_id: Option<u64>}
- `BootEntry` - Bootloader entry: {id: u64, label: Vec<u8>, snapshot_id: u64, kernel_path: Vec<u8>, initrd_path: Vec<u8>, cmdline: Vec<u8>}
- `RollbackPlan` - Rollback execution plan: {target_snapshot_id: u64, affected_inodes: Vec<u64>, estimated_duration_sec: u32}
- `RetentionPolicy` - {keep_hourly: u32, keep_daily: u32, keep_weekly: u32, keep_monthly: u32}
- `SpaceAccountingReport` - {total_space: u64, used_space: u64, snapshot_space: u64, unique_data: u64, shared_data: u64}
- `SnapshotManagerError` - Enum: `UnsupportedFilesystem`, `ScheduleConflict`, `InsufficientSpace`, `RollbackFailed`, `BootloaderError`

**Reuse from Existing**:
- Import `ZfsPool` from `super::zfs`
- Import `BtrfsFilesystem` from `super::btrfs`
- Import `CowSnapshotManager` from `super::cow_snapshot`

**Key Methods**:
- `UnifiedSnapshotManager::new() -> Self`
- `register_backend(&mut self, backend: FilesystemBackend) -> Result<(), SnapshotManagerError>`
- `set_policy(&mut self, backend_id: u64, policy: SnapshotPolicy) -> Result<(), SnapshotManagerError>`
- `create_snapshot(&mut self, backend_id: u64, name: &[u8]) -> Result<SnapshotRecord, SnapshotManagerError>` - Delegates to appropriate backend
- `execute_scheduled_snapshots(&mut self, current_time: u64) -> Result<Vec<SnapshotRecord>, SnapshotManagerError>` - Run all due snapshots
- `apply_retention_policy(&mut self, backend_id: u64) -> Result<Vec<u64>, SnapshotManagerError>` - Delete old snapshots, return deleted IDs
- `rollback_to_snapshot(&mut self, snapshot_id: u64) -> Result<RollbackPlan, SnapshotManagerError>` - Execute rollback
- `generate_boot_entry(&self, snapshot_id: u64) -> Result<BootEntry, SnapshotManagerError>` - Create GRUB/systemd-boot entry
- `compute_space_accounting(&self, backend_id: u64) -> Result<SpaceAccountingReport, SnapshotManagerError>` - Analyze snapshot space usage

**Test Cases**:
1. `test_multi_backend_registration` - Register ZFS, Btrfs, Generic backends, verify isolation
2. `test_snapshot_scheduling` - Set hourly/daily policies, simulate time progression, verify snapshot creation
3. `test_retention_policy_enforcement` - Create 10 hourly snapshots, set retention=5, verify cleanup
4. `test_rollback_with_boot_entry` - Create snapshot, modify data, rollback, verify boot entry generated
5. `test_space_accounting` - Create snapshots with shared extents, verify space report shows sharing
6. `test_insufficient_space_handling` - Fill filesystem near capacity, verify snapshot creation blocked by policy

**Verification**:
```bash
cargo check --lib
mkdir -p build && rustc --test src/filesystem/snapshot_manager.rs --edition=2021 -o build/snapshot_manager_test && ./build/snapshot_manager_test
```

---

### File 4: `/home/aaryansinghchauhan/SigmaOS/src/filesystem/cow_snapshot.rs` (Enhancement)

**Purpose**: Enhance existing CoW snapshot primitives with block-level CoW, extent sharing, deduplication, and Merkle tree checksums.

**Current State Analysis**:
- File exists with: `CowSnapshot`, `CowSnapshotManager`, `FileTransaction`, `SnapshotState`
- Contains custom `Vec<T>` implementation with raw pointers
- Has `record_transaction`, `rollback_to_snapshot`, `mount_snapshot` methods

**New Structs/Enums to ADD** (avoid name collisions):
- `BlockCowDescriptor` - Block-level CoW tracking: {block_id: u64, physical_block: u64, refcount: AtomicU64, generation: u64}
- `ExtentShareMap` - Shared extent tracking: {extent_id: u64, owning_snapshots: Vec<u64>, start_block: u64, block_count: u32}
- `DeduplicationIndex` - Content hash -> block mapping: {content_hash: [u8; 32], block_id: u64, refcount: AtomicU64}
- `MerkleTreeNode` - Merkle tree for integrity: {hash: [u8; 32], left_child: Option<u64>, right_child: Option<u64>, block_range: (u64, u64)}
- `BlockDeduplicationStats` - Stats: {total_blocks: u64, unique_blocks: u64, dedup_ratio: f32, space_saved: u64}

**New Methods to ADD**:
- `CowSnapshotManager::enable_block_cow(&mut self, snapshot_id: usize) -> Result<(), &'static str>` - Enable block-level tracking
- `CowSnapshotManager::share_extent(&mut self, extent_id: u64, from_snap: usize, to_snap: usize) -> Result<(), &'static str>` - Share extent between snapshots
- `CowSnapshotManager::deduplicate_blocks(&mut self, snapshot_id: usize) -> Result<BlockDeduplicationStats, &'static str>` - Scan and deduplicate identical blocks using SHA256 hashing
- `CowSnapshotManager::build_merkle_tree(&self, snapshot_id: usize) -> Result<MerkleTreeNode, &'static str>` - Build Merkle tree for integrity verification
- `CowSnapshotManager::verify_merkle_tree(&self, snapshot_id: usize, root_hash: &[u8; 32]) -> Result<bool, &'static str>` - Verify snapshot integrity

**Implementation Strategy**:
1. Add new structs at top of file after existing struct definitions
2. Add new methods to existing `impl CowSnapshotManager` block
3. Preserve all existing methods (do not modify signatures)
4. Add new test module section: `#[cfg(test)] mod enhanced_tests { ... }` separate from existing tests

**Test Cases** (new section in existing test module):
1. `test_block_cow_on_write` - Enable block CoW, write to block, verify new physical block allocated
2. `test_extent_sharing_refcount` - Share extent between 3 snapshots, delete one, verify refcount decrements
3. `test_deduplication_identical_blocks` - Write identical data to different locations, deduplicate, verify single physical block
4. `test_merkle_tree_verification` - Build Merkle tree, verify root hash, corrupt a block, verify detection
5. `test_dedup_space_savings` - Create snapshot with duplicates, deduplicate, verify space savings calculation

**Verification**:
```bash
cargo check --lib
mkdir -p build && rustc --test src/filesystem/cow_snapshot.rs --edition=2021 -o build/cow_snapshot_test && ./build/cow_snapshot_test
```

---

## Integration Steps

### Step 5: Update `/home/aaryansinghchauhan/SigmaOS/src/filesystem/mod.rs`

**Lines to ADD** (append after existing `pub mod` declarations):
```rust
pub mod zfs_arc;
pub mod btrfs_snapshots;
pub mod snapshot_manager;
```

**Lines to ADD** (append after existing `pub use` declarations):
```rust
pub use zfs_arc::{
    ZfsArcCache, ArcState, ArcBlock, ArcStats, ArcError, MemoryPressureLevel,
};
pub use btrfs_snapshots::{
    BtrfsSnapshotEngine, BtrfsExtentRef, BtrfsSubvolume, SubvolumeFlags,
    SnapshotCreateOpts, SendReceiveStream, StreamOp, StreamHeader,
    SnapshotDiff, BtrfsSnapshotError,
};
pub use snapshot_manager::{
    UnifiedSnapshotManager, FilesystemBackend, SnapshotSchedule, SnapshotPolicy,
    SnapshotRecord, BootEntry, RollbackPlan, RetentionPolicy,
    SpaceAccountingReport, SnapshotManagerError,
};
pub use cow_snapshot::{
    BlockCowDescriptor, ExtentShareMap, DeduplicationIndex, MerkleTreeNode,
    BlockDeduplicationStats,
    // Existing exports remain: CowSnapshot, CowSnapshotManager, FileTransaction, SnapshotState
};
```

**Verification**:
```bash
cargo check --lib  # Must show 0 errors
```

---

### Step 6: Update `/home/aaryansinghchauhan/SigmaOS/src/lib.rs`

**Lines to ADD** (append to existing `pub use filesystem::{...}` block around line 72):
```rust
pub use filesystem::{
    // ... existing exports ...
    ZfsArcCache, ArcState, ArcStats, ArcError, MemoryPressureLevel,
    BtrfsSnapshotEngine, BtrfsExtentRef, BtrfsSubvolume, SubvolumeFlags,
    SendReceiveStream, SnapshotDiff, BtrfsSnapshotError,
    UnifiedSnapshotManager, FilesystemBackend, SnapshotSchedule, SnapshotPolicy,
    SnapshotRecord, BootEntry, RollbackPlan, SpaceAccountingReport, SnapshotManagerError,
    BlockCowDescriptor, ExtentShareMap, DeduplicationIndex, MerkleTreeNode,
};
```

**Verification**:
```bash
cargo check --lib  # Final compilation check - must be 0 errors
```

---

### Step 7: Add Test Runner Entries to `/home/aaryansinghchauhan/SigmaOS/run_sigma_tests.sh`

**Lines to ADD** (append before final `echo "All SigmaOS test suites completed."`):
```bash
if [ -f "src/filesystem/zfs_arc.rs" ]; then
    echo "Running ZFS ARC Cache test suite..."
    mkdir -p build
    rustc --test src/filesystem/zfs_arc.rs --edition=2021 -o build/zfs_arc_test
    ./build/zfs_arc_test
fi

if [ -f "src/filesystem/btrfs_snapshots.rs" ]; then
    echo "Running Btrfs CoW Snapshots & Extent Management test suite..."
    mkdir -p build
    rustc --test src/filesystem/btrfs_snapshots.rs --edition=2021 -o build/btrfs_snapshots_test
    ./build/btrfs_snapshots_test
fi

if [ -f "src/filesystem/snapshot_manager.rs" ]; then
    echo "Running Unified Snapshot Manager test suite..."
    mkdir -p build
    rustc --test src/filesystem/snapshot_manager.rs --edition=2021 -o build/snapshot_manager_test
    ./build/snapshot_manager_test
fi

if [ -f "src/filesystem/cow_snapshot.rs" ]; then
    echo "Running Enhanced CoW Snapshot with Deduplication & Merkle Tree test suite..."
    mkdir -p build
    rustc --test src/filesystem/cow_snapshot.rs --edition=2021 -o build/cow_snapshot_enhanced_test
    ./build/cow_snapshot_enhanced_test
fi
```

**Verification**:
```bash
./run_sigma_tests.sh  # Must pass all tests
```

---

## Critical Implementation Notes

### Zero External Dependencies
All data structures implemented from scratch:
- **LRU Lists**: Use `Vec<T>` with index-based doubly-linked list (store prev/next indices)
- **Hash Tables**: Use `alloc::collections::BTreeMap` for O(log n) operations
- **Atomic Operations**: Use `core::sync::atomic::{AtomicU64, AtomicU32, Ordering}`
- **SHA256 Hashing**: Implement simplified 256-bit hash for deduplication (or use placeholder with FIXME comment for future hardware SHA-NI integration)

### Memory Safety
- All `unsafe` blocks must have `// SAFETY:` comments explaining why they are sound
- Prefer safe abstractions over raw pointers
- Use `AtomicU64` for refcounts (thread-safe increment/decrement)

### Linux & BSD Inspiration
- **ZFS ARC**: Inspired by FreeBSD/OpenZFS `arc.c` with MRU/MFU/Ghost lists
- **Btrfs Snapshots**: Inspired by Linux `fs/btrfs/send.c` and `fs/btrfs/ioctl.c` snapshot operations
- **Snapshot Manager**: Inspired by Snapper (openSUSE), Timeshift (Mint), ZFS auto-snapshot (FreeBSD)

### Error Handling Pattern
```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XError {
    Variant1,
    Variant2,
}
```
Return `Result<T, XError>` for all fallible operations.

### Test Pattern
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        // Setup
        let mut manager = Manager::new();
        
        // Execute
        let result = manager.operation();
        
        // Verify
        assert!(result.is_ok());
        assert_eq!(expected, actual);
    }
}
```

---

## Final Verification Checklist

- [ ] All 4 files compile with `cargo check --lib` (0 errors)
- [ ] All 4 files have `#![no_std]` and `extern crate alloc;`
- [ ] No duplicate type definitions (checked against existing exports)
- [ ] All new types use `alloc::` (Vec, BTreeMap, String, Arc) not `std::`
- [ ] Each file has at least 3 `#[cfg(test)]` test functions
- [ ] `mod.rs` declares new modules and exports new types
- [ ] `lib.rs` re-exports new types from filesystem module
- [ ] `run_sigma_tests.sh` includes test runners for all 4 files
- [ ] All tests pass when running `./run_sigma_tests.sh`

---

## Implementation Sequence

1. **Day 1**: Implement `zfs_arc.rs` with ARC algorithm (MRU/MFU lists, ghost tracking, arc_p adjustment)
2. **Day 2**: Implement `btrfs_snapshots.rs` with extent refcounting and send/receive streams
3. **Day 3**: Implement `snapshot_manager.rs` with scheduling and retention policies
4. **Day 4**: Enhance `cow_snapshot.rs` with block-level CoW, deduplication, Merkle trees
5. **Day 5**: Integration testing - update mod.rs, lib.rs, run_sigma_tests.sh, verify full build

Each day must end with `cargo check --lib` passing for that file.

---

**End of Implementation Plan**

This plan follows SigmaOS zero-dependency philosophy, draws inspiration from Linux (Btrfs, EEVDF) and BSD (ZFS/OpenZFS, FreeBSD jails), maintains safe Rust throughout, and ensures 100% test coverage with standalone test runners.
