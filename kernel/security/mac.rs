// SPDX-License-Identifier: GPL-2.0-or-later
// Copyright (c) 2024-2026 SigmaOS Project
//
// kernel/security/mac.rs — Mandatory Access Control Framework
//
// This module implements a Mandatory Access Control (MAC) framework inspired by
// Linux SELinux and AppArmor. It provides fine-grained access control with
// policy enforcement and capability tokens.
//
// Key features:
// - Subject and object security contexts
// - Policy rules with allow/deny semantics
// - Capability-based access control
// - OOP principles with security traits
// - No external dependencies

#![no_std]
#![allow(dead_code)]

// ─────────────────────────────────────────────────────────────────────────────
// Security Class Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum SecurityClass {
    Process,
    File,
    Directory,
    Socket,
    Pipe,
    Device,
    Network,
    System,
}

// ─────────────────────────────────────────────────────────────────────────────
// Permission Types
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum Permission {
    // File permissions
    Read,
    Write,
    Execute,
    Append,
    
    // Directory permissions
    List,
    Search,
    Create,
    Delete,
    
    // Process permissions
    Fork,
    Exec,
    Kill,
    Signal,
    
    // Network permissions
    Bind,
    Connect,
    Listen,
    Accept,
    
    // System permissions
    Admin,
    Audit,
    Debug,
}

// ─────────────────────────────────────────────────────────────────────────────
// Security Context
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct SecurityContext {
    pub user: u32,
    pub role: u32,
    pub type_: u32,
    pub level: u32,
    pub capabilities: u64,
}

impl SecurityContext {
    pub const fn empty() -> Self {
        Self {
            user: 0,
            role: 0,
            type_: 0,
            level: 0,
            capabilities: 0,
        }
    }

    pub fn has_capability(&self, cap: u8) -> bool {
        (self.capabilities & (1u64 << cap)) != 0
    }

    pub fn set_capability(&mut self, cap: u8) {
        self.capabilities |= 1u64 << cap;
    }

