//! Comprehensive User Namespace Tests
//!
//! Tests for Phase 9.3 user namespace implementation
//! covering core structures, UID/GID mapping, subuid/subgid support,
//! and syscall integration.

use sigmaos::security::user_namespace::{
    CapabilitySet, SubgidEntry, SubuidEntry, UidGidMapping, UserContext, UserNamespace,
    UserNamespaceId, UserNamespaceManager, parse_subuid_file, parse_subgid_file,
    SubuidAllocationTracker,
};
use sigmaos::syscall::user_syscalls::{
    UserCloneFlags, UserUnshareFlags, new_user_namespace_manager, sys_clone_user,
    sys_unshare_user, sys_map_uid64, sys_map_gid64, sys_setuid64, sys_setgid64,
    parse_subuid_allocations, parse_subgid_allocations, sys_grant_capability,
    sys_revoke_capability, sys_check_capability, sys_setns_user, UidGidMapConfig,
    SetidCapabilitySpec, UserNamespaceSyscallError,
};

// ============================================================================
// TASK 9.3.1 - Core Structures Tests
// ============================================================================

#[test]
fn test_9_3_1_user_namespace_id_creation() {
    let ns_id = UserNamespaceId(1);
    assert_eq!(ns_id.0, 1);
    
    let ns_id2 = UserNamespaceId(42);
    assert_eq!(ns_id2.0, 42);
}

#[test]
fn test_9_3_1_capability_set_values() {
    assert_eq!(CapabilitySet::CapChown as i32, 0);
    assert_eq!(CapabilitySet::CapKill as i32, 5);
    assert_eq!(CapabilitySet::CapSetuid as i32, 7);
    assert_eq!(CapabilitySet::CapSysAdmin as i32, 13);
}

#[test]
fn test_9_3_1_user_context_creation() {
    let ctx = UserContext::new(1000, 1000);
    assert_eq!(ctx.uid, 1000);
    assert_eq!(ctx.gid, 1000);
}

#[test]
fn test_9_3_1_user_context_add_group() {
    let mut ctx = UserContext::new(1000, 1000);
    assert!(ctx.add_group(1001).is_ok());
    assert!(ctx.groups.contains(&1001));
}

#[test]
fn test_9_3_1_user_namespace_creation() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).expect("Failed to create namespace");
    assert!(ns_id.0 > 0);
}

#[test]
fn test_9_3_1_get_namespace() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).expect("Failed to get namespace");
    let ns_lock = ns.lock().unwrap();
    assert_eq!(ns_lock.id, ns_id);
    assert_eq!(ns_lock.owner_uid, 1000);
}

#[test]
fn test_9_3_1_delete_namespace() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    assert!(manager.delete_namespace(ns_id).is_ok());
    assert!(manager.get_namespace(ns_id).is_err());
}

#[test]
fn test_9_3_1_list_namespaces() {
    let manager = UserNamespaceManager::new();
    let ns1 = manager.create_namespace(1000, None).unwrap();
    let ns2 = manager.create_namespace(2000, None).unwrap();
    let ns3 = manager.create_namespace(3000, None).unwrap();

    let list = manager.list_namespaces().unwrap();
    assert_eq!(list.len(), 3);
    assert!(list.contains(&ns1));
    assert!(list.contains(&ns2));
    assert!(list.contains(&ns3));
}

#[test]
fn test_9_3_1_namespace_count() {
    let manager = UserNamespaceManager::new();
    assert_eq!(manager.count().unwrap(), 0);

    manager.create_namespace(1000, None).unwrap();
    assert_eq!(manager.count().unwrap(), 1);

    manager.create_namespace(2000, None).unwrap();
    assert_eq!(manager.count().unwrap(), 2);
}

#[test]
fn test_9_3_1_parent_namespace_reference() {
    let manager = UserNamespaceManager::new();
    let parent_id = manager.create_namespace(1000, None).unwrap();
    let child_id = manager.create_namespace(2000, Some(parent_id)).unwrap();

    let parent_ns = manager.get_namespace(parent_id).unwrap();
    let child_ns = manager.get_namespace(child_id).unwrap();

    let child_lock = child_ns.lock().unwrap();
    assert_eq!(child_lock.parent_id, Some(parent_id));
}

// ============================================================================
// TASK 9.3.2 - UID/GID Mapping Tests
// ============================================================================

#[test]
fn test_9_3_2_uid_gid_mapping_creation() {
    let mapping = UidGidMapping::new(0, 100000, 65536);
    assert_eq!(mapping.container_id, 0);
    assert_eq!(mapping.host_id, 100000);
    assert_eq!(mapping.count, 65536);
}

