// SPDX-License-Identifier: MIT
// SigmaOS - Firewall Configuration Framework
// Inspired by UFW (https://github.com/ufw-team/ufw), firewalld, iptables
// Provides firewall management and configuration

#![no_std]

extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

/// Rule action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RuleAction {
    Allow,
    Deny,
    Reject,
    Limit,
}

/// Protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    Tcp,
    Udp,
    Icmp,
    Any,
}

/// Direction
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Incoming,
    Outgoing,
    Routed,
}

/// Firewall rule
#[derive(Debug)]
pub struct FirewallRule {
    pub id: u32,
    pub action: RuleAction,
    pub protocol: Protocol,
    pub direction: Direction,
    pub port: Option<u16>,
    pub from_address: Option<String>,
    pub to_address: Option<String>,
    pub enabled: AtomicBool,
}

impl Clone for FirewallRule {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            action: self.action,
            protocol: self.protocol,
            direction: self.direction,
            port: self.port,
            from_address: self.from_address.clone(),
            to_address: self.to_address.clone(),
            enabled: AtomicBool::new(self.enabled.load(Ordering::Relaxed)),
        }
    }
}

/// Firewall profile
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallProfile {
    Home,
    Work,
    Public,
    Trusted,
}

/// Firewall manager
pub struct FirewallManager {
    pub rules: Vec<FirewallRule>,
    pub profile: FirewallProfile,
    pub default_incoming: RuleAction,
    pub default_outgoing: RuleAction,
    pub next_rule_id: AtomicU32,
    pub active: AtomicBool,
}

impl FirewallManager {
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            profile: FirewallProfile::Public,
            default_incoming: RuleAction::Deny,
            default_outgoing: RuleAction::Allow,
            next_rule_id: AtomicU32::new(1),
            active: AtomicBool::new(false),
        }
    }
    
    /// Initialize firewall manager
    pub fn init(&mut self) {
        self.active.store(true, Ordering::Relaxed);
    }
    
    /// Add rule
    pub fn add_rule(&mut self, action: RuleAction, protocol: Protocol, direction: Direction, port: Option<u16>) -> u32 {
        let id = self.next_rule_id.fetch_add(1, Ordering::Relaxed);
        let rule = FirewallRule {
            id,
            action,
            protocol,
            direction,
            port,
            from_address: None,
            to_address: None,
            enabled: AtomicBool::new(true),
        };
        self.rules.push(rule);
        id
    }
    
    /// Remove rule
    pub fn remove_rule(&mut self, rule_id: u32) -> Result<(), &'static str> {
        let pos = self.rules.iter().position(|r| r.id == rule_id).ok_or("Rule not found")?;
        self.rules.remove(pos);
        Ok(())
    }
    
    /// Enable rule
    pub fn enable_rule(&mut self, rule_id: u32) -> Result<(), &'static str> {
        let rule = self.rules.iter_mut().find(|r| r.id == rule_id).ok_or("Rule not found")?;
        rule.enabled.store(true, Ordering::Relaxed);
        Ok(())
    }
    
    /// Disable rule
    pub fn disable_rule(&mut self, rule_id: u32) -> Result<(), &'static str> {
        let rule = self.rules.iter_mut().find(|r| r.id == rule_id).ok_or("Rule not found")?;
        rule.enabled.store(false, Ordering::Relaxed);
        Ok(())
    }
    
    /// Set profile
    pub fn set_profile(&mut self, profile: FirewallProfile) {
        self.profile = profile;
        
        // Adjust defaults based on profile
        match profile {
            FirewallProfile::Home => {
                self.default_incoming = RuleAction::Deny;
                self.default_outgoing = RuleAction::Allow;
            }
            FirewallProfile::Work => {
                self.default_incoming = RuleAction::Deny;
                self.default_outgoing = RuleAction::Allow;
            }
            FirewallProfile::Public => {
                self.default_incoming = RuleAction::Deny;
                self.default_outgoing = RuleAction::Allow;
            }
            FirewallProfile::Trusted => {
                self.default_incoming = RuleAction::Allow;
                self.default_outgoing = RuleAction::Allow;
            }
        }
    }
    
    /// Set default incoming action
    pub fn set_default_incoming(&mut self, action: RuleAction) {
        self.default_incoming = action;
    }
    
    /// Set default outgoing action
    pub fn set_default_outgoing(&mut self, action: RuleAction) {
        self.default_outgoing = action;
    }
    
    /// Add port
    pub fn add_port(&mut self, port: u16, protocol: Protocol) -> u32 {
        self.add_rule(RuleAction::Allow, protocol, Direction::Incoming, Some(port))
    }
    
    /// Remove port
    pub fn remove_port(&mut self, port: u16, protocol: Protocol) -> Result<(), &'static str> {
        let rule_id = self.rules
            .iter()
            .find(|r| r.port == Some(port) && r.protocol == protocol)
            .map(|r| r.id)
            .ok_or("Rule not found")?;
        self.remove_rule(rule_id)
    }
    
    /// Block incoming
    pub fn block_incoming(&mut self) {
        self.default_incoming = RuleAction::Deny;
    }
    
    /// Allow incoming
    pub fn allow_incoming(&mut self) {
        self.default_incoming = RuleAction::Allow;
    }
    
    /// Get rule
    pub fn get_rule(&self, id: u32) -> Option<&FirewallRule> {
        self.rules.iter().find(|r| r.id == id)
    }
    
    /// Get all rules
    pub fn get_all_rules(&self) -> Vec<&FirewallRule> {
        self.rules.iter().collect()
    }
    
    /// Get enabled rules
    pub fn get_enabled_rules(&self) -> Vec<&FirewallRule> {
        self.rules
            .iter()
            .filter(|r| r.enabled.load(Ordering::Relaxed))
            .collect()
    }
    
    /// Is active
    pub fn is_active(&self) -> bool {
        self.active.load(Ordering::Relaxed)
    }
}

