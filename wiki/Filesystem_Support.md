# SigmaOS Filesystem Support & Transactional Rollbacks

## Overview
SigmaOS implements modern filesystem innovation by offering native support for ZFS and Btrfs snapshotting and rollback workflows directly within its storage layer. This architecture ensures that user home directories, configuration trees, and package repositories are transaction-safe, allowing instantaneous system state restoration.

## Snapshot & Rollback Architecture
SigmaOS isolates system state using filesystem subvolumes and dataset snapshots. When a package is upgraded or a system configuration is updated via `sigpkg`, a filesystem snapshot is generated beforehand.

```
                  [Current Running State]
                             │
            ┌────────────────┴────────────────┐
            ▼                                 ▼
   [Btrfs Subvolume @]             [ZFS Dataset zpool/root]
            │                                 │
     (sigpkg snapshot)                (sigpkg snapshot)
            │                                 │
            ▼                                 ▼
[@_snapshot_20260707]           [zpool/root@snapshot_20260707]
```

## Configuration Specification
File systems are mounted with copy-on-write (CoW) configurations defined in `/etc/sigmafs.conf`.

Example configuration:
```toml
[filesystem.root]
type = "btrfs"
device = "UUID=sigma-root-uuid"
options = "compress=zstd:3,ssd,noatime,subvol=@"

[snapshots]
enabled = true
auto_create_before_upgrade = true
max_snapshots = 10
directory = "/.snapshots"
```

## Technical Implementation
The storage daemon (`sigmad-storage`) invokes low-level filesystem ioctls directly using Rust bindings to control snapshots, minimizing overhead and dependency on complex shell execution.

```rust
// kernel/fs/btrfs/sigma_btrfs.rs
pub fn create_subvolume_snapshot(src: &str, dest: &str) -> Result<(), io::Error> {
    // Low-level BTRFS_IOC_SNAP_CREATE_V2 ioctl call
    let src_file = File::open(src)?;
    let dest_dir = File::open(Path::new(dest).parent().unwrap())?;
    
    unsafe {
        let res = ioctl(dest_dir.as_raw_fd(), BTRFS_IOC_SNAP_CREATE_V2, &args);
        if res < 0 {
            return Err(io::Error::last_os_error());
        }
    }
    Ok(())
}
```

## Roadmap & Milestones
- **Phase 1 (Months 0-3)**: Implementation of raw Btrfs subvolume snapshot wrapper.
- **Phase 2 (Months 3-6)**: ZFS pool dataset integration and automatic CLI rollback options.
- **Phase 3 (Months 6-9)**: GUI rollback manager integrated with Zenith Desktop Control Center.
- **Phase 4 (Months 9-12)**: Self-healing integrity scanning and automated rollback on boot failure detection.