#[test]
fn test_9_3_2_mapping_contains_container_id() {
    let mapping = UidGidMapping::new(0, 100000, 100);

    assert!(mapping.contains_container_id(0));
    assert!(mapping.contains_container_id(50));
    assert!(mapping.contains_container_id(99));
    assert!(!mapping.contains_container_id(100));
    assert!(!mapping.contains_container_id(101));
}

#[test]
fn test_9_3_2_mapping_contains_host_id() {
    let mapping = UidGidMapping::new(0, 100000, 100);

    assert!(mapping.contains_host_id(100000));
    assert!(mapping.contains_host_id(100050));
    assert!(mapping.contains_host_id(100099));
    assert!(!mapping.contains_host_id(99999));
    assert!(!mapping.contains_host_id(100100));
}

#[test]
fn test_9_3_2_set_uid_map_single() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping = UidGidMapping::new(0, 100000, 65536);
    let mut ns_lock = ns.lock().unwrap();
    assert!(ns_lock.set_uid_map(vec![mapping]).is_ok());
    assert_eq!(ns_lock.uid_map.len(), 1);
}

#[test]
fn test_9_3_2_set_gid_map_single() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping = UidGidMapping::new(0, 100000, 65536);
    let mut ns_lock = ns.lock().unwrap();
    assert!(ns_lock.set_gid_map(vec![mapping]).is_ok());
    assert_eq!(ns_lock.gid_map.len(), 1);
}

#[test]
fn test_9_3_2_map_uid_ns_to_host() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping = UidGidMapping::new(0, 100000, 65536);
    let mut ns_lock = ns.lock().unwrap();
    ns_lock.set_uid_map(vec![mapping]).unwrap();

    assert_eq!(ns_lock.map_uid_ns_to_host(0).unwrap(), 100000);
    assert_eq!(ns_lock.map_uid_ns_to_host(500).unwrap(), 100500);
    assert_eq!(ns_lock.map_uid_ns_to_host(65535).unwrap(), 165535);
}

#[test]
fn test_9_3_2_map_uid_host_to_ns() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping = UidGidMapping::new(0, 100000, 65536);
    let mut ns_lock = ns.lock().unwrap();
    ns_lock.set_uid_map(vec![mapping]).unwrap();

    assert_eq!(ns_lock.map_uid_host_to_ns(100000).unwrap(), 0);
    assert_eq!(ns_lock.map_uid_host_to_ns(100500).unwrap(), 500);
    assert_eq!(ns_lock.map_uid_host_to_ns(165535).unwrap(), 65535);
}

#[test]
fn test_9_3_2_map_gid_ns_to_host() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping = UidGidMapping::new(0, 100000, 65536);
    let mut ns_lock = ns.lock().unwrap();
    ns_lock.set_gid_map(vec![mapping]).unwrap();

    assert_eq!(ns_lock.map_gid_ns_to_host(0).unwrap(), 100000);
    assert_eq!(ns_lock.map_gid_ns_to_host(500).unwrap(), 100500);
}

#[test]
fn test_9_3_2_map_gid_host_to_ns() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping = UidGidMapping::new(0, 100000, 65536);
    let mut ns_lock = ns.lock().unwrap();
    ns_lock.set_gid_map(vec![mapping]).unwrap();

    assert_eq!(ns_lock.map_gid_host_to_ns(100000).unwrap(), 0);
    assert_eq!(ns_lock.map_gid_host_to_ns(100500).unwrap(), 500);
}

#[test]
fn test_9_3_2_multiple_uid_mappings() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping1 = UidGidMapping::new(0, 100000, 100);
    let mapping2 = UidGidMapping::new(100, 200000, 100);

    let mut ns_lock = ns.lock().unwrap();
    assert!(ns_lock.set_uid_map(vec![mapping1, mapping2]).is_ok());
    assert_eq!(ns_lock.uid_map.len(), 2);

    assert_eq!(ns_lock.map_uid_ns_to_host(0).unwrap(), 100000);
    assert_eq!(ns_lock.map_uid_ns_to_host(100).unwrap(), 200000);
}

#[test]
fn test_9_3_2_overlapping_mappings_rejected() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping1 = UidGidMapping::new(0, 100000, 100);
    let mapping2 = UidGidMapping::new(50, 100050, 100); // Overlaps

    let mut ns_lock = ns.lock().unwrap();
    let result = ns_lock.set_uid_map(vec![mapping1, mapping2]);
    assert!(result.is_err());
}

