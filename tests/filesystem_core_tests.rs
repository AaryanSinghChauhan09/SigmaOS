// SPDX-License-Identifier: MIT
//! Filesystem Core and Mount Manager Unit Tests for SigmaOS.

#[path = "../src/filesystem/core.rs"]
mod core;

#[path = "../src/filesystem/mount.rs"]
mod mount;

use core::{FilesystemCoreManager, FsCoreError};
use mount::{MountError, MountManager};

#[test]
fn test_filesystem_core_capacity_limits() {
    let mut fs = FilesystemCoreManager::new("rootfs", 100);

    let id1 = fs.create_file("file1.txt", 60).unwrap();
    assert_eq!(fs.used_capacity_bytes, 60);

    // Attempting to create a file exceeding capacity returns CapacityExceeded
    assert_eq!(fs.create_file("file2.txt", 50), Err(FsCoreError::CapacityExceeded));

    // Delete file1 and verify capacity reclaim
    assert!(fs.delete_file(id1).is_ok());
    assert_eq!(fs.used_capacity_bytes, 0);

    // Now file2 can be created
    assert!(fs.create_file("file2.txt", 50).is_ok());
    assert_eq!(fs.used_capacity_bytes, 50);
}

#[test]
fn test_mount_manager_integrity_and_double_unmount() {
    let mut mm = MountManager::new();

    let mount_id = mm.mount("/dev/vda1", "/mnt", "sigmafs").unwrap();
    assert_eq!(mount_id, 1);

    // Duplicate mount on same target
    assert_eq!(mm.mount("/dev/vdb1", "/mnt", "ext4"), Err(MountError::AlreadyMounted));

    // Active reference test
    mm.mount_table.get_mut("/mnt").unwrap().active_references = 2;
    assert_eq!(mm.unmount("/mnt"), Err(MountError::Busy));

    // Unmount after releasing reference
    assert!(mm.unmount("/mnt").is_ok());

    // Double unmount prevention
    assert_eq!(mm.unmount("/mnt"), Err(MountError::DoubleUnmount));
}
