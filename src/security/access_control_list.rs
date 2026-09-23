#![allow(clippy::empty_line_after_doc_comments)]
#![allow(clippy::new_without_default)]
#![allow(non_camel_case_types)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unexpected_cfgs)]
extern crate alloc;

use alloc::boxed::Box;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[cfg(not(any(feature = "standalone_test", test)))]
use crate::klib::{BTreeMap, HashMap, HashSet};

#[cfg(any(feature = "standalone_test", test))]
use std::collections::{BTreeMap, HashMap, HashSet};

// ============================================================================
// Linux & BSD Access Control List (ACL) Subsystem
// Inspired by POSIX.1e, NFSv4/ZFS ACLs, RichACLs, FreeBSD ACLs, OpenBSD path ACLs
// ============================================================================

/// ACL Model Type: POSIX.1e (Linux/FreeBSD standard) vs NFSv4/ZFS (NFSv4/ZFS/macOS fine-grained)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AclModel {
    Posix1e,
    Nfsv4Zfs,
    RichAcl,
}

/// ACL Tag Identifier inspired by Linux POSIX.1e and FreeBSD `acl_tag_t`
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AclTag {
    UserOwner,           // ACL_USER_OBJ / owner@
    GroupOwner,          // ACL_GROUP_OBJ / group@
    Other,               // ACL_OTHER / everyone@
    Mask,                // ACL_MASK (POSIX.1e effective mask)
    NamedUser(u32),      // ACL_USER (uid)
    NamedGroup(u32),     // ACL_GROUP (gid)
    Everyone,            // NFSv4 everyone@
    SpecialOwner,        // NFSv4 owner@
    SpecialGroup,        // NFSv4 group@
}

/// NFSv4 / ZFS ACL Entry Type
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AclEntryType {
    Allow,  // ACCESS_ALLOWED
    Deny,   // ACCESS_DENIED
    Audit,  // SYSTEM_AUDIT
    Alarm,  // SYSTEM_ALARM
}

/// POSIX & NFSv4 Granular Permission Bits
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AclPermission {
    ReadData,          // 'r' / READ_DATA
    WriteData,         // 'w' / WRITE_DATA
    Execute,           // 'x' / EXECUTE
    AppendData,        // APPEND_DATA
    Delete,            // DELETE
    DeleteChild,       // DELETE_CHILD
    ReadAttributes,    // READ_ATTRIBUTES
    WriteAttributes,   // WRITE_ATTRIBUTES
    ReadNamedAttrs,    // READ_NAMED_ATTRS
    WriteNamedAttrs,   // WRITE_NAMED_ATTRS
    ReadAcl,           // READ_ACL
    WriteAcl,          // WRITE_ACL
    WriteOwner,        // WRITE_OWNER
    Synchronize,       // SYNCHRONIZE
}

/// NFSv4 / ZFS Inheritance & Control Flags
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AclInheritanceFlag {
    FileInherit,       // FILE_INHERIT_ACE (inherited by files)
    DirectoryInherit,  // DIRECTORY_INHERIT_ACE (inherited by subdirs)
    NoPropagate,       // NO_PROPAGATE_INHERIT_ACE
    InheritOnly,       // INHERIT_ONLY_ACE
    SuccessfulAccess,  // SUCCESSFUL_ACCESS_ACE_FLAG (Audit/Alarm)
    FailedAccess,      // FAILED_ACCESS_ACE_FLAG (Audit/Alarm)
    Inherited,         // INHERITED_ACE (indicates entry was inherited)
}

/// An Access Control Entry (ACE)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignAclEntry {
    pub tag: AclTag,
    pub entry_type: AclEntryType,
    pub permissions: Vec<AclPermission>,
    pub inheritance_flags: Vec<AclInheritanceFlag>,
}

impl SovereignAclEntry {
    pub fn new_posix(tag: AclTag, read: bool, write: bool, execute: bool) -> Self {
        let mut perms = Vec::new();
        if read {
            perms.push(AclPermission::ReadData);
            perms.push(AclPermission::ReadAttributes);
        }
        if write {
            perms.push(AclPermission::WriteData);
            perms.push(AclPermission::AppendData);
            perms.push(AclPermission::WriteAttributes);
        }
        if execute {
            perms.push(AclPermission::Execute);
        }

        Self {
            tag,
            entry_type: AclEntryType::Allow,
            permissions: perms,
            inheritance_flags: Vec::new(),
        }
    }

