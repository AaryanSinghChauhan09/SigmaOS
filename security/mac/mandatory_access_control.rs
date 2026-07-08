// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// security/mac/mandatory_access_control.rs — Mandatory Access Control
//
// Implements SELinux/AppArmor-style mandatory access control for SigmaOS.
// Provides capability-based access control and sandboxing.
//
// Language: Rust (no_std, no alloc)

#![no_std]

type U8 = u8;
type U16 = u16;
type U32 = u32;
type U64 = u64;
type I32 = i32;
type Bool = bool;

// ─── MAC Error Codes ─────────────────────────────────────────────────────

pub const MAC_OK: I32 = 0;
pub const MAC_ERR_NULL_PTR: I32 = -1;
pub const MAC_ERR_PERMISSION_DENIED: I32 = -2;
pub const MAC_ERR_INVALID_CONTEXT: I32 = -3;
pub const MAC_ERR_POLICY_NOT_FOUND: I32 = -4;
pub const MAC_ERR_SUBJECT_NOT_FOUND: I32 = -5;

// ─── MAC Permission Types ───────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MacPermission {
    Read,
    Write,
    Execute,
    Create,
    Delete,
    Link,
    Rename,
    Setattr,
    Getattr,
    Lock,
    Mmap,
    Mprotect,
    UnixLock,
    Ioctl,
    Connect,
    Accept,
    Bind,
    Listen,
    Sendto,
    Recvfrom,
    Getsockopt,
    Setsockopt,
}

// ─── MAC Object Types ─────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MacObjectType {
    File,
    Directory,
    Socket,
    Pipe,
    Device,
    Process,
    Network,
    System,
}

// ─── MAC Security Context ─────────────────────────────────────────────

#[repr(C)]
pub struct MacContext {
    pub user: U32,
    pub role: U32,
    pub type_id: U32,
    pub level: U32,
    pub valid: Bool,
}

impl MacContext {
    pub const fn new() -> Self {
        MacContext {
            user: 0,
            role: 0,
            type_id: 0,
            level: 0,
            valid: false,
        }
    }
}

// ─── MAC Security Class ───────────────────────────────────────────────

#[repr(C)]
pub struct MacClass {
    pub name: [U8; 32],
    pub name_len: U8,
    pub permissions: U64, // Bitmask of permissions
}

impl MacClass {
    pub const fn new() -> Self {
        MacClass {
            name: [0; 32],
            name_len: 0,
            permissions: 0,
        }
    }
}

// ─── MAC Access Rule ─────────────────────────────────────────────────

#[repr(C)]
pub struct MacAccessRule {
    pub subject_context: MacContext,
    pub object_context: MacContext,
    pub object_type: MacObjectType,
    pub permissions: U64,
    pub effect: MacEffect,
    pub enabled: Bool,
}

impl MacAccessRule {
    pub const fn new() -> Self {
        MacAccessRule {
            subject_context: MacContext::new(),
            object_context: MacContext::new(),
            object_type: MacObjectType::File,
            permissions: 0,
            effect: MacEffect::Allow,
            enabled: true,
        }
    }
}

// ─── MAC Effect ─────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MacEffect {
    Allow,
    Deny,
    Audit,
}

// ─── MAC Policy ─────────────────────────────────────────────────────

pub struct MacPolicy {
    pub name: [U8; 32],
    pub name_len: U8,
    pub rules: [MacAccessRule; 256],
    pub rule_count: U32,
    pub default_effect: MacEffect,
    pub enabled: Bool,
}

impl MacPolicy {
    pub const fn new() -> Self {
        MacPolicy {
            name: [0; 32],
            name_len: 0,
            rules: [MacAccessRule::new(); 256],
            rule_count: 0,
            default_effect: MacEffect::Deny,
            enabled: true,
        }
    }
}

// ─── MAC Subject (Process) ───────────────────────────────────────────

#[repr(C)]
pub struct MacSubject {
    pub pid: U32,
    pub context: MacContext,
    pub capabilities: U64,
    pub sandboxed: Bool,
    pub sandbox_profile: [U8; 64],
    pub sandbox_profile_len: U8,
}

