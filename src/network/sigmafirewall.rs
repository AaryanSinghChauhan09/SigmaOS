// SPDX-License-Identifier: MIT
// Copyright (c) 2026 SigmaOS Contributors

//! SigmaFirewall - Firewall Daemon
//!
//! SigmaFirewall provides firewall management for SigmaOS, inspired by nftables from Linux.

#![no_std]
extern crate alloc;

use alloc::collections::BTreeMap;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;

/// Firewall action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    /// Accept
    Accept,
    /// Drop
    Drop,
    /// Reject
    Reject,
}

impl fmt::Display for FirewallAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirewallAction::Accept => write!(f, "accept"),
            FirewallAction::Drop => write!(f, "drop"),
            FirewallAction::Reject => write!(f, "reject"),
        }
    }
}

/// Firewall protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallProtocol {
    /// TCP
    Tcp,
    /// UDP
    Udp,
    /// ICMP
    Icmp,
    /// All
    All,
}

impl fmt::Display for FirewallProtocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FirewallProtocol::Tcp => write!(f, "tcp"),
            FirewallProtocol::Udp => write!(f, "udp"),
            FirewallProtocol::Icmp => write!(f, "icmp"),
            FirewallProtocol::All => write!(f, "all"),
        }
    }
}

/// Firewall rule
#[derive(Debug, Clone)]
pub struct FirewallRule {
    /// Rule ID
    pub rule_id: String,
    /// Chain (input, output, forward)
    pub chain: String,
    /// Source address
    pub source: Option<String>,
    /// Destination address
    pub destination: Option<String>,
    /// Source port
    pub source_port: Option<u16>,
    /// Destination port
    pub destination_port: Option<u16>,
    /// Protocol
    pub protocol: FirewallProtocol,
    /// Action
    pub action: FirewallAction,
    /// Enabled
    pub enabled: bool,
}

impl FirewallRule {
    /// Create a new firewall rule
    pub fn new(
        rule_id: String,
        chain: String,
        protocol: FirewallProtocol,
        action: FirewallAction,
    ) -> Self {
        Self {
            rule_id,
            chain,
            source: None,
            destination: None,
            source_port: None,
            destination_port: None,
            protocol,
            action,
            enabled: true,
        }
    }

    /// Set source address
    pub fn with_source(mut self, source: String) -> Self {
        self.source = Some(source);
        self
    }

    /// Set destination address
    pub fn with_destination(mut self, destination: String) -> Self {
        self.destination = Some(destination);
        self
    }

    /// Set source port
    pub fn with_source_port(mut self, port: u16) -> Self {
        self.source_port = Some(port);
        self
    }

    /// Set destination port
    pub fn with_destination_port(mut self, port: u16) -> Self {
        self.destination_port = Some(port);
        self
    }

    /// Set enabled
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// SigmaFirewall - Firewall Daemon
pub struct SigmaFirewall {
    /// Firewall rules (rule_id -> rule)
    rules: BTreeMap<String, FirewallRule>,
    /// Running state
    running: bool,
    /// Rule counter
    rule_counter: u32,
}

impl SigmaFirewall {
    /// Create a new SigmaFirewall instance
    pub fn new() -> Self {
        Self {
            rules: BTreeMap::new(),
            running: false,
            rule_counter: 0,
        }
    }

    /// Start the firewall daemon
    pub fn start(&mut self) {
        self.running = true;
    }

    /// Stop the firewall daemon
    pub fn stop(&mut self) {
        self.running = false;
    }

    /// Check if the daemon is running
    pub fn is_running(&self) -> bool {
        self.running
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: FirewallRule) -> Result<(), String> {
        if self.rules.contains_key(&rule.rule_id) {
            return Err(format!("Rule '{}' already exists", rule.rule_id));
        }

        self.rules.insert(rule.rule_id.clone(), rule);
        self.rule_counter += 1;
        Ok(())
    }

    /// Remove a rule
    pub fn remove_rule(&mut self, rule_id: &str) -> Result<(), String> {
        if self.rules.remove(rule_id).is_none() {
            return Err(format!("Rule '{}' not found", rule_id));
        }
        Ok(())
    }

    /// Get a rule
    pub fn get_rule(&self, rule_id: &str) -> Option<&FirewallRule> {
        self.rules.get(rule_id)
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, rule_id: &str) -> Result<(), String> {
        match self.rules.get_mut(rule_id) {
            Some(rule) => {
                rule.enabled = true;
                Ok(())
            }
            None => Err(format!("Rule '{}' not found", rule_id)),
        }
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, rule_id: &str) -> Result<(), String> {
        match self.rules.get_mut(rule_id) {
            Some(rule) => {
                rule.enabled = false;
                Ok(())
            }
            None => Err(format!("Rule '{}' not found", rule_id)),
        }
    }

    /// List all rules
    pub fn list_rules(&self) -> Vec<&FirewallRule> {
        self.rules.values().collect()
    }

    /// List rules by chain
    pub fn list_by_chain(&self, chain: &str) -> Vec<&FirewallRule> {
        self.rules.values().filter(|r| r.chain == chain).collect()
    }