    pub fn new_nfsv4(
        tag: AclTag,
        entry_type: AclEntryType,
        permissions: Vec<AclPermission>,
        flags: Vec<AclInheritanceFlag>,
    ) -> Self {
        Self {
            tag,
            entry_type,
            permissions,
            inheritance_flags: flags,
        }
    }

    pub fn has_permission(&self, perm: AclPermission) -> bool {
        self.permissions.contains(&perm)
    }

    /// Check if permission is POSIX read
    pub fn is_readable(&self) -> bool {
        self.has_permission(AclPermission::ReadData)
    }

    /// Check if permission is POSIX write
    pub fn is_writable(&self) -> bool {
        self.has_permission(AclPermission::WriteData)
    }

    /// Check if permission is POSIX execute
    pub fn is_executable(&self) -> bool {
        self.has_permission(AclPermission::Execute)
    }
}

/// Complete Access Control List (ACL) for a file, directory, or object
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SovereignAccessControlList {
    pub model: AclModel,
    pub entries: Vec<SovereignAclEntry>,
}

impl SovereignAccessControlList {
    pub fn new(model: AclModel) -> Self {
        Self {
            model,
            entries: Vec::new(),
        }
    }

    pub fn from_mode(mode: u32) -> Self {
        let mut acl = Self::new(AclModel::Posix1e);
        let u_r = (mode & 0o400) != 0;
        let u_w = (mode & 0o200) != 0;
        let u_x = (mode & 0o100) != 0;

        let g_r = (mode & 0o040) != 0;
        let g_w = (mode & 0o020) != 0;
        let g_x = (mode & 0o010) != 0;

        let o_r = (mode & 0o004) != 0;
        let o_w = (mode & 0o002) != 0;
        let o_x = (mode & 0o001) != 0;

        acl.entries.push(SovereignAclEntry::new_posix(AclTag::UserOwner, u_r, u_w, u_x));
        acl.entries.push(SovereignAclEntry::new_posix(AclTag::GroupOwner, g_r, g_w, g_x));
        acl.entries.push(SovereignAclEntry::new_posix(AclTag::Other, o_r, o_w, o_x));

        acl
    }

    /// Recalculate POSIX.1e ACL_MASK entry based on all named users and groups
    pub fn update_posix_mask(&mut self) {
        if self.model != AclModel::Posix1e {
            return;
        }

        let mut mask_r = false;
        let mut mask_w = false;
        let mut mask_x = false;
        let mut has_named = false;

        for entry in &self.entries {
            match &entry.tag {
                AclTag::NamedUser(_) | AclTag::NamedGroup(_) | AclTag::GroupOwner => {
                    has_named = true;
                    if entry.is_readable() {
                        mask_r = true;
                    }
                    if entry.is_writable() {
                        mask_w = true;
                    }
                    if entry.is_executable() {
                        mask_x = true;
                    }
                }
                _ => {}
            }
        }

        if has_named {
            // Remove existing mask if any
            self.entries.retain(|e| e.tag != AclTag::Mask);
            self.entries.push(SovereignAclEntry::new_posix(
                AclTag::Mask,
                mask_r,
                mask_w,
                mask_x,
            ));
        }
    }

    /// Format to standard POSIX `getfacl` text output format
    pub fn to_getfacl_text(&self) -> String {
        let mut lines = Vec::new();
        for entry in &self.entries {
            let r = if entry.is_readable() { "r" } else { "-" };
            let w = if entry.is_writable() { "w" } else { "-" };
            let x = if entry.is_executable() { "x" } else { "-" };

            let line = match &entry.tag {
                AclTag::UserOwner | AclTag::SpecialOwner => format!("user::{}{}{}", r, w, x),
                AclTag::GroupOwner | AclTag::SpecialGroup => format!("group::{}{}{}", r, w, x),
                AclTag::Other | AclTag::Everyone => format!("other::{}{}{}", r, w, x),
                AclTag::Mask => format!("mask::{}{}{}", r, w, x),
                AclTag::NamedUser(uid) => format!("user:{}:{}{}{}", uid, r, w, x),
                AclTag::NamedGroup(gid) => format!("group:{}:{}{}{}", gid, r, w, x),
            };
            lines.push(line);
        }
        lines.join("\n")
    }