impl MacSubject {
    pub const fn new() -> Self {
        MacSubject {
            pid: 0,
            context: MacContext::new(),
            capabilities: 0,
            sandboxed: false,
            sandbox_profile: [0; 64],
            sandbox_profile_len: 0,
        }
    }
}

// ─── MAC Trait ─────────────────────────────────────────────────────

/// Trait for MAC implementations
pub trait MandatoryAccessControl {
    /// Check if subject has permission on object
    fn check_permission(&self, subject: &MacSubject, object: &MacContext, object_type: MacObjectType, permission: MacPermission) -> I32;
    
    /// Add access rule
    fn add_rule(&mut self, policy_name: &[U8], rule: &MacAccessRule) -> I32;
    
    /// Remove access rule
    fn remove_rule(&mut self, policy_name: &[U8], rule_index: U32) -> I32;
    
    /// Create security context
    fn create_context(&mut self, user: U32, role: U32, type_id: U32, level: U32) -> MacContext;
    
    /// Set process context
    fn set_process_context(&mut self, pid: U32, context: &MacContext) -> I32;
    
    /// Get process context
    fn get_process_context(&self, pid: U32) -> Option<MacContext>;
    
    /// Enable/disable policy
    fn set_policy_enabled(&mut self, policy_name: &[U8], enabled: Bool) -> I32;
    
    /// Enable sandbox for process
    fn enable_sandbox(&mut self, pid: U32, profile: &[U8]) -> I32;
    
    /// Disable sandbox for process
    fn disable_sandbox(&mut self, pid: U32) -> I32;
    
    /// Check if process is sandboxed
    fn is_sandboxed(&self, pid: U32) -> Bool;
}

// ─── SigmaOS MAC Implementation ───────────────────────────────────────

pub struct SigmaMAC {
    policies: [MacPolicy; 8],
    policy_count: U32,
    subjects: [MacSubject; 256],
    subject_count: U32,
    enforcing: Bool,
}

impl SigmaMAC {
    pub const fn new() -> Self {
        SigmaMAC {
            policies: [MacPolicy::new(); 8],
            policy_count: 0,
            subjects: [MacSubject::new(); 256],
            subject_count: 0,
            enforcing: true,
        }
    }

    /// Find policy by name
    fn find_policy(&self, name: &[U8]) -> Option<usize> {
        for i in 0..self.policy_count as usize {
            let policy = &self.policies[i];
            if policy.name_len as usize == name.len() {
                let mut matches = true;
                for j in 0..name.len() {
                    if policy.name[j] != name[j] {
                        matches = false;
                        break;
                    }
                }
                if matches {
                    return Some(i);
                }
            }
        }
        None
    }

    /// Find subject by PID
    fn find_subject(&self, pid: U32) -> Option<usize> {
        for i in 0..self.subject_count as usize {
            if self.subjects[i].pid == pid {
                return Some(i);
            }
        }
        None
    }

    /// Convert permission to bitmask
    fn permission_to_mask(permission: MacPermission) -> U64 {
        match permission {
            MacPermission::Read => 1 << 0,
            MacPermission::Write => 1 << 1,
            MacPermission::Execute => 1 << 2,
            MacPermission::Create => 1 << 3,
            MacPermission::Delete => 1 << 4,
            MacPermission::Link => 1 << 5,
            MacPermission::Rename => 1 << 6,
            MacPermission::Setattr => 1 << 7,
            MacPermission::Getattr => 1 << 8,
            MacPermission::Lock => 1 << 9,
            MacPermission::Mmap => 1 << 10,
            MacPermission::Mprotect => 1 << 11,
            MacPermission::UnixLock => 1 << 12,
            MacPermission::Ioctl => 1 << 13,
            MacPermission::Connect => 1 << 14,
            MacPermission::Accept => 1 << 15,
            MacPermission::Bind => 1 << 16,
            MacPermission::Listen => 1 << 17,
            MacPermission::Sendto => 1 << 18,
            MacPermission::Recvfrom => 1 << 19,
            MacPermission::Getsockopt => 1 << 20,
            MacPermission::Setsockopt => 1 << 21,
        }
    }

