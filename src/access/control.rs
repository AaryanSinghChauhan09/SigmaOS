extern crate alloc;

/// Access Control Engine for SigmaOS
/// Supports Discretionary Access Control (DAC), Mandatory Access Control (MAC - Bell-LaPadula),
/// MAC Address Hardware Network Filtering, POSIX ACLs, Access Control Matrix, and Role-Based Access Control (RBAC).

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

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

// ─────────────────────────────────────────────────────────────────────────────
// 7. EXT4 POSIX INODE MODE BITS & EXTENDED ATTRIBUTES (xattr)
// ─────────────────────────────────────────────────────────────────────────────

pub mod ext4_special_bits {
    pub const SUID: u16 = 0o4000;
    pub const SGID: u16 = 0o2000;
    pub const STICKY: u16 = 0o1000;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ext4FileType {
    RegularFile = 0o100000,
    Directory   = 0o040000,
    Symlink     = 0o120000,
    BlockDev    = 0o060000,
    CharDev     = 0o020000,
    Fifo        = 0o010000,
    Socket      = 0o140000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ext4InodeMode {
    pub file_type: Ext4FileType,
    pub permissions: u16, // 12-bit permissions (3 special + 9 standard rwx)
}

impl Ext4InodeMode {
    pub fn new(file_type: Ext4FileType, permissions: u16) -> Self {
        Self { file_type, permissions }
    }

    pub fn is_suid(&self) -> bool {
        (self.permissions & ext4_special_bits::SUID) != 0
    }

    pub fn is_sgid(&self) -> bool {
        (self.permissions & ext4_special_bits::SGID) != 0
    }

    pub fn is_sticky(&self) -> bool {
        (self.permissions & ext4_special_bits::STICKY) != 0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ext4Xattr {
    pub name: Vec<u8>,
    pub value: Vec<u8>,
}

impl Ext4Xattr {
    pub fn new(name: &[u8], value: &[u8]) -> Self {
        Self {
            name: name.to_vec(),
            value: value.to_vec(),
        }
    }
}

pub struct Ext4AccessCheckEngine;

impl Ext4AccessCheckEngine {
    /// Evaluates Linux ext4 access check flow:
    /// 1. If subject UID is 0 (Root), access is granted.
    /// 2. If POSIX ACL extended attribute (`system.posix_acl_access`) is present, evaluate ACL entries.
    /// 3. Otherwise, fall back to standard 16-bit inode mode bits (owner -> group -> other).
    pub fn evaluate_ext4_access(
        subject_uid: UserID,
        subject_gid: GroupID,
        requested_mode: u16,
        owner_uid: UserID,
        primary_gid: GroupID,
        mode_bits: u16,
        posix_acl_opt: Option<&PosixAclTable>,
    ) -> bool {
        if subject_uid == 0 {
            return true; // Root superuser
        }

        if let Some(acl) = posix_acl_opt {
            return acl.evaluate_acl_access(subject_uid, subject_gid, requested_mode, owner_uid, primary_gid);
        }

        // Fallback to standard owner/group/other bits
        let standard_bits = mode_bits & 0o777;
        let allowed_bits = if subject_uid == owner_uid {
            (standard_bits >> 6) & 0o7
        } else if subject_gid == primary_gid {
            (standard_bits >> 3) & 0o7
        } else {
            standard_bits & 0o7
        };

        (allowed_bits & requested_mode) == requested_mode
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 8. WINDOWS NTFS SECURITY DESCRIPTORS, SIDS, DACLS & ACE EVALUATION
// ─────────────────────────────────────────────────────────────────────────────

pub mod ntfs_access_rights {
    pub const READ: u32 = 0x00000001;
    pub const WRITE: u32 = 0x00000002;
    pub const EXECUTE: u32 = 0x00000004;
    pub const READ_AND_EXECUTE: u32 = READ | EXECUTE;
    pub const MODIFY: u32 = READ | WRITE | EXECUTE;
    pub const DELETE: u32 = 0x00010000;
    pub const FULL_CONTROL: u32 = 0x10000000;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SecurityIdentifier {
    pub revision: u8,
    pub identifier_authority: u64,
    pub sub_authorities: Vec<u32>,
}

impl SecurityIdentifier {
    pub fn new(identifier_authority: u64, sub_authorities: &[u32]) -> Self {
        Self {
            revision: 1,
            identifier_authority,
            sub_authorities: sub_authorities.to_vec(),
        }
    }

    /// Well-known SID for Local System (S-1-5-18)
    pub fn local_system() -> Self {
        Self::new(5, &[18])
    }

    /// Well-known SID for Administrators group (S-1-5-32-544)
    pub fn administrators() -> Self {
        Self::new(5, &[32, 544])
    }

    /// Convert SID to standard string representation (e.g., "S-1-5-21-100-200-300")
    pub fn to_sid_string(&self) -> Vec<u8> {
        let mut res = Vec::new();
        res.extend_from_slice(b"S-");
        res.push(b'0' + self.revision);
        res.push(b'-');

        let auth_str = alloc::format!("{}", self.identifier_authority);
        res.extend_from_slice(auth_str.as_bytes());

        for sub_auth in &self.sub_authorities {
            res.push(b'-');
            let sub_str = alloc::format!("{}", sub_auth);
            res.extend_from_slice(sub_str.as_bytes());
        }

        res
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NtfsAceType {
    AccessAllowed = 0x00,
    AccessDenied  = 0x01,
    SystemAudit   = 0x02,
}

pub mod ntfs_ace_flags {
    pub const OBJECT_INHERIT_ACE: u8 = 0x01;
    pub const CONTAINER_INHERIT_ACE: u8 = 0x02;
    pub const NO_PROPAGATE_INHERIT_ACE: u8 = 0x04;
    pub const INHERIT_ONLY_ACE: u8 = 0x08;
    pub const INHERITED_ACE: u8 = 0x10;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsAce {
    pub ace_type: NtfsAceType,
    pub flags: u8,
    pub access_mask: u32,
    pub sid: SecurityIdentifier,
}

impl NtfsAce {
    pub fn allow(sid: SecurityIdentifier, access_mask: u32, flags: u8) -> Self {
        Self {
            ace_type: NtfsAceType::AccessAllowed,
            flags,
            access_mask,
            sid,
        }
    }

    pub fn deny(sid: SecurityIdentifier, access_mask: u32, flags: u8) -> Self {
        Self {
            ace_type: NtfsAceType::AccessDenied,
            flags,
            access_mask,
            sid,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsDacl {
    pub aces: Vec<NtfsAce>,
}

impl NtfsDacl {
    pub fn new() -> Self {
        Self { aces: Vec::new() }
    }

    pub fn add_ace(&mut self, ace: NtfsAce) {
        self.aces.push(ace);
    }
}

impl Default for NtfsDacl {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsSacl {
    pub aces: Vec<NtfsAce>,
}

impl NtfsSacl {
    pub fn new() -> Self {
        Self { aces: Vec::new() }
    }

    pub fn add_audit_ace(&mut self, ace: NtfsAce) {
        self.aces.push(ace);
    }
}

impl Default for NtfsSacl {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NtfsSecurityDescriptor {
    pub owner_sid: SecurityIdentifier,
    pub group_sid: SecurityIdentifier,
    pub dacl: Option<NtfsDacl>,
    pub sacl: Option<NtfsSacl>,
}

impl NtfsSecurityDescriptor {
    pub fn new(owner_sid: SecurityIdentifier, group_sid: SecurityIdentifier) -> Self {
        Self {
            owner_sid,
            group_sid,
            dacl: None,
            sacl: None,
        }
    }

    /// Evaluates Windows NTFS DACL access sequentially:
    /// 1. Explicit Deny entries are checked first. If a matching deny ACE covers any requested bit, access is immediately blocked.
    /// 2. Explicit Allow entries are checked next, accumulating allowed access rights.
    /// 3. If all requested access rights are granted, return true.
    /// 4. If DACL finishes without granting all requested rights (or DACL is absent), access is implicitly denied.
    pub fn evaluate_access(
        &self,
        subject_sid: &SecurityIdentifier,
        subject_group_sids: &[SecurityIdentifier],
        requested_rights: u32,
    ) -> bool {
        let dacl = match self.dacl {
            Some(ref d) => d,
            None => return false, // Null/Empty DACL denies access by default in Windows rules
        };

        let is_match = |sid: &SecurityIdentifier| -> bool {
            sid == subject_sid || subject_group_sids.contains(sid)
        };

        // 1. Pass 1: Explicit Deny ACEs evaluated first
        for ace in &dacl.aces {
            if ace.ace_type == NtfsAceType::AccessDenied && is_match(&ace.sid) {
                if (ace.access_mask & requested_rights) != 0 {
                    return false; // Immediately denied
                }
            }
        }

        // 2. Pass 2: Explicit Allow ACEs accumulated next
        let mut accumulated_rights = 0u32;
        for ace in &dacl.aces {
            if ace.ace_type == NtfsAceType::AccessAllowed && is_match(&ace.sid) {
                accumulated_rights |= ace.access_mask;
            }
        }

        // Check if all requested rights have been granted
        (accumulated_rights & requested_rights) == requested_rights
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext4_inode_mode_and_access_flow() {
        let mode = Ext4InodeMode::new(Ext4FileType::RegularFile, 0o4750); // SUID + rwxr-x---
        assert!(mode.is_suid());
        assert!(!mode.is_sgid());
        assert!(!mode.is_sticky());

        // Test ext4 check flow without ACL:
        // Owner 1000 requests Read|Write
        assert!(Ext4AccessCheckEngine::evaluate_ext4_access(1000, 1000, dac_flags::READ | dac_flags::WRITE, 1000, 1000, mode.permissions, None));

        // Group 1000 requests Write -> Denied
        assert!(!Ext4AccessCheckEngine::evaluate_ext4_access(2000, 1000, dac_flags::WRITE, 1000, 1000, mode.permissions, None));

        // Test ext4 check flow with POSIX ACL xattr override
        let mut acl = PosixAclTable::new();
        acl.add_entry(PosixAclEntry { tag: PosixAclTag::UserObj, id: 1000, perms: 0o7 });
        acl.add_entry(PosixAclEntry { tag: PosixAclTag::User, id: 2000, perms: 0o6 }); // rw- granted to 2000
        acl.add_entry(PosixAclEntry { tag: PosixAclTag::Other, id: 0, perms: 0o0 });

        assert!(Ext4AccessCheckEngine::evaluate_ext4_access(2000, 3000, dac_flags::WRITE, 1000, 1000, mode.permissions, Some(&acl)));
    }

    #[test]
    fn test_ntfs_security_descriptor_dacl_evaluation() {
        let user_sid = SecurityIdentifier::new(5, &[21, 100, 200, 1001]);
        let group_sid = SecurityIdentifier::new(5, &[21, 100, 200, 513]);
        let sys_sid = SecurityIdentifier::local_system();

        assert_eq!(sys_sid.to_sid_string(), b"S-1-5-18");

        let mut sd = NtfsSecurityDescriptor::new(user_sid.clone(), group_sid.clone());
        let mut dacl = NtfsDacl::new();

        // Add Explicit Allow for user_sid for Read & Write
        dacl.add_ace(NtfsAce::allow(user_sid.clone(), ntfs_access_rights::READ | ntfs_access_rights::WRITE, 0));

        // Add Explicit Deny for user_sid for Write
        dacl.add_ace(NtfsAce::deny(user_sid.clone(), ntfs_access_rights::WRITE, 0));

        sd.dacl = Some(dacl);

        // Explicit Deny evaluated first -> Write requests MUST be denied!
        assert!(!sd.evaluate_access(&user_sid, &[], ntfs_access_rights::WRITE));

        // Read requests SHOULD be granted
        assert!(sd.evaluate_access(&user_sid, &[], ntfs_access_rights::READ));

        // Unspecified user SHOULD be implicitly denied
        let other_sid = SecurityIdentifier::new(5, &[21, 100, 200, 9999]);
        assert!(!sd.evaluate_access(&other_sid, &[], ntfs_access_rights::READ));
    }

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
}
