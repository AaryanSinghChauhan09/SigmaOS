//! Firewall Manager
//!
//! Firewall management inspired by Linux Mint's firewall and Omarchy's
//! firewall utilities, supporting rule management, port forwarding, and zone configuration.

use std::collections::HashMap;

/// Firewall action
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FirewallAction {
    Allow,
    Deny,
    Reject,
    Log,
}

impl FirewallAction {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "allow" => Some(FirewallAction::Allow),
            "deny" => Some(FirewallAction::Deny),
            "reject" => Some(FirewallAction::Reject),
            "log" => Some(FirewallAction::Log),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            FirewallAction::Allow => "Allow",
            FirewallAction::Deny => "Deny",
            FirewallAction::Reject => "Reject",
            FirewallAction::Log => "Log",
        }
    }
}

/// Protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    All,
}

impl Protocol {
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "tcp" => Some(Protocol::TCP),
            "udp" => Some(Protocol::UDP),
            "icmp" => Some(Protocol::ICMP),
            "all" => Some(Protocol::All),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Protocol::TCP => "TCP",
            Protocol::UDP => "UDP",
            Protocol::ICMP => "ICMP",
            Protocol::All => "All",
        }
    }
}

/// Firewall rule
#[derive(Debug, Clone)]
pub struct NetworkFirewallRule {
    pub id: String,
    pub action: FirewallAction,
    pub protocol: Protocol,
    pub source: String,
    pub destination: String,
    pub port: Option<u16>,
    pub is_enabled: bool,
    pub description: String,
}

impl NetworkFirewallRule {
    pub fn new(id: String, action: FirewallAction, protocol: Protocol, description: String) -> Self {
        Self {
            id,
            action,
            protocol,
            source: "0.0.0.0/0".to_string(),
            destination: "0.0.0.0/0".to_string(),
            port: None,
            is_enabled: true,
            description,
        }
    }

    pub fn set_source(&mut self, source: String) {
        self.source = source;
    }

    pub fn set_destination(&mut self, destination: String) {
        self.destination = destination;
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = Some(port);
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.is_enabled = enabled;
    }
}

/// Firewall zone
#[derive(Debug, Clone)]
pub struct NetworkFirewallZone {
    pub id: String,
    pub name: String,
    pub default_action: FirewallAction,
    pub interfaces: Vec<String>,
}

impl NetworkFirewallZone {
    pub fn new(id: String, name: String, default_action: FirewallAction) -> Self {
        Self {
            id,
            name,
            default_action,
            interfaces: Vec::new(),
        }
    }

    pub fn add_interface(&mut self, interface: String) {
        self.interfaces.push(interface);
    }
}

/// Firewall manager
#[derive(Debug)]
pub struct NetworkFirewallManager {
    rules: HashMap<String, NetworkFirewallRule>,
    zones: HashMap<String, NetworkFirewallZone>,
    next_rule_id: u64,
}

impl NetworkFirewallManager {
    pub fn new() -> Self {
        let mut manager = Self {
            rules: HashMap::new(),
            zones: HashMap::new(),
            next_rule_id: 1,
        };

        // Add default zones
        let public_zone = NetworkFirewallZone::new(
            "public".to_string(),
            "Public".to_string(),
            FirewallAction::Deny,
        );
        manager.zones.insert("public".to_string(), public_zone);

        let trusted_zone = NetworkFirewallZone::new(
            "trusted".to_string(),
            "Trusted".to_string(),
            FirewallAction::Allow,
        );
        manager.zones.insert("trusted".to_string(), trusted_zone);

        // Add default rules
        manager.add_default_rules();

        manager
    }

    /// Add default rules
    fn add_default_rules(&mut self) {
        // Allow loopback
        let mut loopback_rule = NetworkFirewallRule::new(
            "loopback".to_string(),
            FirewallAction::Allow,
            Protocol::All,
            "Allow loopback traffic".to_string(),
        );
        loopback_rule.set_source("127.0.0.1/8".to_string());
        loopback_rule.set_destination("127.0.0.1/8".to_string());
        self.rules.insert("loopback".to_string(), loopback_rule);

        // Allow established connections
        let established_rule = NetworkFirewallRule::new(
            "established".to_string(),
            FirewallAction::Allow,
            Protocol::All,
            "Allow established connections".to_string(),
        );
        self.rules.insert("established".to_string(), established_rule);

        // Allow SSH
        let mut ssh_rule = NetworkFirewallRule::new(
            "ssh".to_string(),
            FirewallAction::Allow,
            Protocol::TCP,
            "Allow SSH".to_string(),
        );
        ssh_rule.set_port(22);
        self.rules.insert("ssh".to_string(), ssh_rule);
    }

    /// Add a rule
    pub fn add_rule(&mut self, rule: NetworkFirewallRule) {
        self.rules.insert(rule.id.clone(), rule);
    }

    /// Get a rule
    pub fn get_rule(&self, id: &str) -> Option<&NetworkFirewallRule> {
        self.rules.get(id)
    }