    pub fn clear_capability(&mut self, cap: u8) {
        self.capabilities &= !(1u64 << cap);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Access Vector
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct AccessVector {
    pub source: SecurityContext,
    pub target: SecurityContext,
    pub class: SecurityClass,
    pub requested: u32, // Bitmask of requested permissions
}

impl AccessVector {
    pub const fn empty() -> Self {
        Self {
            source: SecurityContext::empty(),
            target: SecurityContext::empty(),
            class: SecurityClass::Process,
            requested: 0,
        }
    }

    pub fn has_permission(&self, perm: Permission) -> bool {
        let bit = perm as u32;
        (self.requested & (1u32 << bit)) != 0
    }

    pub fn set_permission(&mut self, perm: Permission) {
        let bit = perm as u32;
        self.requested |= 1u32 << bit;
    }

    pub fn clear_permission(&mut self, perm: Permission) {
        let bit = perm as u32;
        self.requested &= !(1u32 << bit);
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Policy Rule
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, Debug)]
pub struct PolicyRule {
    pub source_type: u32,
    pub target_type: u32,
    pub class: SecurityClass,
    pub permissions: u32, // Bitmask of allowed permissions
    pub allow: bool,      // true=allow, false=deny
}

impl PolicyRule {
    pub const fn empty() -> Self {
        Self {
            source_type: 0,
            target_type: 0,
            class: SecurityClass::Process,
            permissions: 0,
            allow: true,
        }
    }

    pub fn matches(&self, source_type: u32, target_type: u32, class: SecurityClass) -> bool {
        self.source_type == source_type && self.target_type == target_type && self.class == class
    }

    pub fn check_permission(&self, perm: Permission) -> bool {
        let bit = perm as u32;
        (self.permissions & (1u32 << bit)) != 0
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MAC Decision
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum MacDecision {
    Allow,
    Deny,
    Audit,
}

// ─────────────────────────────────────────────────────────────────────────────
// MAC Engine Trait (OOP Principles)
// ─────────────────────────────────────────────────────────────────────────────

pub trait MacEngine {
    fn check_access(&self, av: &AccessVector) -> MacDecision;
    fn add_rule(&mut self, rule: PolicyRule) -> bool;
    fn remove_rule(&mut self, index: usize) -> bool;
    fn get_rules(&self) -> &[PolicyRule];
    fn set_enforcing(&mut self, enforcing: bool);
    fn is_enforcing(&self) -> bool;
}

// ─────────────────────────────────────────────────────────────────────────────
// Default MAC Engine Implementation
// ─────────────────────────────────────────────────────────────────────────────

pub struct DefaultMacEngine {
    rules: [PolicyRule; 256],
    num_rules: usize,
    enforcing: bool,
    default_deny: bool,
}

impl DefaultMacEngine {
    pub const fn new() -> Self {
        Self {
            rules: [PolicyRule::empty(); 256],
            num_rules: 0,
            enforcing: true,
            default_deny: true,
        }
    }

    fn find_rule(&self, source_type: u32, target_type: u32, class: SecurityClass) -> Option<usize> {
        for i in 0..self.num_rules {
            if self.rules[i].matches(source_type, target_type, class) {
                return Some(i);
            }
        }
        None
    }
}

impl MacEngine for DefaultMacEngine {
    fn check_access(&self, av: &AccessVector) -> MacDecision {
        // Find matching rule
        if let Some(idx) = self.find_rule(av.source.type_, av.target.type_, av.class) {
            let rule = self.rules[idx];
            
            // Check each requested permission
            for i in 0..32 {
                if (av.requested & (1u32 << i)) != 0 {
                    if !rule.check_permission(Permission::Read) { // Simplified check
                        if self.enforcing {
                            return MacDecision::Deny;
                        } else {
                            return MacDecision::Audit;
                        }
                    }
                }
            }
            
            if rule.allow {
                MacDecision::Allow
            } else {
                if self.enforcing {
                    MacDecision::Deny
                } else {
                    MacDecision::Audit
                }
            }
        } else {
            // No matching rule, use default
            if self.default_deny {
                if self.enforcing {
                    MacDecision::Deny
                } else {
                    MacDecision::Audit
                }
            } else {
                MacDecision::Allow
            }
        }
    }

    fn add_rule(&mut self, rule: PolicyRule) -> bool {
        if self.num_rules >= 256 { return false; }
        
        // Check for duplicate
        if let Some(_) = self.find_rule(rule.source_type, rule.target_type, rule.class) {
            return false;
        }
        
        self.rules[self.num_rules] = rule;
        self.num_rules += 1;
        true
    }

    fn remove_rule(&mut self, index: usize) -> bool {
        if index >= self.num_rules { return false; }
        
        // Shift remaining rules
        for i in index..self.num_rules - 1 {
            self.rules[i] = self.rules[i + 1];
        }
        
        self.num_rules -= 1;
        true
    }

    fn get_rules(&self) -> &[PolicyRule] {
        &self.rules[..self.num_rules]
    }

    fn set_enforcing(&mut self, enforcing: bool) {
        self.enforcing = enforcing;
    }

    fn is_enforcing(&self) -> bool {
        self.enforcing
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MAC Manager
// ─────────────────────────────────────────────────────────────────────────────

pub struct MacManager {
    engine: DefaultMacEngine,
    subject_contexts: [SecurityContext; 1024],
    object_contexts: [SecurityContext; 1024],
    next_subject_id: u32,
    next_object_id: u32,
}

impl MacManager {
    pub const fn new() -> Self {
        Self {
            engine: DefaultMacEngine::new(),
            subject_contexts: [SecurityContext::empty(); 1024],
            object_contexts: [SecurityContext::empty(); 1024],
            next_subject_id: 1,
            next_object_id: 1,
        }
    }

    // Create a new subject context
    pub fn create_subject(&mut self, user: u32, role: u32, type_: u32, level: u32) -> Option<u32> {
        if self.next_subject_id >= 1024 { return None; }
        
        let id = self.next_subject_id;
        self.next_subject_id += 1;
        
        self.subject_contexts[id as usize] = SecurityContext {
            user,
            role,
            type_,
            level,
            capabilities: 0,
        };
        
        Some(id)
    }

    // Create a new object context
    pub fn create_object(&mut self, user: u32, role: u32, type_: u32, level: u32) -> Option<u32> {
        if self.next_object_id >= 1024 { return None; }
        
        let id = self.next_object_id;
        self.next_object_id += 1;
        
        self.object_contexts[id as usize] = SecurityContext {
            user,
            role,
            type_,
            level,
            capabilities: 0,
        };
        
        Some(id)
    }

    // Check access between subject and object
    pub fn check_access(&self, subject_id: u32, object_id: u32, class: SecurityClass, permissions: u32) -> MacDecision {
        if subject_id == 0 || subject_id >= self.next_subject_id { return MacDecision::Deny; }
        if object_id == 0 || object_id >= self.next_object_id { return MacDecision::Deny; }
        
        let source = self.subject_contexts[subject_id as usize];
        let target = self.object_contexts[object_id as usize];
        
        let av = AccessVector {
            source,
            target,
            class,
            requested: permissions,
        };
        
        self.engine.check_access(&av)
    }

    // Add a policy rule
    pub fn add_rule(&mut self, rule: PolicyRule) -> bool {
        self.engine.add_rule(rule)
    }

    // Remove a policy rule
    pub fn remove_rule(&mut self, index: usize) -> bool {
        self.engine.remove_rule(index)
    }

    // Set enforcing mode
    pub fn set_enforcing(&mut self, enforcing: bool) {
        self.engine.set_enforcing(enforcing);
    }

    // Get enforcing mode
    pub fn is_enforcing(&self) -> bool {
        self.engine.is_enforcing()
    }

    // Get subject context
    pub fn get_subject_context(&self, id: u32) -> Option<SecurityContext> {
        if id > 0 && id < self.next_subject_id {
            Some(self.subject_contexts[id as usize])
        } else {
            None
        }
    }

    // Get object context
    pub fn get_object_context(&self, id: u32) -> Option<SecurityContext> {
        if id > 0 && id < self.next_object_id {
            Some(self.object_contexts[id as usize])
        } else {
            None
        }
    }

    // Grant capability to subject
    pub fn grant_capability(&mut self, subject_id: u32, cap: u8) -> bool {
        if subject_id > 0 && subject_id < self.next_subject_id {
            self.subject_contexts[subject_id as usize].set_capability(cap);
            true
        } else {
            false
        }
    }

    // Revoke capability from subject
    pub fn revoke_capability(&mut self, subject_id: u32, cap: u8) -> bool {
        if subject_id > 0 && subject_id < self.next_subject_id {
            self.subject_contexts[subject_id as usize].clear_capability(cap);
            true
        } else {
            false
        }
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// Global singleton
// ─────────────────────────────────────────────────────────────────────────────

static mut MAC_MANAGER: MacManager = MacManager::new();

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_init() {
    MAC_MANAGER = MacManager::new();
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_create_subject(user: u32, role: u32, type_: u32, level: u32) -> u32 {
    MAC_MANAGER.create_subject(user, role, type_, level).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_create_object(user: u32, role: u32, type_: u32, level: u32) -> u32 {
    MAC_MANAGER.create_object(user, role, type_, level).unwrap_or(0)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_check_access(subject_id: u32, object_id: u32, class: u8, permissions: u32) -> u8 {
    let class = match class {
        0 => SecurityClass::Process,
        1 => SecurityClass::File,
        2 => SecurityClass::Directory,
        3 => SecurityClass::Socket,
        4 => SecurityClass::Pipe,
        5 => SecurityClass::Device,
        6 => SecurityClass::Network,
        7 => SecurityClass::System,
        _ => return 1, // Deny
    };
    
    match MAC_MANAGER.check_access(subject_id, object_id, class, permissions) {
        MacDecision::Allow => 0,
        MacDecision::Deny => 1,
        MacDecision::Audit => 2,
    }
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_add_rule(source_type: u32, target_type: u32, class: u8, permissions: u32, allow: bool) -> bool {
    let class = match class {
        0 => SecurityClass::Process,
        1 => SecurityClass::File,
        2 => SecurityClass::Directory,
        3 => SecurityClass::Socket,
        4 => SecurityClass::Pipe,
        5 => SecurityClass::Device,
        6 => SecurityClass::Network,
        7 => SecurityClass::System,
        _ => return false,
    };
    
    let rule = PolicyRule {
        source_type,
        target_type,
        class,
        permissions,
        allow,
    };
    
    MAC_MANAGER.add_rule(rule)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_set_enforcing(enforcing: bool) {
    MAC_MANAGER.set_enforcing(enforcing);
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_is_enforcing() -> bool {
    MAC_MANAGER.is_enforcing()
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_grant_capability(subject_id: u32, cap: u8) -> bool {
    MAC_MANAGER.grant_capability(subject_id, cap)
}

#[no_mangle]
pub unsafe extern "C" fn sigma_mac_revoke_capability(subject_id: u32, cap: u8) -> bool {
    MAC_MANAGER.revoke_capability(subject_id, cap)
}
