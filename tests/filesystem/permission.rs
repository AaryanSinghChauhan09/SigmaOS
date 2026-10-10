// Standalone Unit Tests for Permission Engine
// Part of SigmaOS File Management Subsystem

#[path = "../../src/fs/fd.rs"]
mod fd;
#[path = "../../src/fs/permission.rs"]
mod permission;

use fd::FsError;
use permission::{AccessMode, PermissionEngine, ProcessCredentials, CAP_DAC_OVERRIDE};

#[test]
fn test_permission_check_owner_group_other() {
    let mode = 0o750; // rwxr-x---

    let owner_creds = ProcessCredentials::unprivileged(1000, 1000);
    assert!(PermissionEngine::check_access(&owner_creds, 1000, 1000, mode, AccessMode::Read).is_ok());
    assert!(PermissionEngine::check_access(&owner_creds, 1000, 1000, mode, AccessMode::Write).is_ok());
    assert!(PermissionEngine::check_access(&owner_creds, 1000, 1000, mode, AccessMode::Execute).is_ok());

    let group_creds = ProcessCredentials::unprivileged(2000, 1000);
    assert!(PermissionEngine::check_access(&group_creds, 1000, 1000, mode, AccessMode::Read).is_ok());
    assert!(PermissionEngine::check_access(&group_creds, 1000, 1000, mode, AccessMode::Execute).is_ok());
    assert_eq!(
        PermissionEngine::check_access(&group_creds, 1000, 1000, mode, AccessMode::Write).err(),
        Some(FsError::PermissionDenied)
    );

    let other_creds = ProcessCredentials::unprivileged(3000, 3000);
    assert_eq!(
        PermissionEngine::check_access(&other_creds, 1000, 1000, mode, AccessMode::Read).err(),
        Some(FsError::PermissionDenied)
    );
}

#[test]
fn test_root_and_capability_override() {
    let mode = 0o000;
    let root_creds = ProcessCredentials::root();
    assert!(PermissionEngine::check_access(&root_creds, 1000, 1000, mode, AccessMode::ReadWrite).is_ok());

    let mut cap_creds = ProcessCredentials::unprivileged(5000, 5000);
    cap_creds.capabilities |= CAP_DAC_OVERRIDE;
    assert!(PermissionEngine::check_access(&cap_creds, 1000, 1000, mode, AccessMode::ReadWrite).is_ok());
}