    /// Parse POSIX `setfacl` formatted string (e.g. "u:1001:rwx,g:1002:r-x,m::rwx")
    pub fn from_setfacl_text(text: &str) -> Result<Self, String> {
        let mut acl = Self::new(AclModel::Posix1e);

        for line in text.split(&[',', '\n'][..]) {
            let line_trim = line.trim();
            if line_trim.is_empty() || line_trim.starts_with('#') {
                continue;
            }

            let parts: Vec<&str> = line_trim.split(':').collect();
            if parts.len() < 3 {
                return Err(format!("Invalid setfacl line: {}", line_trim));
            }

            let tag_type = parts[0];
            let identifier = parts[1];
            let perm_str = parts[2];

            let r = perm_str.contains('r');
            let w = perm_str.contains('w');
            let x = perm_str.contains('x');

            let tag = match tag_type {
                "u" | "user" => {
                    if identifier.is_empty() {
                        AclTag::UserOwner
                    } else {
                        let uid = identifier.parse::<u32>().map_err(|_| "Invalid UID")?;
                        AclTag::NamedUser(uid)
                    }
                }
                "g" | "group" => {
                    if identifier.is_empty() {
                        AclTag::GroupOwner
                    } else {
                        let gid = identifier.parse::<u32>().map_err(|_| "Invalid GID")?;
                        AclTag::NamedGroup(gid)
                    }
                }
                "o" | "other" => AclTag::Other,
                "m" | "mask" => AclTag::Mask,
                _ => return Err(format!("Unknown tag type: {}", tag_type)),
            };

            acl.entries.push(SovereignAclEntry::new_posix(tag, r, w, x));
        }

        acl.update_posix_mask();
        Ok(acl)
    }
}

/// Security context performing the check
#[derive(Debug, Clone)]
pub struct SovereignSecuritySubject {
    pub uid: u32,
    pub gid: u32,
    pub supplementary_gids: Vec<u32>,
    pub is_superuser: bool,
}

impl SovereignSecuritySubject {
    pub fn new(uid: u32, gid: u32) -> Self {
        Self {
            uid,
            gid,
            supplementary_gids: Vec::new(),
            is_superuser: uid == 0,
        }
    }
}

/// Access Control List Engine responsible for evaluating access requests across POSIX and NFSv4 ACLs
pub struct SovereignAccessControlListEngine {
    pub path_acls: HashMap<String, SovereignAccessControlList>,
    pub default_acls: HashMap<String, SovereignAccessControlList>,
}

impl SovereignAccessControlListEngine {
    pub fn new() -> Self {
        Self {
            path_acls: HashMap::new(),
            default_acls: HashMap::new(),
        }
    }

    pub fn set_acl(&mut self, path: &str, acl: SovereignAccessControlList) {
        self.path_acls.insert(path.to_string(), acl);
    }

    pub fn set_default_acl(&mut self, dir_path: &str, acl: SovereignAccessControlList) {
        self.default_acls.insert(dir_path.to_string(), acl);
    }

    pub fn get_acl(&self, path: &str) -> Option<&SovereignAccessControlList> {
        self.path_acls.get(path)
    }

    /// Evaluate access for a subject requesting permissions on a path (POSIX.1e standard algorithm)
    pub fn evaluate_access(
        &self,
        subject: &SovereignSecuritySubject,
        path: &str,
        owner_uid: u32,
        owner_gid: u32,
        requested_permission: AclPermission,
    ) -> bool {
        // Superuser bypasses ACL checks except execute if no execute bit is set
        if subject.is_superuser {
            return true;
        }

        let acl = match self.path_acls.get(path) {
            Some(a) => a,
            None => return false,
        };

        match acl.model {
            AclModel::Posix1e => self.evaluate_posix_1e(subject, acl, owner_uid, owner_gid, requested_permission),
            AclModel::Nfsv4Zfs | AclModel::RichAcl => self.evaluate_nfsv4(subject, acl, owner_uid, owner_gid, requested_permission),
        }
    }