impl Default for FirewallManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Global firewall manager
static mut FIREWALL: Option<FirewallManager> = None;

/// Initialize firewall manager
pub fn init_firewall() {
    unsafe {
        FIREWALL = Some(FirewallManager::new());
    }
}

/// Get global firewall manager
pub fn get_firewall() -> Option<&'static mut FirewallManager> {
    unsafe {
        FIREWALL.as_mut()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_firewall_init() {
        let mut manager = FirewallManager::new();
        manager.init();
        assert!(manager.is_active());
    }
    
    #[test]
    fn test_add_rule() {
        let mut manager = FirewallManager::new();
        let id = manager.add_rule(RuleAction::Allow, Protocol::Tcp, Direction::Incoming, Some(80));
        assert!(manager.get_rule(id).is_some());
    }
    
    #[test]
    fn test_add_port() {
        let mut manager = FirewallManager::new();
        let id = manager.add_port(80, Protocol::Tcp);
        assert!(manager.get_rule(id).is_some());
        assert_eq!(manager.get_rule(id).unwrap().port, Some(80));
    }
    
    #[test]
    fn test_remove_port() {
        let mut manager = FirewallManager::new();
        manager.add_port(80, Protocol::Tcp);
        manager.remove_port(80, Protocol::Tcp).unwrap();
        assert_eq!(manager.get_all_rules().len(), 0);
    }
    
    #[test]
    fn test_set_profile() {
        let mut manager = FirewallManager::new();
        manager.set_profile(FirewallProfile::Trusted);
        assert_eq!(manager.default_incoming, RuleAction::Allow);
        assert_eq!(manager.profile, FirewallProfile::Trusted);
    }
    
    #[test]
    fn test_enable_disable_rule() {
        let mut manager = FirewallManager::new();
        let id = manager.add_rule(RuleAction::Allow, Protocol::Tcp, Direction::Incoming, Some(80));
        
        manager.disable_rule(id).unwrap();
        assert!(!manager.get_rule(id).unwrap().enabled.load(Ordering::Relaxed));
        
        manager.enable_rule(id).unwrap();
        assert!(manager.get_rule(id).unwrap().enabled.load(Ordering::Relaxed));
    }
    
    #[test]
    fn test_block_incoming() {
        let mut manager = FirewallManager::new();
        manager.block_incoming();
        assert_eq!(manager.default_incoming, RuleAction::Deny);
    }
}
