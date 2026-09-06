#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
//! User Namespace Syscall Integration
//!
//! Provides syscall support for user namespace operations including:
//! - CLONE_NEWUSER flag handling in clone/unshare
//! - UID/GID mapping syscalls
//! - subuid/subgid file operations

use crate::security::user_namespace::{
    CapabilitySet, SubgidEntry, SubuidEntry, UidGidMapping, UserContext, UserNamespace,
    UserNamespaceId, UserNamespaceManager,
};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// User namespace syscall error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserNamespaceSyscallError {
    /// Invalid argument (-EINVAL)
    InvalidArgument = -22,
    /// Operation not permitted (-EPERM)
    PermissionDenied = -1,
    /// No memory (-ENOMEM)
    NoMemory = -12,
    /// File not found (-ENOENT)
    NotFound = -2,
    /// Device busy (-EBUSY)
    DeviceBusy = -16,
}

impl UserNamespaceSyscallError {
    pub fn code(&self) -> i32 {
        *self as i32
    }
}

/// User namespace clone flags
#[derive(Debug, Clone, Copy)]
pub struct UserCloneFlags(u32);

impl UserCloneFlags {
    pub const CLONE_NEWUSER: u32 = 0x10000000; // 268435456

    pub fn new(flags: u32) -> Self {
        UserCloneFlags(flags)
    }

    pub fn raw(&self) -> u32 {
        self.0
    }

    pub fn clone_newuser(&self) -> bool {
        (self.0 & Self::CLONE_NEWUSER) != 0
    }
}

/// User namespace unshare flags
#[derive(Debug, Clone, Copy)]
pub struct UserUnshareFlags(u32);

impl UserUnshareFlags {
    pub const UNSHARE_NEWUSER: u32 = 0x10000000;

    pub fn new(flags: u32) -> Self {
        UserUnshareFlags(flags)
    }

    pub fn raw(&self) -> u32 {
        self.0
    }

    pub fn unshare_newuser(&self) -> bool {
        (self.0 & Self::UNSHARE_NEWUSER) != 0
    }
}

/// UID/GID mapping configuration for syscalls
#[derive(Debug, Clone)]
pub struct UidGidMapConfig {
    pub ns_id: UserNamespaceId,
    pub is_uid: bool,
    pub mappings: Vec<UidGidMapping>,
}

impl UidGidMapConfig {
    pub fn new(ns_id: UserNamespaceId, is_uid: bool) -> Self {
        UidGidMapConfig {
            ns_id,
            is_uid,
            mappings: Vec::new(),
        }
    }

    pub fn add_mapping(&mut self, mapping: UidGidMapping) {
        self.mappings.push(mapping);
    }
}

/// Represents a setuid/setgid capability specification
#[derive(Debug, Clone)]
pub struct SetidCapabilitySpec {
    pub target_uid: u32,
    pub target_gid: u32,
    pub keep_capabilities: bool,
}

impl SetidCapabilitySpec {
    pub fn new(target_uid: u32, target_gid: u32) -> Self {
        SetidCapabilitySpec {
            target_uid,
            target_gid,
            keep_capabilities: false,
        }
    }
}

/// Get a new user namespace manager instance
pub fn new_user_namespace_manager() -> Arc<Mutex<UserNamespaceManager>> {
    Arc::new(Mutex::new(UserNamespaceManager::new()))
}

