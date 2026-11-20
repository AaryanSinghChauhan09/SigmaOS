//! Mount/unmount operation tests for sigmaos-filesystem.

use sigmaos_filesystem::mount::{MountFlags, MountManager, MountError};

fn flags() -> MountFlags { MountFlags::default() }

/// A fresh manager has no mounts.
#[test]
fn test_manager_starts_empty() {
    let mgr = MountManager::new();
    assert!(mgr.list_mounts().is_empty());
}

/// Mounting tmpfs records it and list_mounts returns the path.
#[test]
fn test_mount_tmpfs() {
    let mut mgr = MountManager::new();
    mgr.mount("none", "/tmp", "tmpfs", flags()).expect("tmpfs mount should succeed");
    let mounts = mgr.list_mounts();
    assert!(mounts.contains(&"/tmp"), "expected /tmp in mount list");
}

/// Unmounting removes the entry from the list.
#[test]
fn test_unmount_removes_entry() {
    let mut mgr = MountManager::new();
    mgr.mount("none", "/tmp", "tmpfs", flags()).unwrap();
    mgr.unmount("/tmp").expect("unmount should succeed");
    assert!(mgr.list_mounts().is_empty());
}

/// Unmounting a path that was never mounted returns NotMounted.
#[test]
fn test_unmount_not_mounted_returns_error() {
    let mut mgr = MountManager::new();
    let result = mgr.unmount("/nonexistent");
    assert_eq!(result, Err(MountError::NotMounted));
}

/// Mounting the same path twice returns AlreadyMounted.
#[test]
fn test_duplicate_mount_rejected() {
    let mut mgr = MountManager::new();
    mgr.mount("none", "/mnt", "tmpfs", flags()).unwrap();
    let result = mgr.mount("none", "/mnt", "ext4", flags());
    assert_eq!(result, Err(MountError::AlreadyMounted));
}

/// Multiple distinct mount points coexist independently.
#[test]
fn test_multiple_mount_points() {
    let mut mgr = MountManager::new();
    for path in &["/tmp", "/home", "/var", "/proc"] {
        mgr.mount("none", path, "tmpfs", flags()).expect("mount should succeed");
    }
    assert_eq!(mgr.list_mounts().len(), 4);
}

/// Unmounting one entry leaves the others intact.
#[test]
fn test_unmount_one_of_many() {
    let mut mgr = MountManager::new();
    for path in &["/a", "/b", "/c"] {
        mgr.mount("none", path, "tmpfs", flags()).unwrap();
    }
    mgr.unmount("/b").unwrap();
    let mounts = mgr.list_mounts();
    assert_eq!(mounts.len(), 2);
    assert!(mounts.contains(&"/a"));
    assert!(mounts.contains(&"/c"));
    assert!(!mounts.contains(&"/b"));
}

/// acquire_ref prevents unmounting a busy mount.
#[test]
fn test_busy_mount_cannot_be_unmounted() {
    let mut mgr = MountManager::new();
    mgr.mount("none", "/busy", "tmpfs", flags()).unwrap();
    mgr.acquire_ref("/busy").unwrap();
    let result = mgr.unmount("/busy");
    assert_eq!(result, Err(MountError::ResourceBusy));
}

/// Release a busy ref, then unmounting succeeds.
#[test]
fn test_release_ref_then_unmount_succeeds() {
    let mut mgr = MountManager::new();
    mgr.mount("none", "/rel", "tmpfs", flags()).unwrap();
    mgr.acquire_ref("/rel").unwrap();
    mgr.release_ref("/rel").unwrap();
    mgr.unmount("/rel").expect("should unmount after ref released");
}

/// mount_with_rollback removes entry when init_fn returns Err.
#[test]
fn test_rollback_on_init_failure() {
    let mut mgr = MountManager::new();
    let result = mgr.mount_with_rollback(
        "none", "/fail", "tmpfs", flags(), || Err(()),
    );
    assert_eq!(result, Err(MountError::RollbackSuccessful));
    assert!(mgr.list_mounts().is_empty(), "rolled-back mount should not appear");
}

/// mount_with_rollback succeeds when init_fn returns Ok.
#[test]
fn test_rollback_not_triggered_on_success() {
    let mut mgr = MountManager::new();
    mgr.mount_with_rollback(
        "none", "/ok", "tmpfs", flags(), || Ok(()),
    ).expect("should succeed");
    assert!(mgr.list_mounts().contains(&"/ok"));
}