    /// List all rules
    pub fn list_rules(&self) -> Vec<&NetworkFirewallRule> {
        self.rules.values().collect()
    }

    /// List enabled rules
    pub fn list_enabled(&self) -> Vec<&NetworkFirewallRule> {
        self.rules.values()
            .filter(|r| r.is_enabled)
            .collect()
    }

    /// Create a rule
    pub fn create_rule(
        &mut self,
        action: FirewallAction,
        protocol: Protocol,
        port: Option<u16>,
        description: String,
    ) -> String {
        let id = format!("rule-{}", self.next_rule_id);
        self.next_rule_id += 1;

        let mut rule = NetworkFirewallRule::new(id.clone(), action, protocol, description);
        if let Some(p) = port {
            rule.set_port(p);
        }

        self.rules.insert(id.clone(), rule);
        id
    }

    /// Enable a rule
    pub fn enable_rule(&mut self, id: &str) -> Result<(), String> {
        let rule = self.rules.get_mut(id)
            .ok_or_else(|| format!("Rule {} not found", id))?;

        rule.set_enabled(true);
        Ok(())
    }

    /// Disable a rule
    pub fn disable_rule(&mut self, id: &str) -> Result<(), String> {
        let rule = self.rules.get_mut(id)
            .ok_or_else(|| format!("Rule {} not found", id))?;

        rule.set_enabled(false);
        Ok(())
    }

    /// Remove a rule
    pub fn remove_rule(&mut self, id: &str) -> Result<(), String> {
        self.rules.remove(id)
            .ok_or_else(|| format!("Rule {} not found", id))?;
        Ok(())
    }

    /// Add a zone
    pub fn add_zone(&mut self, zone: NetworkFirewallZone) {
        self.zones.insert(zone.id.clone(), zone);
    }

    /// Get a zone
    pub fn get_zone(&self, id: &str) -> Option<&NetworkFirewallZone> {
        self.zones.get(id)
    }

    /// List all zones
    pub fn list_zones(&self) -> Vec<&NetworkFirewallZone> {
        self.zones.values().collect()
    }

    /// Get statistics
    pub fn get_statistics(&self) -> NetworkFirewallStatistics {
        let total_rules = self.rules.len();
        let enabled_rules = self.rules.values()
            .filter(|r| r.is_enabled)
            .count();
        let allow_rules = self.rules.values()
            .filter(|r| r.action == FirewallAction::Allow)
            .count();
        let deny_rules = self.rules.values()
            .filter(|r| r.action == FirewallAction::Deny)
            .count();
        let total_zones = self.zones.len();

        NetworkFirewallStatistics {
            total_rules,
            enabled_rules,
            allow_rules,
            deny_rules,
            total_zones,
        }
    }
}

impl Default for NetworkFirewallManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Firewall statistics
#[derive(Debug, Clone)]
pub struct NetworkFirewallStatistics {
    pub total_rules: usize,
    pub enabled_rules: usize,
    pub allow_rules: usize,
    pub deny_rules: usize,
    pub total_zones: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_firewall_action_from_str() {
        assert_eq!(FirewallAction::from_str("allow"), Some(FirewallAction::Allow));
        assert_eq!(FirewallAction::from_str("deny"), Some(FirewallAction::Deny));
    }

    #[test]
    fn test_protocol_from_str() {
        assert_eq!(Protocol::from_str("tcp"), Some(Protocol::TCP));
        assert_eq!(Protocol::from_str("udp"), Some(Protocol::UDP));
    }

    #[test]
    fn test_firewall_rule_creation() {
        let rule = NetworkFirewallRule::new(
            "test".to_string(),
            FirewallAction::Allow,
            Protocol::TCP,
            "Test rule".to_string(),
        );
        assert_eq!(rule.action, FirewallAction::Allow);
    }

    #[test]
    fn test_firewall_manager_creation() {
        let manager = NetworkFirewallManager::new();
        assert!(manager.get_rule("loopback").is_some());
    }

    #[test]
    fn test_create_rule() {
        let mut manager = NetworkFirewallManager::new();
        let id = manager.create_rule(
            FirewallAction::Allow,
            Protocol::TCP,
            Some(80),
            "HTTP".to_string(),
        );
        assert!(manager.get_rule(&id).is_some());
    }

    #[test]
    fn test_enable_disable_rule() {
        let mut manager = NetworkFirewallManager::new();
        manager.disable_rule("ssh").ok();
        assert!(!manager.get_rule("ssh").unwrap().is_enabled);
        manager.enable_rule("ssh").ok();
        assert!(manager.get_rule("ssh").unwrap().is_enabled);
    }

    #[test]
    fn test_zone_creation() {
        let mut manager = NetworkFirewallManager::new();
        let zone = NetworkFirewallZone::new(
            "home".to_string(),
            "Home".to_string(),
            FirewallAction::Allow,
        );
        manager.add_zone(zone);
        assert!(manager.get_zone("home").is_some());
    }

    #[test]
    fn test_statistics() {
        let manager = NetworkFirewallManager::new();
        let stats = manager.get_statistics();
        assert!(stats.total_rules >= 3);
    }
}
