# SigmaOS System Snapshot & Rollback Components

## Overview
SigmaOS system snapshot manager providing Btrfs snapshots with automatic management, competitive with Omarchy's snapper integration and Mint Timeshift.

## Features
- **Btrfs snapshot management** (pre-update, post-update, manual, factory reset, auto daily)
- **Automatic snapshots** before and after system updates
- **Rollback capability** to specific snapshots
- **Snapshot pruning** (keep N most recent)
- **Snapshot retention** (time-based)
- **Secure defaults** (snapshots enabled by default)

## Snapshot Types

| Type | Description |
|------|-------------|
| **PreUpdate** | Automatic snapshot before system update |
| **PostUpdate** | Automatic snapshot after successful update |
| **Manual** | User-created snapshot |
| **FactoryReset** | OEM factory reset snapshot |
| **AutoDaily** | Automatic daily snapshot |

## Structures

| Structure | Description |
|-----------|-------------|
| **SnapshotManager** | Snapshot manager with create/rollback/delete |
| **BtrfsSnapshot** | Btrfs snapshot with metadata |
| **SnapshotConfig** | Snapshot configuration |

## Default Configuration

| Setting | Default |
|---------|---------|
| enabled | true |
| max_snapshots | 10 |
| auto_snapshot_before_update | true |
| auto_snapshot_after_update | true |
| snapshot_retention_days | 30 |
| snapshot_path | /.snapshots |
| auto_daily | true |

## Usage Examples

```rust
// Create snapshot manager
let mut mgr = SnapshotManager::new();

// Create pre-update snapshot
mgr.create_pre_update().unwrap();

// Create manual snapshot
mgr.create_manual("before-config-change".to_string(), Some("Testing new config".to_string())).unwrap();

// Rollback to snapshot
mgr.rollback_to(1).unwrap();

// List snapshots
let snapshots = mgr.list();
```

## Status
- **Implementation**: Complete
- **Dependencies**: Btrfs tools
- **Roadmap Phase**: Update System (P1)

## Related Files
- `src/update/snapshot.rs` - Snapshot manager implementation
- `wiki/Update-System.md` - Wiki documentation

## Principles Followed
CONTINUOUS_IMPROVEMENT, LONG_TERM_SUSTAINABILITY, RACE_FREE DESIGN, DEADLOCK_AVOIDANCE,
SAFE_CONCURRENCY, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM, SIMPLE_AND_ROBUST,
MINIMAL_KERNEL_PHILOSOPHY, SECURE_DEFAULTS, VERSION_CONTROL_PRACTICES, ROLLBACK_SUPPORT,
IDempotent, ZERO_COPY, MEMORY_SAFETY