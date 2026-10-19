# SigmaOS Update & Snapshot Manager Components

## Overview
SigmaOS system update and snapshot manager providing Btrfs snapshot management, competitive with Omarchy snapper, Mint Timeshift, and Arch snapper.

## Features
- **Btrfs snapshot management** (pre-update, post-update, manual, factory reset)
- **Automatic snapshots** before and after system updates
- **Rollback capability** to specific snapshots
- **Snapshot pruning** (keep N most recent)
- **Snapshot retention** (time-based)
- **Idempotent migration scripts** (skip if already applied)
- **Defensive validation** of all inputs
- **Fail-fast** error handling

## Snapshot Types

| Type | Description |
|------|-------------|
| **PreUpdate** | Automatic snapshot before system update |
| **PostUpdate** | Automatic snapshot after successful update |
| **Manual** | User-created snapshot |
| **FactoryReset** | OEM factory reset snapshot |

## Snapshot Status

| Status | Description |
|--------|-------------|
| **Creating** | Snapshot is being created |
| **Complete** | Snapshot is complete and valid |
| **Deleting** | Snapshot is being deleted |
| **Corrupted** | Snapshot is corrupted |

## Structures

| Structure | Description |
|-----------|-------------|
| **UpdateManager** | Update manager with snapshots and migrations |
| **BtrfsSnapshot** | Btrfs snapshot with metadata |
| **SnapshotConfig** | Snapshot configuration (enabled, max count, retention) |
| **MigrationScript** | Idempotent migration script |

## Default Configuration

| Setting | Default |
|---------|---------|
| enabled | true |
| max_snapshots | 10 |
| auto_snapshot_before_update | true |
| auto_snapshot_after_update | true |
| snapshot_retention_days | 30 |
| snapshot_path | /.snapshots |

## Status
- **Implementation**: Complete
- **Dependencies**: Btrfs tools, migration scripts
- **Roadmap Phase**: Update System (P1)

## Related Files
- `src/update/snapshot.rs` - Snapshot manager implementation
- `wiki/Update-System.md` - Update system wiki

## Principles Followed
CONTINUOUS_IMPROVEMENT, LONG_TERM_SUSTAINABILITY, RACE_FREE DESIGN,
DEADLOCK_AVOIDANCE, SAFE_CONCURRENCY, DEFENSIVE_PROGRAMMING,
FAIL_FAST_SYSTEM, SECURE_DEFAULTS, VERSION_CONTROL_PRACTICES,
ROLLBACK_SUPPORT