#[test]
fn test_9_3_2_invalid_mapping_count_zero() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();

    let mapping = UidGidMapping::new(0, 100000, 0); // Invalid: count = 0

    let mut ns_lock = ns.lock().unwrap();
    let result = ns_lock.set_uid_map(vec![mapping]);
    assert!(result.is_err());
}

// ============================================================================
// TASK 9.3.3 - subuid/subgid Support Tests
// ============================================================================

#[test]
fn test_9_3_3_parse_subuid_file_basic() {
    let content = "user1:100000:65536\nuser2:200000:32768\n";
    let result = parse_subuid_file(content);
    assert!(result.is_ok());
    let entries = result.unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].user, "user1");
    assert_eq!(entries[0].start_uid, 100000);
    assert_eq!(entries[0].count, 65536);
}

#[test]
fn test_9_3_3_parse_subgid_file_basic() {
    let content = "user1:100000:65536\nuser2:200000:32768\n";
    let result = parse_subgid_file(content);
    assert!(result.is_ok());
    let entries = result.unwrap();
    assert_eq!(entries.len(), 2);
    assert_eq!(entries[0].user, "user1");
    assert_eq!(entries[0].start_gid, 100000);
    assert_eq!(entries[0].count, 65536);
}

#[test]
fn test_9_3_3_parse_subuid_with_comments() {
    let content = "# This is a comment\nuser1:100000:65536\n# Another comment\n";
    let result = parse_subuid_file(content);
    assert!(result.is_ok());
    let entries = result.unwrap();
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_9_3_3_parse_subgid_with_comments() {
    let content = "# This is a comment\nuser1:100000:65536\n# Another comment\n";
    let result = parse_subgid_file(content);
    assert!(result.is_ok());
    let entries = result.unwrap();
    assert_eq!(entries.len(), 1);
}

#[test]
fn test_9_3_3_parse_subuid_empty_file() {
    let content = "";
    let result = parse_subuid_file(content);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_9_3_3_parse_subgid_empty_file() {
    let content = "";
    let result = parse_subgid_file(content);
    assert!(result.is_ok());
    assert_eq!(result.unwrap().len(), 0);
}

#[test]
fn test_9_3_3_parse_subuid_invalid_format() {
    let content = "user1:100000";
    let result = parse_subuid_file(content);
    assert!(result.is_err());
}

#[test]
fn test_9_3_3_parse_subgid_invalid_format() {
    let content = "user1:100000";
    let result = parse_subgid_file(content);
    assert!(result.is_err());
}

#[test]
fn test_9_3_3_subuid_entry_contains() {
    let entry = SubuidEntry::new("user1".to_string(), 100000, 65536);

    assert!(entry.contains_uid(100000));
    assert!(entry.contains_uid(132768));
    assert!(entry.contains_uid(165535));
    assert!(!entry.contains_uid(165536));
    assert!(!entry.contains_uid(99999));
}

#[test]
fn test_9_3_3_subgid_entry_contains() {
    let entry = SubgidEntry::new("user1".to_string(), 100000, 65536);

    assert!(entry.contains_gid(100000));
    assert!(entry.contains_gid(132768));
    assert!(entry.contains_gid(165535));
    assert!(!entry.contains_gid(165536));
    assert!(!entry.contains_gid(99999));
}

#[test]
fn test_9_3_3_allocation_tracker_allocate() {
    let tracker = SubuidAllocationTracker::new();
    let result = tracker.allocate_range("user1", 100000, 65536);
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), (100000, 65536));
}

#[test]
fn test_9_3_3_allocation_tracker_get_ranges() {
    let tracker = SubuidAllocationTracker::new();
    tracker.allocate_range("user1", 100000, 65536).unwrap();

    let ranges = tracker.get_allocated_ranges("user1").unwrap();
    assert_eq!(ranges.len(), 1);
    assert_eq!(ranges[0], (100000, 65536));
}

#[test]
fn test_9_3_3_allocation_tracker_deallocate() {
    let tracker = SubuidAllocationTracker::new();
    tracker.allocate_range("user1", 100000, 65536).unwrap();

    assert!(tracker.deallocate_range("user1", 100000, 65536).is_ok());
    let ranges = tracker.get_allocated_ranges("user1").unwrap();
    assert_eq!(ranges.len(), 0);
}

#[test]
fn test_9_3_3_allocation_tracker_conflict_detection() {
    let tracker = SubuidAllocationTracker::new();
    tracker.allocate_range("user1", 100000, 100).unwrap();

    // Try overlapping allocation
    let result = tracker.allocate_range("user1", 100050, 100);
    assert!(result.is_err());
}