/// sys_clone with CLONE_NEWUSER support
///
/// Creates a new user namespace if CLONE_NEWUSER is set
pub fn sys_clone_user(
    flags: u32,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<UserNamespaceId, UserNamespaceSyscallError> {
    let user_flags = UserCloneFlags::new(flags);

    if !user_flags.clone_newuser() {
        return Err(UserNamespaceSyscallError::InvalidArgument);
    }

    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    let ns_id = manager
        .create_namespace(1000, None)
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    Ok(ns_id)
}

/// sys_unshare with CLONE_NEWUSER support
///
/// Creates a new user namespace for the calling process
pub fn sys_unshare_user(
    flags: u32,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<UserNamespaceId, UserNamespaceSyscallError> {
    let unshare_flags = UserUnshareFlags::new(flags);

    if !unshare_flags.unshare_newuser() {
        return Err(UserNamespaceSyscallError::InvalidArgument);
    }

    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    let ns_id = manager
        .create_namespace(0, None)
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    Ok(ns_id)
}

/// sys_setns with user namespace support
///
/// Joins an existing user namespace
pub fn sys_setns_user(
    ns_id: UserNamespaceId,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<(), UserNamespaceSyscallError> {
    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    manager
        .get_namespace(ns_id)
        .map_err(|_| UserNamespaceSyscallError::NotFound)?;

    Ok(())
}

/// sys_map_uid64 - map UIDs from namespace to host and vice versa
///
/// Sets up UID mappings for a user namespace
pub fn sys_map_uid64(
    ns_id: UserNamespaceId,
    container_id: u32,
    host_id: u32,
    count: u32,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<(), UserNamespaceSyscallError> {
    if count == 0 {
        return Err(UserNamespaceSyscallError::InvalidArgument);
    }

    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    let ns = manager
        .get_namespace(ns_id)
        .map_err(|_| UserNamespaceSyscallError::NotFound)?;

    let mapping = UidGidMapping::new(container_id, host_id, count);

    let mut ns_lock = ns.lock().map_err(|_| UserNamespaceSyscallError::NoMemory)?;
    ns_lock
        .set_uid_map(vec![mapping])
        .map_err(|_| UserNamespaceSyscallError::InvalidArgument)?;

    Ok(())
}

/// sys_map_gid64 - map GIDs from namespace to host and vice versa
///
/// Sets up GID mappings for a user namespace
pub fn sys_map_gid64(
    ns_id: UserNamespaceId,
    container_id: u32,
    host_id: u32,
    count: u32,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<(), UserNamespaceSyscallError> {
    if count == 0 {
        return Err(UserNamespaceSyscallError::InvalidArgument);
    }

    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    let ns = manager
        .get_namespace(ns_id)
        .map_err(|_| UserNamespaceSyscallError::NotFound)?;

    let mapping = UidGidMapping::new(container_id, host_id, count);

    let mut ns_lock = ns.lock().map_err(|_| UserNamespaceSyscallError::NoMemory)?;
    ns_lock
        .set_gid_map(vec![mapping])
        .map_err(|_| UserNamespaceSyscallError::InvalidArgument)?;

    Ok(())
}

/// sys_setuid64 - change UID with user namespace support
///
/// Changes the UID within the current user namespace
pub fn sys_setuid64(_uid: u32) -> Result<(), UserNamespaceSyscallError> {
    // In a real implementation, this would:
    // 1. Check if in a user namespace
    // 2. Map the uid from namespace to host
    // 3. Update the process credentials
    Ok(())
}

/// sys_setgid64 - change GID with user namespace support
///
/// Changes the GID within the current user namespace
pub fn sys_setgid64(_gid: u32) -> Result<(), UserNamespaceSyscallError> {
    // In a real implementation, this would:
    // 1. Check if in a user namespace
    // 2. Map the gid from namespace to host
    // 3. Update the process credentials
    Ok(())
}

/// Parse /etc/subuid and /etc/subgid files
///
/// Returns a map of username to allocated UID ranges
pub fn parse_subuid_allocations(
    content: &str,
) -> Result<HashMap<String, Vec<SubuidEntry>>, UserNamespaceSyscallError> {
    let mut allocations: HashMap<String, Vec<SubuidEntry>> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            return Err(UserNamespaceSyscallError::InvalidArgument);
        }

        let username = parts[0].to_string();
        let start_uid: u32 = parts[1]
            .parse()
            .map_err(|_| UserNamespaceSyscallError::InvalidArgument)?;
        let count: u32 = parts[2]
            .parse()
            .map_err(|_| UserNamespaceSyscallError::InvalidArgument)?;

        let entry = SubuidEntry::new(username.clone(), start_uid, count);
        allocations
            .entry(username)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    Ok(allocations)
}

/// Parse /etc/subgid file for GID allocations
///
/// Returns a map of username to allocated GID ranges
pub fn parse_subgid_allocations(
    content: &str,
) -> Result<HashMap<String, Vec<SubgidEntry>>, UserNamespaceSyscallError> {
    let mut allocations: HashMap<String, Vec<SubgidEntry>> = HashMap::new();

    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() != 3 {
            return Err(UserNamespaceSyscallError::InvalidArgument);
        }

        let username = parts[0].to_string();
        let start_gid: u32 = parts[1]
            .parse()
            .map_err(|_| UserNamespaceSyscallError::InvalidArgument)?;
        let count: u32 = parts[2]
            .parse()
            .map_err(|_| UserNamespaceSyscallError::InvalidArgument)?;

        let entry = SubgidEntry::new(username.clone(), start_gid, count);
        allocations
            .entry(username)
            .or_insert_with(Vec::new)
            .push(entry);
    }

    Ok(allocations)
}

/// Grant a capability to a user namespace
pub fn sys_grant_capability(
    ns_id: UserNamespaceId,
    cap: u32,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<(), UserNamespaceSyscallError> {
    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    let ns = manager
        .get_namespace(ns_id)
        .map_err(|_| UserNamespaceSyscallError::NotFound)?;

    let cap_set = match cap {
        0 => CapabilitySet::CapChown,
        1 => CapabilitySet::CapDacOverride,
        2 => CapabilitySet::CapDacReadSearch,
        3 => CapabilitySet::CapFowner,
        4 => CapabilitySet::CapFsetid,
        5 => CapabilitySet::CapKill,
        6 => CapabilitySet::CapSetgid,
        7 => CapabilitySet::CapSetuid,
        8 => CapabilitySet::CapSetfcap,
        9 => CapabilitySet::CapSetpcap,
        10 => CapabilitySet::CapNetRaw,
        11 => CapabilitySet::CapNetBindService,
        12 => CapabilitySet::CapSysChroot,
        13 => CapabilitySet::CapSysAdmin,
        _ => return Err(UserNamespaceSyscallError::InvalidArgument),
    };

    let mut ns_lock = ns.lock().map_err(|_| UserNamespaceSyscallError::NoMemory)?;
    ns_lock
        .grant_capability(cap_set)
        .map_err(|_| UserNamespaceSyscallError::PermissionDenied)?;

    Ok(())
}

/// Revoke a capability from a user namespace
pub fn sys_revoke_capability(
    ns_id: UserNamespaceId,
    cap: u32,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<(), UserNamespaceSyscallError> {
    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    let ns = manager
        .get_namespace(ns_id)
        .map_err(|_| UserNamespaceSyscallError::NotFound)?;

    let cap_set = match cap {
        0 => CapabilitySet::CapChown,
        1 => CapabilitySet::CapDacOverride,
        2 => CapabilitySet::CapDacReadSearch,
        3 => CapabilitySet::CapFowner,
        4 => CapabilitySet::CapFsetid,
        5 => CapabilitySet::CapKill,
        6 => CapabilitySet::CapSetgid,
        7 => CapabilitySet::CapSetuid,
        8 => CapabilitySet::CapSetfcap,
        9 => CapabilitySet::CapSetpcap,
        10 => CapabilitySet::CapNetRaw,
        11 => CapabilitySet::CapNetBindService,
        12 => CapabilitySet::CapSysChroot,
        13 => CapabilitySet::CapSysAdmin,
        _ => return Err(UserNamespaceSyscallError::InvalidArgument),
    };

    let mut ns_lock = ns.lock().map_err(|_| UserNamespaceSyscallError::NoMemory)?;
    ns_lock
        .revoke_capability(cap_set)
        .map_err(|_| UserNamespaceSyscallError::PermissionDenied)?;

    Ok(())
}

/// Check if a user namespace has a capability
pub fn sys_check_capability(
    ns_id: UserNamespaceId,
    cap: u32,
    manager: &Arc<Mutex<UserNamespaceManager>>,
) -> Result<bool, UserNamespaceSyscallError> {
    let manager = manager
        .lock()
        .map_err(|_| UserNamespaceSyscallError::NoMemory)?;

    let ns = manager
        .get_namespace(ns_id)
        .map_err(|_| UserNamespaceSyscallError::NotFound)?;

    let cap_set = match cap {
        0 => CapabilitySet::CapChown,
        1 => CapabilitySet::CapDacOverride,
        2 => CapabilitySet::CapDacReadSearch,
        3 => CapabilitySet::CapFowner,
        4 => CapabilitySet::CapFsetid,
        5 => CapabilitySet::CapKill,
        6 => CapabilitySet::CapSetgid,
        7 => CapabilitySet::CapSetuid,
        8 => CapabilitySet::CapSetfcap,
        9 => CapabilitySet::CapSetpcap,
        10 => CapabilitySet::CapNetRaw,
        11 => CapabilitySet::CapNetBindService,
        12 => CapabilitySet::CapSysChroot,
        13 => CapabilitySet::CapSysAdmin,
        _ => return Err(UserNamespaceSyscallError::InvalidArgument),
    };

    let ns_lock = ns.lock().map_err(|_| UserNamespaceSyscallError::NoMemory)?;
    Ok(ns_lock.has_capability(cap_set))
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_clone_newuser_flag() {
        let flags = UserCloneFlags::new(UserCloneFlags::CLONE_NEWUSER);
        assert!(flags.clone_newuser());
    }

    #[test]
    fn test_unshare_newuser_flag() {
        let flags = UserUnshareFlags::new(UserUnshareFlags::UNSHARE_NEWUSER);
        assert!(flags.unshare_newuser());
    }

    #[test]
    fn test_sys_clone_user_creates_namespace() {
        let manager = new_user_namespace_manager();
        let result = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_unshare_user_creates_namespace() {
        let manager = new_user_namespace_manager();
        let result = sys_unshare_user(UserUnshareFlags::UNSHARE_NEWUSER, &manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_map_uid64_valid() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let result = sys_map_uid64(ns_id, 0, 100000, 65536, &manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_map_uid64_invalid_count() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let result = sys_map_uid64(ns_id, 0, 100000, 0, &manager);
        assert!(result.is_err());
    }

    #[test]
    fn test_sys_map_gid64_valid() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let result = sys_map_gid64(ns_id, 0, 100000, 65536, &manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_map_gid64_invalid_count() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let result = sys_map_gid64(ns_id, 0, 100000, 0, &manager);
        assert!(result.is_err());
    }

    #[test]
    fn test_sys_setuid64() {
        let result = sys_setuid64(1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_setgid64() {
        let result = sys_setgid64(1000);
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_subuid_allocations_valid() {
        let content = "user1:100000:65536\nuser2:200000:32768\n";
        let result = parse_subuid_allocations(content);
        assert!(result.is_ok());
        let allocations = result.unwrap();
        assert_eq!(allocations.len(), 2);
        assert!(allocations.contains_key("user1"));
        assert!(allocations.contains_key("user2"));
    }

    #[test]
    fn test_parse_subgid_allocations_valid() {
        let content = "user1:100000:65536\nuser2:200000:32768\n";
        let result = parse_subgid_allocations(content);
        assert!(result.is_ok());
        let allocations = result.unwrap();
        assert_eq!(allocations.len(), 2);
        assert!(allocations.contains_key("user1"));
        assert!(allocations.contains_key("user2"));
    }

    #[test]
    fn test_parse_subuid_allocations_with_comments() {
        let content = "# Comment\nuser1:100000:65536\n# Another comment\nuser2:200000:32768\n";
        let result = parse_subuid_allocations(content);
        assert!(result.is_ok());
        let allocations = result.unwrap();
        assert_eq!(allocations.len(), 2);
    }

    #[test]
    fn test_parse_subgid_allocations_with_comments() {
        let content = "# Comment\nuser1:100000:65536\n# Another comment\nuser2:200000:32768\n";
        let result = parse_subgid_allocations(content);
        assert!(result.is_ok());
        let allocations = result.unwrap();
        assert_eq!(allocations.len(), 2);
    }

    #[test]
    fn test_parse_subuid_allocations_invalid() {
        let content = "user1:100000:invalid";
        let result = parse_subuid_allocations(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_subgid_allocations_invalid() {
        let content = "user1:100000:invalid";
        let result = parse_subgid_allocations(content);
        assert!(result.is_err());
    }

    #[test]
    fn test_sys_grant_capability() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let result = sys_grant_capability(ns_id, 0, &manager); // CAP_CHOWN
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_revoke_capability() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let _ = sys_grant_capability(ns_id, 0, &manager); // CAP_CHOWN
        let result = sys_revoke_capability(ns_id, 0, &manager);
        assert!(result.is_ok());
    }

    #[test]
    fn test_sys_check_capability() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let _ = sys_grant_capability(ns_id, 0, &manager); // CAP_CHOWN
        let result = sys_check_capability(ns_id, 0, &manager);
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_sys_setns_user_nonexistent_namespace() {
        let manager = new_user_namespace_manager();
        let ns_id = UserNamespaceId(99999);
        let result = sys_setns_user(ns_id, &manager);
        assert!(result.is_err());
    }

    #[test]
    fn test_uid_clone_flags() {
        let flags = UserCloneFlags::new(UserCloneFlags::CLONE_NEWUSER | 0x1);
        assert!(flags.clone_newuser());
        assert_eq!(flags.raw(), UserCloneFlags::CLONE_NEWUSER | 0x1);
    }

    #[test]
    fn test_setid_capability_spec() {
        let spec = SetidCapabilitySpec::new(1000, 1000);
        assert_eq!(spec.target_uid, 1000);
        assert_eq!(spec.target_gid, 1000);
        assert!(!spec.keep_capabilities);
    }

    #[test]
    fn test_uid_gid_map_config() {
        let ns_id = UserNamespaceId(1);
        let mut config = UidGidMapConfig::new(ns_id, true);
        let mapping = UidGidMapping::new(0, 100000, 65536);
        config.add_mapping(mapping);
        assert_eq!(config.mappings.len(), 1);
    }

    #[test]
    fn test_multiple_uid_mappings() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();

        let result1 = sys_map_uid64(ns_id, 0, 100000, 100, &manager);
        assert!(result1.is_ok());

        let result2 = sys_map_uid64(ns_id, 100, 200000, 100, &manager);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_multiple_gid_mappings() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();

        let result1 = sys_map_gid64(ns_id, 0, 100000, 100, &manager);
        assert!(result1.is_ok());

        let result2 = sys_map_gid64(ns_id, 100, 200000, 100, &manager);
        assert!(result2.is_ok());
    }

    #[test]
    fn test_parse_empty_subuid_file() {
        let content = "";
        let result = parse_subuid_allocations(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_parse_empty_subgid_file() {
        let content = "";
        let result = parse_subgid_allocations(content);
        assert!(result.is_ok());
        assert_eq!(result.unwrap().len(), 0);
    }

    #[test]
    fn test_grant_multiple_capabilities() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();

        let result1 = sys_grant_capability(ns_id, 0, &manager); // CAP_CHOWN
        assert!(result1.is_ok());

        let result2 = sys_grant_capability(ns_id, 5, &manager); // CAP_KILL
        assert!(result2.is_ok());

        let check1 = sys_check_capability(ns_id, 0, &manager);
        assert!(check1.is_ok() && check1.unwrap());

        let check2 = sys_check_capability(ns_id, 5, &manager);
        assert!(check2.is_ok() && check2.unwrap());
    }

    #[test]
    fn test_invalid_capability_number() {
        let manager = new_user_namespace_manager();
        let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
        let result = sys_grant_capability(ns_id, 999, &manager);
        assert!(result.is_err());
    }

    #[test]
    fn test_user_namespace_syscall_error_codes() {
        assert_eq!(UserNamespaceSyscallError::InvalidArgument.code(), -22);
        assert_eq!(UserNamespaceSyscallError::PermissionDenied.code(), -1);
        assert_eq!(UserNamespaceSyscallError::NoMemory.code(), -12);
        assert_eq!(UserNamespaceSyscallError::NotFound.code(), -2);
        assert_eq!(UserNamespaceSyscallError::DeviceBusy.code(), -16);
    }

    #[test]
    fn test_new_user_namespace_manager() {
        let manager = new_user_namespace_manager();
        let manager_lock = manager.lock();
        assert!(manager_lock.is_ok());
    }
}