    /// Initialize default policy
    pub unsafe fn init(&mut self) -> I32 {
        // Create default policy
        let policy_name = b"default";
        self.policies[0].name_len = policy_name.len() as U8;
        for i in 0..policy_name.len() {
            self.policies[0].name[i] = policy_name[i];
        }
        self.policies[0].default_effect = MacEffect::Deny;
        self.policies[0].enabled = true;
        self.policy_count = 1;

        MAC_OK
    }
}

impl MandatoryAccessControl for SigmaMAC {
    fn check_permission(&self, subject: &MacSubject, object: &MacContext, object_type: MacObjectType, permission: MacPermission) -> I32 {
        if !self.enforcing {
            return MAC_OK;
        }

        let perm_mask = Self::permission_to_mask(permission);

        // Check all policies
        for policy_idx in 0..self.policy_count as usize {
            let policy = &self.policies[policy_idx];
            if !policy.enabled {
                continue;
            }

            // Check rules in order
            for rule_idx in 0..policy.rule_count as usize {
                let rule = &policy.rules[rule_idx];
                if !rule.enabled {
                    continue;
                }

                // Check if rule matches subject and object
                if rule.object_type == object_type {
                    // Check context match (simplified)
                    if rule.subject_context.user == subject.context.user ||
                       rule.subject_context.user == 0xFFFFFFFF {
                        // Check permission
                        if rule.permissions & perm_mask != 0 {
                            return match rule.effect {
                                MacEffect::Allow => MAC_OK,
                                MacEffect::Deny => MAC_ERR_PERMISSION_DENIED,
                                MacEffect::Audit => MAC_OK,
                            };
                        }
                    }
                }
            }
        }

        // No matching rule, use default effect
        match self.policies[0].default_effect {
            MacEffect::Allow => MAC_OK,
            MacEffect::Deny => MAC_ERR_PERMISSION_DENIED,
            MacEffect::Audit => MAC_OK,
        }
    }

    fn add_rule(&mut self, policy_name: &[U8], rule: &MacAccessRule) -> I32 {
        let policy_idx = match self.find_policy(policy_name) {
            Some(idx) => idx,
            None => return MAC_ERR_POLICY_NOT_FOUND,
        };

        let policy = &mut self.policies[policy_idx];
        if policy.rule_count >= 256 {
            return MAC_ERR_INVALID_CONTEXT;
        }

        let rule_idx = policy.rule_count as usize;
        policy.rules[rule_idx] = *rule;
        policy.rule_count += 1;

        MAC_OK
    }

    fn remove_rule(&mut self, policy_name: &[U8], rule_index: U32) -> I32 {
        let policy_idx = match self.find_policy(policy_name) {
            Some(idx) => idx,
            None => return MAC_ERR_POLICY_NOT_FOUND,
        };

        let policy = &mut self.policies[policy_idx];
        if rule_index >= policy.rule_count {
            return MAC_ERR_INVALID_CONTEXT;
        }

        // Shift remaining rules
        for i in rule_index as usize..(policy.rule_count as usize - 1) {
            policy.rules[i] = policy.rules[i + 1];
        }
        policy.rule_count -= 1;

        MAC_OK
    }

    fn create_context(&mut self, user: U32, role: U32, type_id: U32, level: U32) -> MacContext {
        MacContext {
            user,
            role,
            type_id,
            level,
            valid: true,
        }
    }

    fn set_process_context(&mut self, pid: U32, context: &MacContext) -> I32 {
        let subject_idx = match self.find_subject(pid) {
            Some(idx) => idx,
            None => {
                if self.subject_count >= 256 {
                    return MAC_ERR_INVALID_CONTEXT;
                }
                let idx = self.subject_count as usize;
                self.subjects[idx].pid = pid;
                self.subject_count += 1;
                idx
            }
        };

        self.subjects[subject_idx].context = *context;
        MAC_OK
    }

    fn get_process_context(&self, pid: U32) -> Option<MacContext> {
        match self.find_subject(pid) {
            Some(idx) => Some(self.subjects[idx].context),
            None => None,
        }
    }

    fn set_policy_enabled(&mut self, policy_name: &[U8], enabled: Bool) -> I32 {
        match self.find_policy(policy_name) {
            Some(idx) => {
                self.policies[idx].enabled = enabled;
                MAC_OK
            }
            None => MAC_ERR_POLICY_NOT_FOUND,
        }
    }