#[test]
fn test_9_3_3_allocation_tracker_multiple_users() {
    let tracker = SubuidAllocationTracker::new();
    tracker.allocate_range("user1", 100000, 65536).unwrap();
    tracker.allocate_range("user2", 200000, 65536).unwrap();

    let ranges1 = tracker.get_allocated_ranges("user1").unwrap();
    let ranges2 = tracker.get_allocated_ranges("user2").unwrap();

    assert_eq!(ranges1.len(), 1);
    assert_eq!(ranges2.len(), 1);
}

// ============================================================================
// TASK 9.3.4 - Syscall Integration Tests
// ============================================================================

#[test]
fn test_9_3_4_clone_newuser_flag() {
    let flags = UserCloneFlags::new(UserCloneFlags::CLONE_NEWUSER);
    assert!(flags.clone_newuser());
}

#[test]
fn test_9_3_4_unshare_newuser_flag() {
    let flags = UserUnshareFlags::new(UserUnshareFlags::UNSHARE_NEWUSER);
    assert!(flags.unshare_newuser());
}

#[test]
fn test_9_3_4_sys_clone_user() {
    let manager = new_user_namespace_manager();
    let result = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_unshare_user() {
    let manager = new_user_namespace_manager();
    let result = sys_unshare_user(UserUnshareFlags::UNSHARE_NEWUSER, &manager);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_map_uid64() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let result = sys_map_uid64(ns_id, 0, 100000, 65536, &manager);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_map_uid64_invalid_count() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let result = sys_map_uid64(ns_id, 0, 100000, 0, &manager);
    assert!(result.is_err());
}

#[test]
fn test_9_3_4_sys_map_gid64() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let result = sys_map_gid64(ns_id, 0, 100000, 65536, &manager);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_map_gid64_invalid_count() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let result = sys_map_gid64(ns_id, 0, 100000, 0, &manager);
    assert!(result.is_err());
}