    /// List enabled rules
    pub fn list_enabled(&self) -> Vec<&FirewallRule> {
        self.rules.values().filter(|r| r.enabled).collect()
    }

    /// List rules by protocol
    pub fn list_by_protocol(&self, protocol: FirewallProtocol) -> Vec<&FirewallRule> {
        self.rules
            .values()
            .filter(|r| r.protocol == protocol)
            .collect()
    }

    /// Get rule count
    pub fn rule_count(&self) -> usize {
        self.rules.len()
    }

    /// Get firewall statistics
    pub fn firewall_stats(&self) -> BTreeMap<String, String> {
        let mut stats = BTreeMap::new();
        stats.insert("total_rules".to_string(), self.rule_count().to_string());
        stats.insert(
            "enabled_rules".to_string(),
            self.list_enabled().len().to_string(),
        );
        stats.insert("running".to_string(), self.running.to_string());
        stats
    }
}

impl Default for SigmaFirewall {
    fn default() -> Self {
        Self::new()
    }
}

/// Global SigmaFirewall instance (placeholder for actual global state)
static mut SIGMAFIREWALL_INSTANCE: Option<SigmaFirewall> = None;

/// Initialize the global SigmaFirewall instance
pub fn init_sigmafirewall() {
    unsafe {
        SIGMAFIREWALL_INSTANCE = Some(SigmaFirewall::new());
    }
}

/// Get the global SigmaFirewall instance
pub fn get_sigmafirewall() -> Option<&'static mut SigmaFirewall> {
    unsafe { SIGMAFIREWALL_INSTANCE.as_mut() }
}

/// Convenience function to add a rule
pub fn add_firewall_rule(rule: FirewallRule) -> Result<(), String> {
    if let Some(firewall) = get_sigmafirewall() {
        firewall.add_rule(rule)
    } else {
        Err("SigmaFirewall not initialized".to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_action_display() {
        assert_eq!(FirewallAction::Accept.to_string(), "accept");
        assert_eq!(FirewallAction::Drop.to_string(), "drop");
    }

    #[test]
    fn test_firewall_protocol_display() {
        assert_eq!(FirewallProtocol::Tcp.to_string(), "tcp");
        assert_eq!(FirewallProtocol::Udp.to_string(), "udp");
    }

    #[test]
    fn test_firewall_rule() {
        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        )
        .with_source("192.168.1.0/24".to_string())
        .with_destination_port(80)
        .with_enabled(true);

        assert_eq!(rule.rule_id, "rule1");
        assert_eq!(rule.source, Some("192.168.1.0/24".to_string()));
        assert_eq!(rule.destination_port, Some(80));
    }

    #[test]
    fn test_sigmafirewall() {
        let mut firewall = SigmaFirewall::new();
        assert!(!firewall.is_running());
        firewall.start();
        assert!(firewall.is_running());
        firewall.stop();
        assert!(!firewall.is_running());
    }

    #[test]
    fn test_sigmafirewall_add_rule() {
        let mut firewall = SigmaFirewall::new();

        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        );

        assert!(firewall.add_rule(rule).is_ok());
        assert_eq!(firewall.rule_count(), 1);
    }

    #[test]
    fn test_sigmafirewall_enable_rule() {
        let mut firewall = SigmaFirewall::new();

        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        )
        .with_enabled(false);

        firewall.add_rule(rule).unwrap();
        assert!(firewall.enable_rule("rule1").is_ok());

        let rule = firewall.get_rule("rule1").unwrap();
        assert!(rule.enabled);
    }

    #[test]
    fn test_sigmafirewall_disable_rule() {
        let mut firewall = SigmaFirewall::new();

        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        );

        firewall.add_rule(rule).unwrap();
        assert!(firewall.disable_rule("rule1").is_ok());

        let rule = firewall.get_rule("rule1").unwrap();
        assert!(!rule.enabled);
    }

    #[test]
    fn test_sigmafirewall_list_by_chain() {
        let mut firewall = SigmaFirewall::new();

        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        );

        firewall.add_rule(rule).unwrap();

        let rules = firewall.list_by_chain("input");
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_sigmafirewall_list_by_protocol() {
        let mut firewall = SigmaFirewall::new();

        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        );

        firewall.add_rule(rule).unwrap();

        let rules = firewall.list_by_protocol(FirewallProtocol::Tcp);
        assert_eq!(rules.len(), 1);
    }

    #[test]
    fn test_sigmafirewall_stats() {
        let mut firewall = SigmaFirewall::new();

        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        );

        firewall.add_rule(rule).unwrap();

        let stats = firewall.firewall_stats();
        assert_eq!(stats.get("total_rules"), Some(&"1".to_string()));
        assert_eq!(stats.get("enabled_rules"), Some(&"1".to_string()));
    }

    #[test]
    fn test_add_firewall_rule_convenience() {
        init_sigmafirewall();

        let rule = FirewallRule::new(
            "rule1".to_string(),
            "input".to_string(),
            FirewallProtocol::Tcp,
            FirewallAction::Accept,
        );

        let result = add_firewall_rule(rule);
        assert!(result.is_ok());
    }
}