    fn enable_sandbox(&mut self, pid: U32, profile: &[U8]) -> I32 {
        let subject_idx = match self.find_subject(pid) {
            Some(idx) => idx,
            None => {
                if self.subject_count >= 256 {
                    return MAC_ERR_INVALID_CONTEXT;
                }
                let idx = self.subject_count as usize;
                self.subjects[idx].pid = pid;
                self.subject_count += 1;
                idx
            }
        };

        self.subjects[subject_idx].sandboxed = true;
        let profile_len = profile.len().min(63);
        for i in 0..profile_len {
            self.subjects[subject_idx].sandbox_profile[i] = profile[i];
        }
        self.subjects[subject_idx].sandbox_profile_len = profile_len as U8;

        MAC_OK
    }

    fn disable_sandbox(&mut self, pid: U32) -> I32 {
        match self.find_subject(pid) {
            Some(idx) => {
                self.subjects[idx].sandboxed = false;
                self.subjects[idx].sandbox_profile = [0; 64];
                self.subjects[idx].sandbox_profile_len = 0;
                MAC_OK
            }
            None => MAC_ERR_SUBJECT_NOT_FOUND,
        }
    }

    fn is_sandboxed(&self, pid: U32) -> Bool {
        match self.find_subject(pid) {
            Some(idx) => self.subjects[idx].sandboxed,
            None => false,
        }
    }
}

// ─── Global MAC Instance ─────────────────────────────────────────────

static mut GLOBAL_MAC: SigmaMAC = SigmaMAC::new();

// ─── C-ABI Exports ─────────────────────────────────────────────────────

/// Get global MAC
pub unsafe fn get_mac() -> &'static mut SigmaMAC {
    &mut GLOBAL_MAC
}

/// Initialize MAC
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_init() -> I32 {
    GLOBAL_MAC.init()
}

/// Check permission
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_check_permission(
    pid: U32,
    object_user: U32,
    object_role: U32,
    object_type: U32,
    permission: U32,
) -> I32 {
    let subject = match GLOBAL_MAC.find_subject(pid) {
        Some(idx) => &GLOBAL_MAC.subjects[idx],
        None => return MAC_ERR_SUBJECT_NOT_FOUND,
    };

    let object_context = MacContext {
        user: object_user,
        role: object_role,
        type_id: 0,
        level: 0,
        valid: true,
    };

    let mac_object_type = match object_type {
        0 => MacObjectType::File,
        1 => MacObjectType::Directory,
        2 => MacObjectType::Socket,
        3 => MacObjectType::Pipe,
        4 => MacObjectType::Device,
        5 => MacObjectType::Process,
        6 => MacObjectType::Network,
        7 => MacObjectType::System,
        _ => MacObjectType::File,
    };

    let mac_permission = match permission {
        0 => MacPermission::Read,
        1 => MacPermission::Write,
        2 => MacPermission::Execute,
        3 => MacPermission::Create,
        4 => MacPermission::Delete,
        _ => MacPermission::Read,
    };

    GLOBAL_MAC.check_permission(subject, &object_context, mac_object_type, mac_permission)
}

/// Enable sandbox
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_enable_sandbox(pid: U32, profile: *const U8, profile_len: usize) -> I32 {
    if profile.is_null() {
        return MAC_ERR_NULL_PTR;
    }

    let profile_slice = core::slice::from_raw_parts(profile, profile_len);
    GLOBAL_MAC.enable_sandbox(pid, profile_slice)
}

/// Disable sandbox
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_disable_sandbox(pid: U32) -> I32 {
    GLOBAL_MAC.disable_sandbox(pid)
}

/// Check if sandboxed
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_is_sandboxed(pid: U32) -> I32 {
    if GLOBAL_MAC.is_sandboxed(pid) {
        1
    } else {
        0
    }
}

/// Set enforcing mode
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_set_enforcing(enforcing: Bool) {
    GLOBAL_MAC.enforcing = enforcing;
}

/// Get enforcing mode
#[no_mangle]
pub unsafe extern "C" fn sigma_mac_get_enforcing() -> Bool {
    GLOBAL_MAC.enforcing
}
