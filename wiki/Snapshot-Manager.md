# Snapshot Manager

## Overview

The Snapshot Manager provides system snapshot and backup management inspired by Linux Mint's Timeshift. It supports filesystem snapshots, scheduled backups, retention policies, and system rollback functionality.

## Features

- **Snapshot Types**: Manual, Hourly, Daily, Weekly, Monthly, and Boot snapshots
- **Status Tracking**: Creating, Complete, Failed, and Deleting states
- **Retention Policies**: Configurable retention limits per snapshot type
- **Snapshot Metadata**: ID, type, status, creation time, size, description, bootable flag
- **System Rollback**: Restore system state from bootable snapshots
- **Size Tracking**: Human-readable size formatting
- **Automatic Cleanup**: Automatic removal of old snapshots based on retention policy

## Components

### SnapshotType

```rust
pub enum SnapshotType {
    Manual,   // User-initiated snapshots
    Hourly,   // Scheduled hourly snapshots
    Daily,    // Scheduled daily snapshots
    Weekly,   // Scheduled weekly snapshots
    Monthly,  // Scheduled monthly snapshots
    Boot,     // Boot-time snapshots
}
```

### SnapshotStatus

```rust
pub enum SnapshotStatus {
    Creating,   // Snapshot in progress
    Complete,   // Snapshot successfully created
    Failed,     // Snapshot creation failed
    Deleting,   // Snapshot being deleted
}
```

### SnapshotMetadata

Represents a single snapshot with:
- Unique snapshot ID
- Snapshot type and status
- Creation timestamp
- Size in bytes
- Description
- Bootable flag
- Human-readable size formatting

### RetentionPolicy

```rust
pub struct RetentionPolicy {
    pub max_hourly: u32,
    pub max_daily: u32,
    pub max_weekly: u32,
    pub max_monthly: u32,
    pub max_manual: u32,
}
```

Default retention: 6 hourly, 7 daily, 4 weekly, 6 monthly, 10 manual.

### SnapshotManager

Main management interface with:
- Snapshot creation
- Snapshot deletion
- System restoration
- Retention policy enforcement
- Type-based filtering
- Statistics tracking

## Usage

### Basic Usage

```rust
use sigmaos::storage::SnapshotManager;

let mut manager = SnapshotManager::new();

// Create a manual snapshot
let id = manager.create_snapshot(SnapshotType::Manual, "Before update".to_string())?;

// List all snapshots
let snapshots = manager.list_snapshots();

// Get a specific snapshot
let snapshot = manager.get_snapshot(&id);
```

### Custom Retention Policy

```rust
use sigmaos::storage::{SnapshotManager, RetentionPolicy};

let policy = RetentionPolicy {
    max_hourly: 12,
    max_daily: 14,
    max_weekly: 8,
    max_monthly: 12,
    max_manual: 20,
};

let mut manager = SnapshotManager::with_retention(policy);
```

### Creating Snapshots

```rust
// Manual snapshot
manager.create_snapshot(SnapshotType::Manual, "Pre-upgrade backup".to_string())?;

// Boot snapshot
manager.create_snapshot(SnapshotType::Boot, "After system boot".to_string())?;
```

### Deleting Snapshots

```rust
manager.delete_snapshot("snapshot-1")?;
```

### System Restoration

```rust
// Restore from a snapshot
manager.restore_snapshot("snapshot-1")?;
```

### Filtering Snapshots

```rust
// List by type
let daily_snapshots = manager.list_by_type(SnapshotType::Daily);

// List all snapshots (sorted by creation time, newest first)
let all_snapshots = manager.list_snapshots();
```

### Statistics

```rust
let stats = manager.get_statistics();
println!("Total snapshots: {}", stats.total_snapshots);
println!("Total size: {}", stats.total_size);
println!("Daily snapshots: {}", stats.daily_snapshots);
```

## AI Agent Maintenance Instructions

When maintaining the Snapshot Manager:

1. **Retention Enforcement**: Ensure retention policy is correctly applied after each snapshot creation
2. **Status Accuracy**: Maintain accurate snapshot status throughout lifecycle
3. **Size Tracking**: Keep size estimates accurate for storage management
4. **Bootable Verification**: Verify bootable flag is set correctly for restorable snapshots
5. **ID Generation**: Ensure unique snapshot ID generation
6. **Cleanup Logic**: Maintain proper cleanup logic for old snapshots

## Testing

Run the unit tests with:

```bash
cargo test --lib storage::snapshot_manager
```

## Future Enhancements

- Integration with actual filesystem snapshot APIs (Btrfs, ZFS, LVM)
- Differential snapshots for space efficiency
- Snapshot verification and integrity checking
- Scheduled snapshot creation with cron integration
- Snapshot compression
- Remote snapshot storage
- Snapshot export/import
