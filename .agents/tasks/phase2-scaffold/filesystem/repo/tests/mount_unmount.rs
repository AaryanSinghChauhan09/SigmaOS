//! Integration tests: mount / unmount operations.
//!
//! Covers MountManager lifecycle, rollback, ref-count enforcement, and
//! the VfsManager mount table.

use sigmaos_filesystem::{
    MountError, MountFlags, MountManager, MountState,
    VfsMountFlags, VfsManager,
    mount, unmount,
};

// ---------------------------------------------------------------------------
// MountManager tests
// ---------------------------------------------------------------------------

#[test]
fn mount_and_unmount_basic() {
    let mut mgr = MountManager::new();
    let flags = MountFlags::default();

    mgr.mount("/dev/sda1", "/", "ext4", flags).expect("mount should succeed");
    mgr.unmount("/").expect("unmount should succeed");
}

#[test]
fn double_mount_returns_already_mounted() {
    let mut mgr = MountManager::new();
    let flags = MountFlags::default();

    mgr.mount("/dev/sda1", "/", "ext4", flags).unwrap();
    let err = mgr.mount("/dev/sda2", "/", "ext4", flags);
    assert_eq!(err, Err(MountError::AlreadyMounted));
    mgr.unmount("/").unwrap();
}

#[test]
fn double_unmount_returns_not_mounted() {
    let mut mgr = MountManager::new();
    let flags = MountFlags::default();

    mgr.mount("/dev/sda1", "/mnt/test", "ext4", flags).unwrap();
    mgr.unmount("/mnt/test").unwrap();
    assert_eq!(mgr.unmount("/mnt/test"), Err(MountError::NotMounted));
}

#[test]
fn unmount_busy_mount_returns_resource_busy() {
    let mut mgr = MountManager::new();
    let flags = MountFlags::default();

    mgr.mount("/dev/sda1", "/mnt/data", "ext4", flags).unwrap();
    mgr.acquire_ref("/mnt/data").unwrap();
    assert_eq!(mgr.unmount("/mnt/data"), Err(MountError::ResourceBusy));

    // Release the reference, then unmount should succeed.
    mgr.release_ref("/mnt/data").unwrap();
    mgr.unmount("/mnt/data").unwrap();
}

#[test]
fn rollback_on_init_failure() {
    let mut mgr = MountManager::new();
    let flags = MountFlags::default();

    let result = mgr.mount_with_rollback("/dev/sdb", "/mnt/fail", "zfs", flags, || Err(()));
    assert_eq!(result, Err(MountError::RollbackSuccessful));
    // The entry must NOT be present after rollback.
    assert!(!mgr.mounts.contains_key("/mnt/fail"));
}

#[test]
fn rollback_success_path() {
    let mut mgr = MountManager::new();
    let flags = MountFlags::default();

    mgr.mount_with_rollback("/dev/sdb", "/mnt/ok", "ext4", flags, || Ok(())).unwrap();
    assert!(mgr.mounts.contains_key("/mnt/ok"));
    mgr.unmount("/mnt/ok").unwrap();
}

#[test]
fn list_mounts_reflects_active_mounts() {
    let mut mgr = MountManager::new();
    let flags = MountFlags::default();

    mgr.mount("/dev/sda1", "/",     "ext4", flags).unwrap();
    mgr.mount("/dev/sda2", "/home", "ext4", flags).unwrap();
    let mounts = mgr.list_mounts();
    assert_eq!(mounts.len(), 2);
    mgr.unmount("/").unwrap();
    mgr.unmount("/home").unwrap();
}

#[test]
fn free_standing_mount_unmount_helpers() {
    let mut mgr = MountManager::new();
    mount(&mut mgr, "/dev/sda1", "/mnt/helper", "ext4", MountFlags::default()).unwrap();
    unmount(&mut mgr, "/mnt/helper").unwrap();
}

// ---------------------------------------------------------------------------
// VfsManager (mount table) tests
// ---------------------------------------------------------------------------

#[test]
fn vfs_manager_mount_and_lookup() {
    let mut vfs = VfsManager::new();

    vfs.mount(
        "/dev/nvme0n1p1".into(),
        "/".into(),
        "ext4".into(),
        VfsMountFlags::READ_WRITE,
    ).unwrap();

    assert!(vfs.lookup("/etc/passwd").is_ok());
    assert!(vfs.lookup("/home/user/.bashrc").is_ok());
}

#[test]
fn vfs_manager_unmount_removes_entry() {
    let mut vfs = VfsManager::new();

    vfs.mount("/dev/sda1".into(), "/mnt/data".into(), "ext4".into(), VfsMountFlags::READ_WRITE).unwrap();
    vfs.unmount("/mnt/data").unwrap();
    assert!(vfs.lookup("/mnt/data/file.txt").is_err());
}

#[test]
fn vfs_manager_deepest_mount_wins() {
    let mut vfs = VfsManager::new();

    vfs.mount("/dev/sda1".into(), "/".into(),    "ext4".into(), VfsMountFlags::READ_WRITE).unwrap();
    vfs.mount("/dev/sda2".into(), "/home".into(), "ext4".into(), VfsMountFlags::READ_WRITE).unwrap();

    let m = vfs.lookup("/home/user/file.txt").unwrap();
    assert_eq!(m.mount_path, "/home");
}

#[test]
fn vfs_normalize_path() {
    let vfs = VfsManager::new();
    assert_eq!(vfs.normalize_path("/foo/./bar/../baz"), "/foo/baz");
    assert_eq!(vfs.normalize_path("/a/b/c/../../d"),    "/a/d");
}
