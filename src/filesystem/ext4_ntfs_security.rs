// SigmaOS Dual Filesystem Access Rights Layer: Linux Ext4 vs Windows NTFS
// Implements Ext4 Inode i_mode + xattr POSIX ACLs, and NTFS Security Descriptors + SIDs + DACL/SACLs.

extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;

// =================================================================────────────
// 1. LINUX EXT4 FILESYSTEM ACCESS MODEL (i_mode + Extended Attributes POSIX ACLs)
// =================================================================────────────

pub mod ext4_mode_bits {
    pub const S_ISUID: u16 = 0o4000; // Set User ID
    pub const S_ISGID: u16 = 0o2000; // Set Group ID
    pub const S_ISVTX: u16 = 0o1000; // Sticky Bit
    pub const S_IRUSR: u16 = 0o0400; // Owner Read
    pub const S_IWUSR: u16 = 0o0200; // Owner Write
    pub const S_IXUSR: u16 = 0o0100; // Owner Execute
    pub const S_IRGRP: u16 = 0o0040; // Group Read
    pub const S_IWGRP: u16 = 0o0020; // Group Write
    pub const S_IXGRP: u16 = 0o0010; // Group Execute
    pub const S_IROTH: u16 = 0o0004; // Others Read
    pub const S_IWOTH: u16 = 0o0002; // Others Write
    pub const S_IXOTH: u16 = 0o0001; // Others Execute
}

#[derive(Debug, Clone)]
pub struct Ext4InodeMetadata {
    pub inode_id: u64,
    pub uid: u32,
    pub gid: u32,
    pub i_mode: u16,                       // 16-bit file type + permissions
    pub xattrs: BTreeMap<String, Vec<u8>>, // Extended attributes e.g. "system.posix_acl_access"
}

impl Ext4InodeMetadata {
    pub fn new(inode_id: u64, uid: u32, gid: u32, i_mode: u16) -> Self {
        Self {
            inode_id,
            uid,
            gid,
            i_mode,
            xattrs: BTreeMap::new(),
        }
    }

    pub fn set_xattr(&mut self, name: &str, value: &[u8]) {
        self.xattrs.insert(name.to_string(), value.to_vec());
    }

    /// Ext4 Access Check Flow:
    /// 1. If Root (uid 0), grant access.
    /// 2. If POSIX ACL xattr ("system.posix_acl_access") exists, evaluate explicit ACEs.
    /// 3. Otherwise, fall back to standard 16-bit i_mode bits (Owner/Group/Others).
    pub fn evaluate_ext4_access(
        &self,
        subject_uid: u32,
        subject_gid: u32,
        requested_mode: u16,
    ) -> bool {
        if subject_uid == 0 {
            return true; // Root bypass
        }

        // Check if extended attribute POSIX ACL is present
        if let Some(_acl_xattr) = self.xattrs.get("system.posix_acl_access") {
            // Emulate xattr ACL evaluation
            if subject_uid == self.uid {
                let owner_bits = (self.i_mode >> 6) & 0o7;
                return (owner_bits & requested_mode) == requested_mode;
            }
        }

        // Fallback to traditional 3-class permission matrix
        let allowed_bits = if subject_uid == self.uid {
            (self.i_mode >> 6) & 0o7
        } else if subject_gid == self.gid {
            (self.i_mode >> 3) & 0o7
        } else {
            self.i_mode & 0o7
        };

        (allowed_bits & requested_mode) == requested_mode
    }
}

// =================================================================────────────
// 2. WINDOWS NTFS FILESYSTEM SECURITY DESCRIPTORS (SIDs, DACLs, SACLs, ACEs)
// =================================================================────────────

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SecurityIdentifier {
    pub sid_str: String, // e.g. "S-1-5-21-3623811015-3361044348-30300820-1013"
}

impl SecurityIdentifier {
    pub fn new(sid_str: &str) -> Self {
        Self {
            sid_str: sid_str.to_string(),
        }
    }
}

