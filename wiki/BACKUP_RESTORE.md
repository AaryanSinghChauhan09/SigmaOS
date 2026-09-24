# SigmaOS System Backup & Snapshot Restoration Architecture

## 1. Overview

SigmaOS includes an integrated system snapshot, state backup, and disaster recovery engine inspired by Timeshift, Btrfs snapshots, and ZFS send/receive. It enables instantaneous system state rollbacks from the desktop shell or bootloader interface (`sigma-boot`).

## 2. Snapshot Mechanisms

SigmaOS supports three snapshot backends:

```
+-----------------------------------------------------------------+
|               SigmaOS Snapshot & Backup Engine                  |
+-----------------------------------------------------------------+
| 1. Btrfs Subvolume Snapshots (Copy-On-Write Copy-Free Snapshots)|
| 2. ZFS Dataset Snapshots (Instant ZFS Snapshots & Send/Receive) |
| 3. RSYNC Hardlink Fallback (File-level differential copies)     |
+-----------------------------------------------------------------+
```

### 2.1 Bootloader Integration & Pre-Boot Rollback
When system updates or package upgrades occur (`sigpkg upgrade`), the system automatically triggers a pre-update snapshot (e.g. `@snapshot-20260920-pre-upgrade`). If the upgrade causes a boot failure or system regression:
1. Select **"SigmaOS Rollback / System Recovery"** in `sigma-boot`.
2. Pick any prior snapshot timestamp.
3. System boots instantly into the immutable prior snapshot.

## 3. Disaster Recovery & Cloud Backup Engine

- **Encrypted Incremental Backups**: User documents and system state configs are encrypted using AES-256-GCM / Post-Quantum Kyber keys before remote replication.
- **Rsync over SSH / S3 Compatibility**: Supports backup synchronization to local NAS devices, USB storage drives, or S3 cloud buckets.
- **Deduplication**: Block-level data deduplication reduces storage consumption for backup archives.

## 4. CLI Administration Commands

- **Create Snapshot**: `sigsnapshot create --comment "Before GPU Driver Update"`
- **List Snapshots**: `sigsnapshot list`
- **Restore Snapshot**: `sigsnapshot restore @snapshot-20260920-pre-upgrade`
- **Delete Snapshot**: `sigsnapshot delete @snapshot-20260920-pre-upgrade`