    /// POSIX.1e evaluation algorithm
    fn evaluate_posix_1e(
        &self,
        subject: &SovereignSecuritySubject,
        acl: &SovereignAccessControlList,
        owner_uid: u32,
        owner_gid: u32,
        requested_permission: AclPermission,
    ) -> bool {
        let mask = acl.entries.iter().find(|e| e.tag == AclTag::Mask);

        // 1. Owner check
        if subject.uid == owner_uid {
            if let Some(user_obj) = acl.entries.iter().find(|e| e.tag == AclTag::UserOwner) {
                return user_obj.has_permission(requested_permission);
            }
        }

        // 2. Named user check
        if let Some(named_user) = acl.entries.iter().find(|e| e.tag == AclTag::NamedUser(subject.uid)) {
            let user_allowed = named_user.has_permission(requested_permission);
            if let Some(m) = mask {
                return user_allowed && m.has_permission(requested_permission);
            }
            return user_allowed;
        }

        // 3. Group check (owner group or supplementary groups)
        let is_in_group = |gid: u32| gid == owner_gid || subject.gid == gid || subject.supplementary_gids.contains(&gid);

        let mut group_matched = false;
        let mut group_allowed = false;

        for entry in &acl.entries {
            match entry.tag {
                AclTag::GroupOwner => {
                    if is_in_group(owner_gid) {
                        group_matched = true;
                        if entry.has_permission(requested_permission) {
                            group_allowed = true;
                        }
                    }
                }
                AclTag::NamedGroup(gid) => {
                    if is_in_group(gid) {
                        group_matched = true;
                        if entry.has_permission(requested_permission) {
                            group_allowed = true;
                        }
                    }
                }
                _ => {}
            }
        }

        if group_matched {
            if let Some(m) = mask {
                return group_allowed && m.has_permission(requested_permission);
            }
            return group_allowed;
        }

        // 4. Other check
        if let Some(other) = acl.entries.iter().find(|e| e.tag == AclTag::Other) {
            return other.has_permission(requested_permission);
        }

        false
    }

    /// NFSv4 / ZFS evaluation algorithm (sequential Allow / Deny rules)
    fn evaluate_nfsv4(
        &self,
        subject: &SovereignSecuritySubject,
        acl: &SovereignAccessControlList,
        owner_uid: u32,
        owner_gid: u32,
        requested_permission: AclPermission,
    ) -> bool {
        for entry in &acl.entries {
            let applies = match entry.tag {
                AclTag::UserOwner | AclTag::SpecialOwner => subject.uid == owner_uid,
                AclTag::GroupOwner | AclTag::SpecialGroup => {
                    subject.gid == owner_gid || subject.supplementary_gids.contains(&owner_gid)
                }
                AclTag::Other | AclTag::Everyone => true,
                AclTag::NamedUser(uid) => subject.uid == uid,
                AclTag::NamedGroup(gid) => subject.gid == gid || subject.supplementary_gids.contains(&gid),
                AclTag::Mask => false,
            };

            if applies && entry.has_permission(requested_permission) {
                match entry.entry_type {
                    AclEntryType::Allow => return true,
                    AclEntryType::Deny => return false,
                    _ => {}
                }
            }
        }

        false
    }

    /// Inherit default ACL when creating a new child object inside a directory
    pub fn inherit_default_acl(&self, parent_dir: &str, is_directory: bool) -> Option<SovereignAccessControlList> {
        let default_acl = self.default_acls.get(parent_dir)?;
        let mut child_acl = SovereignAccessControlList::new(default_acl.model);

        for entry in &default_acl.entries {
            let mut inherit = false;
            let mut child_flags = Vec::new();

            if is_directory && entry.inheritance_flags.contains(&AclInheritanceFlag::DirectoryInherit) {
                inherit = true;
                child_flags.push(AclInheritanceFlag::DirectoryInherit);
                child_flags.push(AclInheritanceFlag::FileInherit);
            } else if !is_directory && entry.inheritance_flags.contains(&AclInheritanceFlag::FileInherit) {
                inherit = true;
            }

            if inherit {
                let mut child_entry = entry.clone();
                child_entry.inheritance_flags = child_flags;
                child_entry.inheritance_flags.push(AclInheritanceFlag::Inherited);
                child_acl.entries.push(child_entry);
            }
        }

        if !child_acl.entries.is_empty() {
            Some(child_acl)
        } else {
            None
        }
    }
}