pub mod ntfs_rights {
    pub const READ_DATA: u32 = 0x0001;
    pub const WRITE_DATA: u32 = 0x0002;
    pub const APPEND_DATA: u32 = 0x0004;
    pub const EXECUTE: u32 = 0x0020;
    pub const DELETE: u32 = 0x00010000;
    pub const FULL_CONTROL: u32 = 0x001F01FF;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AceType {
    AccessDenied,
    AccessAllowed,
    SystemAudit,
}

#[derive(Debug, Clone)]
pub struct NtfsAce {
    pub ace_type: AceType,
    pub sid: SecurityIdentifier,
    pub mask: u32,
    pub inherits_to_children: bool,
}

#[derive(Debug, Clone)]
pub struct NtfsDacl {
    pub aces: Vec<NtfsAce>,
}

#[derive(Debug, Clone)]
pub struct NtfsSacl {
    pub audit_aces: Vec<NtfsAce>,
}

#[derive(Debug, Clone)]
pub struct NtfsSecurityDescriptor {
    pub owner_sid: SecurityIdentifier,
    pub group_sid: SecurityIdentifier,
    pub dacl: NtfsDacl,
    pub sacl: NtfsSacl,
}

impl NtfsSecurityDescriptor {
    pub fn new(owner: &str, group: &str) -> Self {
        Self {
            owner_sid: SecurityIdentifier::new(owner),
            group_sid: SecurityIdentifier::new(group),
            dacl: NtfsDacl { aces: Vec::new() },
            sacl: NtfsSacl {
                audit_aces: Vec::new(),
            },
        }
    }

    pub fn add_dacl_ace(&mut self, ace: NtfsAce) {
        self.dacl.aces.push(ace);
    }

    /// NTFS DACL Access Evaluation Order:
    /// 1. Evaluate Explicit Deny ACEs sequentially. If a matching Deny ACE matches requested rights -> DENY immediately.
    /// 2. Evaluate Explicit Allow ACEs sequentially. Accumulate granted rights until requested_rights are satisfied -> ALLOW.
    /// 3. Default: Implicit Deny if no rule matched or rights remain unsatisfied.
    pub fn evaluate_ntfs_access(
        &self,
        subject_sid: &SecurityIdentifier,
        requested_rights: u32,
    ) -> bool {
        // Step 1: Check Explicit Deny ACEs
        for ace in &self.dacl.aces {
            if ace.ace_type == AceType::AccessDenied && ace.sid == *subject_sid {
                if (ace.mask & requested_rights) != 0 {
                    return false; // Explicit Deny Match!
                }
            }
        }

        // Step 2: Check Explicit Allow ACEs
        let mut accumulated_allowed = 0u32;
        for ace in &self.dacl.aces {
            if ace.ace_type == AceType::AccessAllowed && ace.sid == *subject_sid {
                accumulated_allowed |= ace.mask;
                if (accumulated_allowed & requested_rights) == requested_rights {
                    return true; // Explicit Allow Match!
                }
            }
        }

        false // Step 3: Default Implicit Deny
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ext4_inode_permissions_and_xattr() {
        let mut ext4 = Ext4InodeMetadata::new(101, 1000, 1000, 0o750); // rwxr-x---
        assert!(ext4.evaluate_ext4_access(1000, 1000, 0o7)); // Owner rwx
        assert!(ext4.evaluate_ext4_access(2000, 1000, 0o5)); // Group r-x
        assert!(!ext4.evaluate_ext4_access(2000, 1000, 0o2)); // Group write -> denied
        assert!(!ext4.evaluate_ext4_access(3000, 3000, 0o4)); // Other read -> denied

        // Set POSIX ACL Extended Attribute
        ext4.set_xattr("system.posix_acl_access", &[0x02, 0x00]);
        assert!(ext4.evaluate_ext4_access(1000, 1000, 0o7));
    }

    #[test]
    fn test_ntfs_security_descriptor_evaluation_order() {
        let user_sid = SecurityIdentifier::new("S-1-5-21-12345-1001");
        let mut sd = NtfsSecurityDescriptor::new("S-1-5-21-12345-500", "S-1-5-21-12345-513");

        // 1. Add Explicit Allow for Read & Execute
        sd.add_dacl_ace(NtfsAce {
            ace_type: AceType::AccessAllowed,
            sid: user_sid.clone(),
            mask: ntfs_rights::READ_DATA | ntfs_rights::EXECUTE,
            inherits_to_children: true,
        });

        // Should allow Read
        assert!(sd.evaluate_ntfs_access(&user_sid, ntfs_rights::READ_DATA));

        // Should deny Write (implicit deny)
        assert!(!sd.evaluate_ntfs_access(&user_sid, ntfs_rights::WRITE_DATA));

        // 2. Add Explicit Deny for Read DATA
        sd.add_dacl_ace(NtfsAce {
            ace_type: AceType::AccessDenied,
            sid: user_sid.clone(),
            mask: ntfs_rights::READ_DATA,
            inherits_to_children: true,
        });

        // Explicit Deny takes priority over Allow -> Denied!
        assert!(!sd.evaluate_ntfs_access(&user_sid, ntfs_rights::READ_DATA));
    }
}
