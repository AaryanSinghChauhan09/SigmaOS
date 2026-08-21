/// Access Control Matrix (ACM), Extended POSIX ACLs, Capability Bounding Sets,
/// Mandatory Access Control (MAC - Bell-LaPadula), and Hardware Network Filters for SigmaOS.

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AclEntry {
    pub acl_type: AclType,
    pub qualifier_id: u32, // UID for NamedUser, GID for NamedGroup
    pub perm_bits: u16,    // 0o7 (rwx)
}

#[derive(Debug, Clone)]
pub struct PosixAcl {
    pub entries: Vec<AclEntry>,
}

impl PosixAcl {
    pub fn new() -> Self {
        Self { entries: Vec::new() }
    }

    pub fn add_entry(&mut self, acl_type: AclType, qualifier_id: u32, perm_bits: u16) {
        self.entries.push(AclEntry {
            acl_type,
            qualifier_id,
            perm_bits: perm_bits & 0o7,
        });
    }

    /// Evaluates Extended POSIX ACL for a user and group
    pub fn evaluate_acl(&self, uid: UserID, gid: GroupID, owner_uid: UserID, group_gid: GroupID, requested_bits: u16) -> bool {
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
            if let Some(group_obj) = self.entries.iter().find(|e| e.acl_type == AclType::GroupObj) {
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
        Self { capability_mask: mask }
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
        Self { level, category_mask }
    }

    pub fn can_read(&self, object_label: &MacSecurityLabel) -> bool {
        self.level >= object_label.level && (self.category_mask & object_label.category_mask) == object_label.category_mask
    }

    pub fn can_write(&self, object_label: &MacSecurityLabel) -> bool {
        self.level <= object_label.level && (object_label.category_mask & self.category_mask) == self.category_mask
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

#[cfg(test)]
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
    fn test_cap_bounding_set() {
        let mut bounds = CapBoundingSet::new(0xFFFF_FFFF);
        assert!(bounds.is_capability_permitted(21)); // CAP_SYS_ADMIN

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
        assert_eq!(gate.evaluate_request(1, 10, acm_rights::READ, 2, &allowed_mac), Ok(()));

        // Unknown MAC -> Blocked
        let blocked_mac = [0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF];
        assert_eq!(gate.evaluate_request(1, 10, acm_rights::READ, 2, &blocked_mac), Err(AccessError::MacAddressBlocked));
    }
}