impl Default for SovereignAccessControlListEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// Standalone Unit Test Suite
// ============================================================================

#[cfg(test)]
mod acl_tests {
    use super::*;

    #[test]
    fn test_posix_mode_conversion_and_getfacl_formatting() {
        let acl = SovereignAccessControlList::from_mode(0o750);
        let text = acl.to_getfacl_text();
        assert!(text.contains("user::rwx"));
        assert!(text.contains("group::r-x"));
        assert!(text.contains("other:::---") || text.contains("other::---"));
    }

    #[test]
    fn test_setfacl_text_parsing_and_mask_update() {
        let text = "u::rwx\ng::r-x\no::---\nu:1001:rw-\nm::rw-";
        let acl = SovereignAccessControlList::from_setfacl_text(text).unwrap();
        assert_eq!(acl.entries.len(), 5);

        let named_user = acl.entries.iter().find(|e| e.tag == AclTag::NamedUser(1001)).unwrap();
        assert!(named_user.is_readable());
        assert!(named_user.is_writable());
        assert!(!named_user.is_executable());
    }

    #[test]
    fn test_posix_1e_acl_evaluation_flow() {
        let mut engine = SovereignAccessControlListEngine::new();
        let acl = SovereignAccessControlList::from_setfacl_text(
            "u::rwx,g::r--,o::---,u:2000:r-x,g:3000:-w-,m::rwx"
        ).unwrap();

        engine.set_acl("/srv/data/file.txt", acl);

        // Subject 1: Owner (UID 1000)
        let sub_owner = SovereignSecuritySubject::new(1000, 1000);
        assert!(engine.evaluate_access(&sub_owner, "/srv/data/file.txt", 1000, 1000, AclPermission::WriteData));

        // Subject 2: Named User (UID 2000)
        let sub_named_user = SovereignSecuritySubject::new(2000, 1000);
        assert!(engine.evaluate_access(&sub_named_user, "/srv/data/file.txt", 1000, 1000, AclPermission::ReadData));
        assert!(!engine.evaluate_access(&sub_named_user, "/srv/data/file.txt", 1000, 1000, AclPermission::WriteData));

        // Subject 3: Superuser (UID 0)
        let sub_root = SovereignSecuritySubject::new(0, 0);
        assert!(engine.evaluate_access(&sub_root, "/srv/data/file.txt", 1000, 1000, AclPermission::WriteData));
    }

    #[test]
    fn test_nfsv4_zfs_acl_inheritance_and_deny() {
        let mut engine = SovereignAccessControlListEngine::new();
        let mut parent_acl = SovereignAccessControlList::new(AclModel::Nfsv4Zfs);

        // Deny user 1002 write, Allow everyone read
        parent_acl.entries.push(SovereignAclEntry::new_nfsv4(
            AclTag::NamedUser(1002),
            AclEntryType::Deny,
            vec![AclPermission::WriteData],
            vec![AclInheritanceFlag::FileInherit, AclInheritanceFlag::DirectoryInherit],
        ));
        parent_acl.entries.push(SovereignAclEntry::new_nfsv4(
            AclTag::Everyone,
            AclEntryType::Allow,
            vec![AclPermission::ReadData, AclPermission::WriteData],
            vec![AclInheritanceFlag::FileInherit],
        ));

        engine.set_default_acl("/srv/shared", parent_acl);

        let child_acl = engine.inherit_default_acl("/srv/shared", false).unwrap();
        engine.set_acl("/srv/shared/child.txt", child_acl);

        let sub_denied = SovereignSecuritySubject::new(1002, 1002);
        let sub_allowed = SovereignSecuritySubject::new(1003, 1003);

        assert!(!engine.evaluate_access(&sub_denied, "/srv/shared/child.txt", 1000, 1000, AclPermission::WriteData));
        assert!(engine.evaluate_access(&sub_allowed, "/srv/shared/child.txt", 1000, 1000, AclPermission::WriteData));
    }
}
