# SigmaOS Filesystem Snapshots

> Native Btrfs/ZFS snapshot management with automatic rollback.

## Overview

SigmaOS provides first-class filesystem snapshot support via Btrfs and ZFS backends. Snapshots are created automatically before system updates and can be selected at boot time for instant rollback.

## Snapshot Types

| Kind            | When Created                    | Bootable |
|-----------------|----------------------------------|----------|
| Manual          | User-initiated via `sigma-snap`  | No       |
| PreUpdate       | Before `sigma-pkg upgrade`       | ✅       |
| PostUpdate      | After successful upgrade         | No       |
| Scheduled       | Cron/timer based                 | No       |
| BootCheckpoint  | After successful boot            | ✅       |

## Operations

### Create a Snapshot
```bash
sigma-snap create --subvolume / --kind pre-update --desc "Before kernel 6.12"
```

### Rollback to a Snapshot
```bash
sigma-snap rollback --id 5
```

### List Bootable Snapshots
```bash
sigma-snap list --bootable
```

### Diff Two Snapshots
```bash
sigma-snap diff --from 3 --to 5
```

## Implementation

- **Source**: `fs/sigma_filesystem_snapshots.rs`
- **Language**: Rust (`no_std`)
- **Backends**: Btrfs, ZFS, SigmaFs (future native CoW filesystem)
- **Key APIs**:
  - `create_snapshot(subvolume, kind, description, timestamp)`
  - `rollback(snapshot_id)` — atomic subvolume swap
  - `list_bootable()` — for GRUB integration
  - `diff_snapshots(id_a, id_b)` — delta analysis

## Auto-Cleanup

When the snapshot limit is reached (`max_snapshots`, default 50):
1. Oldest **non-bootable** snapshots are pruned first
2. Bootable snapshots are preserved until explicitly removed
3. The cleanup is automatic if `auto_cleanup` is enabled

## GRUB Integration

Bootable snapshots are registered with the bootloader:
```
menuentry "SigmaOS - Snapshot #5 (Pre-Update 2025-01-15)" {
    set root='btrfs:/@snapshots/sigma-snap-5'
    linux /vmlinuz-sigma root=subvol=/@snapshots/sigma-snap-5
}
```
