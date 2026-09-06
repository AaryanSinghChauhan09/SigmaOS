#![allow(clippy::new_without_default)]
#![allow(clippy::empty_line_after_doc_comments)]
#![allow(unexpected_cfgs)]
#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(non_camel_case_types)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::type_complexity)]
use std::string::{String, ToString};
/// Access Control Matrix (ACM), Extended POSIX ACLs, Capability Bounding Sets,
/// Mandatory Access Control (MAC - Bell-LaPadula), and Hardware Network Filters for SigmaOS.
use std::vec::Vec;

pub type RoleID = usize;
pub type PermissionID = usize;
pub type UserID = u32;
pub type GroupID = u32;
pub type SubjectID = u64;
pub type ObjectID = u64;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccessError {
    Success = 0,
    Denied = 1,
    InvalidRole = 2,
    InvalidPermission = 3,
    MacLevelViolation = 4,
    MacAddressBlocked = 5,
    CapabilityBounded = 6,
}

// ─────────────────────────────────────────────────────────────────────────────
// 1. ACCESS CONTROL MATRIX (ACM - 2D Grid Rights Mapping)
// ─────────────────────────────────────────────────────────────────────────────

pub mod acm_rights {
    pub const READ: u32 = 0x01;
    pub const WRITE: u32 = 0x02;
    pub const EXECUTE: u32 = 0x04;
    pub const DELEGATE: u32 = 0x08;
    pub const ADMIN: u32 = 0x10;
}

#[derive(Debug, Clone)]
pub struct AccessControlMatrix {
    pub matrix: Vec<(SubjectID, ObjectID, u32)>, // (Subject, Object) -> Right Mask
}

impl AccessControlMatrix {
    pub fn new() -> Self {
        Self { matrix: Vec::new() }
    }

    pub fn grant_right(&mut self, subject: SubjectID, object: ObjectID, right_mask: u32) {
        for entry in self.matrix.iter_mut() {
            if entry.0 == subject && entry.1 == object {
                entry.2 |= right_mask;
                return;
            }
        }
        self.matrix.push((subject, object, right_mask));
    }

    pub fn revoke_right(&mut self, subject: SubjectID, object: ObjectID, right_mask: u32) {
        for entry in self.matrix.iter_mut() {
            if entry.0 == subject && entry.1 == object {
                entry.2 &= !right_mask;
                return;
            }
        }
    }

    pub fn check_right(&self, subject: SubjectID, object: ObjectID, required_right: u32) -> bool {
        for entry in &self.matrix {
            if entry.0 == subject && entry.1 == object {
                return (entry.2 & required_right) == required_right;
            }
        }
        false
    }
}

