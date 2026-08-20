extern crate alloc;

/// Access Control Engine for SigmaOS
/// Supports Discretionary Access Control (DAC), Mandatory Access Control (MAC - Bell-LaPadula),
/// MAC Address Hardware Network Filtering, POSIX ACLs, Access Control Matrix, and Role-Based Access Control (RBAC).

use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use core::sync::atomic::{AtomicUsize, Ordering};

pub type RoleID = usize;
pub type PermissionID = usize;
pub type UserID = u32;
pub type GroupID = u32;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessError {
    Success = 0,
    Denied = 1,
    InvalidRole = 2,
    InvalidPermission = 3,
    MacLevelViolation = 4,
    MacAddressBlocked = 5,
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. DISCRETIONARY ACCESS CONTROL (DAC - POSIX Mode Bits & UID/GID)
// ─────────────────────────────────────────────────────────────────────────────

pub mod dac_flags {
    pub const READ: u16 = 0o4;
    pub const WRITE: u16 = 0o2;
    pub const EXECUTE: u16 = 0o1;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DacPermission {
    pub owner_uid: UserID,
    pub group_gid: GroupID,
    pub mode_bits: u16, // Mode mode bits e.g. 0o755 (rwxr-xr-x)
}

impl DacPermission {
    pub fn new(owner_uid: UserID, group_gid: GroupID, mode_bits: u16) -> Self {
        Self {
            owner_uid,
            group_gid,
            mode_bits,
        }
    }

    /// Evaluates POSIX DAC access for subject (uid, gid) requesting mode (r, w, x)
    pub fn evaluate_access(&self, subject_uid: UserID, subject_gid: GroupID, requested_mode: u16) -> bool {
        let allowed_bits = if subject_uid == 0 {
            0o777 // Root bypasses standard DAC
        } else if subject_uid == self.owner_uid {
            (self.mode_bits >> 6) & 0o7
        } else if subject_gid == self.group_gid {
            (self.mode_bits >> 3) & 0o7
        } else {
            self.mode_bits & 0o7
        };

        (allowed_bits & requested_mode) == requested_mode
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. MANDATORY ACCESS CONTROL (MAC - Bell-LaPadula Multilevel Security)
// ─────────────────────────────────────────────────────────────────────────────

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SensitivityLevel {
    Unclassified = 0,
    Confidential = 1,
    Secret = 2,
    TopSecret = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MacSecurityLabel {
    pub level: SensitivityLevel,
    pub category_mask: u32,
}

impl MacSecurityLabel {
    pub fn new(level: SensitivityLevel, category_mask: u32) -> Self {
        Self { level, category_mask }
    }

    /// Enforces Bell-LaPadula MLS Rules:
    /// 1. Simple Security Property (No Read Up): Subject Level >= Object Level
    /// 2. *-Property (No Write Down): Subject Level <= Object Level
    pub fn can_read(&self, object_label: &MacSecurityLabel) -> bool {
        self.level >= object_label.level && (self.category_mask & object_label.category_mask) == object_label.category_mask
    }

    pub fn can_write(&self, object_label: &MacSecurityLabel) -> bool {
        self.level <= object_label.level && (object_label.category_mask & self.category_mask) == self.category_mask
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. MAC ADDRESS HARDWARE NETWORK FILTERING
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterPolicy {
    Whitelist,
    Blacklist,
}

pub struct MacAddressFilter {
    pub policy: FilterPolicy,
    pub addresses: Vec<[u8; 6]>,
}

impl MacAddressFilter {
    pub fn new(policy: FilterPolicy) -> Self {
        Self {
            policy,
            addresses: Vec::new(),
        }
    }

    pub fn add_mac(&mut self, mac: [u8; 6]) {
        if !self.addresses.contains(&mac) {
            self.addresses.push(mac);
        }
    }

    pub fn is_allowed(&self, mac: &[u8; 6]) -> bool {
        let is_listed = self.addresses.contains(mac);
        match self.policy {
            FilterPolicy::Whitelist => is_listed,
            FilterPolicy::Blacklist => !is_listed,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. ROLE-BASED ACCESS CONTROL (RBAC) & ZERO TRUST POLICIES
// ─────────────────────────────────────────────────────────────────────────────

pub trait Role {
    fn id(&self) -> RoleID;
    fn name(&self) -> &[u8];
    fn has_permission(&self, permission_id: PermissionID) -> bool;
}

pub struct SimpleRole {
    pub id: RoleID,
    pub name: [u8; 64],
    pub permissions: Vec<PermissionID>,
}

impl SimpleRole {
    pub fn new(id: RoleID, name: &[u8]) -> Self {
        let mut name_array = [0u8; 64];
        let name_len = name.len().min(63);
        name_array[..name_len].copy_from_slice(&name[..name_len]);

        SimpleRole {
            id,
            name: name_array,
            permissions: Vec::new(),
        }
    }

    pub fn grant_permission(&mut self, perm_id: PermissionID) {
        if !self.permissions.contains(&perm_id) {
            self.permissions.push(perm_id);
        }
    }
}

impl Role for SimpleRole {
    fn id(&self) -> RoleID {
        self.id
    }
    fn name(&self) -> &[u8] {
        let len = self.name.iter().position(|&b| b == 0).unwrap_or(64);
        &self.name[..len]
    }
    fn has_permission(&self, permission_id: PermissionID) -> bool {
        self.permissions.contains(&permission_id)
    }
}

pub trait Permission {
    fn id(&self) -> PermissionID;
    fn resource(&self) -> &[u8];
    fn action(&self) -> &[u8];
}

pub struct SimplePermission {
    pub id: PermissionID,
    pub resource: [u8; 128],
    pub action: [u8; 64],
}

impl SimplePermission {
    pub fn new(id: PermissionID, resource: &[u8], action: &[u8]) -> Self {
        let mut resource_array = [0u8; 128];
        let mut action_array = [0u8; 64];
        let resource_len = resource.len().min(127);
        let action_len = action.len().min(63);

        resource_array[..resource_len].copy_from_slice(&resource[..resource_len]);
        action_array[..action_len].copy_from_slice(&action[..action_len]);

        SimplePermission {
            id,
            resource: resource_array,
            action: action_array,
        }
    }
}

impl Permission for SimplePermission {
    fn id(&self) -> PermissionID {
        self.id
    }
    fn resource(&self) -> &[u8] {
        let len = self.resource.iter().position(|&b| b == 0).unwrap_or(128);
        &self.resource[..len]
    }
    fn action(&self) -> &[u8] {
        let len = self.action.iter().position(|&b| b == 0).unwrap_or(64);
        &self.action[..len]
    }
}

pub trait AccessController {
    fn grant_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError>;
    fn revoke_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError>;
    fn check_access(&self, role_id: RoleID, resource: &[u8], action: &[u8]) -> Result<bool, AccessError>;
}

pub struct SimpleAccessController {
    pub roles: Vec<Option<SimpleRole>>,
    pub permissions: Vec<Option<SimplePermission>>,
}

impl SimpleAccessController {
    pub fn new() -> Self {
        SimpleAccessController {
            roles: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn add_role(&mut self, role: SimpleRole) {
        self.roles.push(Some(role));
    }

    pub fn add_permission(&mut self, perm: SimplePermission) {
        self.permissions.push(Some(perm));
    }
}

impl AccessController for SimpleAccessController {
    fn grant_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError> {
        for role_option in &mut self.roles {
            if let Some(ref mut role) = *role_option {
                if role.id() == role_id {
                    role.grant_permission(permission_id);
                    return Ok(());
                }
            }
        }
        Err(AccessError::InvalidRole)
    }

    fn revoke_permission(&mut self, role_id: RoleID, permission_id: PermissionID) -> Result<(), AccessError> {
        for role_option in &mut self.roles {
            if let Some(ref mut role) = *role_option {
                if role.id() == role_id {
                    role.permissions.retain(|&p| p != permission_id);
                    return Ok(());
                }
            }
        }
        Err(AccessError::InvalidRole)
    }

    fn check_access(&self, role_id: RoleID, resource: &[u8], action: &[u8]) -> Result<bool, AccessError> {
        for role_option in &self.roles {
            if let Some(ref role) = *role_option {
                if role.id() == role_id {
                    for perm_option in &self.permissions {
                        if let Some(ref perm) = *perm_option {
                            if role.has_permission(perm.id()) && perm.resource() == resource && perm.action() == action {
                                return Ok(true);
                            }
                        }
                    }
                    return Ok(false);
                }
            }
        }
        Err(AccessError::InvalidRole)
    }
}

impl Default for SimpleAccessController {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. POSIX / UNIX USER-CLASS CATEGORIES, IMMUTABLE FLAGS & PRIVILEGE LEVELS
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum UserClassCategory {
    OwnerUser,
    GroupMember,
    OthersWorld,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HardwarePrivilegeRing {
    Ring0KernelSupervisor,
    Ring3UserMode,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileImmutableFlags {
    pub is_immutable: bool,   // Linux chattr +i (prevents modification, deletion, renaming even by root)
    pub is_append_only: bool, // Linux chattr +a (allows append writes only)
}

impl FileImmutableFlags {
    pub fn new() -> Self {
        Self {
            is_immutable: false,
            is_append_only: false,
        }
    }

    /// Evaluates if a write or delete operation is permitted under immutable flags
    pub fn validate_file_write(&self, is_append_operation: bool) -> Result<(), &'static str> {
        if self.is_immutable {
            return Err("EPERM: File is set immutable (+i). Write/modify blocked.");
        }
        if self.is_append_only && !is_append_operation {
            return Err("EPERM: File is set append-only (+a). Overwrite blocked.");
        }
        Ok(())
    }
}

impl Default for FileImmutableFlags {
    fn default() -> Self {
        Self::new()
    }
}

pub struct HardwarePrivilegeGuard;

impl HardwarePrivilegeGuard {
    pub fn evaluate_instruction_execution(
        current_ring: HardwarePrivilegeRing,
        is_privileged_instruction: bool,
    ) -> Result<(), &'static str> {
        if is_privileged_instruction && current_ring != HardwarePrivilegeRing::Ring0KernelSupervisor {
            return Err("General Protection Fault (Ring 3 User Mode instruction violation)");
        }
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. UNIX / POSIX ACCESS CONTROL LISTS (ACLs)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosixAclTag {
    UserObj,  // Owner UID
    User,     // Named User UID
    GroupObj, // Primary Group GID
    Group,    // Named Group GID
    Mask,     // ACL Mask capping permissions
    Other,    // Other / World
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PosixAclEntry {
    pub tag: PosixAclTag,
    pub id: u32,       // UID or GID (ignored for UserObj, GroupObj, Mask, Other)
    pub perms: u16,    // Read (4), Write (2), Execute (1)
}

#[derive(Debug, Clone)]
pub struct PosixAclTable {
    pub entries: Vec<PosixAclEntry>,
}

impl PosixAclTable {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: PosixAclEntry) {
        self.entries.push(entry);
    }

    /// Evaluates POSIX.1e ACL access rule precedence:
    /// 1. UserObj match
    /// 2. Named User match (capped by Mask if present)
    /// 3. GroupObj or Named Group match (capped by Mask if present)
    /// 4. Other
    pub fn evaluate_acl_access(
        &self,
        subject_uid: UserID,
        subject_gid: GroupID,
        requested_perms: u16,
        owner_uid: UserID,
        primary_gid: GroupID,
    ) -> bool {
        if subject_uid == 0 {
            return true; // Root superuser
        }

        // Check for Mask entry
        let mask_perms = self
            .entries
            .iter()
            .find(|e| e.tag == PosixAclTag::Mask)
            .map(|e| e.perms);

        // 1. Owner Match
        if subject_uid == owner_uid {
            if let Some(user_obj) = self.entries.iter().find(|e| e.tag == PosixAclTag::UserObj) {
                return (user_obj.perms & requested_perms) == requested_perms;
            }
        }

        // 2. Named User Match
        if let Some(named_user) = self
            .entries
            .iter()
            .find(|e| e.tag == PosixAclTag::User && e.id == subject_uid)
        {
            let effective = if let Some(m) = mask_perms { named_user.perms & m } else { named_user.perms };
            return (effective & requested_perms) == requested_perms;
        }

        // 3. Group Match (Primary GID or Named Group)
        let mut group_matched = false;
        let mut group_perms_accum = 0u16;

        for e in &self.entries {
            if (e.tag == PosixAclTag::GroupObj && subject_gid == primary_gid)
                || (e.tag == PosixAclTag::Group && e.id == subject_gid)
            {
                group_matched = true;
                group_perms_accum |= e.perms;
            }
        }

        if group_matched {
            let effective = if let Some(m) = mask_perms { group_perms_accum & m } else { group_perms_accum };
            return (effective & requested_perms) == requested_perms;
        }

        // 4. Other / World Match
        if let Some(other) = self.entries.iter().find(|e| e.tag == PosixAclTag::Other) {
            return (other.perms & requested_perms) == requested_perms;
        }

        false
    }
}

impl Default for PosixAclTable {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. ACCESS CONTROL MATRIX & CATEGORY-GATED SECURITY DOMAINS
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AccessDomainCategory {
    KernelCore,
    NetworkDriver,
    StorageDriver,
    UserApplication,
    SecurityEnclave,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileCapabilityMask {
    pub cap_chown: bool,
    pub cap_dac_override: bool,
    pub cap_net_admin: bool,
    pub cap_sys_admin: bool,
}

impl FileCapabilityMask {
    pub fn full() -> Self {
        Self {
            cap_chown: true,
            cap_dac_override: true,
            cap_net_admin: true,
            cap_sys_admin: true,
        }
    }

    pub fn unprivileged() -> Self {
        Self {
            cap_chown: false,
            cap_dac_override: false,
            cap_net_admin: false,
            cap_sys_admin: false,
        }
    }
}

/// 2D Matrix of Subjects (UID) x Objects (Resource Handle / Path ID) -> Allowed Action Mask
pub struct AccessControlMatrix {
    pub matrix: BTreeMap<(UserID, usize), u16>,
    pub domain_categories: BTreeMap<UserID, AccessDomainCategory>,
    pub capabilities: BTreeMap<UserID, FileCapabilityMask>,
}

impl AccessControlMatrix {
    pub fn new() -> Self {
        Self {
            matrix: BTreeMap::new(),
            domain_categories: BTreeMap::new(),
            capabilities: BTreeMap::new(),
        }
    }

    pub fn set_matrix_entry(&mut self, uid: UserID, resource_handle: usize, allowed_actions: u16) {
        self.matrix.insert((uid, resource_handle), allowed_actions);
    }

    pub fn assign_domain_category(&mut self, uid: UserID, category: AccessDomainCategory) {
        self.domain_categories.insert(uid, category);
    }

    pub fn grant_capability(&mut self, uid: UserID, caps: FileCapabilityMask) {
        self.capabilities.insert(uid, caps);
    }

    pub fn check_matrix_access(&self, uid: UserID, resource_handle: usize, requested_action: u16) -> bool {
        if uid == 0 {
            return true; // Root
        }

        if let Some(&allowed) = self.matrix.get(&(uid, resource_handle)) {
            (allowed & requested_action) == requested_action
        } else {
            false
        }
    }
}

impl Default for AccessControlMatrix {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dac_evaluation() {
        // Mode 0o750: owner=rwx, group=r-x, other=---
        let dac = DacPermission::new(1000, 1000, 0o750);

        // Owner (1000) requests Read (4) and Write (2)
        assert!(dac.evaluate_access(1000, 1000, dac_flags::READ | dac_flags::WRITE));

        // Group member (2000, 1000) requests Read (4) and Execute (1)
        assert!(dac.evaluate_access(2000, 1000, dac_flags::READ | dac_flags::EXECUTE));

        // Group member (2000, 1000) requests Write (2) -> Denied
        assert!(!dac.evaluate_access(2000, 1000, dac_flags::WRITE));

        // Other user (2000, 2000) requests Read (4) -> Denied
        assert!(!dac.evaluate_access(2000, 2000, dac_flags::READ));

        // Root (uid 0) requests all -> Granted
        assert!(dac.evaluate_access(0, 0, dac_flags::READ | dac_flags::WRITE | dac_flags::EXECUTE));
    }

    #[test]
    fn test_mac_bell_lapadula_mls() {
        let secret_subject = MacSecurityLabel::new(SensitivityLevel::Secret, 0x01);
        let confidential_object = MacSecurityLabel::new(SensitivityLevel::Confidential, 0x01);
        let topsecret_object = MacSecurityLabel::new(SensitivityLevel::TopSecret, 0x01);

        // Simple Security Property (No Read Up): Secret Subject can read Confidential Object
        assert!(secret_subject.can_read(&confidential_object));

        // Secret Subject CANNOT read TopSecret Object (Read Up violation)
        assert!(!secret_subject.can_read(&topsecret_object));

        // *-Property (No Write Down): Secret Subject CANNOT write Confidential Object (Write Down violation)
        assert!(!secret_subject.can_write(&confidential_object));

        // Secret Subject CAN write TopSecret Object
        assert!(secret_subject.can_write(&topsecret_object));
    }

    #[test]
    fn test_mac_address_filtering() {
        let mut filter = MacAddressFilter::new(FilterPolicy::Whitelist);
        let allowed_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        let blocked_mac = [0xAA, 0xBB, 0xCC, 0xDD, 0xEE, 0xFF];

        filter.add_mac(allowed_mac);

        assert!(filter.is_allowed(&allowed_mac));
        assert!(!filter.is_allowed(&blocked_mac));
    }

    #[test]
    fn test_rbac_access_control() {
        let mut controller = SimpleAccessController::new();

        let mut admin_role = SimpleRole::new(1, b"Admin");
        admin_role.grant_permission(101);

        let perm = SimplePermission::new(101, b"/etc/shadow", b"write");

        controller.add_role(admin_role);
        controller.add_permission(perm);

        assert!(controller.check_access(1, b"/etc/shadow", b"write").unwrap());
        assert!(!controller.check_access(1, b"/etc/shadow", b"execute").unwrap());
    }

    #[test]
    fn test_posix_acl_evaluation() {
        let mut acl = PosixAclTable::new();
        acl.add_entry(PosixAclEntry { tag: PosixAclTag::UserObj, id: 1000, perms: 0o7 });  // Owner: rwx
        acl.add_entry(PosixAclEntry { tag: PosixAclTag::User, id: 1001, perms: 0o6 });     // Named User 1001: rw-
        acl.add_entry(PosixAclEntry { tag: PosixAclTag::Mask, id: 0, perms: 0o4 });        // Mask caps to r--
        acl.add_entry(PosixAclEntry { tag: PosixAclTag::Other, id: 0, perms: 0o0 });       // Other: ---

        // Owner 1000 evaluates UserObj -> Granted rwx
        assert!(acl.evaluate_acl_access(1000, 1000, dac_flags::READ | dac_flags::WRITE | dac_flags::EXECUTE, 1000, 1000));

        // Named User 1001 requests Read (4) -> Granted (rw- masked by r-- gives r--)
        assert!(acl.evaluate_acl_access(1001, 1000, dac_flags::READ, 1000, 1000));

        // Named User 1001 requests Write (2) -> Denied by Mask (r--)
        assert!(!acl.evaluate_acl_access(1001, 1000, dac_flags::WRITE, 1000, 1000));

        // Other user 2000 requests Read (4) -> Denied by Other (0o0)
        assert!(!acl.evaluate_acl_access(2000, 2000, dac_flags::READ, 1000, 1000));
    }

    #[test]
    fn test_access_control_matrix() {
        let mut matrix = AccessControlMatrix::new();
        matrix.set_matrix_entry(1000, 42, dac_flags::READ | dac_flags::WRITE);
        matrix.assign_domain_category(1000, AccessDomainCategory::StorageDriver);
        matrix.grant_capability(1000, FileCapabilityMask::full());

        assert!(matrix.check_matrix_access(1000, 42, dac_flags::READ));
        assert!(matrix.check_matrix_access(1000, 42, dac_flags::WRITE));
        assert!(!matrix.check_matrix_access(1000, 42, dac_flags::EXECUTE));
        assert_eq!(*matrix.domain_categories.get(&1000).unwrap(), AccessDomainCategory::StorageDriver);
        assert!(matrix.capabilities.get(&1000).unwrap().cap_sys_admin);
    }

    #[test]
    fn test_immutable_flags_and_privilege_rings() {
        let mut flags = FileImmutableFlags::new();
        assert!(flags.validate_file_write(false).is_ok());

        flags.is_append_only = true;
        assert!(flags.validate_file_write(false).is_err());
        assert!(flags.validate_file_write(true).is_ok());

        flags.is_immutable = true;
        assert!(flags.validate_file_write(true).is_err());

        assert!(HardwarePrivilegeGuard::evaluate_instruction_execution(HardwarePrivilegeRing::Ring0KernelSupervisor, true).is_ok());
        assert!(HardwarePrivilegeGuard::evaluate_instruction_execution(HardwarePrivilegeRing::Ring3UserMode, false).is_ok());
        assert!(HardwarePrivilegeGuard::evaluate_instruction_execution(HardwarePrivilegeRing::Ring3UserMode, true).is_err());
    }
}
