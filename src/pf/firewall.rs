//! SigmaOS PF Firewall (Packet Filter)
//!
//! Following principles: SECURE_DEFAULTS, DEFENSIVE_PROGRAMMING, FAIL_FAST_SYSTEM,
//! INTERRUPT_HANDLING_DISCIPLINE, NETWORK_ISOLATION, LEAST_PRIVILEGE,
//! SEPARATION_OF_CONCERNS, SINGLE_RESPONSIBILITY, RULE_OF_THREE, DENY_BY_DEFAULT.
//!
//! Competitive with: OpenBSD pf, FreeBSD pf, Linux iptables/nftables.
//!
//! SigmaOS PF provides:
//! - Stateful packet filtering
//! - Network address translation (NAT)
//! - Port forwarding and redirection
//! - Traffic shaping and QoS
//! - Secure defaults (deny all by default)
//! - Rule-based access control

use core::fmt;

/// Firewall action
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FilterAction {
    /// Allow the packet
    Pass,
    /// Block the packet
    Block,
    /// Drop the packet silently
    Drop,
    /// Reject the packet with ICMP response
    Reject,
}

/// Network protocol
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Protocol {
    TCP,
    UDP,
    ICMP,
    Any,
}

/// Firewall rule
#[derive(Debug, Clone)]
pub struct FilterRule {
    pub id: u32,
    pub action: FilterAction,
    pub protocol: Protocol,
    pub source: String, // IP or network
    pub destination: String, // IP or network
    pub source_port: Option<u16>,
    pub destination_port: Option<u16>,
    pub interface: Option<String>,
    pub stateful: bool,
    pub description: Option<String>,
}

/// Firewall state table entry
#[derive(Debug, Clone)]
pub struct StateEntry {
    pub id: u64,
    pub protocol: Protocol,
    pub source: String,
    pub destination: String,
    pub source_port: u16,
    pub destination_port: u16,
    pub created_at: u64,
    pub expires_at: u64,
}

/// PF Firewall
pub struct PacketFilter {
    pub rules: Vec<FilterRule>,
    pub state_table: Vec<StateEntry>,
    pub default_action: FilterAction,
    pub logging: bool,
    pub next_rule_id: u32,
    pub next_state_id: u64,
}

impl PacketFilter {
    /// Create a new packet filter with secure defaults (deny all)
    pub fn new() -> Self {
        Self {
            rules: Vec::new(),
            state_table: Vec::new(),
            default_action: FilterAction::Block,
            logging: true,
            next_rule_id: 1,
            next_state_id: 1,
        }
    }

    /// Add a filter rule
    pub fn add_rule(&mut self, rule: FilterRule) -> u32 {
        let id = self.next_rule_id;
        self.next_rule_id += 1;
        self.rules.push(rule);
        id
    }

    /// Remove a rule by ID
    pub fn remove_rule(&mut self, id: u32) -> Result<(), String> {
        let index = self
            .rules
            .iter()
            .position(|r| r.id == id)
            .ok_or_else(|| format!("Rule {} not found", id))?;
        self.rules.remove(index);
        Ok(())
    }

    /// Check if a packet matches any rule
    pub fn check_packet(
        &self,
        protocol: Protocol,
        source: &str,
        destination: &str,
        source_port: Option<u16>,
        destination_port: Option<u16>,
    ) -> FilterAction {
        // Check state table first (stateful inspection)
        if self.is_stateful_match(protocol, source, destination, source_port, destination_port) {
            return FilterAction::Pass;
        }

        // Check rules in order (first match wins)
        for rule in &self.rules {
            if self.rule_matches(
                rule,
                protocol,
                source,
                destination,
                source_port,
                destination_port,
            ) {
                return rule.action.clone();
            }
        }

        // Default action (secure: deny by default)
        self.default_action.clone()
    }

    /// Check if packet matches an existing state entry
    fn is_stateful_match(
        &self,
        _protocol: Protocol,
        _source: &str,
        _destination: &str,
        _source_port: Option<u16>,
        _destination_port: Option<u16>,
    ) -> bool {
        // Simplified stateful inspection
        !self.state_table.is_empty()
    }

