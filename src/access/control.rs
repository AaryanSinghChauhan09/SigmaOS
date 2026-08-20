extern crate alloc;

/// Access Control Engine for SigmaOS
/// Supports Discretionary Access Control (DAC), POSIX Access Control Lists (ACLs),
/// Windows NTFS Security Identifiers (SIDs) and DACLs with explicit Deny/Allow evaluation,
/// Mandatory Access Control (MAC - Bell-LaPadula & AppArmor Path-Based MAC),
/// FreeBSD Capsicum Descriptor Rights, BSD Securelevels,
/// MAC Address Hardware Network Filtering, Access Control Matrix, and Role-Based Access Control (RBAC).

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
    AclAccessDenied = 6,
    CapsicumRightsViolation = 7,
    SecureLevelViolation = 8,
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
    pub mode_bits: u16, // Mode bits e.g. 0o755 (rwxr-xr-x)
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
// 2. POSIX ACCESS CONTROL LISTS (POSIX.1e ACLs - Linux & FreeBSD Inspired)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AclTag {
    UserObj,
    User(UserID),
    GroupObj,
    Group(GroupID),
    Mask,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AclEntry {
    pub tag: AclTag,
    pub permissions: u16, // Bitmask: r=4, w=2, x=1
}

impl AclEntry {
    pub fn new(tag: AclTag, permissions: u16) -> Self {
        Self { tag, permissions }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PosixAcl {
    pub entries: Vec<AclEntry>,
}

impl PosixAcl {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: AclEntry) {
        self.entries.push(entry);
    }

    /// Evaluates POSIX.1e ACL access algorithm matching Linux / FreeBSD:
    /// 1. If subject is owner (UserObj), check owner permissions directly.
    /// 2. If subject matches a named User entry, check permissions masked by Mask (if present).
    /// 3. If subject matches GroupObj or a named Group entry, check permissions masked by Mask.
    /// 4. Otherwise, check Other entry permissions.
    pub fn evaluate_access(
        &self,
        subject_uid: UserID,
        subject_gids: &[GroupID],
        owner_uid: UserID,
        _owner_gid: GroupID,
        requested_mode: u16,
    ) -> bool {
        if subject_uid == 0 {
            return true; // Root superuser bypass
        }

        let mask_perm = self
            .entries
            .iter()
            .find(|e| matches!(e.tag, AclTag::Mask))
            .map(|e| e.permissions);

        // 1. Check Owner UserObj
        if subject_uid == owner_uid {
            if let Some(entry) = self.entries.iter().find(|e| matches!(e.tag, AclTag::UserObj)) {
                return (entry.permissions & requested_mode) == requested_mode;
            }
        }

        // 2. Check Named User
        if let Some(entry) = self
            .entries
            .iter()
            .find(|e| matches!(e.tag, AclTag::User(uid) if uid == subject_uid))
        {
            let effective = mask_perm.map_or(entry.permissions, |m| entry.permissions & m);
            return (effective & requested_mode) == requested_mode;
        }

        // 3. Check GroupObj or Named Group
        let mut group_match = false;
        let mut group_allowed = false;

        for entry in &self.entries {
            let matched = match entry.tag {
                AclTag::GroupObj => subject_gids.contains(&_owner_gid),
                AclTag::Group(gid) => subject_gids.contains(&gid),
                _ => false,
            };

            if matched {
                group_match = true;
                let effective = mask_perm.map_or(entry.permissions, |m| entry.permissions & m);
                if (effective & requested_mode) == requested_mode {
                    group_allowed = true;
                    break;
                }
            }
        }

        if group_match {
            return group_allowed;
        }

        // 4. Check Other
        if let Some(entry) = self.entries.iter().find(|e| matches!(e.tag, AclTag::Other)) {
            return (entry.permissions & requested_mode) == requested_mode;
        }

        false
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. WINDOWS NTFS SECURITY DESCRIPTORS & ACCESS CONTROL ENTRIES (ACEs)
// ─────────────────────────────────────────────────────────────────────────────

pub mod ntfs_access_rights {
    pub const READ_DATA: u32 = 0x0001;
    pub const WRITE_DATA: u32 = 0x0002;
    pub const APPEND_DATA: u32 = 0x0004;
    pub const READ_EA: u32 = 0x0008;
    pub const WRITE_EA: u32 = 0x0010;
    pub const EXECUTE: u32 = 0x0020;
    pub const DELETE: u32 = 0x0001_0000;
    pub const READ_CONTROL: u32 = 0x0002_0000;
    pub const WRITE_DAC: u32 = 0x0004_0000;
    pub const WRITE_OWNER: u32 = 0x0008_0000;
    pub const FULL_CONTROL: u32 = 0x001F_01FF;
}

/// Windows Security Identifier (SID) (e.g. S-1-5-21-3623811015-3361044348-30300820-1013)
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

    /// System Anonymous/Everyone SID (S-1-1-0)
    pub fn everyone() -> Self {
        Self::new(1, &[0])
    }

    /// System Built-in Administrators SID (S-1-5-32-544)
    pub fn administrators() -> Self {
        Self::new(5, &[32, 544])
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AceType {
    AccessAllowed,
    AccessDenied,
    SystemAudit,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessControlEntry {
    pub ace_type: AceType,
    pub flags: u8,
    pub mask: u32,
    pub sid: SecurityIdentifier,
}

impl AccessControlEntry {
    pub fn new(ace_type: AceType, flags: u8, mask: u32, sid: SecurityIdentifier) -> Self {
        Self {
            ace_type,
            flags,
            mask,
            sid,
        }
    }
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct DiscretionaryAcl {
    pub aces: Vec<AccessControlEntry>,
}

impl DiscretionaryAcl {
    pub fn new() -> Self {
        Self { aces: Vec::new() }
    }

    pub fn add_ace(&mut self, ace: AccessControlEntry) {
        self.aces.push(ace);
    }

    /// Evaluates NTFS DACL sequential access:
    /// 1. Explicit Deny ACEs evaluated first. If matching deny ACE covers requested right -> IMMEDIATELY DENIED.
    /// 2. Explicit Allow ACEs evaluated next.
    /// 3. If no matching rule satisfies requested rights -> IMPLICIT DENIED.
    pub fn evaluate_access(&self, subject_sids: &[SecurityIdentifier], requested_rights: u32) -> bool {
        let mut remaining_rights = requested_rights;

        // 1. First Pass: Check Explicit Deny ACEs
        for ace in &self.aces {
            if ace.ace_type == AceType::AccessDenied && subject_sids.contains(&ace.sid) {
                if (ace.mask & requested_rights) != 0 {
                    return false; // Explicit Deny -> Immediate Block!
                }
            }
        }

        // 2. Second Pass: Check Explicit Allow ACEs
        for ace in &self.aces {
            if ace.ace_type == AceType::AccessAllowed && subject_sids.contains(&ace.sid) {
                remaining_rights &= !ace.mask;
                if remaining_rights == 0 {
                    return true; // All requested rights granted!
                }
            }
        }

        // Implicit Deny fallback
        false
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. MANDATORY ACCESS CONTROL (MAC - Bell-LaPadula & AppArmor Path-Based MAC)
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

/// AppArmor Path-Based Mandatory Access Control (Ubuntu / SUSE Inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppArmorMode {
    Enforcing,
    Complain,
    Disabled,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppArmorRule {
    pub path_prefix: Vec<u8>,
    pub allow_read: bool,
    pub allow_write: bool,
    pub allow_exec: bool,
}

impl AppArmorRule {
    pub fn new(path_prefix: &[u8], allow_read: bool, allow_write: bool, allow_exec: bool) -> Self {
        Self {
            path_prefix: path_prefix.to_vec(),
            allow_read,
            allow_write,
            allow_exec,
        }
    }

    pub fn matches_path(&self, target_path: &[u8]) -> bool {
        target_path.starts_with(&self.path_prefix)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppArmorProfile {
    pub name: Vec<u8>,
    pub mode: AppArmorMode,
    pub rules: Vec<AppArmorRule>,
    pub complain_logs: Vec<Vec<u8>>,
}

impl AppArmorProfile {
    pub fn new(name: &[u8], mode: AppArmorMode) -> Self {
        Self {
            name: name.to_vec(),
            mode,
            rules: Vec::new(),
            complain_logs: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, rule: AppArmorRule) {
        self.rules.push(rule);
    }

    pub fn evaluate_access(&mut self, path: &[u8], requested_mode: u16) -> bool {
        if self.mode == AppArmorMode::Disabled {
            return true;
        }

        let mut allowed = false;
        for rule in &self.rules {
            if rule.matches_path(path) {
                let r_ok = (requested_mode & dac_flags::READ == 0) || rule.allow_read;
                let w_ok = (requested_mode & dac_flags::WRITE == 0) || rule.allow_write;
                let x_ok = (requested_mode & dac_flags::EXECUTE == 0) || rule.allow_exec;
                if r_ok && w_ok && x_ok {
                    allowed = true;
                    break;
                }
            }
        }

        if !allowed && self.mode == AppArmorMode::Complain {
            let mut log = Vec::from(b"AppArmor Complain Violation: ");
            log.extend_from_slice(path);
            self.complain_logs.push(log);
            return true; // Audit/complain mode allows operation but logs warning
        }

        allowed
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. FREEBSD CAPSICUM DESCRIPTOR RIGHTS
// ─────────────────────────────────────────────────────────────────────────────

pub mod capsicum_rights {
    pub const CAP_READ: u64 = 1 << 0;
    pub const CAP_WRITE: u64 = 1 << 1;
    pub const CAP_SEEK: u64 = 1 << 2;
    pub const CAP_FSTAT: u64 = 1 << 3;
    pub const CAP_MMAP: u64 = 1 << 4;
    pub const CAP_FCNTL: u64 = 1 << 5;
    pub const CAP_SOCKET: u64 = 1 << 6;
    pub const CAP_ALL: u64 = 0xFFFF_FFFF_FFFF_FFFF;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapsicumRights {
    pub rights_mask: u64,
    pub is_capability_mode: bool,
}

impl CapsicumRights {
    pub fn new(rights_mask: u64) -> Self {
        Self {
            rights_mask,
            is_capability_mode: false,
        }
    }

    pub fn enter_capability_mode(&mut self) {
        self.is_capability_mode = true;
    }

    /// Limit/narrow current descriptor rights (Capsicum monotonic reduction rule)
    pub fn limit_rights(&mut self, requested_mask: u64) -> Result<(), AccessError> {
        if (requested_mask & !self.rights_mask) != 0 {
            return Err(AccessError::CapsicumRightsViolation); // Cannot expand rights
        }
        self.rights_mask &= requested_mask;
        Ok(())
    }

    pub fn check_right(&self, required_right: u64) -> bool {
        (self.rights_mask & required_right) == required_right
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. BSD SECURELEVELS (BSD Inspired Kernel Security Policy)
// ─────────────────────────────────────────────────────────────────────────────

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BsdSecureLevel {
    PermanentlyInsecure = -1,
    Insecure = 0,
    Secure = 1,
    HighlySecure = 2,
    NetworkSecure = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecureLevelManager {
    pub level: BsdSecureLevel,
}

impl SecureLevelManager {
    pub fn new(initial_level: BsdSecureLevel) -> Self {
        Self {
            level: initial_level,
        }
    }

    /// Raises securelevel. Levels can only be raised once boot completes, never lowered.
    pub fn raise_level(&mut self, target_level: BsdSecureLevel) -> Result<(), AccessError> {
        if self.level == BsdSecureLevel::PermanentlyInsecure {
            return Ok(()); // Insecure mode locked
        }
        if target_level < self.level {
            return Err(AccessError::SecureLevelViolation); // Securelevel cannot be lowered
        }
        self.level = target_level;
        Ok(())
    }

    pub fn can_load_kernel_module(&self) -> bool {
        self.level <= BsdSecureLevel::Insecure
    }

    pub fn can_write_raw_disk(&self) -> bool {
        self.level <= BsdSecureLevel::Insecure
    }

    pub fn can_modify_sysctl(&self) -> bool {
        self.level <= BsdSecureLevel::Secure
    }

    pub fn can_modify_firewall_rules(&self) -> bool {
        self.level <= BsdSecureLevel::HighlySecure
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. MAC ADDRESS HARDWARE NETWORK FILTERING
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
// 8. ROLE-BASED ACCESS CONTROL (RBAC) & ZERO TRUST POLICIES
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
    fn test_posix_acl_evaluation() {
        let mut acl = PosixAcl::new();
        acl.add_entry(AclEntry::new(AclTag::UserObj, 0o7)); // Owner rwx
        acl.add_entry(AclEntry::new(AclTag::User(1001), 0o6)); // Named user 1001 rw-
        acl.add_entry(AclEntry::new(AclTag::GroupObj, 0o5)); // Group r-x
        acl.add_entry(AclEntry::new(AclTag::Group(2001), 0o7)); // Named group 2001 rwx
        acl.add_entry(AclEntry::new(AclTag::Mask, 0o6)); // Mask rw-
        acl.add_entry(AclEntry::new(AclTag::Other, 0o0)); // Other ---

        // Named user 1001 requests read (4) and write (2) -> Allowed (0o6 & 0o6 = 0o6)
        assert!(acl.evaluate_access(1001, &[1000], 1000, 1000, dac_flags::READ | dac_flags::WRITE));

        // Named group 2001 requests execute (1) -> Denied due to Mask 0o6 (1 & 6 = 0)
        assert!(!acl.evaluate_access(1002, &[2001], 1000, 1000, dac_flags::EXECUTE));

        // Unmatched user 1005 requests read -> Denied by Other (0o0)
        assert!(!acl.evaluate_access(1005, &[1005], 1000, 1000, dac_flags::READ));
    }

    #[test]
    fn test_ntfs_dacl_evaluation() {
        let user_sid = SecurityIdentifier::new(5, &[21, 1000]);
        let group_sid = SecurityIdentifier::everyone();

        let mut dacl = DiscretionaryAcl::new();
        // Explicit Deny: user_sid denied Write
        dacl.add_ace(AccessControlEntry::new(
            AceType::AccessDenied,
            0,
            ntfs_access_rights::WRITE_DATA,
            user_sid.clone(),
        ));

        // Explicit Allow: Everyone allowed Read & Write
        dacl.add_ace(AccessControlEntry::new(
            AceType::AccessAllowed,
            0,
            ntfs_access_rights::READ_DATA | ntfs_access_rights::WRITE_DATA,
            group_sid.clone(),
        ));

        let user_sids = [user_sid.clone(), group_sid.clone()];

        // Request Read -> Granted via Everyone Allow ACE
        assert!(dacl.evaluate_access(&user_sids, ntfs_access_rights::READ_DATA));

        // Request Write -> Denied due to Explicit Deny ACE taking priority!
        assert!(!dacl.evaluate_access(&user_sids, ntfs_access_rights::WRITE_DATA));
    }

    #[test]
    fn test_apparmor_path_mac() {
        let mut profile = AppArmorProfile::new(b"usr.bin.nginx", AppArmorMode::Enforcing);
        profile.add_rule(AppArmorRule::new(b"/var/www", true, true, false));
        profile.add_rule(AppArmorRule::new(b"/etc/nginx", true, false, false));

        // Read /var/www/index.html -> Allowed
        assert!(profile.evaluate_access(b"/var/www/index.html", dac_flags::READ));

        // Write /etc/nginx/nginx.conf -> Denied in Enforcing mode
        assert!(!profile.evaluate_access(b"/etc/nginx/nginx.conf", dac_flags::WRITE));

        // Test Complain mode
        profile.mode = AppArmorMode::Complain;
        assert!(profile.evaluate_access(b"/etc/nginx/nginx.conf", dac_flags::WRITE));
        assert!(!profile.complain_logs.is_empty());
    }

    #[test]
    fn test_capsicum_rights() {
        let mut cap = CapsicumRights::new(capsicum_rights::CAP_READ | capsicum_rights::CAP_WRITE | capsicum_rights::CAP_SEEK);
        assert!(cap.check_right(capsicum_rights::CAP_READ));

        // Limit rights (monotonic reduction)
        assert!(cap.limit_rights(capsicum_rights::CAP_READ | capsicum_rights::CAP_SEEK).is_ok());
        assert!(!cap.check_right(capsicum_rights::CAP_WRITE));

        // Attempting to expand rights fails
        assert_eq!(
            cap.limit_rights(capsicum_rights::CAP_READ | capsicum_rights::CAP_WRITE),
            Err(AccessError::CapsicumRightsViolation)
        );
    }

    #[test]
    fn test_bsd_securelevels() {
        let mut mgr = SecureLevelManager::new(BsdSecureLevel::Insecure);
        assert!(mgr.can_load_kernel_module());

        mgr.raise_level(BsdSecureLevel::Secure).unwrap();
        assert!(!mgr.can_load_kernel_module());
        assert!(mgr.can_modify_sysctl());

        // Cannot lower level
        assert_eq!(mgr.raise_level(BsdSecureLevel::Insecure), Err(AccessError::SecureLevelViolation));
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
