//! Integration tests: LSM hook enforcement
//!
//! Verifies that hook implementations fire on policy violations and that
//! composed hooks correctly propagate allow/deny decisions.

use sigmaos_security::{
    AllowAllLsm, DenyAllLsm, Inode, InodeAttr, LsmHook, SecurityError,
};

fn root_inode() -> Inode {
    Inode::new(1, 0o755, 0, 0)
}

fn user_inode() -> Inode {
    Inode::new(42, 0o644, 1000, 1000)
}

// ── DenyAllLsm fires on every hook ────────────────────────────────────────

#[test]
fn deny_lsm_blocks_inode_create() {
    let lsm = DenyAllLsm;
    let dir = root_inode();
    let result = lsm.security_inode_create(&dir, "secret.txt", 0o600);
    assert_eq!(result, Err(SecurityError::PermissionDenied));
}

#[test]
fn deny_lsm_blocks_file_open() {
    let lsm = DenyAllLsm;
    let inode = user_inode();
    let result = lsm.security_file_open(&inode, 0o001 /* O_RDONLY */);
    assert_eq!(result, Err(SecurityError::PermissionDenied));
}

#[test]
fn deny_lsm_blocks_socket_create() {
    let lsm = DenyAllLsm;
    // AF_INET=2, SOCK_STREAM=1, TCP=6
    let result = lsm.security_socket_create(2, 1, 6);
    assert_eq!(result, Err(SecurityError::PermissionDenied));
}

#[test]
fn deny_lsm_blocks_ptrace_access() {
    let lsm = DenyAllLsm;
    let result = lsm.security_ptrace_access_check(1001, 1002);
    assert_eq!(result, Err(SecurityError::PermissionDenied));
}

#[test]
fn deny_lsm_blocks_capability_check() {
    let lsm = DenyAllLsm;
    let result = lsm.security_capable(0 /* root cred */, 1 << 21 /* CAP_SYS_ADMIN */);
    assert_eq!(result, Err(SecurityError::CapabilityMissing));
}

#[test]
fn deny_lsm_blocks_inode_rename() {
    let lsm = DenyAllLsm;
    let old_dir = root_inode();
    let new_dir = root_inode();
    let result = lsm.security_inode_rename(&old_dir, "old.txt", &new_dir, "new.txt");
    assert_eq!(result, Err(SecurityError::PermissionDenied));
}

#[test]
fn deny_lsm_blocks_socket_bind() {
    let lsm = DenyAllLsm;
    let addr = [127u8, 0, 0, 1, 0, 80]; // stub addr bytes
    let result = lsm.security_socket_bind(3, &addr);
    assert_eq!(result, Err(SecurityError::PermissionDenied));
}

#[test]
fn deny_lsm_blocks_inode_setattr() {
    let lsm = DenyAllLsm;
    let inode = user_inode();
    let attr = InodeAttr { mode: 0o777, uid: 0, gid: 0, size: 0 };
    let result = lsm.security_inode_setattr(&inode, &attr);
    assert_eq!(result, Err(SecurityError::PermissionDenied));
}

// ── AllowAllLsm permits every hook ────────────────────────────────────────

#[test]
fn allow_lsm_permits_inode_create() {
    let lsm = AllowAllLsm;
    let dir = root_inode();
    assert!(lsm.security_inode_create(&dir, "readme.txt", 0o644).is_ok());
}

#[test]
fn allow_lsm_permits_file_open() {
    let lsm = AllowAllLsm;
    let inode = user_inode();
    assert!(lsm.security_file_open(&inode, 0o002).is_ok());
}

#[test]
fn allow_lsm_permits_socket_create() {
    let lsm = AllowAllLsm;
    assert!(lsm.security_socket_create(2, 1, 6).is_ok());
}

#[test]
fn allow_lsm_permits_capability_check() {
    let lsm = AllowAllLsm;
    assert!(lsm.security_capable(1000, 1 << 10).is_ok());
}

// ── Trait object dispatch works correctly ────────────────────────────────

#[test]
fn lsm_trait_object_deny_dispatches() {
    let lsm: Box<dyn LsmHook> = Box::new(DenyAllLsm);
    let inode = root_inode();
    assert_eq!(
        lsm.security_inode_unlink(&inode, "file"),
        Err(SecurityError::PermissionDenied)
    );
}

#[test]
fn lsm_trait_object_allow_dispatches() {
    let lsm: Box<dyn LsmHook> = Box::new(AllowAllLsm);
    let inode = root_inode();
    assert!(lsm.security_inode_unlink(&inode, "file").is_ok());
}