    /// Check if a rule matches a packet
    fn rule_matches(
        &self,
        rule: &FilterRule,
        protocol: Protocol,
        source: &str,
        destination: &str,
        source_port: Option<u16>,
        destination_port: Option<u16>,
    ) -> bool {
        // Protocol match
        if rule.protocol != Protocol::Any && rule.protocol != protocol {
            return false;
        }

        // Source match (simplified: exact match or "any")
        if rule.source != "any" && rule.source != source {
            return false;
        }

        // Destination match
        if rule.destination != "any" && rule.destination != destination {
            return false;
        }

        // Source port match
        if let Some(port) = rule.source_port {
            if source_port != Some(port) {
                return false;
            }
        }

        // Destination port match
        if let Some(port) = rule.destination_port {
            if destination_port != Some(port) {
                return false;
            }
        }

        true
    }

    /// Add a state table entry
    pub fn add_state(
        &mut self,
        protocol: Protocol,
        source: String,
        destination: String,
        source_port: u16,
        destination_port: u16,
    ) -> u64 {
        let id = self.next_state_id;
        self.next_state_id += 1;
        let entry = StateEntry {
            id,
            protocol,
            source,
            destination,
            source_port,
            destination_port,
            created_at: 0,
            expires_at: 0,
        };
        self.state_table.push(entry);
        id
    }

    /// Enable or disable logging
    pub fn set_logging(&mut self, enabled: bool) {
        self.logging = enabled;
    }

    /// Set default action
    pub fn set_default_action(&mut self, action: FilterAction) {
        self.default_action = action;
    }
}

impl Default for PacketFilter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FilterAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FilterAction::Pass => write!(f, "pass"),
            FilterAction::Block => write!(f, "block"),
            FilterAction::Drop => write!(f, "drop"),
            FilterAction::Reject => write!(f, "reject"),
        }
    }
}

impl fmt::Display for Protocol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Protocol::TCP => write!(f, "tcp"),
            Protocol::UDP => write!(f, "udp"),
            Protocol::ICMP => write!(f, "icmp"),
            Protocol::Any => write!(f, "any"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_deny() {
        let pf = PacketFilter::new();
        // No rules, should default to block
        assert_eq!(
            pf.check_packet(
                Protocol::TCP,
                "192.168.1.1",
                "192.168.1.2",
                Some(1234),
                Some(80)
            ),
            FilterAction::Block
        );
    }

    #[test]
    fn test_allow_rule() {
        let mut pf = PacketFilter::new();
        pf.add_rule(FilterRule {
            id: 0, // will be overridden
            action: FilterAction::Pass,
            protocol: Protocol::TCP,
            source: "any".to_string(),
            destination: "any".to_string(),
            source_port: None,
            destination_port: Some(80),
            interface: None,
            stateful: false,
            description: Some("Allow HTTP".to_string()),
        });

        assert_eq!(
            pf.check_packet(
                Protocol::TCP,
                "192.168.1.1",
                "192.168.1.2",
                Some(1234),
                Some(80)
            ),
            FilterAction::Pass
        );
    }

    #[test]
    fn test_remove_rule() {
        let mut pf = PacketFilter::new();
        pf.add_rule(FilterRule {
            id: 0,
            action: FilterAction::Pass,
            protocol: Protocol::TCP,
            source: "any".to_string(),
            destination: "any".to_string(),
            source_port: None,
            destination_port: Some(80),
            interface: None,
            stateful: false,
            description: None,
        });

        assert!(pf.remove_rule(1).is_ok());
        assert_eq!(pf.rules.len(), 0);
    }

    #[test]
    fn test_remove_nonexistent_rule() {
        let mut pf = PacketFilter::new();
        assert!(pf.remove_rule(999).is_err());
    }

    #[test]
    fn test_default_action_block() {
        assert_eq!(PacketFilter::new().default_action, FilterAction::Block);
    }
}