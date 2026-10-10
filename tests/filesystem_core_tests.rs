#[path = "../src/filesystem/core.rs"]
mod core;

#[path = "../src/filesystem/mount.rs"]
mod mount;

use self::core::*;
use mount::*;

#[test]
fn test_filesystem_core_and_mount_conformance() {
    let mut fscore = FilesystemCoreManager::new();
    assert!(fscore.register_volume("data", 10000, 100).is_ok());

    assert!(fscore.allocate_space("data", 4000, 10).is_ok());

    let tx = fscore.begin_transaction("/data/file1.txt").unwrap();
    assert!(fscore.commit_transaction(tx, 1).is_ok());

    let mut mount_mgr = MountManager::new();
    let flags = CustomMountFlags::default();

    assert!(mount_mgr.mount("/dev/data", "/mnt/data", "ext4", flags).is_ok());
    assert!(mount_mgr.acquire_ref("/mnt/data").is_ok());
    assert_eq!(mount_mgr.unmount("/mnt/data"), Err(MountError::ResourceBusy));

    assert!(mount_mgr.release_ref("/mnt/data").is_ok());
    assert!(mount_mgr.unmount("/mnt/data").is_ok());
    assert_eq!(mount_mgr.unmount("/mnt/data"), Err(MountError::NotMounted));
}
