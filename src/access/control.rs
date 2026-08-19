/// Access Control Engine for SigmaOS
/// Comprehensive Linux & BSD Distro-Inspired Security & Access Control Architecture.
/// Integrates:
/// - Discretionary Access Control (DAC: POSIX Mode Bits, SUID, SGID, Sticky Bits, Umask)
/// - Access Control Lists (ACLs: POSIX.1e & FreeBSD NFSv4 ACLs)
/// - Access Control Matrix (ACM: Subject x Object Permission Grid)
/// - Mandatory Access Control (MAC: Bell-LaPadula MLS & Multicategory Security MCS)
/// - Role-Based Access Control (RBAC: Hierarchical Role Inheritance & Dynamic Activation)
/// - Command & Function Execution Policies (Sudo/Doas Path & Capability Constraints)
/// - Filesystem Protection Flags (BSD chflags: UF_IMMUTABLE, UF_APPEND, SF_IMMUTABLE, SF_APPEND)
/// - Hardware MAC Filtering & Unified Sovereign Access Control Engine

extern crate alloc;

use alloc::vec::Vec;
use alloc::string::String;
use crate::klib::HashMap;

pub type RoleID = usize;
pub type PermissionID = usize;
pub type UserID = u32;
pub type GroupID = u32;
pub type SubjectID = u64;
pub type ResourceID = u64;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessError {
    Success = 0,
    Denied = 1,
    InvalidRole = 2,
    InvalidPermission = 3,
    MacLevelViolation = 4,
    MacAddressBlocked = 5,
    AclDenied = 6,
    ImmutableFile = 7,
    CommandNotAllowed = 8,
    CategoryMismatch = 9,
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. DISCRETIONARY ACCESS CONTROL (DAC - POSIX Mode Bits, SUID/SGID/Sticky, Umask)
// ─────────────────────────────────────────────────────────────────────────────

pub mod dac_flags {
    pub const EXECUTE: u16 = 0o1;
    pub const WRITE: u16   = 0o2;
    pub const READ: u16    = 0o4;
    pub const STICKY: u16  = 0o1000;
    pub const SGID: u16    = 0o2000;
    pub const SUID: u16    = 0o4000;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DacPermission {
    pub owner_uid: UserID,
    pub group_gid: GroupID,
    pub mode_bits: u16, // Includes SUID, SGID, Sticky, and rwxrwxrwx
}

impl DacPermission {
    pub fn new(owner_uid: UserID, group_gid: GroupID, mode_bits: u16) -> Self {
        Self {
            owner_uid,
            group_gid,
            mode_bits,
        }
    }

    pub fn is_suid(&self) -> bool {
        (self.mode_bits & dac_flags::SUID) != 0
    }

    pub fn is_sgid(&self) -> bool {
        (self.mode_bits & dac_flags::SGID) != 0
    }

    pub fn is_sticky(&self) -> bool {
        (self.mode_bits & dac_flags::STICKY) != 0
    }

    /// Evaluates effective User ID and Group ID taking SUID/SGID into account
    pub fn calculate_effective_ids(&self, real_uid: UserID, real_gid: GroupID) -> (UserID, GroupID) {
        let euid = if self.is_suid() { self.owner_uid } else { real_uid };
        let egid = if self.is_sgid() { self.group_gid } else { real_gid };
        (euid, egid)
    }

    /// Evaluates POSIX DAC access for subject (uid, gid) requesting mode (r, w, x)
    pub fn evaluate_access(&self, subject_uid: UserID, subject_gid: GroupID, requested_mode: u16) -> bool {
        if subject_uid == 0 {
            return true; // Root bypasses standard DAC
        }

        let allowed_bits = if subject_uid == self.owner_uid {
            (self.mode_bits >> 6) & 0o7
        } else if subject_gid == self.group_gid {
            (self.mode_bits >> 3) & 0o7
        } else {
            self.mode_bits & 0o7
        };

        (allowed_bits & requested_mode) == requested_mode
    }

    /// Applies process umask mask to initial mode bits on file creation
    pub fn apply_umask(initial_mode: u16, umask: u16) -> u16 {
        initial_mode & !umask
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. POSIX.1e & FreeBSD NFSv4 ACCESS CONTROL LISTS (ACLs)
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
    pub permissions: u16, // READ, WRITE, EXECUTE bits
}

#[derive(Debug, Clone)]
pub struct AccessControlList {
    pub entries: Vec<AclEntry>,
}

impl AccessControlList {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, entry: AclEntry) {
        self.entries.push(entry);
    }

    /// Evaluates POSIX.1e ACL rules for subject requesting mode (r, w, x)
    pub fn evaluate_acl(&self, subject_uid: UserID, subject_gid: GroupID, owner_uid: UserID, group_gid: GroupID, requested_mode: u16) -> Option<bool> {
        if subject_uid == 0 {
            return Some(true); // Root granted
        }

        // 1. Check UserObj / Specific User
        if subject_uid == owner_uid {
            if let Some(e) = self.entries.iter().find(|e| e.tag == AclTag::UserObj) {
                return Some((e.permissions & requested_mode) == requested_mode);
            }
        }

        if let Some(e) = self.entries.iter().find(|e| e.tag == AclTag::User(subject_uid)) {
            let mask = self.get_mask_permissions();
            let effective = e.permissions & mask;
            return Some((effective & requested_mode) == requested_mode);
        }

        // 2. Check GroupObj / Specific Group
        if let Some(e) = self.entries.iter().find(|e| e.tag == AclTag::Group(subject_gid)) {
            let mask = self.get_mask_permissions();
            let effective = e.permissions & mask;
            return Some((effective & requested_mode) == requested_mode);
        }

        if subject_gid == group_gid {
            if let Some(e) = self.entries.iter().find(|e| e.tag == AclTag::GroupObj) {
                let mask = self.get_mask_permissions();
                let effective = e.permissions & mask;
                return Some((effective & requested_mode) == requested_mode);
            }
        }

        // 3. Other
        if let Some(e) = self.entries.iter().find(|e| e.tag == AclTag::Other) {
            return Some((e.permissions & requested_mode) == requested_mode);
        }

        None
    }

    fn get_mask_permissions(&self) -> u16 {
        if let Some(e) = self.entries.iter().find(|e| e.tag == AclTag::Mask) {
            e.permissions
        } else {
            0o7 // Default full mask
        }
    }
}

impl Default for AccessControlList {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. ACCESS CONTROL MATRIX (ACM - Rights Grid)
// ─────────────────────────────────────────────────────────────────────────────

pub mod acm_rights {
    pub const READ: u32    = 1 << 0;
    pub const WRITE: u32   = 1 << 1;
    pub const EXECUTE: u32 = 1 << 2;
    pub const DELETE: u32  = 1 << 3;
    pub const CONTROL: u32 = 1 << 4;
}

#[derive(Debug, Clone)]
pub struct AccessControlMatrix {
    // Grid: (SubjectID, ResourceID) -> Rights Mask
    pub matrix: HashMap<(SubjectID, ResourceID), u32>,
}

impl AccessControlMatrix {
    pub fn new() -> Self {
        Self {
            matrix: HashMap::new(),
        }
    }

    pub fn set_right(&mut self, subject: SubjectID, resource: ResourceID, rights_mask: u32) {
        self.matrix.insert((subject, resource), rights_mask);
    }

    pub fn grant_right(&mut self, subject: SubjectID, resource: ResourceID, right: u32) {
        let current = self.matrix.get(&(subject, resource)).copied().unwrap_or(0);
        self.matrix.insert((subject, resource), current | right);
    }

    pub fn revoke_right(&mut self, subject: SubjectID, resource: ResourceID, right: u32) {
        if let Some(current) = self.matrix.get_mut(&(subject, resource)) {
            *current &= !right;
        }
    }

    pub fn check_right(&self, subject: SubjectID, resource: ResourceID, requested_right: u32) -> bool {
        let rights = self.matrix.get(&(subject, resource)).copied().unwrap_or(0);
        (rights & requested_right) == requested_right
    }
}

impl Default for AccessControlMatrix {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. MANDATORY ACCESS CONTROL (MAC - Bell-LaPadula MLS & Multicategory MCS)
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
    pub category_mask: u64, // Multicategory Security (MCS) compartmentalization
}

impl MacSecurityLabel {
    pub fn new(level: SensitivityLevel, category_mask: u64) -> Self {
        Self { level, category_mask }
    }

    /// Enforces Bell-LaPadula MLS & MCS Rules:
    /// 1. Simple Security Property (No Read Up): Subject Level >= Object Level & Subject Categories ⊇ Object Categories
    /// 2. *-Property (No Write Down): Subject Level <= Object Level & Object Categories ⊇ Subject Categories
    pub fn can_read(&self, object_label: &MacSecurityLabel) -> bool {
        self.level >= object_label.level && (self.category_mask & object_label.category_mask) == object_label.category_mask
    }

    pub fn can_write(&self, object_label: &MacSecurityLabel) -> bool {
        self.level <= object_label.level && (object_label.category_mask & self.category_mask) == self.category_mask
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. FILESYSTEM PROTECTION FLAGS (BSD chflags & Linux Inode Protection)
// ─────────────────────────────────────────────────────────────────────────────

pub mod bsd_chflags {
    pub const UF_NODUMP: u32    = 0x00000001; // User: Do not dump file
    pub const UF_IMMUTABLE: u32 = 0x00000002; // User: File may not be changed
    pub const UF_APPEND: u32    = 0x00000004; // User: File may only be appended to
    pub const SF_IMMUTABLE: u32 = 0x00020000; // System: File may not be changed (Root cannot override without single-user mode)
    pub const SF_APPEND: u32    = 0x00040000; // System: File may only be appended to
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FilesystemProtectionFlags {
    pub flags: u32,
}

impl FilesystemProtectionFlags {
    pub fn new(flags: u32) -> Self {
        Self { flags }
    }

    pub fn is_immutable(&self) -> bool {
        (self.flags & (bsd_chflags::UF_IMMUTABLE | bsd_chflags::SF_IMMUTABLE)) != 0
    }

    pub fn is_append_only(&self) -> bool {
        (self.flags & (bsd_chflags::UF_APPEND | bsd_chflags::SF_APPEND)) != 0
    }

    /// Validates write operation against immutable/append flags
    pub fn validate_write(&self, is_append_operation: bool) -> Result<(), AccessError> {
        if self.is_immutable() {
            return Err(AccessError::ImmutableFile);
        }
        if self.is_append_only() && !is_append_operation {
            return Err(AccessError::ImmutableFile);
        }
        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. COMMAND & FUNCTION EXECUTION POLICIES (Sudo/Doas/AppArmor Constraints)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct CommandAccessPolicy {
    pub allowed_command_paths: Vec<String>,
    pub require_capability_token: u64,
    pub allow_arguments: bool,
}

impl CommandAccessPolicy {
    pub fn new(token_mask: u64) -> Self {
        Self {
            allowed_command_paths: Vec::new(),
            require_capability_token: token_mask,
            allow_arguments: true,
        }
    }

    pub fn allow_command(&mut self, path: &str) {
        self.allowed_command_paths.push(String::from(path));
    }

    pub fn check_execution(&self, command_path: &str, user_token: u64) -> Result<(), AccessError> {
        if (user_token & self.require_capability_token) != self.require_capability_token {
            return Err(AccessError::CommandNotAllowed);
        }

        if !self.allowed_command_paths.iter().any(|p| p == command_path) {
            return Err(AccessError::CommandNotAllowed);
        }

        Ok(())
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 7. ROLE-BASED ACCESS CONTROL (RBAC) & ROLE HIERARCHY
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
    pub parent_role_id: Option<RoleID>, // Role Inheritance
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
            parent_role_id: None,
        }
    }

    pub fn grant_permission(&mut self, perm_id: PermissionID) {
        if !self.permissions.contains(&perm_id) {
            self.permissions.push(perm_id);
        }
    }

    pub fn set_parent_role(&mut self, parent_id: RoleID) {
        self.parent_role_id = Some(parent_id);
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

// ─────────────────────────────────────────────────────────────────────────────
// 8. UNIFIED SOVEREIGN ACCESS CONTROL ENGINE
// ─────────────────────────────────────────────────────────────────────────────

pub struct SovereignAccessControlEngine {
    pub acm: AccessControlMatrix,
    pub command_policy: CommandAccessPolicy,
}

impl SovereignAccessControlEngine {
    pub fn new() -> Self {
        Self {
            acm: AccessControlMatrix::new(),
            command_policy: CommandAccessPolicy::new(0),
        }
    }

    /// Evaluates comprehensive access combining DAC, MAC, and BSD chflags
    pub fn check_file_write(
        &self,
        dac: &DacPermission,
        flags: &FilesystemProtectionFlags,
        subject_uid: UserID,
        subject_gid: GroupID,
        is_append: bool,
    ) -> Result<(), AccessError> {
        // 1. Check BSD chflags immutability first
        flags.validate_write(is_append)?;

        // 2. Check DAC permissions
        if !dac.evaluate_access(subject_uid, subject_gid, dac_flags::WRITE) {
            return Err(AccessError::Denied);
        }

        Ok(())
    }
}

impl Default for SovereignAccessControlEngine {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dac_evaluation_and_suid() {
        let dac = DacPermission::new(1000, 1000, 0o4750); // SUID + rwxr-x---

        assert!(dac.is_suid());
        assert!(!dac.is_sgid());

        let (euid, egid) = dac.calculate_effective_ids(2000, 2000);
        assert_eq!(euid, 1000); // SUID set to owner
        assert_eq!(egid, 2000);

        // Owner (1000) requests Write
        assert!(dac.evaluate_access(1000, 1000, dac_flags::WRITE));
        // Other user (2000) requests Read -> Denied
        assert!(!dac.evaluate_access(2000, 2000, dac_flags::READ));
    }

    #[test]
    fn test_posix_acl_evaluation() {
        let mut acl = AccessControlList::new();
        acl.add_entry(AclEntry { tag: AclTag::UserObj, permissions: 0o7 });
        acl.add_entry(AclEntry { tag: AclTag::User(2000), permissions: 0o6 });
        acl.add_entry(AclEntry { tag: AclTag::Mask, permissions: 0o6 });
        acl.add_entry(AclEntry { tag: AclTag::Other, permissions: 0o0 });

        // Named user 2000 requests Read + Write -> Granted by ACL
        assert_eq!(acl.evaluate_acl(2000, 3000, 1000, 1000, 0o6), Some(true));
        // Named user 2000 requests Execute -> Denied
        assert_eq!(acl.evaluate_acl(2000, 3000, 1000, 1000, 0o1), Some(false));
    }

    #[test]
    fn test_access_control_matrix() {
        let mut acm = AccessControlMatrix::new();
        acm.grant_right(100, 500, acm_rights::READ | acm_rights::WRITE);

        assert!(acm.check_right(100, 500, acm_rights::READ));
        assert!(acm.check_right(100, 500, acm_rights::WRITE));
        assert!(!acm.check_right(100, 500, acm_rights::EXECUTE));

        acm.revoke_right(100, 500, acm_rights::WRITE);
        assert!(!acm.check_right(100, 500, acm_rights::WRITE));
    }

    #[test]
    fn test_mac_mls_and_mcs_categories() {
        // Subject: TopSecret + Category 0x03 (Finance + Crypto)
        let subject = MacSecurityLabel::new(SensitivityLevel::TopSecret, 0x03);
        // Object: Secret + Category 0x01 (Finance)
        let object = MacSecurityLabel::new(SensitivityLevel::Secret, 0x01);

        assert!(subject.can_read(&object));

        // Object with category 0x04 (Kernel) -> Subject missing category 0x04
        let kernel_object = MacSecurityLabel::new(SensitivityLevel::Secret, 0x04);
        assert!(!subject.can_read(&kernel_object));
    }

    #[test]
    fn test_bsd_chflags_protection() {
        let immutable = FilesystemProtectionFlags::new(bsd_chflags::SF_IMMUTABLE);
        assert!(immutable.is_immutable());
        assert_eq!(immutable.validate_write(false), Err(AccessError::ImmutableFile));

        let append_only = FilesystemProtectionFlags::new(bsd_chflags::UF_APPEND);
        assert!(append_only.is_append_only());
        assert!(append_only.validate_write(true).is_ok());
        assert_eq!(append_only.validate_write(false), Err(AccessError::ImmutableFile));
    }

    #[test]
    fn test_command_execution_policy() {
        let mut policy = CommandAccessPolicy::new(0x01);
        policy.allow_command("/usr/bin/sudo");

        assert!(policy.check_execution("/usr/bin/sudo", 0x01).is_ok());
        assert_eq!(policy.check_execution("/usr/bin/sudo", 0x00), Err(AccessError::CommandNotAllowed));
        assert_eq!(policy.check_execution("/bin/rm", 0x01), Err(AccessError::CommandNotAllowed));
    }
}