#[test]
fn test_9_3_4_sys_setuid64() {
    let result = sys_setuid64(1000);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_setgid64() {
    let result = sys_setgid64(1000);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_parse_subuid_allocations() {
    let content = "user1:100000:65536\nuser2:200000:32768\n";
    let result = parse_subuid_allocations(content);
    assert!(result.is_ok());
    let allocations = result.unwrap();
    assert_eq!(allocations.len(), 2);
}

#[test]
fn test_9_3_4_parse_subgid_allocations() {
    let content = "user1:100000:65536\nuser2:200000:32768\n";
    let result = parse_subgid_allocations(content);
    assert!(result.is_ok());
    let allocations = result.unwrap();
    assert_eq!(allocations.len(), 2);
}

#[test]
fn test_9_3_4_sys_grant_capability() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let result = sys_grant_capability(ns_id, 0, &manager); // CAP_CHOWN
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_revoke_capability() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let _ = sys_grant_capability(ns_id, 0, &manager);
    let result = sys_revoke_capability(ns_id, 0, &manager);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_check_capability() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let _ = sys_grant_capability(ns_id, 0, &manager);
    let result = sys_check_capability(ns_id, 0, &manager);
    assert!(result.is_ok());
    assert!(result.unwrap());
}

#[test]
fn test_9_3_4_sys_setns_user() {
    let manager = new_user_namespace_manager();
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    let result = sys_setns_user(ns_id, &manager);
    assert!(result.is_ok());
}

#[test]
fn test_9_3_4_sys_setns_user_nonexistent() {
    let manager = new_user_namespace_manager();
    let ns_id = UserNamespaceId(99999);
    let result = sys_setns_user(ns_id, &manager);
    assert!(result.is_err());
}

#[test]
fn test_9_3_4_uid_gid_map_config() {
    let ns_id = UserNamespaceId(1);
    let mut config = UidGidMapConfig::new(ns_id, true);
    let mapping = UidGidMapping::new(0, 100000, 65536);
    config.add_mapping(mapping);
    assert_eq!(config.mappings.len(), 1);
}

#[test]
fn test_9_3_4_setid_capability_spec() {
    let spec = SetidCapabilitySpec::new(1000, 1000);
    assert_eq!(spec.target_uid, 1000);
    assert_eq!(spec.target_gid, 1000);
    assert!(!spec.keep_capabilities);
}

#[test]
fn test_9_3_4_error_code_mapping() {
    assert_eq!(UserNamespaceSyscallError::InvalidArgument.code(), -22);
    assert_eq!(UserNamespaceSyscallError::PermissionDenied.code(), -1);
    assert_eq!(UserNamespaceSyscallError::NoMemory.code(), -12);
    assert_eq!(UserNamespaceSyscallError::NotFound.code(), -2);
    assert_eq!(UserNamespaceSyscallError::DeviceBusy.code(), -16);
}

// ============================================================================
// Integration Tests (combining multiple components)
// ============================================================================

#[test]
fn test_integration_full_user_namespace_workflow() {
    let manager = UserNamespaceManager::new();
    
    // Create namespace
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();
    
    // Set UID mapping
    let uid_mapping = UidGidMapping::new(0, 100000, 65536);
    let gid_mapping = UidGidMapping::new(0, 100000, 65536);
    
    let mut ns_lock = ns.lock().unwrap();
    assert!(ns_lock.set_uid_map(vec![uid_mapping]).is_ok());
    assert!(ns_lock.set_gid_map(vec![gid_mapping]).is_ok());
    
    // Grant capabilities
    assert!(ns_lock.grant_capability(CapabilitySet::CapChown).is_ok());
    assert!(ns_lock.has_capability(CapabilitySet::CapChown));
    
    // Verify mapping works
    assert_eq!(ns_lock.map_uid_ns_to_host(0).unwrap(), 100000);
    assert_eq!(ns_lock.map_gid_ns_to_host(0).unwrap(), 100000);
}

#[test]
fn test_integration_multiple_namespaces() {
    let manager = UserNamespaceManager::new();
    
    let ns1 = manager.create_namespace(1000, None).unwrap();
    let ns2 = manager.create_namespace(2000, None).unwrap();
    let ns3 = manager.create_namespace(3000, None).unwrap();
    
    let list = manager.list_namespaces().unwrap();
    assert_eq!(list.len(), 3);
    
    // Configure each namespace differently
    let ns1_obj = manager.get_namespace(ns1).unwrap();
    let ns2_obj = manager.get_namespace(ns2).unwrap();
    let ns3_obj = manager.get_namespace(ns3).unwrap();
    
    let mut n1 = ns1_obj.lock().unwrap();
    let mut n2 = ns2_obj.lock().unwrap();
    let mut n3 = ns3_obj.lock().unwrap();
    
    n1.set_uid_map(vec![UidGidMapping::new(0, 100000, 1000)]).ok();
    n2.set_uid_map(vec![UidGidMapping::new(0, 200000, 1000)]).ok();
    n3.set_uid_map(vec![UidGidMapping::new(0, 300000, 1000)]).ok();
    
    assert_eq!(n1.map_uid_ns_to_host(0).ok(), Some(100000));
    assert_eq!(n2.map_uid_ns_to_host(0).ok(), Some(200000));
    assert_eq!(n3.map_uid_ns_to_host(0).ok(), Some(300000));
}

#[test]
fn test_integration_syscall_and_namespace_manager() {
    let manager = new_user_namespace_manager();
    
    let ns_id = sys_clone_user(UserCloneFlags::CLONE_NEWUSER, &manager).unwrap();
    assert!(sys_map_uid64(ns_id, 0, 100000, 65536, &manager).is_ok());
    assert!(sys_map_gid64(ns_id, 0, 100000, 65536, &manager).is_ok());
    assert!(sys_grant_capability(ns_id, 5, &manager).is_ok()); // CAP_KILL
    assert!(sys_check_capability(ns_id, 5, &manager).unwrap());
}

#[test]
fn test_integration_subuid_with_namespace() {
    let content = "user1:100000:65536\nuser2:200000:32768\n";
    let entries = parse_subuid_file(content).unwrap();
    assert_eq!(entries.len(), 2);
    
    // Use the parsed entries to configure namespace
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();
    
    let mapping = UidGidMapping::new(entries[0].start_uid, 0, entries[0].count);
    let mut ns_lock = ns.lock().unwrap();
    assert!(ns_lock.set_uid_map(vec![mapping]).is_ok());
}

#[test]
fn test_integration_complex_mapping_scenario() {
    let manager = UserNamespaceManager::new();
    let ns_id = manager.create_namespace(1000, None).unwrap();
    let ns = manager.get_namespace(ns_id).unwrap();
    
    // Set up multiple non-overlapping mappings
    let mapping1 = UidGidMapping::new(0, 100000, 1000);
    let mapping2 = UidGidMapping::new(1000, 200000, 1000);
    let mapping3 = UidGidMapping::new(2000, 300000, 1000);
    
    let mut ns_lock = ns.lock().unwrap();
    assert!(ns_lock.set_uid_map(vec![mapping1, mapping2, mapping3]).is_ok());
    
    // Verify all mappings work correctly
    assert_eq!(ns_lock.map_uid_ns_to_host(0).ok(), Some(100000));
    assert_eq!(ns_lock.map_uid_ns_to_host(1000).ok(), Some(200000));
    assert_eq!(ns_lock.map_uid_ns_to_host(2000).ok(), Some(300000));
}
