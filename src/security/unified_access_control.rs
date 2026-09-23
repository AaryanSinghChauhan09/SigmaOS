// SigmaOS Comprehensive Unified Access Control Suite
// Inspired by Linux (SELinux, AppArmor, ebtables, POSIX ACLs) and BSD (FreeBSD MAC, OpenBSD PF, Capsicum)
// Integrates Discretionary Access Control (DAC), Mandatory Access Control (MAC),
// Role-Based Access Control (RBAC), and Hardware MAC Address Network Control.

use std::collections::BTreeMap;
use std::format;
use std::string::{String, ToString};
use std::vec::Vec;

// ============================================================================
// 1. DISCRETIONARY ACCESS CONTROL (DAC) - POSIX Permissions & ACLs
// ============================================================================

pub const DAC_READ: u32 = 0x4;
pub const DAC_WRITE: u32 = 0x2;
pub const DAC_EXECUTE: u32 = 0x1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PosixAclTag {
    UserObj,   // Owner permission
    User,      // Named user
    GroupObj,  // Primary group
    Group,     // Named group
    Mask,      // ACL mask limit
    Other,     // World permission
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PosixAclEntry {
    pub tag: PosixAclTag,
    pub qualifier_id: u32, // User ID or Group ID
    pub permissions: u32,  // Bitmask of DAC_READ, DAC_WRITE, DAC_EXECUTE
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DacFileObject {
    pub path: String,
    pub owner_uid: u32,
    pub group_gid: u32,
    pub mode_bits: u32, // e.g. 0o755 or 0o640
    pub acl_entries: Vec<PosixAclEntry>,
}

impl DacFileObject {
    pub fn new(path: &str, owner_uid: u32, group_gid: u32, mode_bits: u32) -> Self {
        Self {
            path: path.to_string(),
            owner_uid,
            group_gid,
            mode_bits,
            acl_entries: Vec::new(),
        }
    }

    pub fn add_acl_entry(&mut self, entry: PosixAclEntry) {
        self.acl_entries.push(entry);
    }

    /// Evaluates Discretionary Access Control (DAC) permission for subject (uid, gids)
    pub fn check_dac_access(&self, uid: u32, gids: &[u32], requested_mask: u32) -> bool {
        // Root override
        if uid == 0 {
            return true;
        }

        // 1. Check named ACL user entry if present
        if let Some(user_acl) = self
            .acl_entries
            .iter()
            .find(|e| e.tag == PosixAclTag::User && e.qualifier_id == uid)
        {
            let mask = self.get_effective_acl_mask();
            return (user_acl.permissions & mask & requested_mask) == requested_mask;
        }

        // 2. Owner check
        if uid == self.owner_uid {
            let owner_perms = (self.mode_bits >> 6) & 0o7;
            return (owner_perms & requested_mask) == requested_mask;
        }

        // 3. Check named ACL group entries
        let mut group_matched = false;
        let mut acl_group_perms = 0u32;
        for gid in gids {
            for entry in &self.acl_entries {
                if entry.tag == PosixAclTag::Group && entry.qualifier_id == *gid {
                    group_matched = true;
                    acl_group_perms |= entry.permissions;
                }
            }
        }
        if group_matched {
            let mask = self.get_effective_acl_mask();
            return (acl_group_perms & mask & requested_mask) == requested_mask;
        }

        // 4. Primary group check
        if gids.contains(&self.group_gid) {
            let group_perms = (self.mode_bits >> 3) & 0o7;
            return (group_perms & requested_mask) == requested_mask;
        }

        // 5. Other / World check
        let other_perms = self.mode_bits & 0o7;
        (other_perms & requested_mask) == requested_mask
    }

    fn get_effective_acl_mask(&self) -> u32 {
        self.acl_entries
            .iter()
            .find(|e| e.tag == PosixAclTag::Mask)
            .map(|e| e.permissions)
            .unwrap_or(0o7)
    }
}

// ============================================================================
// 2. MANDATORY ACCESS CONTROL (MAC) - Security Labels, MLS, Biba & AVC Cache
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SecuritySensitivity {
    Unclassified = 0,
    Confidential = 1,
    Secret = 2,
    TopSecret = 3,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacSecurityLabel {
    pub user: String,
    pub role: String,
    pub domain_type: String,
    pub sensitivity: SecuritySensitivity,
    pub integrity_level: u32, // Biba integrity level (higher = more trusted)
}

impl MacSecurityLabel {
    pub fn new(
        user: &str,
        role: &str,
        domain_type: &str,
        sensitivity: SecuritySensitivity,
        integrity_level: u32,
    ) -> Self {
        Self {
            user: user.to_string(),
            role: role.to_string(),
            domain_type: domain_type.to_string(),
            sensitivity,
            integrity_level,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacAccessOp {
    Read,
    Write,
    Execute,
    Transition,
}

pub struct AccessVectorCache {
    cache: BTreeMap<String, bool>,
}

impl AccessVectorCache {
    pub fn new() -> Self {
        Self {
            cache: BTreeMap::new(),
        }
    }

    pub fn make_key(
        subject: &MacSecurityLabel,
        object: &MacSecurityLabel,
        op: MacAccessOp,
    ) -> String {
        format!(
            "{}:{}:{:?}->{}:{}:{:?}",
            subject.domain_type, subject.sensitivity as u32, op,
            object.domain_type, object.sensitivity as u32
        )
    }

    pub fn get(
        &self,
        subject: &MacSecurityLabel,
        object: &MacSecurityLabel,
        op: MacAccessOp,
    ) -> Option<bool> {
        let key = Self::make_key(subject, object, op);
        self.cache.get(&key).copied()
    }

    pub fn insert(
        &mut self,
        subject: &MacSecurityLabel,
        object: &MacSecurityLabel,
        op: MacAccessOp,
        result: bool,
    ) {
        let key = Self::make_key(subject, object, op);
        self.cache.insert(key, result);
    }

    pub fn clear(&mut self) {
        self.cache.clear();
    }
}

pub struct MandatoryAccessControlEngine {
    pub avc: AccessVectorCache,
    pub enforcing: bool,
    pub allowed_domain_transitions: Vec<(String, String)>,
}

impl MandatoryAccessControlEngine {
    pub fn new(enforcing: bool) -> Self {
        Self {
            avc: AccessVectorCache::new(),
            enforcing,
            allowed_domain_transitions: Vec::new(),
        }
    }

    pub fn allow_transition(&mut self, from_domain: &str, to_domain: &str) {
        self.allowed_domain_transitions
            .push((from_domain.to_string(), to_domain.to_string()));
    }

    /// Evaluates MAC decision incorporating SELinux domain checks, Bell-LaPadula MLS, and Biba Integrity
    pub fn check_mac_access(
        &mut self,
        subject: &MacSecurityLabel,
        object: &MacSecurityLabel,
        op: MacAccessOp,
    ) -> bool {
        if !self.enforcing {
            return true;
        }

        // 1. Check AVC Cache
        if let Some(cached) = self.avc.get(subject, object, op) {
            return cached;
        }

        let result = match op {
            MacAccessOp::Read => {
                // Bell-LaPadula: No read-up (Subject sensitivity >= Object sensitivity)
                let mls_ok = subject.sensitivity >= object.sensitivity;
                // Biba: No read-down (Subject integrity <= Object integrity)
                let biba_ok = subject.integrity_level <= object.integrity_level;
                mls_ok && biba_ok
            }
            MacAccessOp::Write => {
                // Bell-LaPadula: No write-down (Subject sensitivity <= Object sensitivity)
                let mls_ok = subject.sensitivity <= object.sensitivity;
                // Biba: No write-up (Subject integrity >= Object integrity)
                let biba_ok = subject.integrity_level >= object.integrity_level;
                mls_ok && biba_ok
            }
            MacAccessOp::Execute => {
                // Execute requires matching domain or explicit transition rule
                subject.domain_type == object.domain_type
                    || self
                        .allowed_domain_transitions
                        .contains(&(subject.domain_type.clone(), object.domain_type.clone()))
            }
            MacAccessOp::Transition => self
                .allowed_domain_transitions
                .contains(&(subject.domain_type.clone(), object.domain_type.clone())),
        };

        // Cache decision
        self.avc.insert(subject, object, op, result);
        result
    }
}

// ============================================================================
// 3. ROLE-BASED ACCESS CONTROL (RBAC) - Roles, Permissions & Dynamic Sessions
// ============================================================================

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RbacRole {
    pub role_name: String,
    pub parent_roles: Vec<String>,
    pub permissions: Vec<String>, // e.g. "system:reboot", "network:configure"
}

impl RbacRole {
    pub fn new(role_name: &str) -> Self {
        Self {
            role_name: role_name.to_string(),
            parent_roles: Vec::new(),
            permissions: Vec::new(),
        }
    }

    pub fn add_permission(&mut self, perm: &str) {
        if !self.permissions.contains(&perm.to_string()) {
            self.permissions.push(perm.to_string());
        }
    }

    pub fn inherit_from(&mut self, parent_role: &str) {
        if !self.parent_roles.contains(&parent_role.to_string()) {
            self.parent_roles.push(parent_role.to_string());
        }
    }
}

pub struct RbacSession {
    pub username: String,
    pub assigned_roles: Vec<String>,
    pub active_roles: Vec<String>,
}

impl RbacSession {
    pub fn new(username: &str, assigned_roles: Vec<String>) -> Self {
        Self {
            username: username.to_string(),
            active_roles: assigned_roles.clone(),
            assigned_roles,
        }
    }

    pub fn activate_role(&mut self, role_name: &str) -> Result<(), &'static str> {
        if !self.assigned_roles.contains(&role_name.to_string()) {
            return Err("Role not assigned to user");
        }
        if !self.active_roles.contains(&role_name.to_string()) {
            self.active_roles.push(role_name.to_string());
        }
        Ok(())
    }

    pub fn deactivate_role(&mut self, role_name: &str) {
        self.active_roles.retain(|r| r != role_name);
    }
}

pub struct RoleBasedAccessControlEngine {
    pub roles: BTreeMap<String, RbacRole>,
    pub user_sessions: BTreeMap<String, RbacSession>,
}

impl RoleBasedAccessControlEngine {
    pub fn new() -> Self {
        Self {
            roles: BTreeMap::new(),
            user_sessions: BTreeMap::new(),
        }
    }

    pub fn register_role(&mut self, role: RbacRole) {
        self.roles.insert(role.role_name.clone(), role);
    }

    pub fn create_user_session(&mut self, username: &str, roles: Vec<String>) {
        let session = RbacSession::new(username, roles);
        self.user_sessions.insert(username.to_string(), session);
    }

    /// Evaluates if user session has requested permission (resolving role hierarchies)
    pub fn check_permission(&self, username: &str, required_perm: &str) -> bool {
        let session = match self.user_sessions.get(username) {
            Some(s) => s,
            None => return false,
        };

        for role_name in &session.active_roles {
            if self.role_has_permission(role_name, required_perm) {
                return true;
            }
        }
        false
    }

    fn role_has_permission(&self, role_name: &str, required_perm: &str) -> bool {
        let mut visited = Vec::new();
        let mut stack = vec![role_name.to_string()];

        while let Some(curr) = stack.pop() {
            if visited.contains(&curr) {
                continue;
            }
            visited.push(curr.clone());

            if let Some(role) = self.roles.get(&curr) {
                if role.permissions.contains(&required_perm.to_string()) {
                    return true;
                }
                for parent in &role.parent_roles {
                    if !visited.contains(parent) {
                        stack.push(parent.clone());
                    }
                }
            }
        }
        false
    }
}

// ============================================================================
// 4. HARDWARE MAC ADDRESS NETWORK ACCESS CONTROL ENGINE
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MacAddressFilterPolicy {
    AllowList,
    DenyList,
}

#[derive(Debug, Clone)]
pub struct HardwareMacRule {
    pub mac_address: [u8; 6],
    pub vlan_id: Option<u16>,
    pub allow_arp: bool,
    pub description: String,
}

pub struct HardwareMacNetworkAccessEngine {
    pub default_policy: MacAddressFilterPolicy,
    pub rules: Vec<HardwareMacRule>,
    pub blocked_packets_count: u64,
}

impl HardwareMacNetworkAccessEngine {
    pub fn new(default_policy: MacAddressFilterPolicy) -> Self {
        Self {
            default_policy,
            rules: Vec::new(),
            blocked_packets_count: 0,
        }
    }

    pub fn add_rule(&mut self, rule: HardwareMacRule) {
        self.rules.push(rule);
    }

    /// Filters incoming Ethernet frame based on hardware MAC address, VLAN ID, and ARP inspection
    pub fn filter_frame(
        &mut self,
        src_mac: [u8; 6],
        vlan_id: Option<u16>,
        is_arp_packet: bool,
    ) -> bool {
        let rule_opt = self
            .rules
            .iter()
            .find(|r| r.mac_address == src_mac && r.vlan_id == vlan_id);

        let allowed = match (self.default_policy, rule_opt) {
            (MacAddressFilterPolicy::AllowList, Some(rule)) => {
                if is_arp_packet && !rule.allow_arp {
                    false
                } else {
                    true
                }
            }
            (MacAddressFilterPolicy::AllowList, None) => false,
            (MacAddressFilterPolicy::DenyList, Some(rule)) => {
                if is_arp_packet && !rule.allow_arp {
                    false
                } else {
                    false
                }
            }
            (MacAddressFilterPolicy::DenyList, None) => true,
        };

        if !allowed {
            self.blocked_packets_count += 1;
        }

        allowed
    }
}

// ============================================================================
// 5. UNIFIED SOVEREIGN ACCESS CONTROL SUITE HUB
// ==========================================

pub struct UnifiedSovereignAccessControlSuite {
    pub dac: DacFileObject,
    pub mac: MandatoryAccessControlEngine,
    pub rbac: RoleBasedAccessControlEngine,
    pub net_mac: HardwareMacNetworkAccessEngine,
}

impl UnifiedSovereignAccessControlSuite {
    pub fn new(
        file_path: &str,
        owner_uid: u32,
        group_gid: u32,
        mode: u32,
        mac_enforcing: bool,
    ) -> Self {
        Self {
            dac: DacFileObject::new(file_path, owner_uid, group_gid, mode),
            mac: MandatoryAccessControlEngine::new(mac_enforcing),
            rbac: RoleBasedAccessControlEngine::new(),
            net_mac: HardwareMacNetworkAccessEngine::new(MacAddressFilterPolicy::AllowList),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dac_file_access_and_posix_acls() {
        let mut dac_file = DacFileObject::new("/etc/secret.conf", 1000, 1000, 0o640);

        // Owner read & write (6 = read + write)
        assert!(dac_file.check_dac_access(1000, &[1000], DAC_READ | DAC_WRITE));
        // Owner execute should fail
        assert!(!dac_file.check_dac_access(1000, &[1000], DAC_EXECUTE));

        // Group read only (4 = read)
        assert!(dac_file.check_dac_access(2000, &[1000], DAC_READ));
        assert!(!dac_file.check_dac_access(2000, &[1000], DAC_WRITE));

        // Add named user ACL for user 1005 with full permissions
        dac_file.add_acl_entry(PosixAclEntry {
            tag: PosixAclTag::User,
            qualifier_id: 1005,
            permissions: DAC_READ | DAC_WRITE | DAC_EXECUTE,
        });

        assert!(dac_file.check_dac_access(1005, &[2000], DAC_READ | DAC_WRITE | DAC_EXECUTE));
    }

    #[test]
    fn test_mac_selinux_mls_biba_avc() {
        let mut mac_engine = MandatoryAccessControlEngine::new(true);

        let subject_unclass = MacSecurityLabel::new(
            "user_u",
            "user_r",
            "user_t",
            SecuritySensitivity::Unclassified,
            10,
        );
        let object_secret = MacSecurityLabel::new(
            "system_u",
            "object_r",
            "secret_file_t",
            SecuritySensitivity::Secret,
            10,
        );

        // Read up blocked (Unclassified reading Secret)
        assert!(!mac_engine.check_mac_access(&subject_unclass, &object_secret, MacAccessOp::Read));

        // Allow domain transition
        mac_engine.allow_transition("user_t", "secret_file_t");
        let subject_secret = MacSecurityLabel::new(
            "user_u",
            "user_r",
            "secret_file_t",
            SecuritySensitivity::Secret,
            10,
        );

        assert!(mac_engine.check_mac_access(&subject_secret, &object_secret, MacAccessOp::Execute));
    }

    #[test]
    fn test_rbac_roles_inheritance_and_sessions() {
        let mut rbac = RoleBasedAccessControlEngine::new();

        let mut admin_role = RbacRole::new("admin");
        admin_role.add_permission("system:reboot");

        let mut sysadmin_role = RbacRole::new("sysadmin");
        sysadmin_role.inherit_from("admin");
        sysadmin_role.add_permission("network:configure");

        rbac.register_role(admin_role);
        rbac.register_role(sysadmin_role);

        rbac.create_user_session("alice", vec!["sysadmin".to_string()]);

        // Alice inherits "system:reboot" from "admin" parent role
        assert!(rbac.check_permission("alice", "network:configure"));
        assert!(rbac.check_permission("alice", "system:reboot"));
        assert!(!rbac.check_permission("alice", "finance:payroll"));
    }

    #[test]
    fn test_hardware_mac_network_filter() {
        let mut net_mac = HardwareMacNetworkAccessEngine::new(MacAddressFilterPolicy::AllowList);
        let my_mac = [0x00, 0x11, 0x22, 0x33, 0x44, 0x55];

        net_mac.add_rule(HardwareMacRule {
            mac_address: my_mac,
            vlan_id: Some(10),
            allow_arp: true,
            description: "Workstation NIC".to_string(),
        });

        // Frame matching MAC & VLAN 10 passes
        assert!(net_mac.filter_frame(my_mac, Some(10), true));

        // Unknown MAC fails
        let rogue_mac = [0xDE, 0xAD, 0xBE, 0xEF, 0x00, 0x01];
        assert!(!net_mac.filter_frame(rogue_mac, Some(10), true));
        assert_eq!(net_mac.blocked_packets_count, 1);
    }
}