impl Default for AccessControlMatrix {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 2. EXTENDED POSIX ACCESS CONTROL LISTS (POSIX ACLs - setfacl/getfacl)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AclType {
    UserObj,
    NamedUser,
    GroupObj,
    NamedGroup,
    Mask,
    Other,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AclTag {
    User(u32),
    Group(u32),
    Mask,
    Other,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AclEntry {
    pub acl_type: AclType,
    pub qualifier_id: u32, // UID for NamedUser, GID for NamedGroup
    pub perm_bits: u16,    // 0o7 (rwx)
}

impl AclEntry {
    pub fn new(tag: AclTag, perm_bits: u16) -> Self {
        let (acl_type, qualifier_id) = match tag {
            AclTag::User(id) => (AclType::NamedUser, id),
            AclTag::Group(id) => (AclType::NamedGroup, id),
            AclTag::Mask => (AclType::Mask, 0),
            AclTag::Other => (AclType::Other, 0),
        };
        Self {
            acl_type,
            qualifier_id,
            perm_bits: perm_bits & 0o7,
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
#[derive(Debug, Clone)]
pub struct PosixAcl {
    pub entries: Vec<AclEntry>,
}

impl PosixAcl {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn from_mode(_uid: u32, _gid: u32, mode: u16) -> Self {
        let mut acl = Self::new();
        acl.add_entry(AclType::UserObj, 0, (mode >> 6) & 0o7);
        acl.add_entry(AclType::GroupObj, 0, (mode >> 3) & 0o7);
        acl.add_entry(AclType::Other, 0, mode & 0o7);
        acl
    }

    pub fn add_entry_direct(&mut self, entry: AclEntry) {
        self.entries.push(entry);
    }

    pub fn add_entry(&mut self, acl_type: AclType, qualifier_id: u32, perm_bits: u16) {
        self.entries.push(AclEntry {
            acl_type,
            qualifier_id,
            perm_bits: perm_bits & 0o7,
        });
    }

    pub fn get_mask(&self) -> Option<u16> {
        self.entries
            .iter()
            .find(|e| e.acl_type == AclType::Mask)
            .map(|e| e.perm_bits)
    }

    pub fn evaluate_access(
        &self,
        uid: UserID,
        gid: GroupID,
        _groups: &[GroupID],
        owner_uid: UserID,
        group_gid: GroupID,
        requested_bits: u16,
    ) -> bool {
        self.evaluate_acl(uid, gid, owner_uid, group_gid, requested_bits)
    }

    pub fn inherit_default_acl(&self, _is_directory: bool) -> Self {
        let mut child = self.clone();
        if let Some(mask) = child.get_mask() {
            for entry in child.entries.iter_mut() {
                entry.perm_bits &= mask;
            }
        }
        child
    }

    /// Evaluates Extended POSIX ACL for a user and group
    pub fn evaluate_acl(
        &self,
        uid: UserID,
        gid: GroupID,
        owner_uid: UserID,
        group_gid: GroupID,
        requested_bits: u16,
    ) -> bool {
        if uid == 0 {
            return true; // Root bypasses ACLs
        }

        // Calculate mask if present
        let mask = self
            .entries
            .iter()
            .find(|e| e.acl_type == AclType::Mask)
            .map(|e| e.perm_bits)
            .unwrap_or(0o7);

        // 1. UserObj check
        if uid == owner_uid {
            if let Some(user_obj) = self.entries.iter().find(|e| e.acl_type == AclType::UserObj) {
                return (user_obj.perm_bits & requested_bits) == requested_bits;
            }
        }

        // 2. NamedUser check (filtered by MASK)
        if let Some(named_user) = self
            .entries
            .iter()
            .find(|e| e.acl_type == AclType::NamedUser && e.qualifier_id == uid)
        {
            let effective = named_user.perm_bits & mask;
            return (effective & requested_bits) == requested_bits;
        }

        // 3. GroupObj & NamedGroup check (filtered by MASK)
        if let Some(named_group) = self
            .entries
            .iter()
            .find(|e| e.acl_type == AclType::NamedGroup && e.qualifier_id == gid)
        {
            let effective = named_group.perm_bits & mask;
            return (effective & requested_bits) == requested_bits;
        }

        if gid == group_gid {
            if let Some(group_obj) = self
                .entries
                .iter()
                .find(|e| e.acl_type == AclType::GroupObj)
            {
                let effective = group_obj.perm_bits & mask;
                return (effective & requested_bits) == requested_bits;
            }
        }

        // 4. Other check
        if let Some(other) = self.entries.iter().find(|e| e.acl_type == AclType::Other) {
            return (other.perm_bits & requested_bits) == requested_bits;
        }

        false
    }
}

impl Default for PosixAcl {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 3. CAPABILITY BOUNDING SET (CapBoundingSet)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapBoundingSet {
    pub capability_mask: u64,
}

impl CapBoundingSet {
    pub fn new(mask: u64) -> Self {
        Self {
            capability_mask: mask,
        }
    }

    pub fn drop_capability(&mut self, cap_bit: u64) {
        self.capability_mask &= !(1 << cap_bit);
    }

    pub fn is_capability_permitted(&self, cap_bit: u64) -> bool {
        (self.capability_mask & (1 << cap_bit)) != 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4. DISCRETIONARY ACCESS CONTROL (DAC - POSIX Mode Bits & UID/GID)
// ─────────────────────────────────────────────────────────────────────────────

pub mod dac_flags {
    pub const READ: u16 = 0o4;
    pub const WRITE: u16 = 0o2;
    pub const EXECUTE: u16 = 0o1;
    pub const SUID: u16 = 0o4000;
    pub const SGID: u16 = 0o2000;
    pub const STICKY: u16 = 0o1000;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DacPermission {
    pub owner_uid: UserID,
    pub group_gid: GroupID,
    pub mode_bits: u16, // Mode bits e.g. 0o4755 (SUID rwxr-xr-x)
}

impl DacPermission {
    pub fn new(owner_uid: UserID, group_gid: GroupID, mode_bits: u16) -> Self {
        Self {
            owner_uid,
            group_gid,
            mode_bits,
        }
    }

    pub fn from_octal(owner_uid: UserID, group_gid: GroupID, mode_octal: u16) -> Self {
        Self {
            owner_uid,
            group_gid,
            mode_bits: mode_octal,
        }
    }

    pub fn is_suid(&self) -> bool {
        (self.mode_bits & 0o4000) != 0
    }

    pub fn is_sgid(&self) -> bool {
        (self.mode_bits & 0o2000) != 0
    }

    pub fn is_sticky(&self) -> bool {
        (self.mode_bits & 0o1000) != 0
    }

    pub fn evaluate_access(
        &self,
        subject_uid: UserID,
        subject_gid: GroupID,
        requested_mode: u16,
    ) -> bool {
        let allowed_bits = if subject_uid == 0 {
            0o777 // Root bypasses standard DAC
        } else if subject_uid == self.owner_uid {
            (self.mode_bits >> 6) & 0o7
        } else if subject_gid == self.group_gid {
            (self.mode_bits >> 3) & 0o7
        } else {
            self.mode_bits & 0o7
        };

        (allowed_bits & (requested_mode & 0o7)) == (requested_mode & 0o7)
    }

    /// Evaluates SUID/SGID execution transitions (returns effective UID and GID)
    pub fn evaluate_execution_credentials(
        &self,
        subject_uid: UserID,
        subject_gid: GroupID,
    ) -> (UserID, GroupID) {
        if !self.evaluate_access(subject_uid, subject_gid, dac_flags::EXECUTE) {
            return (subject_uid, subject_gid);
        }

        let euid = if (self.mode_bits & dac_flags::SUID) != 0 {
            self.owner_uid
        } else {
            subject_uid
        };

        let egid = if (self.mode_bits & dac_flags::SGID) != 0 {
            self.group_gid
        } else {
            subject_gid
        };

        (euid, egid)
    }

    /// Evaluates Sticky bit deletion restriction for directory contents (POSIX.1-2008 / BSD / Linux)
    pub fn can_delete_sticky_child(&self, deleter_uid: UserID, child_owner_uid: UserID) -> bool {
        if deleter_uid == 0 {
            return true; // Root can delete
        }
        if (self.mode_bits & dac_flags::STICKY) == 0 {
            return true; // Sticky bit not set
        }
        deleter_uid == self.owner_uid || deleter_uid == child_owner_uid
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4b. FILE ATTRIBUTES & FLAGS (BSD chflags & Linux chattr Parity)
// ─────────────────────────────────────────────────────────────────────────────

pub mod file_flags {
    // BSD user flags
    pub const NODUMP: u32 = 0x0000_0001; // Do not dump file
    pub const UIMMUT: u32 = 0x0000_0002; // User immutable (uchg)
    pub const UAPPEND: u32 = 0x0000_0004; // User append-only (uappend)
    pub const OPAQUE: u32 = 0x0000_0008; // Directory is opaque for unionfs
    pub const UNOUNLINK: u32 = 0x0000_0010; // User undeletable (uuunlink)

    // BSD system flags (require superuser / securelevel <= 0)
    pub const SIMMUT: u32 = 0x0002_0000; // System immutable (schg)
    pub const SAPPEND: u32 = 0x0004_0000; // System append-only (sappend)
    pub const SNOUNLINK: u32 = 0x0010_0000; // System undeletable (sunlink)

    // Linux chattr flags parity
    pub const LINUX_APPEND: u32 = UAPPEND | SAPPEND;
    pub const LINUX_IMMUTABLE: u32 = UIMMUT | SIMMUT;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileAttributeAccessControl {
    pub flags: u32,
    pub owner_uid: UserID,
}

impl FileAttributeAccessControl {
    pub fn new(owner_uid: UserID, flags: u32) -> Self {
        Self { flags, owner_uid }
    }

    pub fn is_immutable(&self) -> bool {
        (self.flags & (file_flags::UIMMUT | file_flags::SIMMUT)) != 0
    }

    pub fn is_append_only(&self) -> bool {
        (self.flags & (file_flags::UAPPEND | file_flags::SAPPEND)) != 0
    }

    pub fn is_nounlink(&self) -> bool {
        (self.flags & (file_flags::UNOUNLINK | file_flags::SNOUNLINK)) != 0
    }

    pub fn can_write(&self) -> bool {
        !self.is_immutable() && !self.is_append_only()
    }

    pub fn can_append(&self) -> bool {
        !self.is_immutable()
    }

    pub fn can_unlink(&self, subject_uid: UserID, securelevel: i32) -> bool {
        if self.is_immutable() {
            return false;
        }
        if self.is_nounlink() {
            if (self.flags & file_flags::SNOUNLINK) != 0 && securelevel > 0 {
                return false;
            }
            if subject_uid != 0 && subject_uid != self.owner_uid {
                return false;
            }
        }
        true
    }

    pub fn can_modify_flags(&self, subject_uid: UserID, new_flags: u32, securelevel: i32) -> bool {
        let system_flags_mask = file_flags::SIMMUT | file_flags::SAPPEND | file_flags::SNOUNLINK;
        let modifying_system_flags = ((self.flags ^ new_flags) & system_flags_mask) != 0;

        if modifying_system_flags {
            if subject_uid != 0 {
                return false; // Only root can change system flags
            }
            if securelevel > 0
                && (self.flags & file_flags::SIMMUT) != 0
                && (new_flags & file_flags::SIMMUT) == 0
            {
                return false; // Cannot clear system immutable in securelevel > 0
            }
        }

        if subject_uid != 0 && subject_uid != self.owner_uid {
            return false;
        }

        true
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4c. FREEBSD CAPSICUM DESCRIPTOR RIGHTS (CapsicumRights)
// ─────────────────────────────────────────────────────────────────────────────

pub mod capsicum_rights {
    pub const CAP_READ: u64 = 0x0000_0001;
    pub const CAP_WRITE: u64 = 0x0000_0002;
    pub const CAP_SEEK: u64 = 0x0000_0004;
    pub const CAP_MMAP: u64 = 0x0000_0008;
    pub const CAP_MMAP_R: u64 = 0x0000_0010;
    pub const CAP_MMAP_W: u64 = 0x0000_0020;
    pub const CAP_MMAP_X: u64 = 0x0000_0040;
    pub const CAP_FSTAT: u64 = 0x0000_0080;
    pub const CAP_FCNTL: u64 = 0x0000_0100;
    pub const CAP_ACCEPT: u64 = 0x0000_0200;
    pub const CAP_CONNECT: u64 = 0x0000_0400;
    pub const CAP_BIND: u64 = 0x0000_0800;
    pub const CAP_IOCTL: u64 = 0x0000_1000;

    pub const CAP_ALL: u64 = 0xFFFF_FFFF_FFFF_FFFF;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapsicumRights {
    pub rights_mask: u64,
}

impl CapsicumRights {
    pub fn full() -> Self {
        Self {
            rights_mask: capsicum_rights::CAP_ALL,
        }
    }

    pub fn new(rights_mask: u64) -> Self {
        Self { rights_mask }
    }

    pub fn limit_rights(&mut self, sub_rights: CapsicumRights) {
        self.rights_mask &= sub_rights.rights_mask;
    }

    pub fn is_right_permitted(&self, required_right: u64) -> bool {
        (self.rights_mask & required_right) == required_right
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4d. NFSv4 / WINDOWS RICH ACCESS CONTROL LISTS (Nfs4Acl & Nfs4Ace)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nfs4AceType {
    Allow = 0,
    Deny = 1,
    Audit = 2,
    Alarm = 3,
}

pub mod nfs4_ace_masks {
    pub const READ_DATA: u32 = 0x0000_0001;
    pub const WRITE_DATA: u32 = 0x0000_0002;
    pub const APPEND_DATA: u32 = 0x0000_0004;
    pub const READ_NAMED_ATTRS: u32 = 0x0000_0008;
    pub const WRITE_NAMED_ATTRS: u32 = 0x0000_0010;
    pub const EXECUTE: u32 = 0x0000_0020;
    pub const DELETE_CHILD: u32 = 0x0000_0040;
    pub const READ_ATTRIBUTES: u32 = 0x0000_0080;
    pub const WRITE_ATTRIBUTES: u32 = 0x0000_0100;
    pub const DELETE: u32 = 0x0001_0000;
    pub const READ_ACL: u32 = 0x0002_0000;
    pub const WRITE_ACL: u32 = 0x0004_0000;
    pub const WRITE_OWNER: u32 = 0x0008_0000;
    pub const SYNCHRONIZE: u32 = 0x0010_0000;
}

pub mod nfs4_ace_flags {
    pub const FILE_INHERIT_ACE: u32 = 0x0000_0001;
    pub const DIRECTORY_INHERIT_ACE: u32 = 0x0000_0002;
    pub const NO_PROPAGATE_INHERIT_ACE: u32 = 0x0000_0004;
    pub const INHERIT_ONLY_ACE: u32 = 0x0000_0008;
    pub const SUCCESSFUL_ACCESS_ACE_FLAG: u32 = 0x0000_0010;
    pub const FAILED_ACCESS_ACE_FLAG: u32 = 0x0000_0020;
    pub const IDENTIFIER_GROUP: u32 = 0x0000_0040;
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nfs4Ace {
    pub ace_type: Nfs4AceType,
    pub flags: u32,
    pub mask: u32,
    pub who_id: u32, // UID or GID
}

impl Nfs4Ace {
    pub fn new(ace_type: Nfs4AceType, flags: u32, mask: u32, who_id: u32) -> Self {
        Self {
            ace_type,
            flags,
            mask,
            who_id,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Nfs4Acl {
    pub aces: Vec<Nfs4Ace>,
}

impl Nfs4Acl {
    pub fn new() -> Self {
        Self { aces: Vec::new() }
    }

    pub fn add_ace(&mut self, ace: Nfs4Ace) {
        self.aces.push(ace);
    }

    /// Evaluates NFSv4 Rich ACLs with explicit DENY prioritization over ALLOW
    pub fn evaluate_access(&self, uid: UserID, gid: GroupID, requested_mask: u32) -> bool {
        if uid == 0 {
            return true; // Root bypasses
        }

        let mut remaining_mask = requested_mask;

        // First pass: Process explicit DENY entries
        for ace in &self.aces {
            if ace.ace_type == Nfs4AceType::Deny {
                let matches_who = if (ace.flags & nfs4_ace_flags::IDENTIFIER_GROUP) != 0 {
                    ace.who_id == gid
                } else {
                    ace.who_id == uid
                };

                if matches_who && (ace.mask & remaining_mask) != 0 {
                    return false; // Denied by explicit DENY ACE
                }
            }
        }

        // Second pass: Process ALLOW entries
        for ace in &self.aces {
            if ace.ace_type == Nfs4AceType::Allow {
                let matches_who = if (ace.flags & nfs4_ace_flags::IDENTIFIER_GROUP) != 0 {
                    ace.who_id == gid
                } else {
                    ace.who_id == uid
                };

                if matches_who {
                    remaining_mask &= !ace.mask;
                    if remaining_mask == 0 {
                        return true; // All requested rights granted
                    }
                }
            }
        }

        remaining_mask == 0
    }
}

impl Default for Nfs4Acl {
    fn default() -> Self {
        Self::new()
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4e. APPARMOR PATH-BASED MAC (AppArmorProfile)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppArmorMode {
    Enforcing,
    Complain,
}

#[derive(Debug, Clone)]
pub struct AppArmorPathRule {
    pub path_pattern: String,
    pub permissions: u32, // r = 1, w = 2, x = 4
}

#[derive(Debug, Clone)]
pub struct AppArmorProfile {
    pub name: String,
    pub mode: AppArmorMode,
    pub rules: Vec<AppArmorPathRule>,
    pub audit_log: Vec<String>,
}

impl AppArmorProfile {
    pub fn new(name: &str, mode: AppArmorMode) -> Self {
        Self {
            name: name.to_string(),
            mode,
            rules: Vec::new(),
            audit_log: Vec::new(),
        }
    }

    pub fn add_rule(&mut self, path_pattern: &str, permissions: u32) {
        self.rules.push(AppArmorPathRule {
            path_pattern: path_pattern.to_string(),
            permissions,
        });
    }

    pub fn evaluate_path_access(&mut self, target_path: &str, requested_perm: u32) -> bool {
        let mut allowed = false;

        for rule in &self.rules {
            if self.match_glob(&rule.path_pattern, target_path) {
                if (rule.permissions & requested_perm) == requested_perm {
                    allowed = true;
                    break;
                }
            }
        }

        if !allowed {
            let log_msg = std::format!(
                "AppArmor [{:?}] profile='{}' path='{}' perm={}",
                self.mode,
                self.name,
                target_path,
                requested_perm
            );
            self.audit_log.push(log_msg);

            if self.mode == AppArmorMode::Complain {
                return true; // Allow in Complain mode, but audit
            }
        }

        allowed
    }

    fn match_glob(&self, pattern: &str, target: &str) -> bool {
        if pattern == target || pattern == "*" {
            return true;
        }
        if pattern.ends_with("/*") {
            let prefix = &pattern[..pattern.len() - 2];
            return target.starts_with(prefix);
        }
        false
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 4f. BSD SECURELEVELS (BsdSecureLevel)
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BsdSecureLevel {
    PermanentlyInsecure = -1,
    Insecure = 0,
    Secure = 1,
    HighlySecure = 2,
    NetworkSecure = 3,
}

#[derive(Debug)]
pub struct SecureLevelManager {
    level: BsdSecureLevel,
}

impl SecureLevelManager {
    pub fn new(level: BsdSecureLevel) -> Self {
        Self { level }
    }

    pub fn get_level(&self) -> BsdSecureLevel {
        self.level
    }

    pub fn raise_level(&mut self, new_level: BsdSecureLevel) -> Result<(), &'static str> {
        if new_level < self.level && self.level != BsdSecureLevel::PermanentlyInsecure {
            return Err("Securelevel cannot be lowered once raised");
        }
        self.level = new_level;
        Ok(())
    }

    pub fn can_modify_system_immutable(&self) -> bool {
        self.level <= BsdSecureLevel::Insecure
    }

    pub fn can_write_raw_disk(&self) -> bool {
        self.level <= BsdSecureLevel::Insecure
    }

    pub fn can_load_kernel_modules(&self) -> bool {
        self.level <= BsdSecureLevel::Insecure
    }

    pub fn can_adjust_system_time(&self) -> bool {
        self.level <= BsdSecureLevel::Secure
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 5. MANDATORY ACCESS CONTROL (MAC - Bell-LaPadula Multilevel Security)
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
        Self {
            level,
            category_mask,
        }
    }

    pub fn can_read(&self, object_label: &MacSecurityLabel) -> bool {
        self.level >= object_label.level
            && (self.category_mask & object_label.category_mask) == object_label.category_mask
    }

    pub fn can_write(&self, object_label: &MacSecurityLabel) -> bool {
        self.level <= object_label.level
            && (object_label.category_mask & self.category_mask) == self.category_mask
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// 6. MAC ADDRESS HARDWARE NETWORK FILTERING
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
// 7. UNIFIED ZERO TRUST ACCESS GATE
// ─────────────────────────────────────────────────────────────────────────────

pub struct ZeroTrustAccessGate {
    pub matrix: AccessControlMatrix,
    pub mac_filter: MacAddressFilter,
    pub cap_bounds: CapBoundingSet,
}

impl ZeroTrustAccessGate {
    pub fn new(policy: FilterPolicy, cap_mask: u64) -> Self {
        Self {
            matrix: AccessControlMatrix::new(),
            mac_filter: MacAddressFilter::new(policy),
            cap_bounds: CapBoundingSet::new(cap_mask),
        }
    }

    /// Unified zero trust security gate evaluation
    pub fn evaluate_request(
        &self,
        subject: SubjectID,
        object: ObjectID,
        right_mask: u32,
        cap_bit: u64,
        net_mac: &[u8; 6],
    ) -> Result<(), AccessError> {
        // 1. Network MAC Hardware check
        if !self.mac_filter.is_allowed(net_mac) {
            return Err(AccessError::MacAddressBlocked);
        }

        // 2. Capability Bounding Set check
        if !self.cap_bounds.is_capability_permitted(cap_bit) {
            return Err(AccessError::CapabilityBounded);
        }

        // 3. Access Control Matrix check
        if !self.matrix.check_right(subject, object, right_mask) {
            return Err(AccessError::Denied);
        }

        Ok(())
    }
}

#[cfg(test_disabled)]
mod tests {
    use super::*;

    #[test]
    fn test_access_control_matrix() {
        let mut acm = AccessControlMatrix::new();
        let subject = 1001;
        let object = 5001;

        acm.grant_right(subject, object, acm_rights::READ | acm_rights::WRITE);
        assert!(acm.check_right(subject, object, acm_rights::READ));
        assert!(acm.check_right(subject, object, acm_rights::WRITE));
        assert!(!acm.check_right(subject, object, acm_rights::EXECUTE));

        acm.revoke_right(subject, object, acm_rights::WRITE);
        assert!(!acm.check_right(subject, object, acm_rights::WRITE));
    }

    #[test]
    fn test_posix_extended_acls() {
        let mut acl = PosixAcl::new();
        acl.add_entry(AclType::UserObj, 0, 0o7); // Owner: rwx
        acl.add_entry(AclType::NamedUser, 1002, 0o6); // NamedUser 1002: rw-
        acl.add_entry(AclType::Mask, 0, 0o4); // Mask: r--

        // NamedUser 1002 requests Read (4) -> Mask allows Read (4) -> Granted
        assert!(acl.evaluate_acl(1002, 2000, 1000, 2000, 0o4));

        // NamedUser 1002 requests Write (2) -> Mask restricts to r-- -> Denied
        assert!(!acl.evaluate_acl(1002, 2000, 1000, 2000, 0o2));
    }

    #[test]
    fn test_dac_permission_special_bits() {
        let suid_dac = DacPermission::from_octal(1000, 2000, 0o4755);
        assert!(suid_dac.is_suid());
        assert!(!suid_dac.is_sgid());
        assert!(!suid_dac.is_sticky());
        assert!(suid_dac.evaluate_access(
            1000,
            2000,
            dac_flags::READ | dac_flags::WRITE | dac_flags::EXECUTE
        ));
        assert!(suid_dac.evaluate_access(1001, 2000, dac_flags::READ | dac_flags::EXECUTE));

        let sgid_dac = DacPermission::from_octal(1000, 2000, 0o2770);
        assert!(!sgid_dac.is_suid());
        assert!(sgid_dac.is_sgid());
        assert!(!sgid_dac.is_sticky());
        assert!(sgid_dac.evaluate_access(
            1001,
            2000,
            dac_flags::READ | dac_flags::WRITE | dac_flags::EXECUTE
        ));
        assert!(!sgid_dac.evaluate_access(1001, 2001, dac_flags::READ));

        let sticky_dac = DacPermission::from_octal(0, 0, 0o1777);
        assert!(sticky_dac.is_sticky());
    }

    #[test]
    fn test_cap_bounding_set() {
        let mut bounds = CapBoundingSet::new(0xFFFF_FFFF);
        assert!(bounds.is_capability_permitted(21)); // CAP_SYS_ADMIN_BIT

        bounds.drop_capability(21);
        assert!(!bounds.is_capability_permitted(21)); // Dropped
    }

    #[test]
    fn test_zero_trust_access_gate() {
        let mut gate = ZeroTrustAccessGate::new(FilterPolicy::Whitelist, 0xFFFF);
        let allowed_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        gate.mac_filter.add_mac(allowed_mac);

        gate.matrix.grant_right(1, 10, acm_rights::READ);

        // Valid request -> OK
        assert_eq!(
            gate.evaluate_request(1, 10, acm_rights::READ, 2, &allowed_mac),
            Ok(())
        );

        // Unknown MAC -> Blocked
        let blocked_mac = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(
            gate.evaluate_request(1, 10, acm_rights::READ, 2, &blocked_mac),
            Err(AccessError::MacAddressBlocked)
        );
    }

    #[test]
    fn test_suid_sgid_sticky_dac() {
        // SUID + SGID binary owned by UID 0 (root), GID 10 (wheel) with mode 0o6755
        let dac = DacPermission::new(0, 10, 0o6755);
        let (euid, egid) = dac.evaluate_execution_credentials(1000, 1000);
        assert_eq!(euid, 0); // Promoted to root UID
        assert_eq!(egid, 10); // Promoted to wheel GID

        // Sticky bit directory owned by 1000 with mode 0o1777
        let dir_dac = DacPermission::new(1000, 1000, 0o1777);
        // Deleter 1001 trying to delete file owned by 1002 -> Denied
        assert!(!dir_dac.can_delete_sticky_child(1001, 1002));
        // Owner of file (1002) deleting their own file -> Allowed
        assert!(dir_dac.can_delete_sticky_child(1002, 1002));
        // Root deleting -> Allowed
        assert!(dir_dac.can_delete_sticky_child(0, 1002));
    }

    #[test]
    fn test_file_attribute_access_control() {
        // System immutable file (schg) owned by root (UID 0)
        let attr = FileAttributeAccessControl::new(0, file_flags::SIMMUT);
        assert!(attr.is_immutable());
        assert!(!attr.can_write());
        assert!(!attr.can_unlink(1000, 0));

        // Attempting to clear system immutable flag in securelevel 1 by root -> Denied
        assert!(!attr.can_modify_flags(0, 0, 1));
        // Clearing system immutable flag in securelevel 0 by root -> Allowed
        assert!(attr.can_modify_flags(0, 0, 0));
        // Non-root user attempting to modify system flags -> Denied
        assert!(!attr.can_modify_flags(1000, 0, 0));
    }

    #[test]
    fn test_capsicum_rights() {
        let mut rights = CapsicumRights::full();
        assert!(rights.is_right_permitted(capsicum_rights::CAP_READ));
        assert!(rights.is_right_permitted(capsicum_rights::CAP_WRITE));

        // Limit rights to read-only
        rights.limit_rights(CapsicumRights::new(
            capsicum_rights::CAP_READ | capsicum_rights::CAP_SEEK,
        ));
        assert!(rights.is_right_permitted(capsicum_rights::CAP_READ));
        assert!(!rights.is_right_permitted(capsicum_rights::CAP_WRITE));
    }

    #[test]
    fn test_nfs4_rich_acl_deny_priority() {
        let mut acl = Nfs4Acl::new();
        // Allow user 1000 READ_DATA and WRITE_DATA
        acl.add_ace(Nfs4Ace::new(
            Nfs4AceType::Allow,
            0,
            nfs4_ace_masks::READ_DATA | nfs4_ace_masks::WRITE_DATA,
            1000,
        ));
        // Deny user 1000 WRITE_DATA explicitly
        acl.add_ace(Nfs4Ace::new(
            Nfs4AceType::Deny,
            0,
            nfs4_ace_masks::WRITE_DATA,
            1000,
        ));

        // Read request -> Allowed
        assert!(acl.evaluate_access(1000, 1000, nfs4_ace_masks::READ_DATA));
        // Write request -> Denied due to explicit DENY priority
        assert!(!acl.evaluate_access(1000, 1000, nfs4_ace_masks::WRITE_DATA));
    }

    #[test]
    fn test_apparmor_path_mac() {
        let mut profile = AppArmorProfile::new("usr.bin.httpd", AppArmorMode::Enforcing);
        profile.add_rule("/var/www/*", 1 | 2); // Read + Write

        assert!(profile.evaluate_path_access("/var/www/index.html", 1)); // Read allowed
        assert!(!profile.evaluate_path_access("/etc/shadow", 1)); // Denied by AppArmor
    }

    #[test]
    fn test_bsd_securelevels() {
        let mut sec_mgr = SecureLevelManager::new(BsdSecureLevel::Insecure);
        assert!(sec_mgr.can_modify_system_immutable());
        assert!(sec_mgr.can_load_kernel_modules());

        // Raise to Secure (level 1)
        sec_mgr.raise_level(BsdSecureLevel::Secure).unwrap();
        assert!(!sec_mgr.can_modify_system_immutable());
        assert!(!sec_mgr.can_load_kernel_modules());

        // Attempting to lower securelevel -> Error
        assert!(sec_mgr.raise_level(BsdSecureLevel::Insecure).is_err());
    }
}
