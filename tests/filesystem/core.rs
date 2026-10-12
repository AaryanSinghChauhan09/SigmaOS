#[path = "../../src/filesystem/core.rs"]
mod core;
#[path = "../../src/filesystem/mount.rs"]
mod mount;

use core::{FilesystemCoreManager, FsCoreError, FsHealthStatus};
use mount::{CustomMountFlags, MountError, MountManager, MountState};

#[test]
fn test_filesystem_core_capacity_and_lost_update_prevention() {
    let mut fs = FilesystemCoreManager::new();
    assert!(fs.register_volume("rootvol", 1000, 100).is_ok());
    assert_eq!(
        fs.health_map.get("rootvol"),
        Some(&FsHealthStatus::Healthy)
    );

    // Allocate 512 bytes -> succeeds
    assert!(fs.allocate_space("rootvol", 512, 10).is_ok());

    // Allocate another 600 bytes -> exceeds capacity limit (1000 - 512 = 488 left)
    let err = fs.allocate_space("rootvol", 600, 10);
    assert_eq!(err, Err(FsCoreError::CapacityExceeded));

    // Transaction logging for lost update prevention
    let tx_id = fs.begin_transaction("/etc/config").unwrap();
    assert!(tx_id > 0);
    assert_eq!(
        fs.commit_transaction(tx_id, 999),
        Err(FsCoreError::LostUpdateDetected)
    );
    assert!(fs.commit_transaction(tx_id, 1).is_ok());
}

#[test]
fn test_mount_lifecycle_and_double_unmount_prevention() {
    let mut mount_mgr = MountManager::new();

    let flags = CustomMountFlags {
        read_only: false,
        no_exec: false,
        no_suid: false,
        sync: true,
        bind: false,
    };

    assert!(mount_mgr
        .mount("/dev/sda1", "/mnt/data", "ext4", flags)
        .is_ok());

    let entry = mount_mgr.mounts.get("/mnt/data").unwrap();
    assert_eq!(entry.state, MountState::Mounted);

    // Double mount check
    let err = mount_mgr.mount("/dev/sda1", "/mnt/data", "ext4", flags);
    assert_eq!(err, Err(MountError::AlreadyMounted));

    // Unmount
    assert!(mount_mgr.unmount("/mnt/data").is_ok());

    // Double unmount check
    let err2 = mount_mgr.unmount("/mnt/data");
    assert_eq!(err2, Err(MountError::NotMounted));
}
